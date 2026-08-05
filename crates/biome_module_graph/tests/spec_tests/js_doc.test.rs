use super::*;
use biome_module_graph::{SymbolFromModuleInfo, find_jsdoc_for_exported_symbol};

#[test]
fn finds_jsdoc_for_separately_exported_ambient_declarations() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            /** @deprecated Use Grid2. */
            declare const Grid: unknown;
            export { Grid };

            /** @deprecated Use generateText. */
            declare function generateObject(): void;
            export { generateObject };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], false);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for (name, expected) in [
        ("Grid", "@deprecated Use Grid2."),
        ("generateObject", "@deprecated Use generateText."),
    ] {
        let symbol = SymbolFromModuleInfo::new(&db, name, module);
        let jsdoc = find_jsdoc_for_exported_symbol(&db, symbol)
            .as_ref()
            .unwrap_or_else(|| panic!("{name} must have JSDoc"));
        assert_eq!(jsdoc.as_ref(), expected);
    }
}
