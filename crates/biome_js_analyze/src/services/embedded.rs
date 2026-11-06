use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use biome_astro_semantic::{
    Access, AstroSemanticModel, AstroSemanticModelOptions, Namespace, RefSite, Reference,
    Resolution, Snippet, Symbol, astro_model_from_source_with_options,
};
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
use biome_js_syntax::{JsSyntaxToken, unescape_js_identifier};
use biome_languages::{DocumentFileSource, LanguageDb, has_embedded_content};
use biome_parser::db::SourceExt;
use biome_rowan::{TextRange, TextSize, TokenText};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct EmbeddedService {
    source: EmbeddedSource,
    semantic_model: Option<EmbeddedSemanticModel>,
    /// The offset of the analyzed snippet in its host document.
    snippet_offset: TextSize,
    semantic_model_is_current: bool,
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

#[derive(Clone)]
enum EmbeddedSemanticModel {
    Astro(AstroSemanticModel),
    // TODO: Add Vue and Svelte when their analyzer integrations use native semantic models.
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct EmbeddedBindingUsage {
    any: bool,
    read: bool,
    write: bool,
    writes_known: bool,
    value: bool,
    ty: bool,
}

impl EmbeddedBindingUsage {
    pub(crate) fn any(self) -> bool {
        self.any
    }

    pub(crate) fn is_read(self) -> bool {
        self.read
    }

    pub(crate) fn is_written(self) -> bool {
        self.write
    }

    pub(crate) fn writes_are_known(self) -> bool {
        self.writes_known
    }

    pub(crate) fn is_used_as_value(self) -> bool {
        self.value
    }

    pub(crate) fn is_used_as_type(self) -> bool {
        self.ty
    }
}

fn astro_contains_binding(snippet: Option<&Snippet>, name: &str) -> bool {
    let name = unescape_js_identifier(name);
    let Some(scope) = snippet.and_then(Snippet::scope) else {
        return false;
    };
    [Namespace::Value, Namespace::Type, Namespace::Namespace]
        .into_iter()
        .any(|namespace| {
            scope.lookup(namespace, name.as_ref()).is_some() || scope.is_open(namespace)
        })
}

fn is_cross_language_use(reference: &Reference, snippet: &Snippet) -> bool {
    reference.site() == RefSite::Implicit || reference.snippet().as_ref() != Some(snippet)
}

fn astro_is_used(snippet: Option<&Snippet>, name: &str, namespaces: &[Namespace]) -> bool {
    let name = unescape_js_identifier(name);
    let Some(snippet) = snippet else {
        return false;
    };
    let Some(scope) = snippet.scope() else {
        return false;
    };
    namespaces.iter().copied().any(|namespace| {
        scope.is_open(namespace)
            || scope
                .lookup(namespace, name.as_ref())
                .is_some_and(|symbol| {
                    symbol.uses().any(|reference| {
                        reference.namespace() == namespace
                            && is_cross_language_use(&reference, snippet)
                    })
                })
    })
}

fn astro_symbol(
    snippet: Option<&Snippet>,
    token: &JsSyntaxToken,
    snippet_offset: TextSize,
) -> Option<Symbol> {
    snippet?
        .scope()?
        .symbol_at(token.text_trimmed_range() + snippet_offset)
}

impl EmbeddedSemanticModel {
    fn is_known_unresolved_reference(
        &self,
        snippet_offset: TextSize,
        semantic_model_is_current: bool,
        range: TextRange,
    ) -> bool {
        match self {
            Self::Astro(model) => {
                if !semantic_model_is_current || model.snippet_at_offset(snippet_offset).is_none() {
                    return false;
                }
                model
                    .reference_at(range + snippet_offset)
                    .is_some_and(|reference| reference.resolution() == Resolution::Unresolved)
            }
        }
    }

    fn contains_binding(
        &self,
        snippet_offset: TextSize,
        semantic_model_is_current: bool,
        name: &str,
    ) -> bool {
        if !semantic_model_is_current {
            return true;
        }
        match self {
            Self::Astro(model) => {
                astro_contains_binding(model.snippet_at_offset(snippet_offset).as_ref(), name)
            }
        }
    }

    fn binding_usage(
        &self,
        snippet_offset: TextSize,
        semantic_model_is_current: bool,
        binding: &JsSyntaxToken,
    ) -> Option<EmbeddedBindingUsage> {
        match self {
            Self::Astro(model) => {
                if !semantic_model_is_current {
                    return None;
                }
                let snippet = model.snippet_at_offset(snippet_offset)?;
                let scope = snippet.scope()?;
                if [Namespace::Value, Namespace::Type, Namespace::Namespace]
                    .into_iter()
                    .any(|namespace| scope.is_open(namespace))
                {
                    return Some(EmbeddedBindingUsage {
                        any: true,
                        read: true,
                        write: true,
                        writes_known: false,
                        value: true,
                        ty: true,
                    });
                }
                let Some(symbol) = astro_symbol(Some(&snippet), binding, snippet_offset) else {
                    return Some(EmbeddedBindingUsage {
                        writes_known: true,
                        ..EmbeddedBindingUsage::default()
                    });
                };
                let mut usage = EmbeddedBindingUsage {
                    writes_known: true,
                    ..EmbeddedBindingUsage::default()
                };
                for reference in symbol
                    .uses()
                    .filter(|reference| is_cross_language_use(reference, &snippet))
                {
                    usage.any = true;
                    usage.read |= reference.access() != Access::Write;
                    usage.write |= matches!(reference.access(), Access::Write | Access::Mutate);
                    if reference.access() == Access::Type {
                        usage.ty = true;
                    } else {
                        usage.value = true;
                    }
                }
                Some(usage)
            }
        }
    }

    fn contains_binding_declaration(&self, _binding: &JsSyntaxToken) -> bool {
        match self {
            Self::Astro(_) => false,
        }
    }

    fn is_binding_used(
        &self,
        snippet_offset: TextSize,
        semantic_model_is_current: bool,
        binding: &JsSyntaxToken,
    ) -> bool {
        self.binding_usage(snippet_offset, semantic_model_is_current, binding)
            .is_none_or(EmbeddedBindingUsage::any)
    }

    fn is_binding_read(
        &self,
        snippet_offset: TextSize,
        semantic_model_is_current: bool,
        binding: &JsSyntaxToken,
    ) -> bool {
        self.binding_usage(snippet_offset, semantic_model_is_current, binding)
            .is_none_or(EmbeddedBindingUsage::is_read)
    }

    fn is_used_as_value(
        &self,
        snippet_offset: TextSize,
        semantic_model_is_current: bool,
        name: &str,
    ) -> bool {
        if !semantic_model_is_current {
            return true;
        }
        match self {
            Self::Astro(model) => astro_is_used(
                model.snippet_at_offset(snippet_offset).as_ref(),
                name,
                &[Namespace::Value],
            ),
        }
    }

    fn is_used_as_type(
        &self,
        snippet_offset: TextSize,
        semantic_model_is_current: bool,
        name: &str,
    ) -> bool {
        if !semantic_model_is_current {
            return true;
        }
        match self {
            Self::Astro(model) => astro_is_used(
                model.snippet_at_offset(snippet_offset).as_ref(),
                name,
                &[Namespace::Type, Namespace::Namespace],
            ),
        }
    }

    fn is_used(
        &self,
        snippet_offset: TextSize,
        semantic_model_is_current: bool,
        name: &str,
    ) -> bool {
        if !semantic_model_is_current {
            return true;
        }
        match self {
            Self::Astro(model) => astro_is_used(
                model.snippet_at_offset(snippet_offset).as_ref(),
                name,
                &[Namespace::Value, Namespace::Type, Namespace::Namespace],
            ),
        }
    }

    fn is_svelte_store_used(&self, _snippet_offset: TextSize, _name: &str) -> bool {
        match self {
            Self::Astro(_) => false,
        }
    }

    fn is_vue_directive_used(&self, _snippet_offset: TextSize, _name: &str) -> bool {
        match self {
            Self::Astro(_) => false,
        }
    }
}

impl EmbeddedService {
    /// Creates the service for a source whose host document in `db` is
    /// `document`. Without a document, the service finds no embedded content.
    pub(crate) fn new_with_globals(
        db: Rc<dyn LanguageDb>,
        document: Option<Source>,
        snippet_offset: TextSize,
        semantic_model_is_current: bool,
        globals: &[Box<str>],
    ) -> Self {
        let is_astro = document.is_some_and(|document| {
            matches!(
                db.source_from_index(document.document_source_index(db.as_ref())),
                Some(DocumentFileSource::Html(html)) if html.is_astro()
            )
        });
        let semantic_model = if is_astro {
            document.map(|document| {
                EmbeddedSemanticModel::Astro(astro_model_from_source_with_options(
                    db.as_ref(),
                    document,
                    AstroSemanticModelOptions {
                        globals: globals.to_vec(),
                    },
                ))
            })
        } else {
            None
        };
        let source = match document {
            Some(document) if is_astro || has_embedded_content(db.as_ref(), document) => {
                EmbeddedSource::Workspace { db, document }
            }
            _ => EmbeddedSource::Empty,
        };
        Self {
            source,
            semantic_model,
            snippet_offset,
            semantic_model_is_current,
        }
    }

    pub(crate) fn from_data(
        data: Arc<EmbeddedData>,
        snippet_offset: TextSize,
        semantic_model_is_current: bool,
    ) -> Self {
        let semantic_model = data
            .astro_model()
            .cloned()
            .map(EmbeddedSemanticModel::Astro);
        Self {
            source: EmbeddedSource::Interned(data),
            semantic_model,
            snippet_offset,
            semantic_model_is_current,
        }
    }

    pub(crate) fn is_known_unresolved_reference(&self, range: TextRange) -> Option<bool> {
        self.semantic_model.as_ref().map(|model| {
            model.is_known_unresolved_reference(
                self.snippet_offset,
                self.semantic_model_is_current,
                range,
            )
        })
    }

    pub(crate) fn contains_binding_declaration(&self, binding: &JsSyntaxToken) -> bool {
        self.semantic_model.as_ref().map_or_else(
            || self.contains_binding(binding.token_text_trimmed()),
            |model| model.contains_binding_declaration(binding),
        )
    }

    pub(crate) fn binding_usage(&self, binding: &JsSyntaxToken) -> Option<EmbeddedBindingUsage> {
        if let Some(model) = &self.semantic_model {
            return model.binding_usage(
                self.snippet_offset,
                self.semantic_model_is_current,
                binding,
            );
        }
        let name = binding.token_text_trimmed();
        let value = self.is_used_as_value(name.clone());
        let ty = self.is_used_as_type(name.clone());
        Some(EmbeddedBindingUsage {
            any: value || ty,
            read: value || ty,
            value,
            ty,
            ..EmbeddedBindingUsage::default()
        })
    }

    pub(crate) fn is_binding_used(&self, binding: &JsSyntaxToken) -> bool {
        self.semantic_model.as_ref().map_or_else(
            || self.is_used(binding.token_text_trimmed()),
            |model| {
                model.is_binding_used(self.snippet_offset, self.semantic_model_is_current, binding)
            },
        )
    }

    pub(crate) fn is_binding_read(&self, binding: &JsSyntaxToken) -> bool {
        self.semantic_model.as_ref().map_or_else(
            || self.is_used(binding.token_text_trimmed()),
            |model| {
                model.is_binding_read(self.snippet_offset, self.semantic_model_is_current, binding)
            },
        )
    }

    pub(crate) fn contains_binding(&self, binding: TokenText) -> bool {
        if let Some(model) = &self.semantic_model {
            return model.contains_binding(
                self.snippet_offset,
                self.semantic_model_is_current,
                binding.text(),
            );
        }
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
        if let Some(model) = &self.semantic_model {
            return model.contains_binding(
                self.snippet_offset,
                self.semantic_model_is_current,
                binding,
            );
        }
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
        if let Some(model) = &self.semantic_model {
            return model.is_used_as_value(
                self.snippet_offset,
                self.semantic_model_is_current,
                identifier.text(),
            );
        }
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
        if let Some(model) = &self.semantic_model {
            return model.is_used_as_type(
                self.snippet_offset,
                self.semantic_model_is_current,
                identifier.text(),
            );
        }
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
        if let Some(model) = &self.semantic_model {
            return model.is_used(
                self.snippet_offset,
                self.semantic_model_is_current,
                identifier.text(),
            );
        }
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
        if let Some(model) = &self.semantic_model {
            return model.is_svelte_store_used(self.snippet_offset, identifier.text());
        }
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
        if let Some(model) = &self.semantic_model {
            return model.is_vue_directive_used(self.snippet_offset, identifier.text());
        }
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
    use biome_embeds::{EmbeddedSnippet, collect_embedded_data};
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_js_parser::{JsParserOptions, parse, parse_js_with_offset};
    use biome_js_syntax::AnyJsRoot;
    use biome_languages::javascript::JsEmbeddingKind;
    use biome_languages::{DocumentFileSource, HtmlFileSource, JsFileSource};
    use biome_parser::AnyParse;
    use biome_parser::db::{ParseDb, SnippetOutput, SourceOutput};
    use biome_rowan::{AstNode, Direction, RawSyntaxKind};
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
        fn parse_source(&self, source: Source) -> SourceOutput {
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
                    Some(SnippetOutput::new(parsed.into(), range, range, start, 0))
                })
                .collect();
            let parsed = parse(text, JsFileSource::js_module(), JsParserOptions::default());
            SourceOutput::new(parsed.into(), 0).with_snippets(snippets)
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

        let service = EmbeddedService::new_with_globals(
            Rc::new(db),
            Some(document),
            TextSize::default(),
            true,
            &[],
        );
        (service, events)
    }

    const ASTRO_SOURCE: usize = 0;
    const ASTRO_FRONTMATTER_SOURCE: usize = 1;
    const ASTRO_EXPRESSION_SOURCE: usize = 2;
    const ASTRO_SCRIPT_SOURCE: usize = 3;

    #[salsa::db]
    struct AstroTestDb {
        parsed: Option<Source>,
        snippets: Vec<(&'static str, usize)>,
        storage: Storage<Self>,
    }

    impl AstroTestDb {
        fn new(snippets: Vec<(&'static str, usize)>) -> Self {
            Self {
                parsed: None,
                snippets,
                storage: Storage::default(),
            }
        }
    }

    #[salsa::db]
    impl LanguageDb for AstroTestDb {
        fn source_from_index(&self, index: usize) -> Option<DocumentFileSource> {
            Some(match index {
                ASTRO_SOURCE => DocumentFileSource::Html(HtmlFileSource::astro()),
                ASTRO_FRONTMATTER_SOURCE => DocumentFileSource::Js(JsFileSource::astro()),
                ASTRO_EXPRESSION_SOURCE => DocumentFileSource::Js(
                    JsFileSource::tsx().with_embedding_kind(JsEmbeddingKind::Astro {
                        frontmatter: false,
                        is_class_attribute: false,
                    }),
                ),
                ASTRO_SCRIPT_SOURCE => DocumentFileSource::Js(JsFileSource::js_module()),
                _ => return None,
            })
        }
    }

    #[salsa::db]
    impl ParseDb for AstroTestDb {
        fn parse_source(&self, source: Source) -> SourceOutput {
            let text = source.text(self);
            let source_index = source.file_source_index(self);
            let parse = parse_html(text, HtmlParserOptions::from(&HtmlFileSource::astro()));
            let snippets = self
                .snippets
                .iter()
                .filter_map(|(snippet, index)| {
                    let start = TextSize::from(text.rfind(snippet)? as u32);
                    let range = TextRange::at(start, TextSize::of(*snippet));
                    let source = self.source_from_index(*index)?.to_js_file_source()?;
                    let parsed =
                        parse_js_with_offset(snippet, start, source, JsParserOptions::default());
                    Some(SnippetOutput::new(
                        parsed.into(),
                        range,
                        range,
                        start,
                        *index,
                    ))
                })
                .collect();
            SourceOutput::new(parse.into(), source_index).with_snippets(snippets)
        }
    }

    #[salsa::db]
    impl biome_db::Db for AstroTestDb {
        fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<Source> {
            self.parsed
        }
    }

    #[salsa::db]
    impl salsa::Database for AstroTestDb {}

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

    #[test]
    fn malformed_astro_snippets_are_conservatively_used() {
        const FRONTMATTER: &str = "const visible = 1;\nconst broken =";
        let source = format!("---\n{FRONTMATTER}\n---\n");
        let frontmatter_offset = TextSize::from(source.find(FRONTMATTER).unwrap() as u32);
        let mut db = AstroTestDb::new(vec![(FRONTMATTER, ASTRO_FRONTMATTER_SOURCE)]);
        let document = Source::new(
            &db,
            Utf8PathBuf::from("src/component.astro"),
            source.into(),
            ASTRO_SOURCE,
        );
        db.parsed = Some(document);
        let service = EmbeddedService::new_with_globals(
            Rc::new(db),
            Some(document),
            frontmatter_offset,
            true,
            &[],
        );

        assert!(service.contains_binding_text("possiblyHidden"));
        assert!(service.is_used(token_text("possiblyHidden")));
        assert!(service.is_used_as_value(token_text("possiblyHidden")));
        assert!(service.is_used_as_type(token_text("possiblyHidden")));
    }

    #[test]
    fn workspace_astro_model_applies_configured_globals() {
        const SCRIPT: &str = "configuredGlobal;";
        let source = format!("<script>{SCRIPT}</script>\n");
        let mut db = AstroTestDb::new(vec![(SCRIPT, ASTRO_SCRIPT_SOURCE)]);
        let document = Source::new(
            &db,
            Utf8PathBuf::from("src/component.astro"),
            source.into(),
            ASTRO_SOURCE,
        );
        db.parsed = Some(document);
        let model = astro_model_from_source_with_options(
            &db,
            document,
            AstroSemanticModelOptions {
                globals: vec!["configuredGlobal".into()],
            },
        );

        assert!(model.references().any(|reference| {
            reference.name() == "configuredGlobal"
                && reference.resolution() == biome_astro_semantic::Resolution::Global
        }));
    }

    #[test]
    fn astro_model_answers_binding_and_reference_queries() {
        const FRONTMATTER: &str = r#"import Card from "./Card.astro";
const \u0074itle = "Hello";
interface Props {}"#;
        const EXPRESSION: &str = "title";
        let source = format!("---\n{FRONTMATTER}\n---\n<Card>{{{EXPRESSION}}}</Card>\n");
        let frontmatter_offset = TextSize::from(source.find(FRONTMATTER).unwrap() as u32);
        let expression_offset = TextSize::from(source.rfind(EXPRESSION).unwrap() as u32);
        let title_offset = TextSize::from(source.find(r"\u0074itle").unwrap() as u32);
        let mut db = AstroTestDb::new(vec![
            (FRONTMATTER, ASTRO_FRONTMATTER_SOURCE),
            (EXPRESSION, ASTRO_EXPRESSION_SOURCE),
        ]);
        let document = Source::new(
            &db,
            Utf8PathBuf::from("src/component.astro"),
            source.into(),
            ASTRO_SOURCE,
        );
        db.parsed = Some(document);
        let db: Rc<dyn LanguageDb> = Rc::new(db);
        let frontmatter = EmbeddedService::new_with_globals(
            db.clone(),
            Some(document),
            frontmatter_offset,
            true,
            &[],
        );
        let expression =
            EmbeddedService::new_with_globals(db, Some(document), expression_offset, true, &[]);

        let Some(EmbeddedSemanticModel::Astro(model)) = &frontmatter.semantic_model else {
            panic!("expected Astro semantics");
        };
        let snippet = model.snippet_at_offset(frontmatter_offset).unwrap();
        let title = snippet
            .scope()
            .unwrap()
            .lookup(Namespace::Value, "title")
            .unwrap();
        assert_eq!(
            title.host_range(),
            TextRange::at(title_offset, TextSize::of(r"\u0074itle"))
        );
        assert!(title.uses().any(|reference| {
            reference.host_range() == TextRange::at(expression_offset, TextSize::of(EXPRESSION))
        }));

        assert!(frontmatter.is_used(token_text("Card")));
        assert!(frontmatter.is_used_as_value(token_text(r"\u0074itle")));
        assert!(frontmatter.is_used_as_type(token_text("Props")));
        assert!(!frontmatter.is_used_as_value(token_text("Props")));
        assert!(!frontmatter.is_used(token_text("missing")));

        assert!(expression.contains_binding(token_text("title")));
        assert!(expression.contains_binding(token_text("Astro")));
        assert!(!expression.contains_binding(token_text("missing")));
    }

    #[test]
    fn interned_astro_data_uses_host_ranges_and_cross_language_references() {
        const FRONTMATTER: &str = r#"const unused = 2;
const used = 1;
const recursive = () => recursive();
interface Props {}"#;
        const USED: &str = "used";
        const MISSING: &str = "missing";
        let source = format!("---\n{FRONTMATTER}\n---\n{{{USED}}}\n{{{MISSING}}}\n");
        let frontmatter_offset = TextSize::from(source.find(FRONTMATTER).unwrap() as u32);
        let used_offset = TextSize::from(source.rfind(USED).unwrap() as u32);
        let missing_offset = TextSize::from(source.rfind(MISSING).unwrap() as u32);
        let frontmatter_range = TextRange::at(frontmatter_offset, TextSize::of(FRONTMATTER));
        let used_range = TextRange::at(used_offset, TextSize::of(USED));
        let missing_range = TextRange::at(missing_offset, TextSize::of(MISSING));
        let expression_source = JsFileSource::tsx().with_embedding_kind(JsEmbeddingKind::Astro {
            frontmatter: false,
            is_class_attribute: false,
        });

        let host: AnyParse =
            parse_html(&source, HtmlParserOptions::from(&HtmlFileSource::astro())).into();
        let frontmatter: AnyParse = parse_js_with_offset(
            FRONTMATTER,
            frontmatter_offset,
            JsFileSource::astro(),
            JsParserOptions::default(),
        )
        .into();
        let used_expression: AnyParse = parse_js_with_offset(
            USED,
            used_offset,
            expression_source,
            JsParserOptions::default(),
        )
        .into();
        let missing_expression: AnyParse = parse_js_with_offset(
            MISSING,
            missing_offset,
            expression_source,
            JsParserOptions::default(),
        )
        .into();
        let data = Arc::new(collect_embedded_data(
            DocumentFileSource::Html(HtmlFileSource::astro()),
            &host,
            vec![
                EmbeddedSnippet::new(
                    &frontmatter,
                    frontmatter_range,
                    DocumentFileSource::Js(JsFileSource::astro()),
                ),
                EmbeddedSnippet::new(
                    &used_expression,
                    used_range,
                    DocumentFileSource::Js(expression_source),
                ),
                EmbeddedSnippet::new(
                    &missing_expression,
                    missing_range,
                    DocumentFileSource::Js(expression_source),
                ),
            ],
        ));
        let service = EmbeddedService::from_data(data.clone(), frontmatter_offset, true);
        let missing_service = EmbeddedService::from_data(data.clone(), missing_offset, true);

        let root: AnyJsRoot = parse(
            FRONTMATTER,
            JsFileSource::astro(),
            JsParserOptions::default(),
        )
        .tree();
        let binding = |name| {
            root.syntax()
                .descendants_tokens(Direction::Next)
                .find(|token| token.text_trimmed() == name)
                .unwrap()
        };
        let used = binding("used");
        let usage = service.binding_usage(&used).unwrap();
        assert!(usage.any());
        assert!(usage.is_read());
        assert!(!usage.is_written());
        assert!(usage.is_used_as_value());
        assert!(!usage.is_used_as_type());
        assert!(service.is_binding_read(&used));
        assert!(!service.is_binding_read(&binding("unused")));
        assert!(!service.is_binding_read(&binding("recursive")));
        assert!(service.is_binding_read(&binding("Props")));

        let missing_root: AnyJsRoot =
            parse(MISSING, expression_source, JsParserOptions::default()).tree();
        let missing_token = missing_root.syntax().first_token().unwrap();
        assert_eq!(
            missing_service.is_known_unresolved_reference(missing_token.text_trimmed_range()),
            Some(true)
        );
        let stale_missing_service = EmbeddedService::from_data(data.clone(), missing_offset, false);
        assert_eq!(
            stale_missing_service.is_known_unresolved_reference(missing_token.text_trimmed_range()),
            Some(false)
        );
        assert!(stale_missing_service.contains_binding(token_text(MISSING)));
        assert!(stale_missing_service.is_used_as_value(token_text(MISSING)));
        assert!(stale_missing_service.is_used_as_type(token_text(MISSING)));
        assert!(stale_missing_service.is_used(token_text(MISSING)));

        let lengthened_frontmatter = FRONTMATTER.replace("const unused", "const _unused");
        let lengthened_root: AnyJsRoot = parse(
            &lengthened_frontmatter,
            JsFileSource::astro(),
            JsParserOptions::default(),
        )
        .tree();
        let shifted_used = lengthened_root
            .syntax()
            .descendants_tokens(Direction::Next)
            .find(|token| token.text_trimmed() == "used")
            .unwrap();
        let lengthened_service =
            EmbeddedService::from_data(data.clone(), frontmatter_offset, false);
        assert!(lengthened_service.binding_usage(&shifted_used).is_none());
        assert!(lengthened_service.is_binding_read(&shifted_used));

        let mutated_frontmatter = FRONTMATTER.replace("const unused", "const alterd");
        let mutated_root: AnyJsRoot = parse(
            &mutated_frontmatter,
            JsFileSource::astro(),
            JsParserOptions::default(),
        )
        .tree();
        let stale_used = mutated_root
            .syntax()
            .descendants_tokens(Direction::Next)
            .find(|token| token.text_trimmed() == "used")
            .unwrap();
        let stale_service = EmbeddedService::from_data(data, frontmatter_offset, false);
        assert!(stale_service.binding_usage(&stale_used).is_none());
        assert!(stale_service.is_binding_read(&stale_used));
    }
}
