use crate::model::SemanticModel;
use crate::semantic_model;
use biome_css_syntax::selector_ext::AnyCssPseudoClassFunctionSelector;
use biome_css_syntax::{
    AnyCssRoot, CssComplexSelector, CssCompoundSelector, CssNestedQualifiedRule,
    CssPseudoClassIdentifier, CssQualifiedRule, CssSyntaxNode, decode_css_identifier,
};
use biome_db::{Db, FileSource};
use biome_parser::{AnyParse, AnyParsedSource};
use biome_rowan::{AstNode, AstNodeList, TextRange, TokenText};

/// The name and source range of a custom property definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssPropertyDefinition {
    name: TokenText,
    range: TextRange,
    globally_scoped: bool,
}

#[salsa::interned]
pub struct SemanticInput {
    file_source: FileSource,
    parsed: AnyParse,
}

impl CssPropertyDefinition {
    pub fn matches(&self, name: &str) -> bool {
        decode_css_identifier(self.name.text()) == decode_css_identifier(name)
    }

    pub fn name_token(&self) -> &TokenText {
        &self.name
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn is_globally_scoped(&self) -> bool {
        self.globally_scoped
    }
}

#[salsa::tracked(returns(ref))]
pub(crate) fn css_model_from_parsed_source<'db>(
    db: &'db dyn Db,
    input: SemanticInput<'db>,
) -> SemanticModel {
    let root: AnyCssRoot = input.parsed(db).tree();
    semantic_model(&root)
}

#[salsa::tracked(returns(ref))]
pub(crate) fn css_model_from_parsed_snippet<'db>(
    db: &'db dyn Db,
    input: SemanticInput<'db>,
) -> SemanticModel {
    let root: AnyCssRoot = input.parsed(db).tree();
    semantic_model(&root)
}

/// Returns custom property definitions from a parsed CSS document.
#[salsa::tracked(returns(ref))]
pub fn css_property_definitions_from_source<'db>(
    db: &'db dyn Db,
    input: SemanticInput<'db>,
) -> Vec<CssPropertyDefinition> {
    collect_property_definitions(css_model_from_parsed_source(db, input))
}

/// Returns custom property definitions from an embedded CSS document.
#[salsa::tracked(returns(ref))]
pub fn css_property_definitions_from_snippet<'db>(
    db: &'db dyn Db,
    input: SemanticInput<'db>,
) -> Vec<CssPropertyDefinition> {
    collect_property_definitions(css_model_from_parsed_snippet(db, input))
}

fn collect_property_definitions(model: &SemanticModel) -> Vec<CssPropertyDefinition> {
    let mut definitions = model
        .custom_property_declarations()
        .map(|declaration| {
            let property = declaration.property();
            CssPropertyDefinition {
                name: declaration.name().clone(),
                range: declaration.range(),
                globally_scoped: is_globally_scoped(property.syntax()),
            }
        })
        .chain(
            model
                .global_custom_variables()
                .at_property_registration_candidates()
                .map(|property| CssPropertyDefinition {
                    name: property.name().clone(),
                    range: property.range(),
                    globally_scoped: true,
                }),
        )
        .collect::<Vec<_>>();
    definitions.sort_unstable_by_key(|definition| definition.range.start());
    definitions
}

fn is_globally_scoped(property: &CssSyntaxNode) -> bool {
    let mut containing_rules = property.ancestors().filter_map(|ancestor| {
        if let Some(rule) = CssQualifiedRule::cast(ancestor.clone()) {
            Some(has_standalone_global_selector(rule.prelude().syntax()))
        } else {
            CssNestedQualifiedRule::cast(ancestor)
                .map(|rule| has_standalone_global_selector(rule.prelude().syntax()))
        }
    });
    containing_rules
        .next()
        .is_some_and(|first| first && containing_rules.all(|is_global| is_global))
}

fn has_standalone_global_selector(prelude: &CssSyntaxNode) -> bool {
    prelude
        .descendants()
        .filter_map(AnyCssPseudoClassFunctionSelector::cast)
        .any(|selector| selector.is_global_pseudo() && is_standalone_selector(selector.syntax()))
        || prelude
            .descendants()
            .filter_map(CssPseudoClassIdentifier::cast)
            .any(|selector| {
                is_global_pseudo_identifier(&selector) && is_standalone_selector(selector.syntax())
            })
}

fn is_standalone_selector(selector: &CssSyntaxNode) -> bool {
    if selector.ancestors().any(|ancestor| {
        ancestor != *selector && AnyCssPseudoClassFunctionSelector::can_cast(ancestor.kind())
    }) {
        return false;
    }
    let Some(compound) = selector.ancestors().find_map(CssCompoundSelector::cast) else {
        return false;
    };
    compound.nesting_selectors().is_empty()
        && compound.simple_selector().is_none()
        && compound.sub_selectors().len() == 1
        && !compound
            .syntax()
            .ancestors()
            .any(|ancestor| CssComplexSelector::can_cast(ancestor.kind()))
}

