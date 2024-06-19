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
    /// let a = ElementTheme::new("#000", "#FFF");
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

/// A simple struct to represent the color in the RGB format.
pub struct Color {
    /// A number between 0 and 255 which represents Red spectrum.
    pub r: u8,
    /// A number between 0 and 255 which represents Green spectrum.
    pub g: u8,
    /// A number between 0 and 255 which represents Blue spectrum.
    pub b: u8,
}

impl Color {
    /// Creates a new instance of Color with the hex code
    /// ```rust
    /// use tldr::markdown::color::Color;
    /// let white = Color::new("#FFF");
    /// assert_eq!(white.r, 255)
    /// ```
    pub fn new(hex_color: &str) -> Self {
        let mut color_code = hex_color.to_string();

        color_code.retain(|c| !r#"#"#.contains(c));

        let r_str = format!("{}{}", &color_code[0..1], &color_code[0..1]);
        let g_str = format!("{}{}", &color_code[1..2], &color_code[1..2]);
        let b_str = format!("{}{}", &color_code[2..3], &color_code[2..3]);

        if color_code.len() == 3 {
            let r = u8::from_str_radix(&r_str, 16);
            let g = u8::from_str_radix(&g_str, 16);
            let b = u8::from_str_radix(&b_str, 16);

            if r.is_err() || g.is_err() || b.is_err() {
                panic!("Invalid Hex color code {}", hex_color);
            }

            return Self {
                r: r.unwrap(),
                g: g.unwrap(),
                b: b.unwrap(),
            };
        } else if color_code.len() == 6 {
            let r = u8::from_str_radix(&color_code[0..2], 16);
            let g = u8::from_str_radix(&color_code[2..4], 16);
            let b = u8::from_str_radix(&color_code[4..6], 16);

            return Self {
                r: r.unwrap(),
                g: g.unwrap(),
                b: b.unwrap(),
            };
        } else {
            panic!("Invalid length of codes");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_000000_to_black() {
        let color = Color::new("#000000");
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn should_convert_000_to_black() {
        let color = Color::new("#000");
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn should_convert_ffffff_to_white() {
        let color = Color::new("#FFFFFF");
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
    }

    #[test]
    fn should_convert_fff_to_white() {
        let color = Color::new("#FFF");
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
    }

    #[test]
    #[should_panic]
    fn input_codes_cannot_be_1_characters() {
        let _color = Color::new("0");
    }

    #[test]
    #[should_panic]
    fn input_codes_cannot_be_1_characters_with_hash() {
        let _color = Color::new("#0");
    }

    #[test]
    #[should_panic]
    fn input_codes_cannot_be_2_characters() {
        let _color = Color::new("00");
    }

    #[test]
    #[should_panic]
    fn input_codes_cannot_be_2_characters_with_hash() {
        let _color = Color::new("#00");
    }

    #[test]
    #[should_panic]
    fn input_codes_cannot_be_4_characters() {
        let _color = Color::new("0000");
    }

    #[test]
    #[should_panic]
    fn input_codes_cannot_be_4_characters_with_hash() {
        let _color = Color::new("#0000");
    }

    #[test]
    #[should_panic]
    fn input_codes_cannot_be_5_characters() {
        let _color = Color::new("00000");
    }

    #[test]
    #[should_panic]
    fn input_codes_cannot_be_5_characters_with_hash() {
        let _color = Color::new("#00000");
    }

    #[test]
    #[should_panic]
    fn input_codes_cannot_be_7_characters() {
        let _color = Color::new("0000000");
    }

    #[test]
    #[should_panic]
    fn input_codes_cannot_be_7_characters_with_hash() {
        let _color = Color::new("#0000000");
    }

    #[test]
    fn input_codes_can_be_3_digits() {
        let _color = Color::new("000");
        // No Panic
    }

    #[test]
    fn input_codes_can_be_3_digits_with_hash() {
        let _color = Color::new("#000");
        // No Panic
    }

    #[test]
    fn input_codes_can_be_6_digits() {
        let _color = Color::new("000000");
    }

    #[test]
    fn input_codes_can_be_6_digits_with_hash() {
        let _color = Color::new("#000000");
    }

    #[test]
    #[should_panic]
    fn should_fail_for_invalid_rgb_code_in_red() {
        Color::new("#GG0011");
    }

    #[test]
    #[should_panic]
    fn should_fail_for_invalid_rgb_code_in_green() {
        Color::new("#00GG11");
    }

    #[test]
    #[should_panic]
    fn should_fail_for_invalid_rgb_code_in_blue() {
        Color::new("#00AAZZ");
    }
}