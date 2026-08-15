use crate::model::SemanticModel;
use crate::semantic_model;
use biome_css_syntax::AnyCssRoot;
use biome_db::{AnyParsedSource, Db, ParsedSnippet, ParsedSource};
use biome_rowan::{TextRange, TokenText};

/// The name and source range of an `@property` registration candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssPropertyDefinition {
    name: TokenText,
    range: TextRange,
}

impl CssPropertyDefinition {
    pub fn name(&self) -> &str {
        self.name.text()
    }

    pub fn range(&self) -> TextRange {
        self.range
    }
}

#[salsa::tracked(returns(ref))]
pub(crate) fn css_model_from_parsed_source(db: &dyn Db, file: ParsedSource) -> SemanticModel {
    let parsed: AnyCssRoot = file.parsed(db).tree();
    semantic_model(&parsed)
}

#[salsa::tracked(returns(ref))]
pub(crate) fn css_model_from_parsed_snippet(db: &dyn Db, file: ParsedSnippet) -> SemanticModel {
    let parsed: AnyCssRoot = file.parsed(db).tree();
    semantic_model(&parsed)
}

/// Returns `@property` registration candidates from a parsed CSS document.
#[salsa::tracked(returns(ref))]
pub fn css_property_definitions_from_source(
    db: &dyn Db,
    file: ParsedSource,
) -> Vec<CssPropertyDefinition> {
    let _ = file.parsed(db);
    collect_property_definitions(css_model_from_parsed_source(db, file))
}

/// Returns `@property` registration candidates from an embedded CSS document.
#[salsa::tracked(returns(ref))]
pub fn css_property_definitions_from_snippet(
    db: &dyn Db,
    file: ParsedSnippet,
) -> Vec<CssPropertyDefinition> {
    let _ = file.parsed(db);
    collect_property_definitions(css_model_from_parsed_snippet(db, file))
}

fn collect_property_definitions(model: &SemanticModel) -> Vec<CssPropertyDefinition> {
    model
        .global_custom_variables()
        .at_property_registration_candidates()
        .map(|property| CssPropertyDefinition {
            name: property.name().clone(),
            range: property.range(),
        })
        .collect()
}

pub fn css_semantic_model<'db>(db: &'db dyn Db, file: &AnyParsedSource) -> &'db SemanticModel {
    match file {
        AnyParsedSource::ParsedSource(s) => css_model_from_parsed_source(db, *s),
        AnyParsedSource::ParsedSnippet(s) => css_model_from_parsed_snippet(db, *s),
    }
}

