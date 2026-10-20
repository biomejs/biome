use std::sync::Arc;

use biome_rowan::{TextRange, TextSize};
use camino::Utf8Path;
use tsrs_vfs::{bundled, vfstest};

use crate::TsrsProgram;

fn program(files: &[(&str, &str)]) -> TsrsProgram {
    let fs = Arc::new(bundled::wrap_fs(vfstest::from_map(
        files.iter().copied(),
        true,
    )));
    let roots: Vec<_> = files.iter().map(|(path, _)| Utf8Path::new(path)).collect();
    TsrsProgram::from_root_files_in(Utf8Path::new("/src"), &roots, fs)
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
    let program = program(&[("/src/index.ts", text)]);
    let file = Utf8Path::new("/src/index.ts");
    assert_eq!(
        program
            .type_to_string(file, range_of(text, "Promise.resolve(1)"))
            .as_deref(),
        Some("Promise<number>")
    );
    assert_eq!(
        program
            .type_to_string(file, statement_expression(text, "f()"))
            .as_deref(),
        Some("Promise<string>")
    );
}

#[test]
fn matches_ranges_without_leading_trivia() {
    let text = "declare const n: number;\n  /* comment */ (n);\n";
    let program = program(&[("/src/index.ts", text)]);
    let file = Utf8Path::new("/src/index.ts");
    assert_eq!(
        program
            .type_to_string(file, range_of(text, "(n)"))
            .as_deref(),
        Some("number")
    );
    assert_eq!(program.type_to_string(file, range_of(text, "n)")), None);
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
    let program = program(&[("/src/index.ts", text)]);
    let file = Utf8Path::new("/src/index.ts");
    let is_promise = |expression: &str| {
        program
            .is_promise_like(file, statement_expression(text, expression))
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
    let program = program(&[("/src/index.ts", text)]);
    let file = Utf8Path::new("/src/index.ts");
    let is_array = |needle: &str| {
        program
            .is_promise_array(file, range_of(text, needle))
            .unwrap()
    };
    assert!(is_array("[p, p]"));
    assert!(!is_array("[1, 2]"));
    assert!(is_array("[p].map(async () => {})"));
}

#[test]
fn resolves_imports() {
    let dep = "export async function load(): Promise<number> { return 1; }\n";
    let index = "import { load } from './dep';\nload();\n";
    let program = program(&[("/src/index.ts", index), ("/src/dep.ts", dep)]);
    let file = Utf8Path::new("/src/index.ts");
    assert_eq!(
        program
            .type_to_string(file, range_of(index, "load()"))
            .as_deref(),
        Some("Promise<number>")
    );
    assert_eq!(
        program.is_promise_like(file, range_of(index, "load()")),
        Some(true)
    );
}
