use clap::{builder::Str, Parser};
use reqwest::header::USER_AGENT;
use std::{
    env, fs::{self, File}, io::Write, path::{Path, PathBuf}
};

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
        initialize_tldr(&config_dir);
    }
}

fn initialize_tldr(config_dir: &PathBuf) {
    print!("Initializing tldr\n");
    let file_buf = download_file();
    extract_file(&file_buf, config_dir);
    let current_version = get_current_version();
    let mut file = File::create(config_dir.join("version")).unwrap();
    file.write(current_version.as_bytes()).unwrap();
}

fn get_current_version() -> String {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://api.github.com/repos/tldr-pages/tldr/releases/latest")
        .header(USER_AGENT, "tldr-rust")
        .send()
        .unwrap_or_else(|error| {
            panic!("Failed to get latest release: {}", error);
        })
        .json::<serde_json::Value>()
        .unwrap_or_else(|error| {
            panic!("Failed to parse json: {}", error);
        });

    let version = response.get("tag_name").unwrap().as_str().unwrap().to_owned();
        
    print!("{:?}", version);

    return version;
}

fn download_file() -> PathBuf {
    let dir = env::temp_dir();
    let path_to_dowload = dir.join("tldr/tldr.zip");

    if (dir.join(&path_to_dowload)).exists() {
        println!("File already exists at {:?}", dir);
        return path_to_dowload;
    }

    println!("Downloading file to {:?}", path_to_dowload);
    let response = reqwest::blocking::get(
        "https://github.com/tldr-pages/tldr/releases/latest/download/tldr.zip",
    )
    .unwrap_or_else(|error| panic!("Failed to download file: {}", error))
    .copy_to(
        &mut std::fs::File::create(dir.join(&path_to_dowload))
            .unwrap_or_else(|error| panic!("Failed to create file: {}", error)),
    )
    .unwrap();

    let response_bytes = response as f32 / (1024 * 1024) as f32;
    println!(
        "File ({:.2} MB) downloaded to {:?}",
        response_bytes,
        dir.join(&path_to_dowload)
    );

    //TODO: Fix mess.
    return path_to_dowload;
}

fn extract_file(file_buf: &PathBuf, config_dir: &PathBuf) {
    let file = fs::File::open(&file_buf).unwrap();

    println!("Extracting file {:?}", file);
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 00..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = config_dir.join(file.enclosed_name().unwrap());

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}
