#![warn(missing_docs)]

//! A cross platform library to render markdown on the terminal.

use colored::{ColoredString, Colorize};
use std::io::BufRead;
use std::{
    fs::File,
    io::{self},
    path::PathBuf,
};

use themes::{ElementTheme, Theme, color::Color};

/// A module that contains the structs and functions required to theme the markdown.
pub mod themes;

/// Renders the provided file in markdown.
pub fn render_file(file_path: &PathBuf, theme: &Theme) {
    let file = File::open(file_path).unwrap();
    let lines = io::BufReader::new(file).lines();

    for line in lines.flatten() {
        let (_, l_text) = render_line(&line, theme);
        println!(" {}", l_text);
    }
    println!("\n");
}

fn render_line<'a>(line: &'a str, theme: &Theme) -> (&'a str, String) {
    if line.starts_with("- ") {
        let styled_line = get_bullet_line(line, theme);
        return ("bullet", styled_line);
    }

    if line.starts_with("# ") {
        let styled_line = render_header(line, theme);
        return ("header", styled_line);
    }

    if line.starts_with("> ") {
        let styled_line = render_indent(line, theme);
        return ("indent", styled_line);
    }

    if line.starts_with("`") && line.ends_with("`") {
        let styled_line = render_code(line, theme);
        return ("code", styled_line);
    }

    return ("", String::from(line));
}

fn render_header(line: &str, theme: &Theme) -> String {
    let item_to_print = format!(" {} ", &line[2..]);
    let colored_text = get_colored_text(&item_to_print, &theme.header);

    return format!(
        "\n{}",
        colored_text.bold()
    );
}

fn get_bullet_line(line: &str, theme: &Theme) -> String {
    let mut line_to_render = render_text(line, theme, &theme.list);
    line_to_render = line_to_render.replacen("-", "•", 1);
    return format!("{}", line_to_render);
}

fn render_indent(line: &str, theme: &Theme) -> String {
    let mut line_to_render = render_text(line, theme, &theme.indents);
    line_to_render = line_to_render.replacen("> ", "│ ", 1);
    return format!("{}", line_to_render);
}

fn render_text(line: &str, theme: &Theme, _cur_element: &ElementTheme) -> String {
    let mut result = String::new();

    let mut current_block_type: Option<&str> = None;
    let mut current_block_start: Option<usize> = None;
    for (idx, char) in line.chars().enumerate() {
        if current_block_type == Some("code") {
            if char == '`' {
                let code_block = &line[current_block_start.unwrap()..idx + 1];
                let styled_code = render_code(code_block, theme);
                result.push_str(&styled_code);
                current_block_start = None;
                current_block_type = None;
                continue;
            }
        } else if current_block_type == Some("link") {
            if char == '>' {
                let link_block = &line[current_block_start.unwrap() + 1..idx];
                let styled_code = render_link(link_block, theme);
                result.push_str(&styled_code);
                current_block_start = None;
                current_block_type = None;
            }
        } else if current_block_type.is_none() {
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

    return result;
}

fn render_link(line: &str, theme: &Theme) -> String {
    let mut colored_line = get_colored_text(line, &theme.link);
    colored_line = colored_line.underline();
    
    return format!(
        "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
        line, colored_line
    );
}

fn render_code(line: &str, theme: &Theme) -> String {
    let len = line.len();
    let mut line_to_render = format!(" {} ", &line[1..len - 1]);
    line_to_render = line_to_render.replace("{{", "").replace("}}", "");

    let styled_line = get_colored_text(&line_to_render, &theme.code_block);

    return format!("{}", styled_line);
}

fn get_colored_text(text: &str, color: &ElementTheme) -> ColoredString {
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

fn convert_color(color: &Color) -> colored::customcolors::CustomColor {
    return colored::CustomColor {
        r: color.r,
        g: color.g,
        b: color.b,
    };
}