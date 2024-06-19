/// A module to assist with setting colors.
pub mod color;

pub use color::Color;
/// Properties required to theme the element.
pub struct ElementTheme {
    /// Foreground color
    pub fg: Option<Color>,

    /// Background color
    pub bg: Option<Color>,
}

/// A top level struct that contains all the elements and their styles.
pub struct Theme {
    /// The theme for all header elements. 
    /// 
    /// Header elements start with #, ##, ### or ####)
    pub header: ElementTheme,

    /// The theme for code blocks.
    /// Code blocks are elements that are surrounded by ``
    pub code_block: ElementTheme,

    /// The theme for indentations
    /// Indent Elements start with >
    pub indents: ElementTheme,

    /// The theme for links.
    /// Links are surrounded by < >
    pub link: ElementTheme,

    /// The theme for lists
    pub list: ElementTheme,
}

impl ElementTheme {
    /// Creates a new instance of ElementTheme.
    /// 
    /// Example
    /// ```rust
    /// let a = markdown::ElementTheme::new(Some("#000"), Some("#FFF"));
    /// ```
    pub fn new(fg: Option<&str>, bg: Option<&str>) -> Self {
        let bg_color = match bg {
            Some(x) => Some(Color::new(x)),
            None => None,
        };

        let fg_color = match fg {
            Some(x) => Some(Color::new(x)),
            None => None,
        };

        return Self {
            fg: fg_color,
            bg: bg_color,
        };
    }
}

/// Gets the default theme for the library.
pub fn get_default_theme() -> Theme {
    let theme = Theme {
        header: ElementTheme::new(None, Some("#6155FB")),
        code_block: ElementTheme::new(Some("#FF6060"), Some("#303030")),
        indents: ElementTheme::new(Some("#555"), None),
        link: ElementTheme::new(Some("#008787"), None),
        list: ElementTheme::new(None, None),
    };

    return theme;
}
