# dioxus-terminal
<hr/>
<img width="1319" height="477" alt="Screenshot 2026-02-12 at 03 58 28" src="https://github.com/user-attachments/assets/794fcff9-8307-4306-9283-8c998d816e24" />

<hr/>

Terminal emulator widget for [Dioxus](https://dioxuslabs.com/) desktop applications.

Built on [alacritty_terminal](https://crates.io/crates/alacritty_terminal) for terminal emulation and [portable-pty](https://crates.io/crates/portable-pty) for cross-platform PTY support.

## Features

- Terminal emulation (VT100/xterm compatible)
- ANSI color support (16 and 256 colors)
- Keyboard input
- Customizable themes

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-terminal = "0.1"
```

## Usage

```rust
use dioxus::prelude::*;
use dioxus_terminal::{Terminal, Theme};

fn app() -> Element {
    rsx! {
        // Simple: run a shell command
        Terminal {
            shell: "ls -la",
        }

        // With a theme
        Terminal {
            shell: "htop",
            theme: Theme::nord(),
        }
    }
}
```

## Themes

Built-in themes:

| Theme | Description |
|-------|-------------|
| `Theme::dark()` | Default - black background (default) |
| `Theme::zinc()` | Tailwind zinc-900/zinc-200 |
| `Theme::slate()` | Tailwind slate-900/slate-200 |
| `Theme::nord()` | Nord color scheme |
| `Theme::dracula()` | Dracula color scheme |
| `Theme::monokai()` | Monokai color scheme |
| `Theme::gruvbox()` | Gruvbox dark |
| `Theme::catppuccin()` | Catppuccin Mocha |
| `Theme::one_dark()` | Atom One Dark |
| `Theme::tokyo_night()` | Tokyo Night |
| `Theme::github_dark()` | GitHub Dark |
| `Theme::solarized_dark()` | Solarized Dark |
| `Theme::solarized_light()` | Solarized Light |
| `Theme::light()` | Light theme |

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `shell` | `String` | `""` | Shell command (parsed via `sh -c`) |
| `command` | `String` | `$SHELL` | Command to run |
| `args` | `Vec<String>` | `[]` | Command arguments |
| `rows` | `u16` | `24` | Terminal rows |
| `cols` | `u16` | `120` | Terminal columns |
| `theme` | `Theme` | `Theme::dark()` | Color theme |
| `background` | `Option<Color>` | `None` | Override theme background |
| `foreground` | `Option<Color>` | `None` | Override theme foreground |
| `font_size` | `u16` | `13` | Font size in pixels |
| `font_family` | `String` | JetBrains Mono + fallbacks | Font family |
| `class` | `String` | `""` | CSS class for container |

## Customization

```rust
use dioxus_terminal::{Terminal, Theme, Color};

rsx! {
    // Use a preset theme
    Terminal {
        command: "/bin/zsh",
        args: vec!["-l".to_string()],
        theme: Theme::catppuccin(),
        rows: 30,
        cols: 120,
    }

    // Override theme colors
    Terminal {
        shell: "bash",
        theme: Theme::dark(),
        background: Color::new(30, 30, 30),  // overrides theme bg
        foreground: Color::new(200, 200, 200),
    }

    // Custom theme with font settings
    Terminal {
        shell: "fish",
        theme: Theme::new(
            Color::new(20, 20, 30),   // background
            Color::new(200, 200, 220), // foreground
        ),
        font_size: 14,
        font_family: "Fira Code, monospace".to_string(),
    }
}
```

## License

MIT
