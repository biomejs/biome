use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_db::Source;
use biome_embeds::EmbeddedData;
use biome_embeds::bindings::{
    InternedBindingText, InternedBindingTokenText, get_binding_by_name, get_binding_by_text,
};
use biome_embeds::components::{ComponentNameSegments, component_at};
use biome_embeds::references::{
    InternedReference, is_reference_used, is_svelte_store_reference_used, is_type_reference_used,
    is_value_reference_used, is_vue_directive_reference_used,
};
use biome_languages::{LanguageDb, has_embedded_content};
use biome_rowan::{TextRange, TextSize, TokenText};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct EmbeddedService {
    source: EmbeddedSource,
    /// The offset of the analyzed snippet in its host document.
    snippet_offset: TextSize,
}

#[derive(Clone)]
enum EmbeddedSource {
    Empty,
    Workspace {
        db: Rc<dyn LanguageDb>,
        /// The host document of the analyzed source.
        document: Source,
    },
    Interned(Arc<EmbeddedData>),
}

impl EmbeddedService {
    /// Creates the service for the analyzed source of `document`, its host
    /// document in `db`. Without a document, the service finds no embedded
    /// content.
    pub(crate) fn new(
        db: Rc<dyn LanguageDb>,
        document: Option<Source>,
        snippet_offset: TextSize,
    ) -> Self {
        let source = match document {
            Some(document) if has_embedded_content(db.as_ref(), document) => {
                EmbeddedSource::Workspace { db, document }
            }
            _ => EmbeddedSource::Empty,
        };
        Self {
            source,
            snippet_offset,
        }
    }

    pub(crate) fn from_data(data: Arc<EmbeddedData>, snippet_offset: TextSize) -> Self {
        Self {
            source: EmbeddedSource::Interned(data),
            snippet_offset,
        }
    }

    pub(crate) fn contains_binding(&self, binding: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Empty => false,
            EmbeddedSource::Workspace { db, document } => get_binding_by_name(
                db.as_ref(),
                InternedBindingTokenText::new(db.as_ref(), *document, binding),
            )
            .is_some(),
            EmbeddedSource::Interned(data) => data.contains_binding(binding.text()),
        }
    }

    pub(crate) fn contains_binding_text(&self, binding: &str) -> bool {
        match &self.source {
            EmbeddedSource::Empty => false,
            EmbeddedSource::Workspace { db, document } => get_binding_by_text(
                db.as_ref(),
                InternedBindingText::new(db.as_ref(), *document, binding.to_string()),
            )
            .is_some(),
            EmbeddedSource::Interned(data) => data.contains_binding(binding),
        }
    }

    pub(crate) fn is_used_as_value(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Empty => false,
            EmbeddedSource::Workspace { db, document } => is_value_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), *document, identifier),
            ),
            EmbeddedSource::Interned(data) => data.is_used_as_value(identifier.text()),
        }
    }

    pub(crate) fn is_used_as_type(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Empty => false,
            EmbeddedSource::Workspace { db, document } => is_type_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), *document, identifier),
            ),
            EmbeddedSource::Interned(data) => data.is_used_as_type(identifier.text()),
        }
    }

    pub(crate) fn is_used(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Empty => false,
            EmbeddedSource::Workspace { db, document } => is_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), *document, identifier),
            ),
            EmbeddedSource::Interned(data) => data.is_used(identifier.text()),
        }
    }

    /// Svelte stores are a special case. The `$` prefix is used to "dereference" the store and get its value.
    ///
    /// See also: https://svelte.dev/docs/svelte/stores
    pub(crate) fn is_svelte_store_used(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Empty => false,
            EmbeddedSource::Workspace { db, document } => is_svelte_store_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), *document, identifier),
            ),
            EmbeddedSource::Interned(data) => data.is_svelte_store_used(identifier.text()),
        }
    }

    /// Vue custom directives are a special case. The template spells them in
    /// kebab-case (e.g. `v-highlight`), while the JS binding they refer to is
    /// spelled in camelCase (e.g. `vHighlight`).
    ///
    /// See also: https://vuejs.org/guide/reusability/custom-directives.html
    pub(crate) fn is_vue_directive_used(&self, identifier: TokenText) -> bool {
        match &self.source {
            EmbeddedSource::Empty => false,
            EmbeddedSource::Workspace { db, document } => is_vue_directive_reference_used(
                db.as_ref(),
                InternedReference::new(db.as_ref(), *document, identifier),
            ),
            EmbeddedSource::Interned(data) => data.is_vue_directive_used(identifier.text()),
        }
    }

    /// Returns the name segments of the component whose tag in the host
    /// document contains `range`, such as `Card` and `Root` for `<Card.Root>`.
    /// `range` is relative to the analyzed snippet.
    ///
    /// Returns `None` if the tag is a native element, or if no tag contains
    /// `range`.
    pub(crate) fn component_at(&self, range: TextRange) -> Option<ComponentNameSegments> {
        let range = range + self.snippet_offset;
        match &self.source {
            EmbeddedSource::Empty => None,
            EmbeddedSource::Workspace { db, document } => {
                component_at(db.as_ref(), *document, range)
            }
            EmbeddedSource::Interned(data) => data.component_at(range).cloned(),
        }
    }
}

