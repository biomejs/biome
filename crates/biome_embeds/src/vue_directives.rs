use crate::data::VueDirectiveResolution;
use crate::visitor::vue_directive_declarations_from_source;
use biome_languages::LanguageDb;
use camino::Utf8PathBuf;

/// An interned Vue custom-directive lookup in a host document.
#[salsa::interned]
#[derive(Debug)]
pub struct InternedVueDirective {
    #[returns(ref)]
    path: Utf8PathBuf,
    #[returns(ref)]
    name: String,
}

/// Resolves a Vue custom directive from workspace-backed embedded data.
#[salsa::tracked]
pub fn resolve_vue_directive(
    db: &dyn LanguageDb,
    directive: InternedVueDirective<'_>,
) -> VueDirectiveResolution {
    // A host document that is absent from the database carries no declarations we can
    // read, which is not the same as a document that declares nothing. Reporting here
    // would turn "the file was not indexed" into "the directive is undeclared".
    let Some(parsed_source) = db.parsed_source_for_path(directive.path(db)) else {
        return VueDirectiveResolution::Unknown;
    };

    vue_directive_declarations_from_source(db, parsed_source).resolve(directive.name(db))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{
        TestDb, VUE_SCRIPT_SETUP_SOURCE_INDEX, VUE_SCRIPT_SETUP_TS_SOURCE_INDEX,
        VUE_SCRIPT_SOURCE_INDEX, parse_vue_source_with_js_snippets,
    };
    use biome_db::Db;
    use biome_db::testing::assert_function_query_was_not_run;

    fn resolve(db: &TestDb, path: &Utf8PathBuf, name: &str) -> VueDirectiveResolution {
        resolve_vue_directive(
            db,
            InternedVueDirective::new(db, path.clone(), name.to_string()),
        )
    }

    #[test]
    fn resolve_vue_directive_is_unknown_for_a_document_outside_the_database() {
        let db = TestDb::new();

        assert_eq!(
            resolve(&db, &Utf8PathBuf::from("src/Missing.vue"), "v-highlight"),
            VueDirectiveResolution::Unknown
        );
    }

    #[test]
    fn resolve_vue_directive_finds_script_setup_bindings() {
        let db = TestDb::new();
        let setup = "const vHighlight = {};";
        let path = parse_vue_source_with_js_snippets(
            &db,
            &format!("<script setup>{setup}</script><template><div v-highlight /></template>"),
            &[(setup, VUE_SCRIPT_SETUP_SOURCE_INDEX)],
        );

        assert_eq!(
            resolve(&db, &path, "v-highlight"),
            VueDirectiveResolution::Declared
        );
        assert_eq!(
            resolve(&db, &path, "v-missing"),
            VueDirectiveResolution::Undeclared
        );
    }

    #[test]
    fn resolve_vue_directive_finds_directives_option() {
        let db = TestDb::new();
        let script = "export default { directives: { highlight: {} } };";
        let path = parse_vue_source_with_js_snippets(
            &db,
            &format!("<script>{script}</script><template><div v-highlight /></template>"),
            &[(script, VUE_SCRIPT_SOURCE_INDEX)],
        );

        assert_eq!(
            resolve(&db, &path, "v-highlight"),
            VueDirectiveResolution::Declared
        );
        assert_eq!(
            resolve(&db, &path, "v-missing"),
            VueDirectiveResolution::Undeclared
        );
    }

    #[test]
    fn resolve_vue_directive_is_unknown_when_options_cannot_be_resolved() {
        let db = TestDb::new();
        let script = "export default { mixins: [shared] };";
        let path = parse_vue_source_with_js_snippets(
            &db,
            &format!("<script>{script}</script><template><div v-highlight /></template>"),
            &[(script, VUE_SCRIPT_SOURCE_INDEX)],
        );

        assert_eq!(
            resolve(&db, &path, "v-highlight"),
            VueDirectiveResolution::Unknown
        );
    }

    #[test]
    fn resolve_vue_directive_is_unknown_when_the_default_export_is_re_exported() {
        for script in [
            r#"export { default } from "./options";"#,
            r#"export { options as default } from "./options";"#,
        ] {
            let db = TestDb::new();
            let path = parse_vue_source_with_js_snippets(
                &db,
                &format!("<script>{script}</script><template><div v-highlight /></template>"),
                &[(script, VUE_SCRIPT_SOURCE_INDEX)],
            );

            assert_eq!(
                resolve(&db, &path, "v-highlight"),
                VueDirectiveResolution::Unknown,
                "`{script}` declares the component options elsewhere"
            );
        }
    }

    #[test]
    fn resolve_vue_directive_ignores_type_only_declarations() {
        // Vue erases these before the component runs, so none of them can back a
        // custom directive even though they all bind the name `vHighlight`.
        for script in [
            "type vHighlight = {};",
            "interface vHighlight {}",
            r#"import type { vHighlight } from "./types";"#,
            r#"import { type vHighlight } from "./types";"#,
            r#"import type vHighlight from "./types";"#,
            r#"import type * as vHighlight from "./types";"#,
        ] {
            let db = TestDb::new();
            let path = parse_vue_source_with_js_snippets(
                &db,
                &format!(
                    "<script setup lang=\"ts\">{script}</script><template><div v-highlight /></template>"
                ),
                &[(script, VUE_SCRIPT_SETUP_TS_SOURCE_INDEX)],
            );

            assert_eq!(
                resolve(&db, &path, "v-highlight"),
                VueDirectiveResolution::Undeclared,
                "`{script}` does not declare a runtime binding"
            );
        }
    }

    #[test]
    fn resolve_vue_directive_still_finds_runtime_declarations_in_typescript() {
        for script in [
            "const vHighlight = {};",
            "function vHighlight() {}",
            "class vHighlight {}",
            "enum vHighlight { A }",
            r#"import { vHighlight } from "./directives";"#,
            r#"import vHighlight from "./directives";"#,
        ] {
            let db = TestDb::new();
            let path = parse_vue_source_with_js_snippets(
                &db,
                &format!(
                    "<script setup lang=\"ts\">{script}</script><template><div v-highlight /></template>"
                ),
                &[(script, VUE_SCRIPT_SETUP_TS_SOURCE_INDEX)],
            );

            assert_eq!(
                resolve(&db, &path, "v-highlight"),
                VueDirectiveResolution::Declared,
                "`{script}` declares a runtime binding"
            );
        }
    }

    #[test]
    fn resolve_vue_directive_is_unknown_for_an_external_script_block() {
        // `<script … />` parses as a self-closing element rather than an element with
        // an opening tag, so both spellings have to be recognised.
        for host in [
            r#"<script src="./options.js"></script><template><div v-highlight /></template>"#,
            r#"<script src="./options.js" /><template><div v-highlight /></template>"#,
            r#"<script setup src="./setup.js" /><template><div v-highlight /></template>"#,
        ] {
            let db = TestDb::new();
            let path = parse_vue_source_with_js_snippets(&db, host, &[]);

            assert_eq!(
                resolve(&db, &path, "v-highlight"),
                VueDirectiveResolution::Unknown,
                "`{host}` sources its script from another file"
            );
        }
    }

    #[test]
    fn resolve_vue_directive_ignores_a_self_closing_script_without_src() {
        let db = TestDb::new();
        let path = parse_vue_source_with_js_snippets(
            &db,
            r#"<script /><template><div v-highlight /></template>"#,
            &[],
        );

        assert_eq!(
            resolve(&db, &path, "v-highlight"),
            VueDirectiveResolution::Undeclared
        );
    }

    #[test]
    fn resolve_vue_directive_is_unknown_for_an_external_script_beside_script_setup() {
        let db = TestDb::new();
        let setup = "const vLocal = {};";
        let path = parse_vue_source_with_js_snippets(
            &db,
            &format!(
                r#"<script src="./options.js"></script><script setup>{setup}</script><template><div v-highlight /></template>"#
            ),
            &[(setup, VUE_SCRIPT_SETUP_SOURCE_INDEX)],
        );

        // The local binding still resolves; anything else may come from the external file.
        assert_eq!(
            resolve(&db, &path, "v-local"),
            VueDirectiveResolution::Declared
        );
        assert_eq!(
            resolve(&db, &path, "v-highlight"),
            VueDirectiveResolution::Unknown
        );
    }

    #[test]
    fn vue_directive_declarations_from_source_is_memoized() {
        let db = TestDb::new();
        let setup = "const vHighlight = {};";
        let path = parse_vue_source_with_js_snippets(
            &db,
            &format!("<script setup>{setup}</script><template><div v-highlight /></template>"),
            &[(setup, VUE_SCRIPT_SETUP_SOURCE_INDEX)],
        );
        let file = db
            .parsed_source_for_path(&path)
            .expect("parsed source should be stored");

        let _ = vue_directive_declarations_from_source(&db, file);

        db.clear_salsa_events();
        let _ = vue_directive_declarations_from_source(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_not_run(
            &db,
            vue_directive_declarations_from_source,
            file,
            &events,
        );
    }

    #[test]
    fn resolve_vue_directive_does_not_recollect_declarations_per_directive() {
        let db = TestDb::new();
        let setup = "const vHighlight = {};";
        let path = parse_vue_source_with_js_snippets(
            &db,
            &format!("<script setup>{setup}</script><template><div v-highlight /></template>"),
            &[(setup, VUE_SCRIPT_SETUP_SOURCE_INDEX)],
        );
        let file = db
            .parsed_source_for_path(&path)
            .expect("parsed source should be stored");

        let _ = resolve(&db, &path, "v-highlight");

        db.clear_salsa_events();
        let _ = resolve(&db, &path, "v-other");
        let events = db.take_salsa_events();

        assert_function_query_was_not_run(
            &db,
            vue_directive_declarations_from_source,
            file,
            &events,
        );
    }
}
