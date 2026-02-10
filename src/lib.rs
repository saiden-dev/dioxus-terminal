//! # dioxus-terminal
//!
//! Terminal emulator widget for Dioxus desktop applications.
//!
//! Built on top of `alacritty_terminal` for terminal emulation and
//! `portable-pty` for cross-platform PTY support.
//!
//! ## Features
//!
//! - Full terminal emulation (VT100/xterm compatible)
//! - ANSI color support (16, 256, and true color)
//! - Keyboard and mouse input
//! - Scrollback buffer
//! - Copy/paste support
//! - Customizable themes
//!
//! ## Example
//!
//! ```ignore
//! use dioxus::prelude::*;
//! use dioxus_terminal::{Terminal, Theme};
//!
//! fn app() -> Element {
//!     rsx! {
//!         // Simple: just a shell command
//!         Terminal {
//!             shell: "ls -la",
//!         }
//!
//!         // With theme
//!         Terminal {
//!             shell: "htop",
//!             theme: Theme::nord(),
//!         }
//!
//!         // Full control
//!         Terminal {
//!             command: "ssh",
//!             args: vec!["user@host".to_string()],
//!             theme: Theme::zinc(),
//!             rows: 30,
//!             cols: 120,
//!         }
//!     }
//! }
//! ```

mod error;
mod pty;
mod term;
mod theme;
mod widget;

pub use error::Error;
pub use pty::Pty;
pub use term::{Cell, Color, Grid, Style};
pub use theme::Theme;
pub use widget::{Terminal, TerminalProps, DEFAULT_FONT_FAMILY};

/// Result type for dioxus-terminal operations
pub type Result<T> = std::result::Result<T, Error>;
