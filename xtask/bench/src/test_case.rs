use crate::err_to_string;
use ansi_rgb::{red, Foreground};
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Hash)]
pub struct TestCase {
    code: String,
    id: String,
    path: PathBuf,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl TestCase {
    pub fn try_from(file_url: &str) -> Result<TestCase, String> {
        let url = url::Url::from_str(file_url).map_err(err_to_string)?;
        let segments = url
            .path_segments()
            .ok_or_else(|| "lib url has no segments".to_string())?;
        let filename = segments
            .last()
            .ok_or_else(|| "lib url has no segments".to_string())?;

        let path = Path::new(
            &env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
        )
        .ancestors()
        .nth(2)
        .unwrap()
        .join("target")
        // cache the file name to avoid to save files that have the same name, but they come from different repos
        .join(format!(
            "{filename}_{}",
            calculate_hash(&filename.to_string())
        ));

        let content = std::fs::read_to_string(&path)
            .map_err(err_to_string)
            .or_else(|_| {
                println!(
                    "[{}] - Downloading [{}] to [{}]",
                    filename,
                    file_url,
                    path.display()
                );
                match ureq::get(file_url).call() {
                    Ok(response) => {
                        let mut reader = response.into_reader();

                        let mut writer = std::fs::File::create(&path).map_err(err_to_string)?;
                        if let Err(err) = std::io::copy(&mut reader, &mut writer) {
                            drop(writer);
                            std::fs::remove_file(&path).ok();
                            return Err(err_to_string(err));
                        }
                        std::fs::read_to_string(&path).map_err(err_to_string)
                    }
                    Err(e) => Err(err_to_string(e)),
                }
            });

        content.map(|code| {
            println!("[{}] - using [{}]", filename.fg(red()), path.display());
            TestCase {
                id: filename.to_string(),
                code,
                path,
            }
        })
    }

    pub fn filename(&self) -> &str {
        &self.id
    }

    pub fn filename_hash(&self) -> String {
        format!("{}_{}", self.filename(), calculate_hash(&self))
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn extension(&self) -> &str {
        self.path
            .extension()
            .expect("Expected test case to have extension")
            .to_str()
            .expect("Expected extension to be valid UTF8")
    }
}
