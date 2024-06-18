use clap::{Parser};

mod tldr;
mod markdown;

#[derive(Parser)]
#[command(about, long_about=None)]
#[derive(Debug)]
struct Cli {
    /// The name of the tool you want to see the tldr page for.
    name: String,

    #[arg(short, long, value_name = "platform", help="Specify the platform of the command.")]
    platform: Option<String>,

    #[arg(short, long, help="Update the TLDR cache.")]
    update: bool,

    #[arg(short, long, help="Deletes the local tldr cache and refreshes it.")]
    reset: bool,

    #[arg(short, long, help="Print version.")]
    version: bool,
}

fn main() {
    let cli = Cli::parse();
    let config_dir = dirs::home_dir().unwrap().join(".config/tldr-2");

    if !config_dir.join("version").exists() {
        print!("TLDR has not been initialized. Initializing now.");
        tldr::initialize(&config_dir);
    }

    tldr::read_page(&cli.name, &config_dir)
}
