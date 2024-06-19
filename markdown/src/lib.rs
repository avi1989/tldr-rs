#![warn(missing_docs)]
//! A cross platform library to render markdown on the terminal.

mod renderer;
/// Contains the utilities required to theme the markdown.
pub mod themes;
pub use themes::{color::Color, get_default_theme, ElementTheme, Theme};

use std::io::BufRead;
use std::{
    fs::File,
    io::{self},
    path::PathBuf,
};

use renderer::get_line_to_render;

/// A module that contains the structs and functions required to theme the markdown.

/// Renders the provided file in markdown.
pub fn render_file(file_path: &PathBuf, theme: &Theme) {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            panic!("Unable to open file: {}", e);
        }
    };

    let lines = io::BufReader::new(file).lines();

    for line in lines.flatten() {
        let (_, l_text) = get_line_to_render(&line, theme);
        println!(" {}", l_text);
    }
    println!("\n");
}

/// Renders the passed in text as markdown
pub fn render(text: &str, theme: &Theme) {
    for line in text.lines() {
        let (_, l_text) = get_line_to_render(&line, theme);
        println!(" {}", l_text);
    }
}
