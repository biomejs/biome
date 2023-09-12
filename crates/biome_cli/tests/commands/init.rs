use crate::configs::{CONFIG_INIT_DEFAULT, CONFIG_INIT_DEFAULT_WHEN_INSTALLED};
use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::{FileSystemExt, MemoryFileSystem};
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn init_help() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("init"), "--help"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "init_help",
        fs,
        console,
        result,
    ));
}

#[test]
fn creates_config_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("init")].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    let file_path = Path::new("biome.json");

    let mut file = fs
        .open(file_path)
        .expect("configuration file was not written on disk");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");
    let parsed = parse_json(CONFIG_INIT_DEFAULT, JsonParserOptions::default());
    let formatted =
        biome_json_formatter::format_node(JsonFormatOptions::default(), &parsed.syntax())
            .expect("valid format document")
            .print()
            .expect("valid format document");
    assert_eq!(content, formatted.as_code());

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "creates_config_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn creates_config_file_when_rome_installed_via_package_manager() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("./node_modules/@biomejs/biome/configuration_schema.json");
    fs.insert(file_path.into(), *b"{}");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("init")].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    let file_path = Path::new("biome.json");

    let mut file = fs
        .open(file_path)
        .expect("configuration file was not written on disk");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");
    let parsed = parse_json(
        CONFIG_INIT_DEFAULT_WHEN_INSTALLED,
        JsonParserOptions::default(),
    );
    let formatted =
        biome_json_formatter::format_node(JsonFormatOptions::default(), &parsed.syntax())
            .expect("valid format document")
            .print()
            .expect("valid format document");
    assert_eq!(content, formatted.as_code());

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "creates_config_file_when_rome_installed_via_package_manager",
        fs,
        console,
        result,
    ));
}
