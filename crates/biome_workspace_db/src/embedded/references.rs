use biome_rowan::{TextRange, TokenText};
use camino::Utf8PathBuf;

#[salsa::input]
pub struct EmbeddedValueReference {
    /// Where it's been used
    pub range: TextRange,

    /// The text of the reference
    pub text: TokenText,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedReference {
    #[returns(ref)]
    path: Utf8PathBuf,

    #[returns(ref)]
    name: TokenText,
}

#[salsa::tracked]
pub fn is_value_reference_used(
    db: &dyn crate::embedded::EmbeddedDb,
    reference: InternedReference<'_>,
) -> bool {
    db.references(reference.path(db)).iter().any(|refs| {
        refs.iter()
            .any(|value_reference| value_reference.text(db).text() == *reference.name(db))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedded::EmbeddedDb;
    use crate::embedded::bindings::EmbeddedBinding;
    use biome_db::testing::{Events, assert_function_query_was_not_run};
    use biome_rowan::{RawSyntaxKind, TextSize};
    use camino::Utf8Path;
    use papaya::HashMap;
    use salsa::Storage;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        references: HashMap<Utf8PathBuf, Vec<Vec<EmbeddedValueReference>>>,
        events: Events,
        storage: Storage<Self>,
    }

    impl TestDb {
        fn new() -> Self {
            let events = Events::default();
            Self {
                references: HashMap::new(),
                storage: salsa::Storage::new(Some(Box::new({
                    let events = events.clone();
                    move |event| {
                        events.0.lock().unwrap().push(event);
                    }
                }))),
                events,
            }
        }

        fn take_salsa_events(&self) -> Vec<salsa::Event> {
            std::mem::take(&mut *self.events.0.lock().unwrap())
        }

        fn clear_salsa_events(&self) {
            self.take_salsa_events();
        }

        fn insert_references(
            &mut self,
            path: Utf8PathBuf,
            references: Vec<Vec<EmbeddedValueReference>>,
        ) {
            self.references.pin().insert(path, references);
        }
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<biome_db::ParsedSource> {
            None
        }
    }

    #[salsa::db]
    impl EmbeddedDb for TestDb {
        fn bindings(&self, _path: &Utf8Path) -> Vec<Vec<EmbeddedBinding>> {
            Vec::new()
        }

        fn references(&self, path: &Utf8Path) -> Vec<Vec<EmbeddedValueReference>> {
            self.references.pin().get(path).cloned().unwrap_or_default()
        }
    }

    fn token_text(text: &str) -> TokenText {
        TokenText::new_raw(RawSyntaxKind(0), text)
    }

    fn range(start: u32, end: u32) -> TextRange {
        TextRange::new(TextSize::from(start), TextSize::from(end))
    }

    fn reference(db: &TestDb, name: &str, range: TextRange) -> EmbeddedValueReference {
        EmbeddedValueReference::new(db, range, token_text(name))
    }

    #[test]
    fn is_value_reference_used_finds_references_across_groups() {
        let mut db = TestDb::new();
        let path = Utf8PathBuf::from("/file.html");
        db.insert_references(
            path.clone(),
            vec![
                vec![reference(&db, "First", range(0, 5))],
                vec![reference(&db, "Second", range(10, 16))],
            ],
        );

        assert!(is_value_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("First"))
        ));
        assert!(is_value_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("Second"))
        ));
        assert!(!is_value_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("Missing"))
        ));
    }

    #[test]
    fn is_value_reference_used_is_memoized() {
        let mut db = TestDb::new();
        let path = Utf8PathBuf::from("/file.html");
        db.insert_references(
            path.clone(),
            vec![vec![reference(&db, "First", range(0, 5))]],
        );
        let reference = InternedReference::new(&db, path.clone(), token_text("First"));

        let _ = is_value_reference_used(&db, reference);

        db.clear_salsa_events();
        let _ = is_value_reference_used(&db, reference);
        let events = db.take_salsa_events();

        assert_function_query_was_not_run(&db, is_value_reference_used, reference, &events);
    }
}