impl std::fmt::Debug for EmbeddedService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmbeddedService").finish_non_exhaustive()
    }
}

impl FromServices for EmbeddedService {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let service: &Self = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["EmbeddedService"]))?;

        Ok(service.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_db::testing::Events;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_languages::{DocumentFileSource, JsFileSource};
    use biome_parser::db::{ParseDb, SnippetParse, SourceParse};
    use biome_rowan::RawSyntaxKind;
    use camino::{Utf8Path, Utf8PathBuf};
    use salsa::{EventKind, Storage};

    #[salsa::db]
    struct TestDb {
        parsed: Option<Source>,
        /// The texts of the snippets of the document, each at its last
        /// occurrence in the document.
        snippets: Vec<String>,
        storage: Storage<Self>,
    }

    impl TestDb {
        fn new(events: Events) -> Self {
            Self {
                parsed: None,
                snippets: Vec::new(),
                storage: Storage::new(Some(Box::new(move |event| {
                    events.0.lock().unwrap().push(event)
                }))),
            }
        }
    }

    #[salsa::db]
    impl LanguageDb for TestDb {
        fn source_from_index(&self, _index: usize) -> Option<DocumentFileSource> {
            Some(DocumentFileSource::Js(JsFileSource::js_module()))
        }
    }

    #[salsa::db]
    impl ParseDb for TestDb {
        fn parse_source(&self, source: Source) -> SourceParse {
            let text = source.text(self);
            let snippets = self
                .snippets
                .iter()
                .filter_map(|snippet| {
                    let start = TextSize::from(text.rfind(snippet.as_str())? as u32);
                    let range = TextRange::at(start, TextSize::of(snippet.as_str()));
                    let parsed = parse(
                        snippet,
                        JsFileSource::js_module(),
                        JsParserOptions::default(),
                    );
                    Some(SnippetParse::new(parsed.into(), range, range, start, 0))
                })
                .collect();
            let parsed = parse(text, JsFileSource::js_module(), JsParserOptions::default());
            SourceParse::new(parsed.into(), 0).with_snippets(snippets)
        }
    }

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<Source> {
            self.parsed
        }
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    /// Builds the service for a JavaScript module and returns it with the
    /// event log of its database. Each entry of `snippets` becomes an embedded
    /// snippet at its last occurrence in `source`.
    fn service_for(source: &str, snippets: &[&str]) -> (EmbeddedService, Events) {
        let events = Events::default();
        let mut db = TestDb::new(events.clone());
        db.snippets = snippets.iter().map(ToString::to_string).collect();
        let path = Utf8PathBuf::from("src/file.js");
        let document = Source::new(&db, path, source.into(), 0);
        db.parsed = Some(document);

        let service = EmbeddedService::new(Rc::new(db), Some(document), TextSize::default());
        (service, events)
    }

    fn token_text(text: &str) -> TokenText {
        TokenText::new_raw(RawSyntaxKind(0), text)
    }

    #[test]
    fn document_without_embedded_content_answers_without_the_database() {
        let source = "const used = 1; used;";
        let (service, events) = service_for(source, &[]);
        events.0.lock().unwrap().clear();

        assert!(!service.contains_binding(token_text("used")));
        assert!(!service.contains_binding_text("used"));
        assert!(!service.is_used(token_text("used")));
        assert!(!service.is_used_as_value(token_text("used")));
        assert!(!service.is_used_as_type(token_text("used")));
        assert!(!service.is_svelte_store_used(token_text("used")));
        assert!(!service.is_vue_directive_used(token_text("vUsed")));
        assert!(
            service
                .component_at(TextRange::up_to(TextSize::of(source)))
                .is_none()
        );

        let events = events.0.lock().unwrap();
        let database_work = events.iter().find(|event| {
            matches!(
                event.kind,
                EventKind::WillExecute { .. } | EventKind::DidInternValue { .. }
            )
        });
        assert!(database_work.is_none(), "{database_work:?}");
    }

    #[test]
    fn document_with_a_snippet_finds_its_references() {
        let (service, _) = service_for("const used = 1; html`${used}`;", &["used"]);

        assert!(service.is_used(token_text("used")));
        assert!(service.is_used_as_value(token_text("used")));
        assert!(!service.is_used(token_text("missing")));
    }
}
