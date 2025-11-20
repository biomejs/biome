use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext,
    VisitorFinishContext,
};
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_js_semantic::{SemanticEventExtractor, SemanticModel, SemanticModelBuilder};
use biome_js_syntax::{AnyJsRoot, JsFileSource, JsLanguage, JsSyntaxNode, TextRange, WalkEvent};
use biome_rowan::AstNode;
use regex::Regex;
use std::sync::{Arc, LazyLock};

/// Generic service that holds the original source text for ANY embedded language
/// (before template/component transformation).
///
/// This is needed because parsers transform embedded templates (Glimmer, Vue, Svelte, Astro)
/// by replacing `<template>` tags with markers like `__BIOME_GLIMMER_TEMPLATE_0__`.
/// The semantic analysis needs access to the original untransformed source
/// to scan for template references.
///
/// Usage:
/// - File handlers (e.g., `glimmer_module.rs`) should populate this service
///   when dealing with embedded languages
/// - Semantic model builder uses this to scan templates for any embedded language
#[derive(Clone)]
pub struct OriginalSourceText(Arc<String>);

impl OriginalSourceText {
    pub fn new(source: String) -> Self {
        Self(Arc::new(source))
    }

    pub fn text(&self) -> &str {
        &self.0
    }
}

/// Regex to match Glimmer <template> tags
static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid Glimmer template regex")
});

/// Regex to match mustache expressions: {{...}}
static MUSTACHE_EXPR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{\{([^}]+)\}\}").expect("Invalid mustache regex"));

/// Regex to match this.property or this.#privateField patterns
static THIS_MEMBER: LazyLock<Regex> = LazyLock::new(|| {
    // Matches: this.property, this.#private, this.property(), etc.
    // Captures the member name (with # if private)
    Regex::new(r"this\.([#]?[a-zA-Z_$][a-zA-Z0-9_$]*)").expect("Invalid this.member regex")
});

pub struct SemanticServices {
    model: SemanticModel,
}

impl SemanticServices {
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }
}

impl FromServices for SemanticServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,

        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let model: &SemanticModel = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"]))?;
        Ok(Self {
            model: model.clone(),
        })
    }
}

impl Phase for SemanticServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// The [SemanticServices] types can be used as a queryable to get an instance
/// of the whole [SemanticModel] without matching on a specific AST node
impl Queryable for SemanticServices {
    type Input = SemanticModelEvent;
    type Output = SemanticModel;

    type Language = JsLanguage;
    type Services = Self;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
        analyzer.add_visitor(Phases::Semantic, || SemanticModelVisitor);
    }

    fn unwrap_match(services: &ServiceBag, _: &SemanticModelEvent) -> Self::Output {
        services
            .get_service::<SemanticModel>()
            .expect("SemanticModel service is not registered")
            .clone()
    }
}

/// Query type usable by lint rules **that uses the semantic model** to match on specific [AstNode] types
#[derive(Clone)]
pub struct Semantic<N>(pub N);

impl<N> Queryable for Semantic<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = SemanticServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

pub struct SemanticModelBuilderVisitor {
    extractor: SemanticEventExtractor,
    builder: SemanticModelBuilder,
}

impl SemanticModelBuilderVisitor {
    pub(crate) fn new(root: &AnyJsRoot) -> Self {
        Self {
            extractor: SemanticEventExtractor::default(),
            builder: SemanticModelBuilder::new(root.clone()),
        }
    }
}

impl Visitor for SemanticModelBuilderVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, _ctx: VisitorContext<JsLanguage>) {
        match event {
            WalkEvent::Enter(node) => {
                self.builder.push_node(node);
                self.extractor.enter(node);
            }
            WalkEvent::Leave(node) => {
                self.extractor.leave(node);
            }
        }

        while let Some(e) = self.extractor.pop() {
            self.builder.push_event(e);
        }
    }

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        let mut builder = self.builder;

        // For any embedded language (Glimmer, Vue, Svelte, Astro), scan templates
        // and add synthetic references to the semantic model
        if let Some(file_source) = ctx.services.get_service::<JsFileSource>() {
            let embedding_kind = file_source.as_embedding_kind();

            // Check if this is any embedded language
            if embedding_kind.is_glimmer()
                || embedding_kind.is_vue()
                || embedding_kind.is_svelte()
                || embedding_kind.is_astro()
            {
                // Try to get the original source from services
                // If not available, fall back to the (transformed) AST text
                let source_text = if let Some(original_source) =
                    ctx.services.get_service::<OriginalSourceText>()
                {
                    original_source.text().to_string()
                } else {
                    builder.root().syntax().text_with_trivia().to_string()
                };

                // Currently only Glimmer is fully implemented, but this is extensible
                if embedding_kind.is_glimmer() {
                    add_template_references(&mut builder, &source_text);
                }
                // TODO: Add support for Vue, Svelte, Astro template scanning here
                // Each would have their own scanning function similar to add_template_references()
            }
        }

        let model = builder.build();
        ctx.services.insert_service(model);
    }
}

