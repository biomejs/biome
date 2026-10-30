use crate::TestArgs as Args;
use crate::run_cli_with_server_workspace;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

const NO_UNRESOLVED_IMPORTS: &[u8] = br#"{
    "linter": {
        "rules": { "correctness": { "noUnresolvedImports": "error" } }
    }
}
"#;

#[test]
fn modules_that_astro_declares_resolve() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), NO_UNRESOLVED_IMPORTS);
    fs.insert(
        Utf8Path::new("package.json").into(),
        br#"{ "dependencies": { "astro": "^7.0.0" } }"#,
    );
    fs.insert(
        Utf8Path::new("node_modules/astro/package.json").into(),
        br#"{ "name": "astro", "version": "7.4.2" }"#,
    );
    fs.insert(
        Utf8Path::new("node_modules/astro/client.d.ts").into(),
        br#"/// <reference path="./types/content.d.ts" />

declare module "astro:assets" {
	export const Image: (props: unknown) => unknown;
}
"#,
    );
    fs.insert(
        Utf8Path::new("node_modules/astro/types/content.d.ts").into(),
        br#"declare module "astro:content" {
	export const getCollection: (...args: any[]) => any;
}
"#,
    );
    let index = Utf8Path::new("src/index.ts");
    fs.insert(
        index.into(),
        br#"import { Image } from "astro:assets";
import { getCollection, missing } from "astro:content";
import { unknown } from "astro:unknown";

export { getCollection, Image, missing, unknown };
"#,
    );

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["lint", index.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "modules_that_astro_declares_resolve",
        fs,
        console,
        result,
    ));
}

#[test]
fn package_types_declaring_the_package_export_its_members() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), NO_UNRESOLVED_IMPORTS);
    fs.insert(
        Utf8Path::new("package.json").into(),
        br#"{ "dependencies": { "legacy": "^1.0.0" } }"#,
    );
    fs.insert(
        Utf8Path::new("node_modules/legacy/package.json").into(),
        br#"{ "name": "legacy", "version": "1.0.0", "types": "./index.d.ts" }"#,
    );
    fs.insert(
        Utf8Path::new("node_modules/legacy/index.d.ts").into(),
        br#"declare module "legacy" {
	export const named: number;
}
"#,
    );
    let index = Utf8Path::new("src/index.ts");
    fs.insert(
        index.into(),
        br#"import { named } from "legacy";

export { named };
"#,
    );

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["lint", index.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "package_types_declaring_the_package_export_its_members",
        fs,
        console,
        result,
    ));
}