fn is_global_pseudo_identifier(selector: &CssPseudoClassIdentifier) -> bool {
    selector
        .name()
        .ok()
        .and_then(|name| name.as_css_identifier().cloned())
        .and_then(|name| name.value_token().ok())
        .is_some_and(|token| {
            decode_css_identifier(token.text_trimmed()).eq_ignore_ascii_case("global")
        })
}

pub fn css_property_definitions(root: &AnyCssRoot) -> Vec<CssPropertyDefinition> {
    collect_property_definitions(&semantic_model(root))
}

pub fn css_semantic_model<'db>(
    db: &'db dyn Db,
    file: FileSource,
    parse: &AnyParsedSource,
) -> &'db SemanticModel {
    match parse {
        AnyParsedSource::ParsedSource(s) => {
            css_model_from_parsed_source(db, SemanticInput::new(db, file, s.clone()))
        }
        AnyParsedSource::ParsedSnippet(s) => {
            css_model_from_parsed_snippet(db, SemanticInput::new(db, file, s.parsed.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SemanticInput, collect_property_definitions, css_model_from_parsed_source,
        css_property_definitions_from_snippet, css_property_definitions_from_source,
    };
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_css_syntax::property_syntax::{
        PropertySyntax, PropertySyntaxComponentName, PropertySyntaxResult, PropertySyntaxType,
    };
    use biome_db::testing::{
        Events, assert_function_query_was_not_run, assert_function_query_was_run,
    };
    use biome_db::FileSource;
    use biome_languages::css::CssFileSource;
    use biome_languages::{DocumentFileSource, LanguageDb};
    use biome_rowan::TextSize;
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
        fn file_source_for_path(&self, _path: &Utf8Path) -> Option<FileSource> {
            unreachable!("Not used in this test")
        }

        fn for_each_file_source(&self, _f: &mut dyn FnMut(FileSource)) {
            unreachable!("Not used in this test")
        }
    }

    #[salsa::db]
    impl LanguageDb for TestDb {
        fn source_from_index(&self, _index: usize) -> Option<DocumentFileSource> {
            unreachable!("Not used in this test")
        }
    }

    fn make_file(db: &TestDb, source: &str) -> FileSource {
        FileSource::new(
            db,
            Utf8PathBuf::from("test.css"),
            source.to_string(),
            0,
            None,
        )
    }

    #[salsa::tracked(returns(ref))]
    fn test_css_model(db: &dyn LanguageDb, file: FileSource) -> SemanticModel {
        let parsed: AnyParse = parse_css(
            file.content(db),
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into();
        let input = SemanticInput::new(db, file, parsed);
        css_model_from_parsed_source(db, input).clone()
    }

    #[test]
    fn semantic_model_is_memoized() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");

        let _model = test_css_model(&db, file);

        db.clear_salsa_events();
        let _model = test_css_model(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_not_run(&db, test_css_model, file, &events);
    }

    #[test]
    fn property_definition_ranges_track_parse_changes() {
        let mut db = TestDb::new();
        let source = "@property --value { syntax: '<color>'; inherits: true; initial-value: red; }";
        let file = make_file(&db, source);
        let source_input = SemanticInput::new(
            &db,
            file,
            parse_css(source, CssFileSource::css(), CssParserOptions::default()).into(),
        );
        let snippet_input = SemanticInput::new(
            &db,
            file,
            parse_css(source, CssFileSource::css(), CssParserOptions::default()).into(),
        );
        let source_start = css_property_definitions_from_source(&db, source_input)[0]
            .range()
            .start();
        let snippet_start = css_property_definitions_from_snippet(&db, snippet_input)[0]
            .range()
            .start();
        let updated = format!("\n{source}");

        let source_input = SemanticInput::new(
            &db,
            file,
            parse_css(&updated, CssFileSource::css(), CssParserOptions::default()).into(),
        );
        let snippet_input = SemanticInput::new(
            &db,
            file,
            parse_css(&updated, CssFileSource::css(), CssParserOptions::default()).into(),
        );

        assert_eq!(
            css_property_definitions_from_source(&db, source_input)[0]
                .range()
                .start(),
            source_start + TextSize::from(1)
        );
        assert_eq!(
            css_property_definitions_from_snippet(&db, snippet_input)[0]
                .range()
                .start(),
            snippet_start + TextSize::from(1)
        );
    }

    #[test]
    fn escaped_global_selectors_are_globally_scoped() {
        let parse = parse_css(
            r#":g\6c obal(:root) { --function: red; }
:g\6c obal { --identifier: blue; }"#,
            CssFileSource::css(),
            CssParserOptions::default().allow_css_modules(),
        );
        let model = crate::semantic_model(&parse.tree());
        let definitions = collect_property_definitions(&model);

        assert_eq!(definitions.len(), 2);
        assert!(
            definitions
                .iter()
                .all(|definition| definition.globally_scoped)
        );
    }

    #[salsa::tracked]
    fn rule_count(db: &dyn LanguageDb, file: FileSource) -> usize {
        let model = test_css_model(db, file);
        model.rules().len()
    }

    #[salsa::tracked]
    fn property_syntax_type(db: &dyn LanguageDb, file: FileSource) -> Option<PropertySyntaxType> {
        let model = test_css_model(db, file);
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
        let _ = test_css_model(&db, file);

        salsa::Setter::to(
            file.set_content(&mut db),
            "span { color: red; }".to_string(),
        );

        db.clear_salsa_events();
        let _ = test_css_model(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, test_css_model, file, &events);
    }

    #[test]
    fn value_change_does_recompute_downstream() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");
        let count = rule_count(&db, file);
        assert_eq!(count, 1);

        salsa::Setter::to(file.set_content(&mut db), "p { color: blue; }".to_string());

        db.clear_salsa_events();
        let count = rule_count(&db, file);
        assert_eq!(count, 1);
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, test_css_model, file, &events);
        assert_function_query_was_run(&db, rule_count, file, &events);
    }

    #[test]
    fn incomplete_property_value_does_not_panic() {
        let mut db = TestDb::new();
        let file = make_file(&db, ".incomplete {\n  height: 1px\n}\n");

        assert_eq!(rule_count(&db, file), 1);

        salsa::Setter::to(
            file.set_content(&mut db),
            ".incomplete {\n  height:\n}\n".to_string(),
        );

        assert_eq!(rule_count(&db, file), 1);
    }

    #[test]
    fn declaration_count_change_does_recompute_downstream() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");
        let count = rule_count(&db, file);
        assert_eq!(count, 1);

        salsa::Setter::to(
            file.set_content(&mut db),
            "p { color: red; font-size: 12px; }".to_string(),
        );

        db.clear_salsa_events();
        let _ = rule_count(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, test_css_model, file, &events);
        assert_function_query_was_run(&db, rule_count, file, &events);
    }

    #[test]
    fn whitespace_change_does_not_recompute_downstream() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");
        let _ = rule_count(&db, file);

        salsa::Setter::to(
            file.set_content(&mut db),
            "p  {  color:  red;  }".to_string(),
        );

        db.clear_salsa_events();
        let _ = rule_count(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, test_css_model, file, &events);
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

        salsa::Setter::to(
            file.set_content(&mut db),
            r#"@property  --value  { syntax:  "<color>"; inherits:  true; initial-value:  red; }"#
                .to_string(),
        );

        db.clear_salsa_events();
        assert_eq!(
            property_syntax_type(&db, file),
            Some(PropertySyntaxType::Color)
        );
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, test_css_model, file, &events);
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

        salsa::Setter::to(
            file.set_content(&mut db),
            r#"@property --value { syntax: "<length>"; inherits: true; initial-value: 10px; }"#
                .to_string(),
        );

        db.clear_salsa_events();
        assert_eq!(
            property_syntax_type(&db, file),
            Some(PropertySyntaxType::Length)
        );
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, test_css_model, file, &events);
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

        salsa::Setter::to(
            file.set_content(&mut db),
            r#"@property --value { syntax: "<number>"; inherits: true; initial-value: 1; }
@property --value { syntax: "<length>"; inherits: true; initial-value: 1px; }"#
                .to_string(),
        );

        db.clear_salsa_events();
        assert_eq!(
            property_syntax_type(&db, file),
            Some(PropertySyntaxType::Length)
        );
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, test_css_model, file, &events);
        assert_function_query_was_run(&db, property_syntax_type, file, &events);
    }

    #[test]
    fn new_rule_does_recompute_downstream() {
        let mut db = TestDb::new();
        let file = make_file(&db, "p { color: red; }");
        let count = rule_count(&db, file);
        assert_eq!(count, 1);

        salsa::Setter::to(
            file.set_content(&mut db),
            "p { color: red; } span { color: blue; }".to_string(),
        );

        db.clear_salsa_events();
        let count = rule_count(&db, file);
        assert_eq!(count, 2);
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, test_css_model, file, &events);
        assert_function_query_was_run(&db, rule_count, file, &events);
    }
}