#[cfg(test)]
mod tests {
    use super::css_model_from_parsed_source;
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_css_syntax::property_syntax::{
        PropertySyntax, PropertySyntaxComponentName, PropertySyntaxResult, PropertySyntaxType,
    };
    use biome_db::ParsedSource;
    use biome_db::testing::{
        Events, assert_function_query_was_not_run, assert_function_query_was_run,
    };
    use biome_languages::css::CssFileSource;
    use biome_languages::{DocumentFileSource, LanguageDb};
    use camino::{Utf8Path, Utf8PathBuf};
    use salsa::Storage;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        events: Events,
        storage: Storage<Self>,
    }

    impl TestDb {
        fn new() -> Self {
            let events = Events::default();
            Self {
                storage: salsa::Storage::new(Some(Box::new({
                    let events = events.clone();
                    move |event| {
                        events.0.lock().unwrap().push(event);
                    }
                }))),
                events,
            }
        }

        fn take_salsa_events(&mut self) -> Vec<salsa::Event> {
            std::mem::take(&mut *self.events.0.lock().unwrap())
        }

        fn clear_salsa_events(&mut self) {
            self.take_salsa_events();
        }
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<ParsedSource> {
            unreachable!("Not used in this test")
        }
    }

    #[salsa::db]
    impl LanguageDb for TestDb {
        fn source_from_index(&self, _index: usize) -> Option<DocumentFileSource> {
            unreachable!("Not used in this test")
        }
    }

    fn make_file(db: &TestDb, source: &str) -> ParsedSource {
        let parsed = parse_css(source, CssFileSource::css(), CssParserOptions::default()).into();
        ParsedSource::new(db, Utf8PathBuf::from("test.css"), parsed, 0, vec![])
    }

    #[test]
    fn semantic_model_is_memoized() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");

        let _model = css_model_from_parsed_source(&db, file);

        db.clear_salsa_events();
        let _model = css_model_from_parsed_source(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_not_run(&db, css_model_from_parsed_source, file, &events);
    }

    #[salsa::tracked]
    fn rule_count(db: &dyn LanguageDb, file: ParsedSource) -> usize {
        let model = css_model_from_parsed_source(db, file);
        model.rules().len()
    }

    #[salsa::tracked]
    fn property_syntax_type(db: &dyn LanguageDb, file: ParsedSource) -> Option<PropertySyntaxType> {
        let model = css_model_from_parsed_source(db, file);
        let at_property = model
            .global_custom_variables()
            .get("--value")?
            .at_property()?;
        let PropertySyntaxResult::Value(PropertySyntax::Components(components)) =
            at_property.syntax()
        else {
            return None;
        };
        let PropertySyntaxComponentName::Type(syntax_type) = components.first()?.name else {
            return None;
        };
        Some(syntax_type)
    }

    #[test]
    fn selector_change_does_recompute() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");
        let _ = css_model_from_parsed_source(&db, file);

        let new_parsed = parse_css(
            "span { color: red; }",
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into();
        salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

        db.clear_salsa_events();
        let _ = css_model_from_parsed_source(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, css_model_from_parsed_source, file, &events);
    }

    #[test]
    fn value_change_does_recompute_downstream() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");
        let count = rule_count(&db, file);
        assert_eq!(count, 1);

        let new_parsed = parse_css(
            "p { color: blue; }",
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into();
        salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

        db.clear_salsa_events();
        let count = rule_count(&db, file);
        assert_eq!(count, 1);
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, css_model_from_parsed_source, file, &events);
        assert_function_query_was_run(&db, rule_count, file, &events);
    }

    #[test]
    fn incomplete_property_value_does_not_panic() {
        let mut db = TestDb::new();
        let file = make_file(&db, ".incomplete {\n  height: 1px\n}\n");

        assert_eq!(rule_count(&db, file), 1);

        let new_parsed = parse_css(
            ".incomplete {\n  height:\n}\n",
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into();
        salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

        assert_eq!(rule_count(&db, file), 1);
    }

    #[test]
    fn declaration_count_change_does_recompute_downstream() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");
        let count = rule_count(&db, file);
        assert_eq!(count, 1);

        let new_parsed = parse_css(
            "p { color: red; font-size: 12px; }",
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into();
        salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

        db.clear_salsa_events();
        let _ = rule_count(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, css_model_from_parsed_source, file, &events);
        assert_function_query_was_run(&db, rule_count, file, &events);
    }

    #[test]
    fn whitespace_change_does_not_recompute_downstream() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");
        let _ = rule_count(&db, file);

        let new_parsed = parse_css(
            "p  {  color:  red;  }",
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into();
        salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

        db.clear_salsa_events();
        let _ = rule_count(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, css_model_from_parsed_source, file, &events);
        assert_function_query_was_not_run(&db, rule_count, file, &events);
    }

    #[test]
    fn at_property_whitespace_change_does_not_recompute_downstream() {
        let mut db = TestDb::new();
        let file = make_file(
            &db,
            r#"@property --value { syntax: "<color>"; inherits: true; initial-value: red; }"#,
        );
        assert_eq!(
            property_syntax_type(&db, file),
            Some(PropertySyntaxType::Color)
        );

        let new_parsed = parse_css(
            r#"@property  --value  { syntax:  "<color>"; inherits:  true; initial-value:  red; }"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into();
        salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

        db.clear_salsa_events();
        assert_eq!(
            property_syntax_type(&db, file),
            Some(PropertySyntaxType::Color)
        );
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, css_model_from_parsed_source, file, &events);
        assert_function_query_was_not_run(&db, property_syntax_type, file, &events);
    }

    #[test]
    fn at_property_syntax_change_recomputes_downstream() {
        let mut db = TestDb::new();
        let file = make_file(
            &db,
            r#"@property --value { syntax: "<color>"; inherits: true; initial-value: red; }"#,
        );
        assert_eq!(
            property_syntax_type(&db, file),
            Some(PropertySyntaxType::Color)
        );

        let new_parsed = parse_css(
            r#"@property --value { syntax: "<length>"; inherits: true; initial-value: 10px; }"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into();
        salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

        db.clear_salsa_events();
        assert_eq!(
            property_syntax_type(&db, file),
            Some(PropertySyntaxType::Length)
        );
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, css_model_from_parsed_source, file, &events);
        assert_function_query_was_run(&db, property_syntax_type, file, &events);
    }

    #[test]
    fn shadowed_at_property_change_recomputes_downstream() {
        let mut db = TestDb::new();
        let file = make_file(
            &db,
            r#"@property --value { syntax: "<color>"; inherits: true; initial-value: red; }
@property --value { syntax: "<length>"; inherits: true; initial-value: 1px; }"#,
        );
        assert_eq!(
            property_syntax_type(&db, file),
            Some(PropertySyntaxType::Length)
        );

        let new_parsed = parse_css(
            r#"@property --value { syntax: "<number>"; inherits: true; initial-value: 1; }
@property --value { syntax: "<length>"; inherits: true; initial-value: 1px; }"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into();
        salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

        db.clear_salsa_events();
        assert_eq!(
            property_syntax_type(&db, file),
            Some(PropertySyntaxType::Length)
        );
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, css_model_from_parsed_source, file, &events);
        assert_function_query_was_run(&db, property_syntax_type, file, &events);
    }

    #[test]
    fn new_rule_does_recompute_downstream() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");
        let count = rule_count(&db, file);
        assert_eq!(count, 1);

        let new_parsed = parse_css(
            "p { color: red; } span { color: blue; }",
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into();
        salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

        db.clear_salsa_events();
        let count = rule_count(&db, file);
        assert_eq!(count, 2);
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, css_model_from_parsed_source, file, &events);
        assert_function_query_was_run(&db, rule_count, file, &events);
    }
}
