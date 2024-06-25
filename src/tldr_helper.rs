use markterm::themes::Theme;
use reqwest::header::USER_AGENT;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

pub fn initialize(config_dir: &Path) {
    println!("Initializing tldr");
    let file_buf = download_release();
    extract_file(&file_buf, config_dir);
    let current_version = get_latest_version();
    let mut file = File::create(config_dir.join("version")).unwrap();
    file.write_all(current_version.as_bytes()).unwrap();
}

pub fn read_page(name: &str, config_dir: &Path, platform: Option<String>, theme: &Theme) {
    let page_location = get_page_location(name, config_dir, platform);
    if page_location.is_none() {
        println!("Command: {name} not found");
        return;
    }

    let page_folder = page_location.unwrap();

    let file_to_read = config_dir
        .join("pages")
        .join(&page_folder)
        .join(format!("{name}.md"));

    println!("Loaded {} from platform: {}", name, &page_folder);

    for _i in 0..80 {
        print!("⎯");
    }

    println!();
    let _ = markterm::render_file_to_stdout(&file_to_read, Some(theme));
    // markdown::render_file(&file_to_read, theme);
}

pub fn get_latest_version() -> String {
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

    let version = response
        .get("tag_name")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();

    version
}

pub fn get_page_location(
    name: &str,
    config_dir: &Path,
    platform: Option<String>,
) -> Option<String> {
    let current_os = env::consts::OS;
    let base_page_path = config_dir.join("pages");

    let first_platform = platform.unwrap_or(current_os.to_string());

    let folders_to_check = [
        first_platform.as_str(),
        "common",
        "android",
        "freebsd",
        "linux",
        "netbsd",
        "openbsd",
        "osx",
        "sunos",
        "windows",
    ];

    for folder in folders_to_check {
        if base_page_path
            .join(folder)
            .join(format!("{name}.md"))
            .exists()
        {
            return Some(String::from(folder));
        }
    }

    None
}

fn download_release() -> PathBuf {
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

    path_to_dowload
}

fn extract_file(file_buf: &PathBuf, config_dir: &Path) {
    let file = fs::File::open(file_buf).unwrap();

    println!("Extracting file {:?}", file);
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 00..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = config_dir.join(file.enclosed_name().unwrap());

        if (file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}
