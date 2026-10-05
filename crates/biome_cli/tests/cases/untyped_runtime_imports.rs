use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use crate::{TestArgs as Args, run_cli_with_server_workspace};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8PathBuf;

#[test]
fn untyped_runtime_entrypoint_and_missing_package() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Utf8PathBuf::from("biome.json"),
        r#"{ "linter": { "rules": { "correctness": { "noUnresolvedImports": "error" } } } }"#,
    );
    for (name, main) in [
        ("example", "lib/index.js"),
        ("extensionless", "lib/index"),
        ("directory", "lib"),
    ] {
        fs.insert(
            Utf8PathBuf::from(format!("node_modules/{name}/package.json")),
            format!(r#"{{ "name": "{name}", "main": "{main}" }}"#),
        );
        fs.insert(
            Utf8PathBuf::from(format!("node_modules/{name}/lib/index.js")),
            "module.exports = { value: 1 };",
        );
    }
    fs.insert(
        Utf8PathBuf::from("valid.mjs"),
        "import example from 'example'; import extensionless from 'extensionless'; import directory from 'directory'; console.log(example.value, extensionless.value, directory.value);",
    );
    fs.insert(
        Utf8PathBuf::from("missing.mjs"),
        "import missing from 'missing'; console.log(missing);",
    );
    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--only=correctness/noUnresolvedImports",
                "valid.mjs",
                "missing.mjs",
            ]
            .as_slice(),
        ),
    );
    assert!(result.is_err());
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "untyped_runtime_entrypoint_and_missing_package",
        fs,
        console,
        result,
    ));
}

#[test]
fn untyped_runtime_imports_preserve_symbol_and_type_checks() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Utf8PathBuf::from("biome.json"),
        r#"{ "linter": { "rules": { "correctness": { "noUnresolvedImports": "error" } } } }"#,
    );
    fs.insert(Utf8PathBuf::from("package.json"), r#"{ "type": "module" }"#);
    fs.insert(
        Utf8PathBuf::from("node_modules/example/package.json"),
        r#"{ "name": "example", "main": "../../runtime.js" }"#,
    );
    fs.insert(Utf8PathBuf::from("runtime.js"), "export const present = 1;");
    for (name, source) in [
        ("literal", "export default 1;"),
        ("empty", "export {};"),
        ("implicit", ""),
    ] {
        fs.insert(
            Utf8PathBuf::from(format!("node_modules/{name}/package.json")),
            format!(r#"{{ "name": "{name}", "main": "../../{name}.js" }}"#),
        );
        fs.insert(Utf8PathBuf::from(format!("{name}.js")), source);
    }
    fs.insert(
        Utf8PathBuf::from("index.ts"),
        r#"
import { present } from 'example';
import runtime = require('example');
import type EqualsType = require('example');
import { absent } from 'example';
import type { TypeOnly } from 'example';
import { type InlineType } from 'example';
import { present as value, type MixedType } from 'example';
export { present, type MixedExportType } from 'example';
export type { TypeExport } from 'example';
import literal from 'literal';
import { absent as missingLiteral } from 'literal';
import missingDefault from 'empty';
import implicitDefault, { absent as implicitNamed } from 'implicit';
console.log(present, absent, value, runtime, literal, missingLiteral, missingDefault, implicitDefault, implicitNamed);
"#,
    );
    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--only=correctness/noUnresolvedImports",
                "index.ts",
                "runtime.js",
                "literal.js",
                "empty.js",
                "implicit.js",
            ]
            .as_slice(),
        ),
    );
    assert!(result.is_err());
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "untyped_runtime_imports_preserve_symbol_and_type_checks",
        fs,
        console,
        result,
    ));
}

#[test]
fn runtime_module_syntax_and_commonjs_boundaries() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Utf8PathBuf::from("biome.json"),
        r#"{ "linter": { "rules": { "correctness": { "noUnresolvedImports": "error" } } } }"#,
    );
    let mut imports = String::new();
    let mut paths = vec!["index.mjs".to_string()];
    for (name, source, esm) in [
        ("meta", "console.log(import.meta.url);", true),
        ("await", "await Promise.resolve();", true),
        ("using", "await using resource = null;", true),
        (
            "loop",
            "for await (const value of []) { console.log(value); }",
            true,
        ),
        ("lexical", "const require = 1; console.log(require);", true),
        ("class", "class module {}", true),
        (
            "function",
            "async function run() { await Promise.resolve(); } module.exports = { run };",
            false,
        ),
        (
            "method",
            "module.exports = { async run() { await Promise.resolve(); } };",
            false,
        ),
        (
            "block",
            "{ const require = 1; console.log(require); } module.exports = {};",
            false,
        ),
        ("hoisted", "var require = 1; module.exports = {};", false),
    ] {
        fs.insert(
            Utf8PathBuf::from(format!("node_modules/{name}/package.json")),
            format!(r#"{{ "name": "{name}", "main": "../../{name}.js" }}"#),
        );
        fs.insert(Utf8PathBuf::from(format!("{name}.js")), source);
        paths.push(format!("{name}.js"));
        if esm {
            imports.push_str(&format!(
                "import {{ absent as missing_{name} }} from '{name}';\n"
            ));
        } else {
            imports.push_str(&format!(
                "import valid_{name} from '{name}'; console.log(valid_{name});\n"
            ));
        }
    }
    fs.insert(Utf8PathBuf::from("index.mjs"), imports);
    let mut arguments = vec!["lint", "--only=correctness/noUnresolvedImports"];
    arguments.extend(paths.iter().map(String::as_str));
    let (fs, result) =
        run_cli_with_server_workspace(fs, &mut console, Args::from(arguments.as_slice()));
    assert!(result.is_err());
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "runtime_module_syntax_and_commonjs_boundaries",
        fs,
        console,
        result,
    ));
}
