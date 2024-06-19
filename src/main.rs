use std::{fs, io::IsTerminal, process::exit};

use clap::{Parser, ValueEnum};

mod tldr_helper;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Platform {
    Android,
    Common,
    FreeBsd,
    NetBsd,
    OpenBsd,
    Osx,
    SunOs,
    Windows,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Parser)]
#[command(about, long_about=None)]
#[derive(Debug)]
struct Cli {
    /// The name of the tool you want to see the tldr page for.
    name: Option<Vec<String>>,

    #[arg(
        short,
        long,
        require_equals = true,
        value_name = "platform",
        help = "Specify the platform of the command.",
        value_enum
    )]
    platform: Option<Platform>,

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
    let theme = markdown_rs_terminal::themes::get_default_theme();
    // let tldr_cache = dirs::home_dir().unwrap().join(".config/tldr-2");
    let tldr_cache = dirs::cache_dir().unwrap().join("tldr-rs");

    if !tldr_cache.join("version").exists() {
        print!("TLDR has not been initialized. Initializing now.");
        tldr_helper::initialize(&tldr_cache);
    }

    if cli.version {
        let version = fs::read_to_string(tldr_cache.join("version")).unwrap();
        print!("{}", version);

        return;
    }

    if cli.cache_dir {
        if std::io::stdout().is_terminal() {
            print!("Cache Directory: {:?}", &tldr_cache.display());
        } else {
            print!("{}", &tldr_cache.display());
        }
        return;
    }

    if cli.update {
        let current_version = fs::read_to_string(tldr_cache.join("version")).unwrap();
        let latest_version = tldr_helper::get_latest_version();

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

        tldr_helper::initialize(&tldr_cache);

        return;
    }

    if cli.name.is_none() {
        println!("Please enter a command. use tldr --help to see usage");
        return;
    }

    let command = match cli.name {
        Some(x) if !x.is_empty() => x.join("-"),
        _ => {
            println!("Please enter a command. use tldr --help to see usage");
            exit(-1);
        }
    };

    let selected_platform = match cli.platform {
        Some(x) => Some(x.to_string()),
        None => None,
    };

    tldr_helper::read_page(&command, &tldr_cache, selected_platform, &theme);
}
