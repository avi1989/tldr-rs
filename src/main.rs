use std::fs;

use clap::Parser;
use markdown::themes::{self};

mod markdown;
mod tldr;

#[derive(Parser)]
#[command(about, long_about=None)]
#[derive(Debug)]
struct Cli {
    /// The name of the tool you want to see the tldr page for.
    name: Option<String>,

    #[arg(
        short,
        long,
        value_name = "platform",
        help = "Specify the platform of the command."
    )]
    platform: Option<String>,

    #[arg(short, long, help = "Update the TLDR cache.")]
    update: bool,

    #[arg(short, long, help = "Deletes the local tldr cache and refreshes it.")]
    reset: bool,

    #[arg(short, long, help = "Print version.")]
    version: bool,

    #[arg(long, name = "cache-dir", help = "Gets the cache directory")]
    cache_dir: bool,
}

fn main() {
    let cli = Cli::parse();
    let theme = themes::get_theme();
    // let tldr_cache = dirs::home_dir().unwrap().join(".config/tldr-2");
    let tldr_cache = dirs::cache_dir().unwrap().join("tldr-rs");

    if !tldr_cache.join("version").exists() {
        print!("TLDR has not been initialized. Initializing now.");
        tldr::initialize(&tldr_cache);
    }

    if cli.version {
        let version = fs::read_to_string(tldr_cache.join("version")).unwrap();
        print!("{}", version);

        return;
    }

    if cli.cache_dir {
        print!("Cache Directory: {:?}", &tldr_cache);
        return;
    }

    if cli.update {
        let current_version = fs::read_to_string(tldr_cache.join("version")).unwrap();
        let latest_version = tldr::get_latest_version();

        if current_version == latest_version {
            println!("No new updates...");
        }

        return;
    }

    if cli.reset {
        println!("Deleting {:?}", &tldr_cache);
        std::fs::remove_dir_all(&tldr_cache).unwrap_or_else(|err| {
            println!(
                "Unable to delete directory {:?} due to {}",
                &tldr_cache, err
            )
        });

        tldr::initialize(&tldr_cache);

        return;
    }

    if cli.name.is_none() {
        println!("Please enter a command. use tldr --help to see usage");
        return;
    }

    let selected_platform = match cli.platform {
        Some(x) => Some(x),
        None => None,
    };

    tldr::read_page(&cli.name.unwrap(), &tldr_cache, selected_platform, &theme);
}