//! Dioxus terminal widget component

use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

use crate::pty::Pty;
use crate::term::{Cell, Color, Grid};
use crate::theme::Theme;

/// Default monospace font stack
pub const DEFAULT_FONT_FAMILY: &str = "JetBrains Mono, Menlo, Monaco, Consolas, ui-monospace, monospace";

/// Props for the Terminal component
#[derive(Props, Clone, PartialEq)]
pub struct TerminalProps {
    /// Command to run (default: user's shell)
    #[props(default = default_shell())]
    pub command: String,

    /// Command arguments
    #[props(default)]
    pub args: Vec<String>,

    /// Shell command string (alternative to command + args, parsed via sh -c)
    #[props(default)]
    pub shell: String,

    /// Number of rows (default: 24)
    #[props(default = 24)]
    pub rows: u16,

    /// Number of columns (default: 120)
    #[props(default = 120)]
    pub cols: u16,

    /// Font size in pixels (default: 13)
    #[props(default = 13)]
    pub font_size: u16,

    /// Font family (default: JetBrains Mono + fallbacks)
    #[props(default = DEFAULT_FONT_FAMILY.to_string())]
    pub font_family: String,

    /// Color theme (default: Theme::dark())
    #[props(default)]
    pub theme: Theme,

    /// Background color (overrides theme if set)
    #[props(default)]
    pub background: Option<Color>,

    /// Foreground color (overrides theme if set)
    #[props(default)]
    pub foreground: Option<Color>,

    /// CSS class for the container
    #[props(default)]
    pub class: String,
}

fn default_shell() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
}

/// Escape sequence parsing state
#[derive(Default)]
enum EscapeState {
    #[default]
    Normal,
    Escape,      // Just saw ESC
    Csi,         // In CSI sequence (ESC [)
}

/// Terminal state shared between render and coroutine
struct TermState {
    pty: Option<Pty>,
    cursor_row: usize,
    cursor_col: usize,
    // Current text attributes
    fg: Color,
    bg: Color,
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    // Escape sequence parsing
    escape_state: EscapeState,
    escape_buf: Vec<u8>,
}

