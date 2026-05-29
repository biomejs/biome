use crate::model::SemanticModel;
use crate::semantic_model;
use biome_css_syntax::AnyCssRoot;
use biome_db::{AnyParsedSource, ParsedSnippet, ParsedSource};

#[salsa::db]
pub trait CssSemanticDb: biome_db::Db {}

#[salsa::tracked]
pub(crate) fn css_model_from_parsed_source(
    db: &dyn CssSemanticDb,
    file: ParsedSource,
) -> SemanticModel {
    let parsed: AnyCssRoot = file.parsed(db).tree();
    semantic_model(&parsed)
}

#[salsa::tracked]
pub(crate) fn css_model_from_parsed_snippet(
    db: &dyn CssSemanticDb,
    file: ParsedSnippet,
) -> SemanticModel {
    let parsed: AnyCssRoot = file.parsed(db).tree();
    semantic_model(&parsed)
}

pub fn css_semantic_model<Db>(db: &Db, file: AnyParsedSource) -> SemanticModel
where
    Db: CssSemanticDb,
{
    match file {
        AnyParsedSource::ParsedSource(s) => css_model_from_parsed_source(db, s),
        AnyParsedSource::ParsedSnippet(s) => css_model_from_parsed_snippet(db, s),
    }
}

#[cfg(test)]
mod tests {
    use super::{CssSemanticDb, css_model_from_parsed_source};
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_db::ParsedSource;
    use biome_db::testing::{
        Events, assert_function_query_was_not_run, assert_function_query_was_run,
    };
    use biome_languages::css::CssFileSource;
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
    impl CssSemanticDb for TestDb {}

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
    fn rule_count(db: &dyn CssSemanticDb, file: ParsedSource) -> usize {
        let model = css_model_from_parsed_source(db, file);
        model.rules().len()
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
    fn value_change_does_not_recompute_downstream() {
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
        assert_function_query_was_not_run(&db, rule_count, file, &events);
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
