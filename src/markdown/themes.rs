use super::color::{self, Color};

pub struct ElementColors {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

pub struct Theme {
    pub header: ElementColors,
    pub code_block: ElementColors,
    pub indents: ElementColors,
    pub link: ElementColors,
    pub list: ElementColors,
}

impl ElementColors {
    pub fn new(fg: Option<&str>, bg: Option<&str>) -> Self {
        let bg_color = match bg {
            Some(x) => Some(color::Color::new(x)),
            None => None,
        };

        let fg_color = match fg {
            Some(x) => Some(color::Color::new(x)),
            None => None,
        };

        return Self {
            fg: fg_color,
            bg: bg_color,
        };
    }
}

pub fn get_theme() -> Theme {
    let theme = Theme {
        header: ElementColors::new(None, Some("#6155FB")),
        code_block: ElementColors::new(Some("#FF6060"), Some("#303030")),
        indents: ElementColors::new(Some("#555"), None),
        link: ElementColors::new(Some("#008787"), None),
        list: ElementColors::new(None, None),
    };

    return theme;
}
