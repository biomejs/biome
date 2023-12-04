//! Takes comments from biome_js_parser and turns them into test data.
//! This code is derived from rust_analyzer/xtask/codegen/gen_parser_tests

use std::{
    collections::HashMap,
    fs, mem,
    path::{Path, PathBuf},
};

use crate::{update, Mode};
use xtask::{project_root, Result};

fn extract_comment_blocks(
    text: &str,
    allow_blocks_with_empty_lines: bool,
) -> Vec<(usize, Vec<String>)> {
    let mut res = Vec::new();

    let prefix = "// ";
    let lines = text.lines().map(str::trim_start);

    let mut block = (0, vec![]);
    for (line_num, line) in lines.enumerate() {
        if line == "//" && allow_blocks_with_empty_lines {
            block.1.push(String::new());
            continue;
        }

        let is_comment = line.starts_with(prefix);
        if is_comment {
            block.1.push(line[prefix.len()..].to_string());
        } else {
            if !block.1.is_empty() {
                res.push(mem::take(&mut block));
            }
            block.0 = line_num + 2;
        }
    }
    if !block.1.is_empty() {
        res.push(block)
    }
    res
}

pub fn generate_parser_tests(mode: Mode) -> Result<()> {
    let tests = tests_from_dir(&project_root().join(Path::new("crates/biome_js_parser/src")))?;
    fn install_tests(tests: &HashMap<String, Test>, into: &str, mode: Mode) -> Result<bool> {
        let tests_dir = project_root().join(into);
        if !tests_dir.is_dir() {
            fs::create_dir_all(&tests_dir)?;
        }
        // ok is never actually read, but it needs to be specified to create a Test in existing_tests
        let existing = existing_tests(&tests_dir, true)?;

        if let Some(t) = existing.keys().find(|&t| !tests.contains_key(t)) {
            panic!("Test is deleted: '{}'", t);
        }

        let mut some_file_was_updated = false;

        for (name, test) in tests {
            let path = match existing.get(name) {
                Some((path, _test)) => path.clone(),
                None => tests_dir
                    .join(name)
                    .with_extension(test.language.extension()),
            };
            if let crate::UpdateResult::Updated = update(&path, &test.text, &mode)? {
                some_file_was_updated = true;
            }

            if let Some(options) = &test.options {
                let path = tests_dir.join(name).with_extension("options.json");

                if let crate::UpdateResult::Updated = update(&path, options, &mode)? {
                    some_file_was_updated = true;
                }
            }
        }

        Ok(some_file_was_updated)
    }

    let mut some_file_was_updated = false;
    some_file_was_updated |= install_tests(
        &tests.ok,
        "crates/biome_js_parser/test_data/inline/ok",
        mode,
    )?;
    some_file_was_updated |= install_tests(
        &tests.err,
        "crates/biome_js_parser/test_data/inline/err",
        mode,
    )?;

    if some_file_was_updated {
        let _ = filetime::set_file_mtime(
            "crates/biome_js_parser/src/tests.rs",
            filetime::FileTime::now(),
        );
    }

    Ok(())
}

#[derive(Debug)]
struct Test {
    pub name: String,
    pub text: String,
    pub ok: bool,
    pub language: Language,
    pub options: Option<String>,
}

#[derive(Debug)]
enum Language {
    JavaScript,
    TypeScript,
    TypeScriptDefinition,
    Jsx,
    Tsx,
}

impl Language {
    const fn extension(&self) -> &'static str {
        match self {
            Language::JavaScript => "js",
            Language::TypeScript => "ts",
            Language::TypeScriptDefinition => "d.ts",
            Language::Jsx => "jsx",
            Language::Tsx => "tsx",
        }
    }

    fn from_file_name(name: &str) -> Option<Language> {
        let language = match name.rsplit_once('.')? {
            (_, "js") => Language::JavaScript,
            (rest, "ts") => match rest.rsplit_once('.') {
                Some((_, "d")) => Language::TypeScriptDefinition,
                _ => Language::TypeScript,
            },
            (_, "jsx") => Language::Jsx,
            (_, "tsx") => Language::Tsx,
            _ => {
                return None;
            }
        };

        Some(language)
    }
}

#[derive(Default, Debug)]
struct Tests {
    pub ok: HashMap<String, Test>,
    pub err: HashMap<String, Test>,
}

fn collect_tests(s: &str) -> Vec<Test> {
    let mut res = Vec::new();
    for comment_block in extract_comment_blocks(s, false).into_iter().map(|(_, x)| x) {
        let first_line = &comment_block[0];

        let (ok, suffix) = match first_line.split_once(' ') {
            Some(("test", suffix)) => (true, suffix),
            Some(("test_err", suffix)) => (false, suffix),
            _ => continue,
        };

        let (language, suffix) = match suffix.split_once(' ') {
            Some(("jsx", suffix)) => (Language::Jsx, suffix),
            Some(("js", suffix)) => (Language::JavaScript, suffix),
            Some(("ts", suffix)) => (Language::TypeScript, suffix),
            Some(("d.ts", suffix)) => (Language::TypeScriptDefinition, suffix),
            Some(("tsx", suffix)) => (Language::Tsx, suffix),
            Some((_, suffix)) => (Language::JavaScript, suffix),
            _ => panic!("wrong test configuration: {:?}", suffix),
        };

        let (name, options) = match suffix.split_once(' ') {
            Some((name, options)) => (name, Some(options.to_string())),
            _ => (suffix, None),
        };

        let text: String = comment_block[1..]
            .iter()
            .cloned()
            .chain([String::new()])
            .collect::<Vec<_>>()
            .join("\n");

        assert!(!text.trim().is_empty() && text.ends_with('\n'));
        res.push(Test {
            name: name.to_string(),
            options,
            text,
            ok,
            language,
        })
    }
    res
}

fn tests_from_dir(dir: &Path) -> Result<Tests> {
    let mut res = Tests::default();
    for entry in ::walkdir::WalkDir::new(dir) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().unwrap_or_default() != "rs" {
            continue;
        }
        process_file(&mut res, entry.path())?;
    }
    return Ok(res);
    fn process_file(res: &mut Tests, path: &Path) -> Result<()> {
        let text = fs::read_to_string(path)?;

        for test in collect_tests(&text) {
            if test.ok {
                if let Some(old_test) = res.ok.insert(test.name.clone(), test) {
                    anyhow::bail!("Duplicate test: {}", old_test.name);
                }
            } else if let Some(old_test) = res.err.insert(test.name.clone(), test) {
                anyhow::bail!("Duplicate test: {}", old_test.name);
            }
        }
        Ok(())
    }
}

fn existing_tests(dir: &Path, ok: bool) -> Result<HashMap<String, (PathBuf, Test)>> {
    let mut res = HashMap::new();
    for file in fs::read_dir(dir)? {
        let path = file?.path();
        let language = path
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(Language::from_file_name);

        if let Some(language) = language {
            let name = path
                .file_stem()
                .map(|x| x.to_string_lossy().to_string())
                .unwrap();
            let text = fs::read_to_string(&path)?;
            let test = Test {
                name: name.clone(),
                options: None,
                text,
                ok,
                language,
            };
            if let Some(old) = res.insert(name, (path, test)) {
                println!("Duplicate test: {:?}", old);
            }
        }
    }
    Ok(res)
}
