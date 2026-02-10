# dioxus-terminal

Terminal emulator widget for [Dioxus](https://dioxuslabs.com/) desktop applications.

Built on [alacritty_terminal](https://crates.io/crates/alacritty_terminal) for terminal emulation and [portable-pty](https://crates.io/crates/portable-pty) for cross-platform PTY support.

## Features

- Full terminal emulation (VT100/xterm compatible)
- ANSI color support (16, 256, and true color)
- Keyboard and mouse input
- Scrollback buffer
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
    }

    // Custom theme
    Terminal {
        shell: "fish",
        theme: Theme::new(
            Color::new(20, 20, 30),   // background
            Color::new(200, 200, 220), // foreground
        ),
    }
}
```

## License

MIT
