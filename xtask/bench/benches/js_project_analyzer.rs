use std::{env, hash::Hash, io::Write, process::Command, str::FromStr, time::Duration};

use ansi_rgb::{Foreground, red};
use biome_cli::{BiomeCommand, CliDiagnostic, CliOptions, CliSession, LoggingLevel};
use biome_console::BufferConsole;
use biome_fs::OsFileSystem;
use biome_service::{App, WorkspaceRef, workspace};
use camino::{Utf8Path, Utf8PathBuf};
use xtask_bench::{BenchmarkId, Criterion, criterion_group, criterion_main, err_to_string};

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(target_env = "musl"),
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

const BIOME_JSONC: &str = r#"{
    "files": {
        "includes": ["**"]
    },
    "assist": {
        "enabled": false
    },
    "formatter": {
        "enabled": false
    },
    "linter": {
        "enabled": true,
        "rules": {
            "recommended": false // Avoid generating too many diagnostics.
        },
        "domains": {
            "project": "recommended"
        }
    }
}"#;

// Jemallocator does not work on aarch64 with musl, so we'll use the system allocator instead
#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;
fn bench_project_analyzer(criterion: &mut Criterion) {
    let suites = [include_str!("project-analyzer-libs-ts.txt")];
    let libs: Vec<_> = suites
        .iter()
        .flat_map(|suite| suite.lines().filter_map(|line| line.split_once(": ")))
        .collect();

    let mut group = criterion.benchmark_group("js_project_analyzer");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(60));

    for (folder, lib) in libs {
        let test_case = match ProjectTestCase::try_from(lib, folder) {
            Ok(test_case) => test_case,
            Err(err) => {
                println!("Error before running benchmark on {lib}: {err}");
                continue;
            }
        };

        group.bench_with_input(BenchmarkId::from_parameter(folder), folder, |b, _| {
            b.iter(|| match run_lint_in_folder(test_case.path.clone()) {
                Ok(()) => criterion::black_box(()),
                Err(err) => {
                    println!("Error while running benchmark on {lib}: {err:?}");
                }
            })
        });
    }

    group.finish();
}

criterion_group!(js_project_analyzer, bench_project_analyzer);
criterion_main!(js_project_analyzer);

#[derive(Hash)]
pub struct ProjectTestCase {
    id: String,
    path: Utf8PathBuf,
}

impl ProjectTestCase {
    pub fn try_from(file_url: &str, folder: &str) -> Result<Self, String> {
        let url = url::Url::from_str(file_url).map_err(err_to_string)?;
        let segments = url.path_segments().expect("lib url has no segments");
        let filename = segments.last().expect("lib url has no segments");

        let target_path = Utf8Path::new(
            &env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
        )
        .ancestors()
        .nth(2)
        .unwrap()
        .join("target");
        let file_path = target_path.join(filename);
        let folder_path = target_path.join(folder);

        if folder_path.is_dir() {
            println!("[{folder}] - Existing folder found");
        } else {
            println!("[{folder}] - Downloading [{file_url}] to [{file_path}]");
            match ureq::get(file_url).call() {
                Ok(response) => {
                    let mut reader = response.into_body().into_reader();

                    let mut writer = std::fs::File::create(&file_path).map_err(err_to_string)?;
                    if let Err(err) = std::io::copy(&mut reader, &mut writer) {
                        drop(writer);
                        std::fs::remove_file(&file_path).ok();
                        return Err(err_to_string(err));
                    }

                    drop(writer);

                    println!("[{folder}] - Extracting [{file_path}] to [{folder_path}]");

                    let output = Command::new("tar")
                        .arg("xzf")
                        .arg(filename)
                        .current_dir(target_path)
                        .output()
                        .map_err(|err| {
                            format!("failed to extract archive: {}", err_to_string(err))
                        })?;
                    if !output.status.success() {
                        return Err(format!(
                            "failed to extract archive. exit code: {}\nstderr:\n{}\nstdout:\n{}",
                            output.status,
                            String::from_utf8(output.stderr).expect("invalid utf-8 in stderr"),
                            String::from_utf8(output.stdout).expect("invalid utf-8 in stdout"),
                        ));
                    }

                    assert!(
                        folder_path.is_dir(),
                        "expected folder [{folder_path}] doesn't exist"
                    );

                    let output = if folder_path.join("pnpm-lock.yaml").is_file() {
                        println!("[{folder}] - Running `pnpm install` in [{folder_path}]");
                        Command::new("pnpm")
                            .arg("install")
                            .current_dir(&folder_path)
                            .output()
                    } else if folder_path.join("yarn.lock").is_file() {
                        println!("[{folder}] - Running `yarn install` in [{folder_path}]");
                        Command::new("yarn")
                            .arg("install")
                            .current_dir(&folder_path)
                            .output()
                    } else {
                        println!("[{folder}] - Running `npm install` in [{folder_path}]");
                        Command::new("npm")
                            .arg("install")
                            .current_dir(&folder_path)
                            .output()
                    };
                    let output = output.map_err(|err| {
                        format!("failed to install dependencies: {}", err_to_string(err))
                    })?;
                    if !output.status.success() {
                        return Err(format!(
                            "failed to install dependencies. {}\nstderr:\n{}\nstdout:\n{}",
                            output.status,
                            String::from_utf8(output.stderr).expect("invalid utf-8 in stderr"),
                            String::from_utf8(output.stdout).expect("invalid utf-8 in stdout"),
                        ));
                    }

                    std::fs::remove_file(&file_path).ok();

                    println!("[{folder}] - Creating [{folder_path}/biome.jsonc]");

                    let mut writer = std::fs::File::create(folder_path.join("biome.jsonc"))
                        .map_err(err_to_string)?;
                    writer
                        .write(BIOME_JSONC.as_bytes())
                        .map_err(err_to_string)?;
                }
                Err(e) => return Err(err_to_string(e)),
            }
        }

        println!("[{}] - Using [{folder_path}]", filename.fg(red()));
        Ok(Self {
            id: filename.to_string(),
            path: folder_path,
        })
    }

    pub fn filename(&self) -> &str {
        &self.id
    }

    pub fn path(&self) -> &Utf8Path {
        self.path.as_path()
    }
}

fn run_lint_in_folder(folder_path: Utf8PathBuf) -> Result<(), CliDiagnostic> {
    let mut console = BufferConsole::default();
    let workspace = workspace::server(Box::new(OsFileSystem::new(folder_path)), None);
    let app = App::new(&mut console, WorkspaceRef::Owned(workspace));
    let session = CliSession { app };
    session.run(BiomeCommand::Lint {
        write: false,
        unsafe_: false,
        fix: false,
        suppress: false,
        suppression_reason: None,
        linter_configuration: None,
        vcs_configuration: None,
        files_configuration: None,
        javascript_linter: None,
        json_linter: None,
        css_linter: None,
        graphql_linter: None,
        cli_options: CliOptions {
            colors: None,
            use_server: false,
            verbose: false,
            config_path: None,
            max_diagnostics: Default::default(),
            skip_parse_errors: false,
            no_errors_on_unmatched: false,
            error_on_warnings: false,
            reporter: Default::default(),
            log_level: LoggingLevel::Info,
            log_kind: Default::default(),
            diagnostic_level: Default::default(),
        },
        only: Vec::new(),
        skip: Vec::new(),
        stdin_file_path: None,
        staged: false,
        changed: false,
        since: None,
        paths: vec![".".into()],
    })
}
