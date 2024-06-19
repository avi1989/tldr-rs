use colored::{ColoredString, Colorize};

use crate::{ElementTheme, Theme};

pub fn get_line_to_render<'a>(line: &'a str, theme: &Theme) -> (&'a str, String) {
    if line.starts_with("- ") {
        let styled_line = get_bullet_line(line, theme);
        return ("bullet", styled_line);
    }

    if line.starts_with("# ") {
        let styled_line = get_header(line, theme);
        return ("header", styled_line);
    }

    if line.starts_with("> ") {
        let styled_line = get_indent_line(line, theme);
        return ("indent", styled_line);
    }

    if line.starts_with("`") && line.ends_with("`") {
        let styled_line = render_code(line, theme);
        return ("code", styled_line);
    }

    return ("", String::from(line));
}

pub fn get_header(line: &str, theme: &Theme) -> String {
    let item_to_print = format!(" {} ", &line[2..]);
    let colored_text = get_colored_text(&item_to_print, &theme.header);

    return format!("\n{}", colored_text.bold());
}

pub fn get_bullet_line(line: &str, theme: &Theme) -> String {
    let mut line_to_render = render_text(line, theme, &theme.list);
    line_to_render = line_to_render.replacen("-", "•", 1);
    return format!("{}", line_to_render);
}

pub fn get_indent_line(line: &str, theme: &Theme) -> String {
    let mut line_to_render = render_text(line, theme, &theme.indents);
    line_to_render = line_to_render.replacen("> ", "│ ", 1);
    return format!("{}", line_to_render);
}

pub fn get_link(line: &str, theme: &Theme) -> String {
    let mut colored_line = get_colored_text(line, &theme.link);
    colored_line = colored_line.underline();

    return format!(
        "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
        line, colored_line
    );
}

pub fn render_text(line: &str, theme: &Theme, _cur_element: &ElementTheme) -> String {
    let mut result = String::new();

    let mut current_block_type: Option<&str> = None;
    let mut current_block_start: Option<usize> = None;
    for (idx, char) in line.chars().enumerate() {
        match (current_block_type, current_block_start) {
            (Some("code"), Some(block_start_idx)) => {
                if char == '`' {
                    let code_block = &line[block_start_idx..idx + 1];
                    let styled_code = render_code(code_block, theme);
                    result.push_str(&styled_code);
                    current_block_start = None;
                    current_block_type = None;
                    continue;
                }
            },
            (Some("link"), Some(block_start_idx)) => {
                if char == '>' {
                    let link_block = &line[block_start_idx + 1..idx];
                    let styled_code = get_link(link_block, theme);
                    result.push_str(&styled_code);
                    current_block_start = None;
                    current_block_type = None;
                }
            },
            _ => {
                if char == '`' {
                    current_block_type = Some("code");
                    current_block_start = Some(idx);
                } else if char == '<' {
                    current_block_type = Some("link");
                    current_block_start = Some(idx);
                } else {
                    result.push(char);
                }
            }
        }
    }

    return result;
}


pub fn render_code(line: &str, theme: &Theme) -> String {
    let len = line.len();
    let mut line_to_render = format!(" {} ", &line[1..len - 1]);
    line_to_render = line_to_render.replace("{{", "").replace("}}", "");

    let styled_line = get_colored_text(&line_to_render, &theme.code_block);

    return format!("{}", styled_line);
}

pub fn get_colored_text(text: &str, color: &ElementTheme) -> ColoredString {
    let mut colored_text = colored::ColoredString::from(text);

    if color.fg.is_some() {
        let fg_color = convert_color(color.fg.as_ref().unwrap());
        colored_text = colored_text.custom_color(fg_color);
    }

    if color.bg.is_some() {
        let bg_color = convert_color(color.bg.as_ref().unwrap());
        colored_text = colored_text.on_custom_color(bg_color);
    }

    return colored_text;
}

pub fn convert_color(color: &crate::Color) -> colored::customcolors::CustomColor {
    return colored::CustomColor {
        r: color.r,
        g: color.g,
        b: color.b,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_header_should_render_header() {
        let theme = &_get_empty_theme();
        let result = get_header("# This is a header", theme);
        insta::assert_debug_snapshot!(result);
    }

    #[test]
    fn get_bullet_line_should_render_bullets() {
        let theme = &_get_empty_theme();
        let result = get_bullet_line("- This is a bullet", theme);
        insta::assert_debug_snapshot!(result); 
    }

    #[test]
    fn get_indent_line_should_render() {
        let theme = &_get_empty_theme();
        let result = get_indent_line("> This is an indent", theme);
        insta::assert_debug_snapshot!(result); 
    }

    #[test]
    fn get_link_should_render() {
        let theme = &_get_empty_theme();
        let result = get_link("<https://insta.rs>", theme);
        insta::assert_debug_snapshot!(result); 
    }

    #[test]
    fn get_code_should_render() {
        let theme = &_get_empty_theme();
        let result = render_code("`https://insta.rs`", theme);
        insta::assert_debug_snapshot!(result);  
    }

    #[test]
    fn get_line_to_render_should_render_header() {
        let theme = &_get_empty_theme();
        let text_to_render = "# This is a test";

        let result = get_line_to_render(text_to_render, theme);
        insta::assert_debug_snapshot!(result); 
    }
    
    #[test]
    fn get_line_to_render_should_render_bullets() {
        let theme = &_get_empty_theme();
        let text_to_render = "- This is a test";

        let result = get_line_to_render(text_to_render, theme);
        insta::assert_debug_snapshot!(result); 
    }

    #[test]
    fn get_line_to_render_should_render_code() {
        let theme = &_get_empty_theme();
        let text_to_render = "`This is a test`";

        let result = get_line_to_render(text_to_render, theme);
        insta::assert_debug_snapshot!(result); 
    }

    #[test]
    fn get_line_to_render_should_render_indents() {
        let theme = &_get_empty_theme();
        let text_to_render = "> This is a test";

        let result = get_line_to_render(text_to_render, theme);
        insta::assert_debug_snapshot!(result); 
    }

    #[test]
    fn render_text_should_render_code_blocks_inline() {
        let theme = &_get_empty_theme();
        let text_to_render = "this is some `embedded code` in text";

        let result = render_text(text_to_render, theme, &theme.indents);
        insta::assert_debug_snapshot!(result);
    }

    #[test]
    fn render_text_should_render_links_inline() {
        let theme = &_get_empty_theme();
        let text_to_render = "this is some <embedded link> in text";

        let result = render_text(text_to_render, theme, &theme.indents);
        insta::assert_debug_snapshot!(result);
    }

    fn _get_empty_theme() -> Theme {
        return Theme {
            header: ElementTheme::new(None, None),
            code_block: ElementTheme::new(Some("#FF6060"), Some("#303030")),
            indents: ElementTheme::new(Some("#555"), None),
            link: ElementTheme::new(Some("#008787"), None),
            list: ElementTheme::new(None, None),
        };
    }
}