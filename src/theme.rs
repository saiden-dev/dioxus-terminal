//! Terminal color themes

use crate::term::Color;

/// Terminal color theme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    /// Background color
    pub background: Color,
    /// Foreground (text) color
    pub foreground: Color,
    /// Cursor color (defaults to foreground if not set)
    pub cursor: Option<Color>,
    /// Selection background color
    pub selection: Option<Color>,
}

impl Theme {
    /// Create a custom theme
    #[must_use]
    pub const fn new(background: Color, foreground: Color) -> Self {
        Self {
            background,
            foreground,
            cursor: None,
            selection: None,
        }
    }

    /// Dark theme (default) - black background, light gray text
    #[must_use]
    pub const fn dark() -> Self {
        Self::new(Color::new(0, 0, 0), Color::new(204, 204, 204))
    }

    /// Zinc theme - Tailwind zinc-900/zinc-200
    #[must_use]
    pub const fn zinc() -> Self {
        Self::new(Color::new(24, 24, 27), Color::new(228, 228, 231))
    }

    /// Slate theme - Tailwind slate-900/slate-200
    #[must_use]
    pub const fn slate() -> Self {
        Self::new(Color::new(15, 23, 42), Color::new(226, 232, 240))
    }

    /// Nord theme - polar night background
    #[must_use]
    pub const fn nord() -> Self {
        Self::new(Color::new(46, 52, 64), Color::new(216, 222, 233))
    }

    /// Dracula theme
    #[must_use]
    pub const fn dracula() -> Self {
        Self::new(Color::new(40, 42, 54), Color::new(248, 248, 242))
    }

    /// Monokai theme
    #[must_use]
    pub const fn monokai() -> Self {
        Self::new(Color::new(39, 40, 34), Color::new(248, 248, 242))
    }

    /// Solarized Dark theme
    #[must_use]
    pub const fn solarized_dark() -> Self {
        Self::new(Color::new(0, 43, 54), Color::new(131, 148, 150))
    }

    /// Solarized Light theme
    #[must_use]
    pub const fn solarized_light() -> Self {
        Self::new(Color::new(253, 246, 227), Color::new(101, 123, 131))
    }

    /// Light theme - white background, dark text
    #[must_use]
    pub const fn light() -> Self {
        Self::new(Color::new(255, 255, 255), Color::new(30, 30, 30))
    }

    /// GitHub Dark theme
    #[must_use]
    pub const fn github_dark() -> Self {
        Self::new(Color::new(13, 17, 23), Color::new(201, 209, 217))
    }

    /// Tokyo Night theme
    #[must_use]
    pub const fn tokyo_night() -> Self {
        Self::new(Color::new(26, 27, 38), Color::new(169, 177, 214))
    }

    /// Catppuccin Mocha theme
    #[must_use]
    pub const fn catppuccin() -> Self {
        Self::new(Color::new(30, 30, 46), Color::new(205, 214, 244))
    }

    /// One Dark theme (Atom)
    #[must_use]
    pub const fn one_dark() -> Self {
        Self::new(Color::new(40, 44, 52), Color::new(171, 178, 191))
    }

    /// Gruvbox Dark theme
    #[must_use]
    pub const fn gruvbox() -> Self {
        Self::new(Color::new(40, 40, 40), Color::new(235, 219, 178))
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_default_is_dark() {
        assert_eq!(Theme::default(), Theme::dark());
    }

    #[test]
    fn test_theme_zinc() {
        let theme = Theme::zinc();
        assert_eq!(theme.background, Color::new(24, 24, 27));
        assert_eq!(theme.foreground, Color::new(228, 228, 231));
    }

    #[test]
    fn test_theme_custom() {
        let theme = Theme::new(Color::new(10, 20, 30), Color::new(200, 210, 220));
        assert_eq!(theme.background.r, 10);
        assert_eq!(theme.foreground.r, 200);
    }
}
