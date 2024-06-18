use colored::Colorize;
use std::io::BufRead;
use std::{
    fs::File,
    io::{self},
    path::PathBuf,
};

pub mod color;

pub fn render_file(file_path: &PathBuf) {
    let file = File::open(file_path).unwrap();
    let lines = io::BufReader::new(file).lines();

    for line in lines.flatten() {
        let (_, l_text) = render_line(&line);
        println!(" {}", l_text);
    }
    println!("\n");
}

pub fn render_line(line: &str) -> (&str, String) {
    if line.starts_with("- ") {
        let styled_line = get_bullet_line(line);
        return ("bullet", styled_line);
    }

    if line.starts_with("# ") {
        let styled_line = render_header(line);
        return ("header", styled_line);
    }

    if line.starts_with("> ") {
        let styled_line = render_indent(line);
        return ("indent", styled_line);
    }

    if line.starts_with("`") && line.ends_with("`") {
        let styled_line = render_code(line);
        return ("code", styled_line);
    }

    return ("", String::from(""));
}

fn render_header(line: &str) -> String {
    let header_background = get_color("#6155FB");
    let header_foreground = get_color("#FFF");

    let item_to_print = format!(" {} ", &line[2..]);

    return format!(
        "\n{}",
        item_to_print
            .on_custom_color(header_background)
            .custom_color(header_foreground)
            .bold()
    );
}

fn get_bullet_line(line: &str) -> String {
    let mut line_to_render = render_text(line);
    line_to_render = line_to_render.replacen("-", "•", 1);
    return format!("{}", line_to_render);
}

fn render_indent(line: &str) -> String {
    let mut line_to_render = render_text(line);
    line_to_render = line_to_render.replacen("> ", "│ ", 1);
    return format!("{}", line_to_render);
}

fn render_text(line: &str) -> String {
    let mut result = String::new();

    let mut current_block_type: Option<&str> = None;
    let mut current_block_start: Option<usize> = None;
    for (idx, char) in line.chars().enumerate() {
        if current_block_type == Some("code") {
            if char == '`' {
                let code_block = &line[current_block_start.unwrap()..idx + 1];
                let styled_code = render_code(code_block);
                result.push_str(&styled_code);
                current_block_start = None;
                current_block_type = None;
                continue;
            }
        } else if current_block_type == Some("link") {
            if char == '>' {
                let link_block = &line[current_block_start.unwrap() + 1..idx];
                let styled_code = render_link(link_block);
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

fn render_link(line: &str) -> String {
    let color = get_color("#008787");

    let styled_line = line.custom_color(color).underline();
    return format!(
        "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
        line, styled_line
    );
}

fn render_code(line: &str) -> String {
    let len = line.len();
    let mut line_to_render = format!(" {} ", &line[1..len - 1]);
    line_to_render = line_to_render.replace("{{", "").replace("}}", "");

    let bg = get_color("#303030");
    let fg = get_color("FF6060");

    return format!("{}", line_to_render.on_custom_color(bg).custom_color(fg));
}

fn get_color(hex_code: &str) -> colored::customcolors::CustomColor {
    let color = color::Color::new(hex_code);
    return colored::CustomColor {
        r: color.r,
        g: color.g,
        b: color.b,
    };
}
