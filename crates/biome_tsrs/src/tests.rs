use std::sync::Arc;

use biome_rowan::{TextRange, TextSize};
use camino::Utf8Path;
use tsrs_vfs::{bundled, vfstest};

use crate::{TsrsProject, TsrsSnapshot};

fn project(files: &[(&str, &str)]) -> TsrsProject {
    let fs = Arc::new(bundled::wrap_fs(vfstest::from_map(
        files.iter().copied(),
        true,
    )));
    let roots: Vec<_> = files.iter().map(|(path, _)| Utf8Path::new(path)).collect();
    TsrsProject::from_root_files_in(Utf8Path::new("/src"), &roots, fs)
}

/// A snapshot of a project whose only file is `/src/index.ts`.
fn snapshot(text: &str) -> TsrsSnapshot {
    project(&[("/src/index.ts", text)])
        .snapshot(Utf8Path::new("/src/index.ts"), text)
        .unwrap()
}

/// The range of `expression` in the first line of `text` that is exactly `expression;`.
fn statement_expression(text: &str, expression: &str) -> TextRange {
    let start = text.find(&format!("\n{expression};\n")).unwrap() + 1;
    TextRange::at(
        TextSize::from(start as u32),
        TextSize::from(expression.len() as u32),
    )
}

/// The range of the first occurrence of `needle` in `text`.
fn range_of(text: &str, needle: &str) -> TextRange {
    let start = text.find(needle).unwrap();
    TextRange::at(
        TextSize::from(start as u32),
        TextSize::from(needle.len() as u32),
    )
}

#[test]
fn prints_expression_types() {
    let text = "const a = Promise.resolve(1);\nasync function f() { return 'x'; }\nf();\n";
    let snapshot = snapshot(text);
    assert_eq!(
        snapshot
            .type_to_string(range_of(text, "Promise.resolve(1)"))
            .as_deref(),
        Some("Promise<number>")
    );
    assert_eq!(
        snapshot
            .type_to_string(statement_expression(text, "f()"))
            .as_deref(),
        Some("Promise<string>")
    );
}

#[test]
fn matches_ranges_without_leading_trivia() {
    let text = "declare const n: number;\n  /* comment */ (n);\n";
    let snapshot = snapshot(text);
    assert_eq!(
        snapshot.type_to_string(range_of(text, "(n)")).as_deref(),
        Some("number")
    );
    assert_eq!(snapshot.type_to_string(range_of(text, "n)")), None);
}

#[test]
fn classifies_promises() {
    let text = "\
class MyPromise<T> extends Promise<T> {}
declare const custom: MyPromise<number>;
declare const thenable: { then(resolve: () => void): void };
declare const maybe: Promise<void> | undefined;
declare const n: number;
custom;
thenable;
maybe;
n;
";
    let snapshot = snapshot(text);
    let is_promise = |expression: &str| {
        snapshot
            .is_promise_like(statement_expression(text, expression))
            .unwrap()
    };
    assert!(is_promise("custom"));
    assert!(!is_promise("thenable"));
    assert!(is_promise("maybe"));
    assert!(!is_promise("n"));
}

#[test]
fn classifies_promise_arrays() {
    let text = "\
declare const p: Promise<void>;
[p, p];
[1, 2];
[p].map(async () => {});
";
    let snapshot = snapshot(text);
    let is_array = |needle: &str| snapshot.is_promise_array(range_of(text, needle)).unwrap();
    assert!(is_array("[p, p]"));
    assert!(!is_array("[1, 2]"));
    assert!(is_array("[p].map(async () => {})"));
}

#[test]
fn resolves_imports() {
    let dep = "export async function load(): Promise<number> { return 1; }\n";
    let index = "import { load } from './dep';\nload();\n";
    let snapshot = project(&[("/src/index.ts", index), ("/src/dep.ts", dep)])
        .snapshot(Utf8Path::new("/src/index.ts"), index)
        .unwrap();
    assert_eq!(
        snapshot
            .type_to_string(range_of(index, "load()"))
            .as_deref(),
        Some("Promise<number>")
    );
    assert_eq!(
        snapshot.is_promise_like(range_of(index, "load()")),
        Some(true)
    );
}

#[test]
fn updates_files_with_new_text() {
    let dep = "export const value = 1;\n";
    let index = "import { value } from './dep';\nvalue;\n";
    let project = project(&[("/src/index.ts", index), ("/src/dep.ts", dep)]);
    let index_path = Utf8Path::new("/src/index.ts");
    let before = project.snapshot(index_path, index).unwrap();
    assert!(
        std::ptr::eq(
            project.snapshot(index_path, index).unwrap().program,
            before.program
        ),
        "unchanged text reuses the program"
    );

    let edited_dep = "export const value = 'one';\n";
    project
        .snapshot(Utf8Path::new("/src/dep.ts"), edited_dep)
        .unwrap();
    let after = project.snapshot(index_path, index).unwrap();
    let range = statement_expression(index, "value");
    assert_eq!(after.type_to_string(range).as_deref(), Some("\"one\""));
    assert_eq!(
        before.type_to_string(range).as_deref(),
        Some("1"),
        "older snapshots keep their program"
    );

    let edited_index = "import { value } from './dep';\nPromise.resolve(value);\n";
    let edited = project.snapshot(index_path, edited_index).unwrap();
    assert!(
        edited.program.get_source_file("/src/dep.ts")
            == after.program.get_source_file("/src/dep.ts"),
        "files that didn't change are reused"
    );
    assert_eq!(
        edited
            .type_to_string(statement_expression(edited_index, "Promise.resolve(value)"))
            .as_deref(),
        Some("Promise<string>")
    );
}

#[test]
fn rejects_files_outside_the_project() {
    let project = project(&[("/src/index.ts", "export {};\n")]);
    assert!(
        project
            .snapshot(Utf8Path::new("/src/other.ts"), "export {};\n")
            .is_none()
    );
}

#[test]
fn projects_share_parsed_files() {
    let shared = "export async function load(): Promise<number> { return 1; }\n";
    let first_index = "import { load } from '../shared/load';\nload();\n";
    let second_index = "import { load } from '../shared/load';\nvoid load();\n";
    let files = [
        ("/sharing/shared/load.ts", shared),
        ("/sharing/first/index.ts", first_index),
        ("/sharing/second/index.ts", second_index),
    ];
    let fs: Arc<dyn tsrs_vfs::FS> = Arc::new(bundled::wrap_fs(vfstest::from_map(
        files.iter().copied(),
        true,
    )));
    let first = TsrsProject::from_root_files_in(
        Utf8Path::new("/sharing/first"),
        &[Utf8Path::new("/sharing/first/index.ts")],
        fs.clone(),
    );
    let second = TsrsProject::from_root_files_in(
        Utf8Path::new("/sharing/second"),
        &[Utf8Path::new("/sharing/second/index.ts")],
        fs,
    );
    let first = first
        .snapshot(Utf8Path::new("/sharing/first/index.ts"), first_index)
        .unwrap();
    let second = second
        .snapshot(Utf8Path::new("/sharing/second/index.ts"), second_index)
        .unwrap();
    assert!(
        first.program.get_source_file("/sharing/shared/load.ts")
            == second.program.get_source_file("/sharing/shared/load.ts"),
        "both programs hold the same parse of the shared file"
    );
    assert_eq!(
        first.is_promise_like(statement_expression(first_index, "load()")),
        Some(true)
    );
    assert_eq!(
        second.is_promise_like(range_of(second_index, "load()")),
        Some(true)
    );
}
