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

const CSS_RULES: &[u8] = br#"{
    "html": { "experimentalFullSupportEnabled": true },
    "linter": {
        "rules": {
            "nursery": {
                "noUndeclaredClasses": "error",
                "noUndeclaredCustomProperties": "error",
                "noUnusedClasses": "error"
            }
        }
    }
}
"#;

const CSS_RULES_WITHOUT_HTML_SUPPORT: &[u8] = br#"{
    "linter": {
        "rules": {
            "nursery": {
                "noUndeclaredClasses": "error",
                "noUndeclaredCustomProperties": "error",
                "noUnusedClasses": "error"
            }
        }
    }
}
"#;

const STARLIGHT_CONFIG: &[u8] = br#"import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

export default defineConfig({
	fonts: [{ name: "Inter", cssVariable: "--font-body" }],
	integrations: [starlight({ customCss: ["./src/styles/custom.css"] })],
});
"#;

/// Inserts a Starlight site in the `site` folder, whose Astro configuration
/// is `astro_config`, and the Biome configuration `biome_config`.
fn insert_starlight_site(fs: &MemoryFileSystem, biome_config: &[u8], astro_config: &[u8]) {
    fs.insert(Utf8Path::new("biome.json").into(), biome_config);
    fs.insert(
        Utf8Path::new("site/package.json").into(),
        br#"{ "dependencies": { "astro": "^5.0.0", "@astrojs/starlight": "^0.30.0" } }"#,
    );
    fs.insert(Utf8Path::new("site/astro.config.mjs").into(), astro_config);
    fs.insert(
        Utf8Path::new("site/node_modules/astro/package.json").into(),
        br#"{ "name": "astro", "version": "5.0.0" }"#,
    );
    fs.insert(
        Utf8Path::new("site/node_modules/astro/client.d.ts").into(),
        br#"declare module "astro:content" {
	export const getCollection: (...args: any[]) => any;
}
"#,
    );
    fs.insert(
        Utf8Path::new("site/node_modules/@astrojs/starlight/package.json").into(),
        br#"{ "name": "@astrojs/starlight", "version": "0.30.0" }"#,
    );
    fs.insert(
        Utf8Path::new("site/node_modules/@astrojs/starlight/dist/style/props.css").into(),
        b":root { --sl-color-text: black; }\n",
    );
    fs.insert(
        Utf8Path::new("site/node_modules/@astrojs/starlight/dist/components/PageFrame.astro")
            .into(),
        br#"---
---
<div class="sidebar-content"><slot /></div>
<style is:global>
	.sl-flex { display: flex; }
</style>
"#,
    );
    fs.insert(
        Utf8Path::new("site/src/styles/custom.css").into(),
        b"@import \"./partial.css\";\n",
    );
    fs.insert(
        Utf8Path::new("site/src/styles/partial.css").into(),
        br#".sidebar-content { color: var(--sl-color-text); }
.card { color: var(--missing-color); font-family: var(--font-body); }
.unused { color: red; }
"#,
    );
    fs.insert(
        Utf8Path::new("site/src/components/card.css").into(),
        b".card-body { padding: 1rem; }\n",
    );
    fs.insert(
        Utf8Path::new("site/src/components/Card.jsx").into(),
        br#"import "./card.css";

export const Card = () => (
	<div
		className="card card-body sl-flex missing-class"
		style="color: var(--sl-color-text); margin: var(--jsx-missing-space)"
	/>
);
"#,
    );
    fs.insert(
        Utf8Path::new("site/src/components/Plain.jsx").into(),
        br#"export const Plain = () => <div className="unchecked-class" />;
"#,
    );
    fs.insert(
        Utf8Path::new("site/src/styles/page.css").into(),
        b".page { display: block; }\n",
    );
    fs.insert(
        Utf8Path::new("site/src/pages/index.astro").into(),
        br#"---
import "../styles/page.css";
---
<div class="page sl-flex astro-missing" style="color: var(--sl-color-text); margin: var(--astro-missing-space)"></div>
"#,
    );
}

/// The files of the Starlight site that the tests lint.
const STARLIGHT_SITE_FILES: [&str; 4] = [
    "site/src/styles/partial.css",
    "site/src/components/Card.jsx",
    "site/src/components/Plain.jsx",
    "site/src/pages/index.astro",
];

#[test]
fn starlight_stylesheets_apply_to_every_page() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    insert_starlight_site(&fs, CSS_RULES, STARLIGHT_CONFIG);

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(
            ["lint"]
                .into_iter()
                .chain(STARLIGHT_SITE_FILES)
                .collect::<Vec<_>>()
                .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "starlight_stylesheets_apply_to_every_page",
        fs,
        console,
        result,
    ));
}

#[test]
fn css_rules_skip_packages_whose_framework_stylesheets_are_unknown() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    insert_starlight_site(
        &fs,
        CSS_RULES,
        br#"import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

function stylesheets() {
	return ["./src/styles/custom.css"];
}

export default defineConfig({
	integrations: [starlight({ customCss: stylesheets() })],
});
"#,
    );

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(
            ["lint"]
                .into_iter()
                .chain(STARLIGHT_SITE_FILES)
                .collect::<Vec<_>>()
                .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "css_rules_skip_packages_whose_framework_stylesheets_are_unknown",
        fs,
        console,
        result,
    ));
}