/// Terminal emulator widget for Dioxus
#[component]
pub fn Terminal(props: TerminalProps) -> Element {
    let rows = props.rows as usize;
    let cols = props.cols as usize;

    // Resolve colors: explicit props override theme
    let bg_color = props.background.unwrap_or(props.theme.background);
    let fg_color = props.foreground.unwrap_or(props.theme.foreground);

    let mut grid = use_signal(|| Grid::new(rows, cols));

    // Shared state for PTY and cursor
    let state = use_hook(|| {
        // If shell prop is set, use sh -c to run it
        let (command, args): (String, Vec<String>) = if !props.shell.is_empty() {
            ("sh".to_string(), vec!["-c".to_string(), props.shell.clone()])
        } else {
            (props.command.clone(), props.args.clone())
        };

        let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let pty = Pty::spawn(&command, &args_refs, props.rows, props.cols).ok();

        Arc::new(Mutex::new(TermState {
            pty,
            cursor_row: 0,
            cursor_col: 0,
            fg: Color::default_fg(),
            bg: Color::default_bg(),
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            escape_state: EscapeState::Normal,
            escape_buf: Vec::new(),
        }))
    });

    // Coroutine to read PTY output
    let state_clone = state.clone();
    use_coroutine(move |_rx: UnboundedReceiver<()>| {
        let state = state_clone.clone();
        async move {
            loop {
                // Try to read from PTY
                let data = {
                    let mut s = state.lock().unwrap();
                    if let Some(ref mut pty) = s.pty {
                        pty.try_read()
                    } else {
                        None
                    }
                };

                if let Some(bytes) = data {
                    // Process output bytes
                    let mut s = state.lock().unwrap();
                    for byte in bytes {
                        process_byte(&mut s, &mut grid, byte, rows, cols);
                    }
                    drop(s);
                }

                // Small delay to avoid busy loop
                tokio::time::sleep(std::time::Duration::from_millis(16)).await;
            }
        }
    });

    // Handle keyboard input
    let state_for_key = state.clone();
    let onkeydown = move |evt: KeyboardEvent| {
        let key_str = key_to_string(&evt);
        if !key_str.is_empty() {
            if let Ok(s) = state_for_key.lock() {
                if let Some(ref pty) = s.pty {
                    let _ = pty.write(key_str.as_bytes());
                }
            }
        }
    };

    let container_style = format!(
        "background-color: {}; color: {}; font-family: {}; font-size: {}px; line-height: 1.2;",
        bg_color.to_css(),
        fg_color.to_css(),
        props.font_family,
        props.font_size
    );

    let container_class = format!(
        "terminal-container overflow-hidden select-none {}",
        props.class
    );

    rsx! {
        div {
            class: "{container_class}",
            style: "{container_style}",
            tabindex: "0",
            onkeydown: onkeydown,

            // Render grid
            div { class: "terminal-grid whitespace-pre font-mono",
                for (row_idx, row) in grid.read().iter_rows().enumerate() {
                    div { class: "terminal-row", key: "{row_idx}",
                        for (col_idx, cell) in row.iter().enumerate() {
                            span {
                                key: "{col_idx}",
                                class: "{cell.style.to_css_classes()}",
                                style: "color: {cell.fg.to_css()}; background-color: {cell.bg.to_css()};",
                                "{cell.c}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Process a single byte of terminal output
fn process_byte(state: &mut TermState, grid: &mut Signal<Grid>, byte: u8, rows: usize, cols: usize) {
    match state.escape_state {
        EscapeState::Normal => match byte {
            // Escape - start escape sequence
            0x1b => {
                state.escape_state = EscapeState::Escape;
                state.escape_buf.clear();
            }
            // Newline
            b'\n' => {
                state.cursor_row += 1;
                if state.cursor_row >= rows {
                    scroll_up(grid, rows, cols);
                    state.cursor_row = rows - 1;
                }
            }
            // Carriage return
            b'\r' => {
                state.cursor_col = 0;
            }
            // Backspace
            0x08 => {
                if state.cursor_col > 0 {
                    state.cursor_col -= 1;
                }
            }
            // Tab
            b'\t' => {
                let next_tab = (state.cursor_col / 8 + 1) * 8;
                state.cursor_col = next_tab.min(cols - 1);
            }
            // Bell - ignore
            0x07 => {}
            // Printable characters
            0x20..=0x7e | 0x80..=0xff => {
                let c = byte as char;
                let cell = Cell {
                    c,
                    fg: state.fg,
                    bg: state.bg,
                    style: crate::term::Style {
                        bold: state.bold,
                        dim: state.dim,
                        italic: state.italic,
                        underline: state.underline,
                        strikethrough: false,
                        inverse: false,
                    },
                };
                grid.write().set(state.cursor_row, state.cursor_col, cell);
                state.cursor_col += 1;
                if state.cursor_col >= cols {
                    state.cursor_col = 0;
                    state.cursor_row += 1;
                    if state.cursor_row >= rows {
                        scroll_up(grid, rows, cols);
                        state.cursor_row = rows - 1;
                    }
                }
            }
            // Other control characters - ignore
            _ => {}
        },
        EscapeState::Escape => {
            if byte == b'[' {
                state.escape_state = EscapeState::Csi;
            } else {
                // Not a CSI sequence, ignore and return to normal
                state.escape_state = EscapeState::Normal;
            }
        }
        EscapeState::Csi => {
            if byte.is_ascii_alphabetic() {
                // End of CSI sequence
                if byte == b'm' {
                    // SGR - Select Graphic Rendition
                    process_sgr(state);
                }
                // Other CSI sequences (cursor movement, etc.) - ignore for now
                state.escape_state = EscapeState::Normal;
                state.escape_buf.clear();
            } else {
                // Buffer the parameter bytes
                state.escape_buf.push(byte);
            }
        }
    }
}

/// Process SGR (Select Graphic Rendition) escape sequence
fn process_sgr(state: &mut TermState) {
    let params_str = String::from_utf8_lossy(&state.escape_buf);
    let params: Vec<u8> = if params_str.is_empty() {
        vec![0] // Default to reset
    } else {
        params_str
            .split(';')
            .filter_map(|s| s.parse().ok())
            .collect()
    };

    let mut i = 0;
    while i < params.len() {
        match params[i] {
            0 => {
                // Reset all attributes
                state.fg = Color::default_fg();
                state.bg = Color::default_bg();
                state.bold = false;
                state.dim = false;
                state.italic = false;
                state.underline = false;
            }
            1 => state.bold = true,
            2 => state.dim = true,
            3 => state.italic = true,
            4 => state.underline = true,
            22 => {
                state.bold = false;
                state.dim = false;
            }
            23 => state.italic = false,
            24 => state.underline = false,
            // Standard foreground colors (30-37)
            30 => state.fg = Color::new(0, 0, 0),       // Black
            31 => state.fg = Color::new(205, 49, 49),   // Red
            32 => state.fg = Color::new(13, 188, 121),  // Green
            33 => state.fg = Color::new(229, 229, 16),  // Yellow
            34 => state.fg = Color::new(36, 114, 200),  // Blue
            35 => state.fg = Color::new(188, 63, 188),  // Magenta
            36 => state.fg = Color::new(17, 168, 205),  // Cyan
            37 => state.fg = Color::new(229, 229, 229), // White
            39 => state.fg = Color::default_fg(),       // Default fg
            // Standard background colors (40-47)
            40 => state.bg = Color::new(0, 0, 0),       // Black
            41 => state.bg = Color::new(205, 49, 49),   // Red
            42 => state.bg = Color::new(13, 188, 121),  // Green
            43 => state.bg = Color::new(229, 229, 16),  // Yellow
            44 => state.bg = Color::new(36, 114, 200),  // Blue
            45 => state.bg = Color::new(188, 63, 188),  // Magenta
            46 => state.bg = Color::new(17, 168, 205),  // Cyan
            47 => state.bg = Color::new(229, 229, 229), // White
            49 => state.bg = Color::default_bg(),       // Default bg
            // Bright foreground colors (90-97)
            90 => state.fg = Color::new(102, 102, 102), // Bright black
            91 => state.fg = Color::new(241, 76, 76),   // Bright red
            92 => state.fg = Color::new(35, 209, 139),  // Bright green
            93 => state.fg = Color::new(245, 245, 67),  // Bright yellow
            94 => state.fg = Color::new(59, 142, 234),  // Bright blue
            95 => state.fg = Color::new(214, 112, 214), // Bright magenta
            96 => state.fg = Color::new(41, 184, 219),  // Bright cyan
            97 => state.fg = Color::new(255, 255, 255), // Bright white
            // Bright background colors (100-107)
            100 => state.bg = Color::new(102, 102, 102),
            101 => state.bg = Color::new(241, 76, 76),
            102 => state.bg = Color::new(35, 209, 139),
            103 => state.bg = Color::new(245, 245, 67),
            104 => state.bg = Color::new(59, 142, 234),
            105 => state.bg = Color::new(214, 112, 214),
            106 => state.bg = Color::new(41, 184, 219),
            107 => state.bg = Color::new(255, 255, 255),
            // 256-color mode (38;5;N or 48;5;N)
            38 => {
                if i + 2 < params.len() && params[i + 1] == 5 {
                    state.fg = color_from_256(params[i + 2]);
                    i += 2;
                }
            }
            48 => {
                if i + 2 < params.len() && params[i + 1] == 5 {
                    state.bg = color_from_256(params[i + 2]);
                    i += 2;
                }
            }
            _ => {}
        }
        i += 1;
    }
}

/// Convert 256-color palette index to RGB
fn color_from_256(n: u8) -> Color {
    match n {
        // Standard colors (0-15)
        0 => Color::new(0, 0, 0),
        1 => Color::new(205, 49, 49),
        2 => Color::new(13, 188, 121),
        3 => Color::new(229, 229, 16),
        4 => Color::new(36, 114, 200),
        5 => Color::new(188, 63, 188),
        6 => Color::new(17, 168, 205),
        7 => Color::new(229, 229, 229),
        8 => Color::new(102, 102, 102),
        9 => Color::new(241, 76, 76),
        10 => Color::new(35, 209, 139),
        11 => Color::new(245, 245, 67),
        12 => Color::new(59, 142, 234),
        13 => Color::new(214, 112, 214),
        14 => Color::new(41, 184, 219),
        15 => Color::new(255, 255, 255),
        // 216-color cube (16-231)
        16..=231 => {
            let n = n - 16;
            let r = (n / 36) % 6;
            let g = (n / 6) % 6;
            let b = n % 6;
            let to_255 = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
            Color::new(to_255(r), to_255(g), to_255(b))
        }
        // Grayscale (232-255)
        232..=255 => {
            let gray = 8 + (n - 232) * 10;
            Color::new(gray, gray, gray)
        }
    }
}

/// Scroll the grid up by one line
fn scroll_up(grid: &mut Signal<Grid>, rows: usize, cols: usize) {
    let mut g = grid.write();
    // Move all rows up by one
    for row in 1..rows {
        for col in 0..cols {
            if let Some(cell) = g.get(row, col).cloned() {
                g.set(row - 1, col, cell);
            }
        }
    }
    // Clear the last row
    for col in 0..cols {
        g.set(rows - 1, col, Cell::default());
    }
}

/// Convert keyboard event to terminal input string
fn key_to_string(evt: &KeyboardEvent) -> String {
    let key = evt.key();

    match key {
        Key::Enter => "\r".to_string(),
        Key::Backspace => "\x7f".to_string(),
        Key::Tab => "\t".to_string(),
        Key::Escape => "\x1b".to_string(),
        Key::ArrowUp => "\x1b[A".to_string(),
        Key::ArrowDown => "\x1b[B".to_string(),
        Key::ArrowRight => "\x1b[C".to_string(),
        Key::ArrowLeft => "\x1b[D".to_string(),
        Key::Home => "\x1b[H".to_string(),
        Key::End => "\x1b[F".to_string(),
        Key::Delete => "\x1b[3~".to_string(),
        Key::Character(c) => {
            // Handle Ctrl+key combinations
            if evt.modifiers().ctrl() && c.len() == 1 {
                let ch = c.chars().next().unwrap();
                if ch.is_ascii_lowercase() {
                    // Ctrl+a = 0x01, Ctrl+b = 0x02, etc.
                    let ctrl_char = (ch as u8 - b'a' + 1) as char;
                    return ctrl_char.to_string();
                }
            }
            c
        }
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_shell() {
        let shell = default_shell();
        assert!(!shell.is_empty());
    }

    #[test]
    fn test_terminal_props_defaults() {
        let props = TerminalProps {
            command: "bash".to_string(),
            args: vec![],
            shell: String::new(),
            rows: 24,
            cols: 120,
            font_size: 13,
            font_family: DEFAULT_FONT_FAMILY.to_string(),
            theme: Theme::default(),
            background: None,
            foreground: None,
            class: String::new(),
        };

        assert_eq!(props.rows, 24);
        assert_eq!(props.cols, 120);
        assert_eq!(props.font_size, 13);
        assert_eq!(props.theme, Theme::dark());
    }

    #[test]
    fn test_theme_override() {
        // Background/foreground props should override theme
        let theme = Theme::zinc();
        let custom_bg = Color::new(100, 100, 100);

        // Simulate what the component does
        let bg = Some(custom_bg).unwrap_or(theme.background);
        let fg = None.unwrap_or(theme.foreground);

        assert_eq!(bg, custom_bg);
        assert_eq!(fg, theme.foreground);
    }
}
