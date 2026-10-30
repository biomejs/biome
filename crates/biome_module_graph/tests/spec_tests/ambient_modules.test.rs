use super::*;
use biome_module_graph::{JsAmbientModule, JsModuleInfo};
use std::collections::BTreeSet;

fn js_module_info(fs: &MemoryFileSystem, path: &str) -> JsModuleInfo {
    let ModuleInfoKind::Js(info) = resolve_js_module_kind_for_test(fs, path, false) else {
        panic!("{path} must be a JavaScript module");
    };
    info
}

fn export_names(info: &JsModuleInfo) -> Vec<&str> {
    info.exports.keys().map(Text::text).collect()
}

fn ambient_module_info(module: &JsAmbientModule) -> &JsModuleInfo {
    module
        .info
        .as_ref()
        .unwrap_or_else(|| panic!("{} must have a body", module.name))
}

#[test]
fn ambient_module_collects_the_imports_and_exports_of_its_body() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/node_modules/astro/types/content.d.ts".into(),
        r#"declare module "astro:content" {
            import zod from "astro/zod";
            export type { DataEntry } from "astro/content/config";
            export { defineCollection } from "astro/content/config";
            export const z = zod.z;
            export const getCollection: (...args: any[]) => any;
        }"#,
    );

    let info = js_module_info(&fs, "/node_modules/astro/types/content.d.ts");
    let [module] = info.ambient_modules.as_ref() else {
        panic!("the file must declare one ambient module");
    };
    assert_eq!(module.name.text(), "astro:content");

    let module_info = ambient_module_info(module);
    assert_eq!(
        export_names(module_info),
        ["DataEntry", "defineCollection", "z", "getCollection"]
    );
    assert!(matches!(
        module_info.exports.get("defineCollection"),
        Some(JsExport::Reexport(_))
    ));
    assert!(matches!(
        module_info.exports.get("getCollection"),
        Some(JsExport::Own(_))
    ));
    assert_eq!(
        module_info
            .import_paths
            .named_iter()
            .map(|(specifier, _)| specifier.text())
            .collect::<Vec<_>>(),
        ["astro/zod", "astro/content/config", "astro/content/config"]
    );
    assert!(module_info.static_imports.contains_key("zod"));
}

#[test]
fn ambient_module_without_export_declarations_exports_every_declaration() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/env.d.ts".into(),
        r#"declare module "virtual:config" {
            import type { Options } from "options";
            const title: string;
            function load(): void;
            interface Config {}
            namespace helpers {
                const inner: number;
            }
            export const explicit: number;
        }"#,
    );

    let info = js_module_info(&fs, "/src/env.d.ts");
    let [module] = info.ambient_modules.as_ref() else {
        panic!("the file must declare one ambient module");
    };

    let exports: BTreeSet<_> = export_names(ambient_module_info(module))
        .into_iter()
        .collect();
    assert_eq!(
        exports,
        BTreeSet::from(["Config", "explicit", "helpers", "load", "title"])
    );
}

#[test]
fn ambient_module_without_export_declarations_skips_globals_and_default_export_names() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/env.d.ts".into(),
        r#"declare module "virtual:globals" {
            global {
                interface Window {
                    title: string;
                }
            }
            const title: string;
        }
        declare module "virtual:default" {
            export default function render(): void;
            const version: string;
        }"#,
    );

    let info = js_module_info(&fs, "/src/env.d.ts");
    let modules: Vec<_> = info
        .ambient_modules
        .iter()
        .map(|module| {
            let exports: BTreeSet<_> = export_names(ambient_module_info(module))
                .into_iter()
                .collect();
            (module.name.text(), exports)
        })
        .collect();
    assert_eq!(
        modules,
        [
            ("virtual:globals", BTreeSet::from(["title"])),
            ("virtual:default", BTreeSet::from(["default", "version"])),
        ]
    );
}

#[test]
fn ambient_module_with_export_declaration_exports_only_its_explicit_exports() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/env.d.ts".into(),
        r#"declare module "virtual:config" {
            const hidden: string;
            export const shown: string;
            export {};
        }"#,
    );

    let info = js_module_info(&fs, "/src/env.d.ts");
    let [module] = info.ambient_modules.as_ref() else {
        panic!("the file must declare one ambient module");
    };
    assert_eq!(export_names(ambient_module_info(module)), ["shown"]);
}

#[test]
fn ambient_module_collects_its_export_assignment() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/legacy.d.ts".into(),
        r#"declare module "legacy" {
            const value: number;
            export = value;
        }"#,
    );

    let info = js_module_info(&fs, "/src/legacy.d.ts");
    let [module] = info.ambient_modules.as_ref() else {
        panic!("the file must declare one ambient module");
    };
    assert_eq!(export_names(ambient_module_info(module)), ["default"]);
}

#[test]
fn ambient_modules_are_collected_per_declaration_in_source_order() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/globals.ts".into(),
        r#"declare module "*.css" {
            const classes: Record<string, string>;
            export default classes;
        }
        declare module "*.css";
        declare module "*.svg";"#,
    );

    let info = js_module_info(&fs, "/src/globals.ts");
    let modules: Vec<_> = info
        .ambient_modules
        .iter()
        .map(|module| (module.name.text(), module.info.as_ref().map(export_names)))
        .collect();
    assert_eq!(
        modules,
        [
            ("*.css", Some(vec!["default"])),
            ("*.css", None),
            ("*.svg", None),
        ]
    );
}

#[test]
fn ambient_module_default_export_can_name_a_declaration_of_the_file() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/styles.d.ts".into(),
        r#"declare const styles: Record<string, string>;
        declare module "*.module.css" {
            export default styles;
        }"#,
    );

    let info = js_module_info(&fs, "/src/styles.d.ts");
    let [module] = info.ambient_modules.as_ref() else {
        panic!("the file must declare one ambient module");
    };
    assert_eq!(export_names(ambient_module_info(module)), ["default"]);
}

#[test]
fn module_declarations_in_module_files_are_not_ambient_modules() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/import.d.ts".into(),
        r#"import "vue";
        declare module "vue" {
            export const injected: number;
        }"#,
    );
    fs.insert(
        "/src/export.d.ts".into(),
        r#"export {};
        declare module "vue" {}"#,
    );
    fs.insert(
        "/src/require.ts".into(),
        r#"import fs = require("fs");
        declare module "fs" {}"#,
    );

    for path in ["/src/import.d.ts", "/src/export.d.ts", "/src/require.ts"] {
        let info = js_module_info(&fs, path);
        assert!(
            info.ambient_modules.is_empty(),
            "{path} is a module, so its declarations augment existing modules"
        );
    }
}

#[test]
fn namespace_import_alias_keeps_the_file_a_script() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/alias.ts".into(),
        r#"declare namespace N {
            const B: number;
        }
        import A = N.B;
        declare module "virtual:alias" {}"#,
    );

    let info = js_module_info(&fs, "/src/alias.ts");
    let names: Vec<_> = info
        .ambient_modules
        .iter()
        .map(|module| module.name.text())
        .collect();
    assert_eq!(names, ["virtual:alias"]);
}