#[test]
fn starlight_components_are_read_without_full_html_support() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    insert_starlight_site(&fs, CSS_RULES_WITHOUT_HTML_SUPPORT, STARLIGHT_CONFIG);

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(
            ["lint"]
                .into_iter()
                .chain(STARLIGHT_SITE_FILES)
                .collect::<Vec<_>>()
                .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "starlight_components_are_read_without_full_html_support",
        fs,
        console,
        result,
    ));
}

#[test]
fn css_rules_skip_packages_whose_framework_stylesheets_are_unreadable() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    insert_starlight_site(
        &fs,
        CSS_RULES,
        br#"import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

export default defineConfig({
	integrations: [starlight({ customCss: ["./src/styles/custom.css", "./src/styles/theme.styl"] })],
});
"#,
    );
    fs.insert(
        Utf8Path::new("site/src/styles/theme.styl").into(),
        b".theme\n  color red\n",
    );

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(
            ["lint"]
                .into_iter()
                .chain(STARLIGHT_SITE_FILES)
                .collect::<Vec<_>>()
                .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "css_rules_skip_packages_whose_framework_stylesheets_are_unreadable",
        fs,
        console,
        result,
    ));
}

#[test]
fn unknown_framework_modules_are_not_reported() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    fs.insert(Utf8Path::new("biome.json").into(), NO_UNRESOLVED_IMPORTS);
    fs.insert(
        Utf8Path::new("package.json").into(),
        br#"{ "dependencies": { "astro": "^5.0.0" } }"#,
    );
    let index = Utf8Path::new("src/index.ts");
    fs.insert(
        index.into(),
        br##"import { getCollection } from "astro:content";
import { config } from "virtual:starlight/user-config";
import { helper } from "./missing";
import { alias } from "~/lib/alias";
import { internal } from "#internal";
import { kit } from "$lib/kit";

export { alias, config, getCollection, helper, internal, kit };
"##,
    );

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["lint", index.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unknown_framework_modules_are_not_reported",
        fs,
        console,
        result,
    ));
}

#[test]
fn unindexed_framework_declarations_make_framework_modules_unknown() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8Path::new("biome.json").into(),
        br#"{
    "files": { "maxSize": 200 },
    "linter": {
        "rules": { "correctness": { "noUnresolvedImports": "error" } }
    }
}
"#,
    );
    fs.insert(
        Utf8Path::new("package.json").into(),
        br#"{ "dependencies": { "astro": "^5.0.0" } }"#,
    );
    fs.insert(
        Utf8Path::new("node_modules/astro/package.json").into(),
        br#"{ "name": "astro", "version": "5.0.0" }"#,
    );
    fs.insert(
        Utf8Path::new("node_modules/astro/client.d.ts").into(),
        br#"/// <reference path="./types/large.d.ts" />

declare module "astro:assets" {
	export const Image: unknown;
}
"#,
    );
    // Larger than `files.maxSize`, so Biome doesn't index it.
    fs.insert(
        Utf8Path::new("node_modules/astro/types/large.d.ts").into(),
        "declare const large: number;\n".repeat(10),
    );
    let index = Utf8Path::new("src/index.ts");
    fs.insert(
        index.into(),
        br#"import { Image } from "astro:assets";
import { getCollection } from "astro:content";
import { helper } from "./missing";

export { getCollection, helper, Image };
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
        "unindexed_framework_declarations_make_framework_modules_unknown",
        fs,
        console,
        result,
    ));
}

#[test]
fn css_globals_enable_css_rules_when_framework_stylesheets_are_unknown() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    insert_starlight_site(
        &fs,
        br#"{
    "html": { "experimentalFullSupportEnabled": true },
    "css": { "globals": ["site/src/styles/custom.css"] },
    "linter": {
        "rules": {
            "nursery": {
                "noUndeclaredClasses": "error",
                "noUndeclaredCustomProperties": "error",
                "noUnusedClasses": "error"
            }
        }
    }
}
"#,
        br#"import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

function stylesheets() {
	return ["./src/styles/custom.css"];
}

export default defineConfig({
	integrations: [starlight({ customCss: stylesheets() })],
});
"#,
    );

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(
            ["lint"]
                .into_iter()
                .chain(STARLIGHT_SITE_FILES)
                .collect::<Vec<_>>()
                .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "css_globals_enable_css_rules_when_framework_stylesheets_are_unknown",
        fs,
        console,
        result,
    ));
}

#[test]
fn css_globals_are_not_supported_in_overrides() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8Path::new("biome.json").into(),
        br#"{
    "overrides": [{ "includes": ["**/*.css"], "css": { "globals": ["theme.css"] } }]
}
"#,
    );
    let file = Utf8Path::new("theme.css");
    fs.insert(file.into(), b".theme { color: red; }\n");

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["lint", file.as_str()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "css_globals_are_not_supported_in_overrides",
        fs,
        console,
        result,
    ));
}