/// Scan Glimmer templates and add synthetic references to the semantic model
fn add_template_references(builder: &mut SemanticModelBuilder, source: &str) {
    let template_matches: Vec<_> = GLIMMER_TEMPLATE.find_iter(source).collect();

    for template_match in template_matches {
        let template_content = template_match.as_str();

        let template_range = TextRange::new(
            template_match.start().try_into().unwrap(),
            template_match.end().try_into().unwrap(),
        );

        // Parse with Glimmer-enabled HTML parser
        let file_source = HtmlFileSource::glimmer();
        let options = HtmlParseOptions::from(&file_source);
        let parse = parse_html(template_content, options);

        let root = parse.tree();
        let root_node = root.syntax();

        // 1. Find all HTML elements (component references) in the template
        for node in root_node.descendants() {
            use biome_html_syntax::{HtmlElement, HtmlSelfClosingElement};

            if AnyHtmlElement::can_cast(node.kind()) {
                let element = AnyHtmlElement::unwrap_cast(node.clone());

                // Get the name token (SyntaxToken) and tag name string
                let name_token_data: Option<(biome_rowan::SyntaxToken<biome_html_syntax::HtmlLanguage>, String)> = (|| {
                    match &element {
                        AnyHtmlElement::HtmlElement(el) => {
                            let opening = el.opening_element().ok()?;
                            let name_node = opening.name().ok()?;
                            let token = name_node.value_token().ok()?;
                            let text = token.token_text_trimmed().to_string();
                            Some((token, text))
                        }
                        AnyHtmlElement::HtmlSelfClosingElement(el) => {
                            let name_node = el.name().ok()?;
                            let token = name_node.value_token().ok()?;
                            let text = token.token_text_trimmed().to_string();
                            Some((token, text))
                        }
                        _ => None,
                    }
                })();

                if let Some((name_token, tag_name)) = name_token_data {
                    // Component references are PascalCase
                    if is_pascal_case(&tag_name) {
                        // Find the binding (import) for this component
                        if let Some(binding_id) = builder.find_binding_by_name(&tag_name) {
                            // Calculate the absolute position of the component name within the original source
                            let name_range = name_token.text_range();
                            let absolute_start: u32 =
                                (template_match.start() + usize::from(name_range.start()))
                                    .try_into()
                                    .unwrap();
                            let absolute_end: u32 =
                                (template_match.start() + usize::from(name_range.end()))
                                    .try_into()
                                    .unwrap();

                            let component_range =
                                TextRange::new(absolute_start.into(), absolute_end.into());
                            builder.add_synthetic_reference(binding_id, component_range);
                        }
                    }
                }
            }
        }

        // 2. Find all mustache expressions and extract member references
        scan_mustache_expressions(builder, template_content, template_range);
    }
}

/// Scan mustache expressions for property/method references
fn scan_mustache_expressions(
    builder: &mut SemanticModelBuilder,
    template_content: &str,
    template_range: TextRange,
) {
    // Find all {{...}} expressions
    for mustache_match in MUSTACHE_EXPR.captures_iter(template_content) {
        if let Some(expr) = mustache_match.get(1) {
            let expr_text = expr.as_str().trim();

            // Look for this.member patterns
            for member_match in THIS_MEMBER.captures_iter(expr_text) {
                if let Some(member_name) = member_match.get(1) {
                    let name = member_name.as_str();

                    // Find the binding for this member
                    // Private fields start with #, others don't
                    if let Some(binding_id) = builder.find_binding_by_name(name) {
                        builder.add_synthetic_reference(binding_id, template_range);
                    }
                }
            }

            // Also try to find simple variable references (not prefixed with this.)
            // Pattern: {{variableName}} or {{functionName arg}}
            if !expr_text.contains("this.") && !expr_text.starts_with('@') {
                // Extract the first identifier (could be a variable or helper)
                if let Some(first_word) = expr_text.split_whitespace().next() {
                    // Remove any trailing punctuation like () or ,
                    let identifier = first_word
                        .trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '$');

                    if !identifier.is_empty() && is_valid_identifier(identifier) {
                        // Try to find a binding with this name
                        if let Some(binding_id) = builder.find_binding_by_name(identifier) {
                            builder.add_synthetic_reference(binding_id, template_range);
                        }
                    }
                }
            }
        }
    }
}

/// Check if a string is PascalCase (starts with uppercase letter)
fn is_pascal_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    s.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

/// Check if a string is a valid JavaScript identifier
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    // First character must be letter, $, or _
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        if !first.is_alphabetic() && first != '$' && first != '_' {
            return false;
        }
    }

    // Remaining characters must be alphanumeric, $, or _
    chars.all(|c| c.is_alphanumeric() || c == '$' || c == '_')
}

pub struct SemanticModelVisitor;

pub struct SemanticModelEvent(TextRange);

impl QueryMatch for SemanticModelEvent {
    fn text_range(&self) -> TextRange {
        self.0
    }
}

impl Visitor for SemanticModelVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, mut ctx: VisitorContext<JsLanguage>) {
        let root = match event {
            WalkEvent::Enter(node) => {
                if node.parent().is_some() {
                    return;
                }

                node
            }
            WalkEvent::Leave(_) => return,
        };

        let text_range = root.text_range_with_trivia();
        ctx.match_query(SemanticModelEvent(text_range));
    }
}
