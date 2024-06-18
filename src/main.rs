use clap::{builder::Str, Parser};
use reqwest::header::USER_AGENT;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

mod tldr;

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[derive(Debug)]
struct Cli {
    /// The name of the tool you want to see the tldr page for.
    name: String,

    #[arg(short, long, value_name = "platform")]
    platform: Option<String>,

    #[arg(short, long)]
    update: bool,

    #[arg(short, long)]
    reset: bool,
}

fn main() {
    let cli = Cli::parse();
    print!(
        "platform: {}\n",
        cli.platform.unwrap_or("linux".to_string())
    );
    print!("name: {}\n", cli.name);
    print!("update: {}\n", cli.update);
    print!("reset: {}\n", cli.reset);

    let config_dir = dirs::home_dir().unwrap().join(".config/tldr-2");

    if !config_dir.join("version").exists() {
        print!("TLDR has not been initialized. Initializing now.");
        tldr::initialize(&config_dir);
    }

    tldr::read_page(&cli.name, &config_dir)
}
