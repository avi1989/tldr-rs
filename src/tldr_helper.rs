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

pub fn read_page(
    name: &str,
    config_dir: &Path,
    platform: Option<String>,
    language: Vec<String>,
    theme: &Theme,
) {
    let page_location = get_page_location(name, config_dir, platform, language);
    if page_location.is_none() {
        println!("Command: {name} not found");
        return;
    }

    let (page_location, page_folder) = page_location.unwrap();

    println!("Loaded {} from platform: {}", name, page_folder);

    for _i in 0..80 {
        print!("⎯");
    }

    println!();
    let _ =
        markterm::render_file_to_stdout(&page_location, Some(theme), markterm::ColorChoice::Auto);
}

pub fn get_page_location(
    name: &str,
    config_dir: &Path,
    platform: Option<String>,
    languages: Vec<String>,
) -> Option<(PathBuf, String)> {
    let current_os = env::consts::OS;

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

    for language in languages {
        let language_path = config_dir.join(format!("pages.{}", language));
        for folder in folders_to_check {
            let file_path = language_path.join(folder).join(format!("{name}.md"));
            if file_path.exists() {
                return Some((file_path, folder.to_string()));
            }
        }
    }

    None
}

pub fn get_latest_version() -> String {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://api.github.com/repos/tldr-pages/tldr/releases/latest")
        .header(USER_AGENT, "tldr-rs")
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

fn download_release() -> PathBuf {
    let dir = env::temp_dir().join("tldr");
    let zip_path = dir.join("tldr.zip");

    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }

    if (dir.join(&zip_path)).exists() {
        println!("File already exists at {:?}", dir);
        return zip_path;
    }

    println!("Downloading file to {:?}", zip_path);
    let response = reqwest::blocking::get(
        "https://github.com/tldr-pages/tldr/releases/latest/download/tldr.zip",
    )
    .unwrap_or_else(|error| panic!("Failed to download file: {}", error))
    .copy_to(
        &mut std::fs::File::create(dir.join(&zip_path))
            .unwrap_or_else(|error| panic!("Failed to create file: {}", error)),
    )
    .unwrap();

    let response_bytes = response as f32 / (1024 * 1024) as f32;
    println!(
        "File ({:.2} MB) downloaded to {:?}",
        response_bytes,
        dir.join(&zip_path)
    );

    zip_path
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

pub fn add_page_from_url(url: &str, _config_dir: &Path) -> Result<(), String> {
    // This is a placeholder implementation
    println!("Add page from URL functionality is not implemented yet.");
    println!("Requested URL: {}", url);

    // Return success without performing any operations
    Ok(())
}

pub fn get_languages_from_environment() -> Vec<String> {
    let lang = env::var("LANG").ok();
    let language = env::var("LANGUAGE").ok();
    get_languages(lang.as_deref(), language.as_deref())
}

fn get_languages(lang: Option<&str>, language: Option<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if lang.is_none() {
        return vec!["en".to_string()];
    }

    if let Some(lang) = language {
        let langs = lang.split(':');
        for lang in langs {
            let lang = lang.to_string();
            if result.contains(&lang) {
                continue;
            }

            result.push(lang)
        }
    }

    if let Some(lang) = lang {
        let lang = lang[0..2].to_string();

        if !result.contains(&lang) {
            result.push(lang.to_owned());
        }
    }

    if !result.is_empty() {
        let en = "en".to_string();
        if !result.contains(&en) {
            result.push(en);
        }

        return result;
    }

    vec!["en".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;
    mod get_language {
        use super::*;

        #[test]
        fn should_return_en_if_no_variable_set() {
            let result = get_languages(None, None);
            assert_eq!("en", result[0]);
            assert_eq!(1, result.len())
        }

        #[test]
        fn should_extract_language_from_lang() {
            let result = get_languages(Some("de_US.UTF-8"), None);
            assert_eq!("de", result[0]);
            assert_eq!("en", result[1]);
        }

        #[test]
        fn should_merge_language_and_lang() {
            let language = "it:de:cz";
            let lang = "fr";
            let result = get_languages(Some(lang), Some(language));
            assert_eq!("it", result[0]);
            assert_eq!("de", result[1]);
            assert_eq!("cz", result[2]);
            assert_eq!("fr", result[3]);
            assert_eq!("en", result[4]);
            assert_eq!(5, result.len());
        }

        #[test]
        fn should_merge_language_and_lang_without_duplicates() {
            let language = "it:de:cz:fr";
            let lang = "fr";
            let result = get_languages(Some(lang), Some(language));
            assert_eq!("it", result[0]);
            assert_eq!("de", result[1]);
            assert_eq!("cz", result[2]);
            assert_eq!("fr", result[3]);
            assert_eq!("en", result[4]);
            assert_eq!(5, result.len());
        }

        #[test]
        fn get_language_should_ignore_language_if_lang_not_set() {
            let lang = "it:de:cz";
            let result = get_languages(None, Some(lang));
            assert_eq!("en", result[0]);
            assert_eq!(1, result.len());
        }
    }
}
