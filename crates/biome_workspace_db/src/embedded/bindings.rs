use crate::embedded::EmbeddedDb;
use biome_rowan::{TextRange, TokenText};
use camino::Utf8PathBuf;

#[salsa::input]
#[derive(Debug)]
pub struct EmbeddedBinding {
    /// The range of the binding
    #[returns(clone)]
    pub range: TextRange,
    /// The text of the binding
    #[returns(clone)]
    pub text: TokenText,
    /// Optionally, the source of the binding. It represents the path of the import/dynamic import.
    #[returns(ref)]
    pub source: Option<TokenText>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedBinding {
    #[returns(ref)]
    path: Utf8PathBuf,

    #[returns(ref)]
    name: TokenText,
}

#[salsa::tracked(returns(ref))]
pub fn get_binding_by_name<'db>(
    db: &'db dyn EmbeddedDb,
    binding_name: InternedBinding<'db>,
) -> Option<EmbeddedBinding> {
    for bindings in db.bindings(binding_name.path(db)) {
        for binding in bindings {
            if binding.text(db).text() == *binding_name.name(db) {
                return Some(binding);
            }
        }
    }
    None
}

#[salsa::tracked(returns(ref))]
pub fn get_binding_with_source<'db>(
    db: &'db dyn EmbeddedDb,
    binding_name: InternedBinding<'db>,
) -> Option<EmbeddedBinding> {
    for bindings in db.bindings(binding_name.path(db)) {
        for binding in bindings {
            if binding.text(db).text() == *binding_name.name(db) && binding.source(db).is_some() {
                return Some(binding);
            }
        }
    }
    None
}

// #[salsa::tracked(returns(ref))]
// pub fn bindings_without_source(db: &dyn EmbeddedDb) -> Vec<Vec<(TextRange, TokenText)>> {
//     db.bindings()
//         .into_iter()
//         .map(|bindings| {
//             bindings
//                 .into_iter()
//                 .map(|b| (b.range(db), b.text(db)))
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedded::references::EmbeddedValueReference;
    use biome_db::testing::{Events, assert_function_query_was_not_run};
    use biome_rowan::{RawSyntaxKind, TextSize};
    use camino::Utf8Path;
    use papaya::HashMap;
    use salsa::Storage;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        bindings: HashMap<Utf8PathBuf, Vec<Vec<EmbeddedBinding>>>,
        events: Events,
        storage: Storage<Self>,
    }

    impl TestDb {
        fn new() -> Self {
            let events = Events::default();
            Self {
                bindings: HashMap::new(),
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

        fn insert_bindings(&mut self, path: Utf8PathBuf, bindings: Vec<Vec<EmbeddedBinding>>) {
            self.bindings.pin().insert(path, bindings);
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
        fn bindings(&self, path: &Utf8Path) -> Vec<Vec<EmbeddedBinding>> {
            self.bindings.pin().get(path).cloned().unwrap_or_default()
        }

        fn references(&self, _path: &Utf8Path) -> Vec<Vec<EmbeddedValueReference>> {
            Vec::new()
        }
    }

    fn token_text(text: &str) -> TokenText {
        TokenText::new_raw(RawSyntaxKind(0), text)
    }

    fn range(start: u32, end: u32) -> TextRange {
        TextRange::new(TextSize::from(start), TextSize::from(end))
    }

    fn binding(db: &TestDb, name: &str, range: TextRange, source: Option<&str>) -> EmbeddedBinding {
        EmbeddedBinding::new(db, range, token_text(name), source.map(token_text))
    }

    #[test]
    fn get_binding_by_name_finds_matching_binding() {
        let mut db = TestDb::new();
        let path = Utf8PathBuf::from("src/App.vue");
        db.insert_bindings(
            path.clone(),
            vec![vec![
                binding(&db, "Local", range(0, 5), None),
                binding(&db, "Component", range(10, 19), Some("./Component.vue")),
            ]],
        );

        let found = get_binding_by_name(
            &db,
            InternedBinding::new(&db, path.clone(), token_text("Local")),
        )
        .expect("binding should exist");

        assert_eq!(found.range(&db), range(0, 5));
        assert_eq!(found.text(&db).text(), "Local");
    }

    #[test]
    fn get_binding_with_source_ignores_local_bindings() {
        let mut db = TestDb::new();
        let path = Utf8PathBuf::from("src/App.vue");
        db.insert_bindings(
            path.clone(),
            vec![vec![
                binding(&db, "Local", range(0, 5), None),
                binding(&db, "Component", range(10, 19), Some("./Component.vue")),
            ]],
        );

        assert!(
            get_binding_with_source(
                &db,
                InternedBinding::new(&db, path.clone(), token_text("Local"))
            )
            .is_none()
        );

        let found = get_binding_with_source(
            &db,
            InternedBinding::new(&db, path.clone(), token_text("Component")),
        )
        .expect("imported binding should exist");

        assert_eq!(found.text(&db).text(), "Component");
        assert_eq!(
            found.source(&db).as_ref().map(TokenText::text),
            Some("./Component.vue")
        );
    }

    // #[test]
    // fn bindings_without_source_preserves_groups_and_text() {
    //     let mut db = TestDb::new();
    //     db.insert_bindings(vec![
    //         vec![binding(&db, "first", range(0, 5), None)],
    //         vec![binding(&db, "second", range(10, 16), Some("./second"))],
    //     ]);
    //
    //     let groups = bindings_without_source(&db);
    //
    //     assert_eq!(groups.len(), 2);
    //     assert_eq!(groups[0][0].0, range(0, 5));
    //     assert_eq!(groups[0][0].1.text(), "first");
    //     assert_eq!(groups[1][0].0, range(10, 16));
    //     assert_eq!(groups[1][0].1.text(), "second");
    // }

    #[test]
    fn get_binding_by_name_is_memoized() {
        let mut db = TestDb::new();
        let path = Utf8PathBuf::from("src/App.vue");
        db.insert_bindings(
            path.clone(),
            vec![vec![binding(&db, "Local", range(0, 5), None)]],
        );
        let name = InternedBinding::new(&db, path, token_text("Local"));

        let _ = get_binding_by_name(&db, name);

        db.clear_salsa_events();
        let _ = get_binding_by_name(&db, name);
        let events = db.take_salsa_events();

        assert_function_query_was_not_run(&db, get_binding_by_name, name, &events);
    }
}
