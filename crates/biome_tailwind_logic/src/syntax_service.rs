use std::cell::RefCell;
use std::marker::PhantomData;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use biome_analyze::{
    AddVisitor, DiagnosticSignal, FromServices, Phase, Phases, QueryMatch, Queryable, RuleCategory,
    RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic, SignalEntry, SignalRuleKey, Visitor,
    VisitorContext, options::TailwindOptions,
};
use biome_console::markup;
use biome_diagnostics::{Diagnostic, MessageAndDescription, panic::catch_unwind};
use biome_html_syntax::HtmlAttribute;
use biome_js_syntax::{
    AnyJsExpression, JsCallArguments, JsCallExpression, JsLanguage, JsLiteralMemberName,
    JsStaticMemberExpression, JsStringLiteralExpression, JsTemplateChunkElement,
    JsTemplateExpression, JsxAttribute, JsxString,
};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{
    AstNode, Language, NodeCache, SyntaxNode, TextLen, TextRange, TextSize, TokenText, WalkEvent,
};
use biome_tailwind_parser::{TailwindParse, parse_tailwind_with_cache};
use biome_tailwind_syntax::{TailwindLanguage, TwRoot};
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct SyntaxService<L> {
    inner: Rc<RefCell<SyntaxServiceInner<L>>>,
}

impl<L: Language> Default for SyntaxService<L> {
    fn default() -> Self {
        Self {
            inner: Rc::default(),
        }
    }
}

#[derive(Debug)]
struct SyntaxServiceInner<L> {
    node_cache: NodeCache,
    parsed: FxHashMap<TailwindSyntaxCacheKey, Rc<TailwindParse>>,
    _language: PhantomData<L>,
}

impl<L: Language> Default for SyntaxServiceInner<L> {
    fn default() -> Self {
        Self {
            node_cache: NodeCache::default(),
            parsed: FxHashMap::default(),
            _language: PhantomData,
        }
    }
}

pub type TwSyntaxService = SyntaxService<TailwindLanguage>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TailwindSyntaxCacheKey {
    range: TextRange,
    kind: ClassStringHostKind,
}

impl TailwindSyntaxCacheKey {
    pub fn new(range: TextRange, kind: ClassStringHostKind) -> Self {
        Self { range, kind }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum ClassStringHostKind {
    JsStringLiteralExpression,
    JsLiteralMemberName,
    JsxString,
    JsTemplateChunkElement,
    HtmlString,
}

pub struct TailwindClassString {
    pub key: TailwindSyntaxCacheKey,
    pub text: TokenText,
    /// The range of `text` in the host source file.
    pub inner_range: TextRange,
}

pub struct ParsedTailwindSyntax {
    pub parse: Rc<TailwindParse>,
    pub should_emit_diagnostics: bool,
    panic_diagnostic: Option<TailwindParserPanicDiagnostic>,
}

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(category = "internalError/panic", severity = Fatal, tags(INTERNAL))]
struct TailwindParserPanicDiagnostic {
    #[location(span)]
    span: TextRange,
    #[message]
    #[description]
    message: MessageAndDescription,
}

impl TailwindParserPanicDiagnostic {
    fn new(span: TextRange, message: String) -> Self {
        Self {
            span,
            message: MessageAndDescription::from(
                markup! {
                    "The Tailwind parser panicked while parsing this class string: "{message}
                }
                .to_owned(),
            ),
        }
    }
}

impl TwSyntaxService {
    pub fn parse_for_query(&self, class_string: &TailwindClassString) -> Rc<TailwindParse> {
        self.inner
            .borrow()
            .parsed
            .get(&class_string.key)
            // SAFETY: The visitor caches every class string before emitting its query match.
            .expect("TailwindSyntaxVisitor must parse Tailwind class strings before rule queries")
            .clone()
    }

    pub fn parse_for_visitor(&self, class_string: &TailwindClassString) -> ParsedTailwindSyntax {
        let mut inner = self.inner.borrow_mut();
        parse_with_inner(&mut inner, class_string)
    }
}

fn parse_with_inner(
    inner: &mut SyntaxServiceInner<TailwindLanguage>,
    class_string: &TailwindClassString,
) -> ParsedTailwindSyntax {
    if let Some(parse) = inner.parsed.get(&class_string.key) {
        return ParsedTailwindSyntax {
            parse: parse.clone(),
            should_emit_diagnostics: false,
            panic_diagnostic: None,
        };
    }

    let mut panic_diagnostic = None;
    // Convert parser panics into diagnostics tied to the host class string.
    let parse = match catch_unwind(AssertUnwindSafe(|| {
        parse_tailwind_with_cache(class_string.text.text(), &mut inner.node_cache)
    })) {
        Ok(parse) => Rc::new(parse),
        Err(error) => {
            let message = error.info;
            panic_diagnostic = Some(TailwindParserPanicDiagnostic::new(
                class_string.inner_range,
                message,
            ));
            Rc::new(parse_tailwind_with_cache("", &mut inner.node_cache))
        }
    };
    inner.parsed.insert(class_string.key, parse.clone());
    ParsedTailwindSyntax {
        parse,
        should_emit_diagnostics: true,
        panic_diagnostic,
    }
}

pub trait TailwindClassStringHost: AstNode {
    fn tailwind_class_string(&self, options: &TailwindOptions) -> Option<TailwindClassString>;
}

#[derive(Clone)]
pub struct TailwindSyntax<N> {
    node: N,
    parse: Rc<TailwindParse>,
}

pub struct TailwindSyntaxMatch<L: Language> {
    node: SyntaxNode<L>,
    class_string: TailwindClassString,
}

impl<L: Language + 'static> QueryMatch for TailwindSyntaxMatch<L> {
    fn text_range(&self) -> TextRange {
        self.node.text_trimmed_range()
    }
}

impl<N> TailwindSyntax<N> {
    pub fn node(&self) -> &N {
        &self.node
    }

    pub fn tailwind_root(&self) -> TwRoot {
        self.parse.tree()
    }

    pub fn tailwind_diagnostics(&self) -> &[ParseDiagnostic] {
        self.parse.diagnostics()
    }

    pub fn tailwind_has_errors(&self) -> bool {
        self.parse.has_errors()
    }
}

impl<N> QueryMatch for TailwindSyntax<N>
where
    N: AstNode + 'static,
{
    fn text_range(&self) -> TextRange {
        self.node.syntax().text_trimmed_range()
    }
}

pub struct TailwindSyntaxServices {
    _service: TwSyntaxService,
}

impl FromServices for TailwindSyntaxServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        Ok(Self {
            _service: services
                .get_service::<TwSyntaxService>()
                .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["TwSyntaxService"]))?
                .clone(),
        })
    }
}

impl Phase for TailwindSyntaxServices {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

impl<N> Queryable for TailwindSyntax<N>
where
    N: AstNode + TailwindClassStringHost + 'static,
{
    type Input = TailwindSyntaxMatch<N::Language>;
    type Output = Self;
    type Language = N::Language;
    type Services = TailwindSyntaxServices;

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, TailwindSyntaxVisitor::<N>::default);
    }

    fn unwrap_match(services: &ServiceBag, node: &Self::Input) -> Self::Output {
        let ast_node = N::unwrap_cast(node.node.clone());
        let parse = services
            .get_service::<TwSyntaxService>()
            // SAFETY: TailwindSyntaxServices requires this service before the rule can run.
            .expect("TwSyntaxService service is not registered")
            .parse_for_query(&node.class_string);

        Self {
            node: ast_node,
            parse,
        }
    }
}

pub struct TailwindSyntaxVisitor<N: AstNode> {
    skip_subtree: Option<SyntaxNode<N::Language>>,
    _node: PhantomData<N>,
}

impl<N: AstNode> Default for TailwindSyntaxVisitor<N> {
    fn default() -> Self {
        Self {
            skip_subtree: None,
            _node: PhantomData,
        }
    }
}

impl<N> Visitor for TailwindSyntaxVisitor<N>
where
    N: AstNode + TailwindClassStringHost + 'static,
{
    type Language = N::Language;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        let node = match event {
            WalkEvent::Enter(node) => node,
            WalkEvent::Leave(node) => {
                if let Some(skip_subtree) = &self.skip_subtree
                    && skip_subtree == node
                {
                    self.skip_subtree = None;
                }
                return;
            }
        };

        if self.skip_subtree.is_some() {
            return;
        }

        if let Some(range) = ctx.range
            && node.text_range_with_trivia().ordering(range).is_ne()
        {
            self.skip_subtree = Some(node.clone());
            return;
        }

        let Some(ast_node) = N::cast_ref(node) else {
            return;
        };
        let Some(class_string) = ast_node.tailwind_class_string(ctx.options.tailwind()) else {
            return;
        };
        let Some(service) = ctx.services.get_service::<TwSyntaxService>() else {
            return;
        };
        let parsed = service.parse_for_visitor(&class_string);
        if let Some(diagnostic) = parsed.panic_diagnostic {
            let text_range = diagnostic.span;
            ctx.push_signal(SignalEntry {
                signal: Box::new(DiagnosticSignal::new(move || diagnostic.clone())),
                rule: SignalRuleKey::Rule(RuleKey::new("tailwind", "parse")),
                instances: Box::new([]),
                text_range,
                category: RuleCategory::Syntax,
            });
        }
        if parsed.should_emit_diagnostics {
            emit_parse_diagnostics(&mut ctx, &class_string, parsed.parse.diagnostics());
        }
        ctx.match_query(TailwindSyntaxMatch {
            node: node.clone(),
            class_string,
        });
    }
}

fn emit_parse_diagnostics<L: Language>(
    ctx: &mut VisitorContext<L>,
    class_string: &TailwindClassString,
    diagnostics: &[ParseDiagnostic],
) {
    for diagnostic in diagnostics {
        let text_range = diagnostic
            .location()
            .span
            .map_or(class_string.inner_range, |span| {
                span + class_string.inner_range.start()
            });
        let mut diagnostic = diagnostic.clone();
        diagnostic.set_location_offset(class_string.inner_range.start());
        ctx.push_signal(SignalEntry {
            signal: Box::new(DiagnosticSignal::new(move || diagnostic.clone())),
            rule: SignalRuleKey::Rule(RuleKey::new("tailwind", "parse")),
            instances: Box::new([]),
            text_range,
            category: RuleCategory::Syntax,
        });
    }
}

fn matches_function_pattern(pattern: &str, name: &str) -> bool {
    let mut pattern_parts = pattern.split('.');
    let mut name_parts = name.split('.');
    let all_parts_match = pattern_parts
        .by_ref()
        .zip(name_parts.by_ref())
        .all(|(pattern, name)| pattern == "*" || pattern == name);
    all_parts_match && pattern_parts.next().is_none() && name_parts.next().is_none()
}

const DEFAULT_FUNCTIONS: [&str; 10] = [
    "clsx", "tw", "twMerge", "twJoin", "cva", "tv", "cn", "cc", "cnb", "ctl",
];

fn is_tailwind_function(options: &TailwindOptions, name: &str) -> bool {
    if let Some(functions) = options.functions() {
        functions
            .iter()
            .any(|pattern| matches_function_pattern(pattern, name))
    } else {
        DEFAULT_FUNCTIONS
            .iter()
            .any(|pattern| matches_function_pattern(pattern, name))
    }
}

fn matches_static_member_function_pattern(
    pattern: &str,
    static_member_expression: &JsStaticMemberExpression,
) -> bool {
    let mut current = static_member_expression.clone();

    if !pattern.contains('.') {
        loop {
            let Ok(object) = current.object() else {
                return false;
            };
            if let Some(identifier) = object.as_js_identifier_expression() {
                let Ok(name) = identifier.name() else {
                    return false;
                };
                let Ok(name) = name.name() else {
                    return false;
                };
                return pattern == "*" || pattern == name.text();
            }
            let Some(static_member) = object.as_js_static_member_expression() else {
                return false;
            };
            current = static_member.clone();
        }
    }

    let mut pattern_parts = pattern.rsplit('.');
    loop {
        let Some(pattern_part) = pattern_parts.next() else {
            return false;
        };
        let Ok(member) = current.member() else {
            return false;
        };
        let Some(member) = member.as_js_name() else {
            return false;
        };
        let Ok(member) = member.value_token() else {
            return false;
        };
        if pattern_part != "*" && pattern_part != member.text_trimmed() {
            return false;
        }

        let Ok(object) = current.object() else {
            return false;
        };
        if let Some(identifier) = object.as_js_identifier_expression() {
            let Ok(name) = identifier.name() else {
                return false;
            };
            let Ok(name) = name.name() else {
                return false;
            };
            let Some(pattern_part) = pattern_parts.next() else {
                return false;
            };
            return (pattern_part == "*" || pattern_part == name.text())
                && pattern_parts.next().is_none();
        }
        let Some(static_member) = object.as_js_static_member_expression() else {
            return false;
        };
        current = static_member.clone();
    }
}

fn is_tailwind_static_member_function(
    options: &TailwindOptions,
    static_member_expression: &JsStaticMemberExpression,
) -> bool {
    if let Some(functions) = options.functions() {
        functions.iter().any(|pattern| {
            matches_static_member_function_pattern(pattern, static_member_expression)
        })
    } else {
        DEFAULT_FUNCTIONS.iter().any(|pattern| {
            matches_static_member_function_pattern(pattern, static_member_expression)
        })
    }
}

fn get_callee_name(call_expression: &JsCallExpression) -> Option<TokenText> {
    call_expression
        .callee()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .name()
        .ok()
}

fn is_call_expression_of_configured_function(
    call_expression: &JsCallExpression,
    options: &TailwindOptions,
) -> bool {
    if let Some(name) = get_callee_name(call_expression) {
        return is_tailwind_function(options, name.text());
    }

    let Ok(callee) = call_expression.callee() else {
        return false;
    };
    let Some(callee) = callee.as_js_static_member_expression() else {
        return false;
    };
    is_tailwind_static_member_function(options, callee)
}

fn is_static_member_expression_of_configured_function(
    static_member_expression: &JsStaticMemberExpression,
    options: &TailwindOptions,
) -> bool {
    is_tailwind_static_member_function(options, static_member_expression)
}

fn get_jsx_attribute_name(attribute: &JsxAttribute) -> Option<TokenText> {
    Some(
        attribute
            .name()
            .ok()?
            .as_jsx_name()?
            .value_token()
            .ok()?
            .token_text_trimmed(),
    )
}

fn is_configured_attribute(options: &TailwindOptions, name: &str) -> bool {
    options.attributes().map_or_else(
        || matches!(name, "class" | "className"),
        |attributes| {
            attributes
                .iter()
                .any(|attribute| attribute.as_ref() == name)
        },
    )
}

fn inspect_string_literal(
    node: &SyntaxNode<JsLanguage>,
    options: &TailwindOptions,
) -> Option<bool> {
    let mut in_arguments = false;
    for ancestor in node.ancestors().skip(1) {
        if let Some(jsx_attribute) = JsxAttribute::cast_ref(&ancestor) {
            let Some(attribute_name) = get_jsx_attribute_name(&jsx_attribute) else {
                continue;
            };
            if is_configured_attribute(options, attribute_name.text()) {
                return Some(true);
            }
        }

        if let Some(call_expression) = JsCallExpression::cast_ref(&ancestor) {
            return in_arguments
                .then(|| is_call_expression_of_configured_function(&call_expression, options));
        }

        if JsCallArguments::can_cast(ancestor.kind()) {
            in_arguments = true;
        }
    }

    None
}

fn tailwind_class_string(
    text: TokenText,
    value_start: TextSize,
    kind: ClassStringHostKind,
) -> TailwindClassString {
    let inner_range = TextRange::at(value_start, text.text_len());
    TailwindClassString {
        key: TailwindSyntaxCacheKey::new(inner_range, kind),
        text,
        inner_range,
    }
}

impl TailwindClassStringHost for JsStringLiteralExpression {
    fn tailwind_class_string(&self, options: &TailwindOptions) -> Option<TailwindClassString> {
        if !inspect_string_literal(self.syntax(), options).unwrap_or(false) {
            return None;
        }
        tailwind_class_string(
            self.inner_string_text().ok()?,
            self.value_token().ok()?.text_trimmed_range().start() + TextSize::from(1),
            ClassStringHostKind::JsStringLiteralExpression,
        )
        .into()
    }
}

impl TailwindClassStringHost for JsLiteralMemberName {
    fn tailwind_class_string(&self, options: &TailwindOptions) -> Option<TailwindClassString> {
        if !inspect_string_literal(self.syntax(), options).unwrap_or(false) {
            return None;
        }
        tailwind_class_string(
            self.name().ok()?,
            self.value().ok()?.text_trimmed_range().start() + TextSize::from(1),
            ClassStringHostKind::JsLiteralMemberName,
        )
        .into()
    }
}

impl TailwindClassStringHost for JsxString {
    fn tailwind_class_string(&self, options: &TailwindOptions) -> Option<TailwindClassString> {
        let jsx_attribute = self
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(JsxAttribute::cast)?;
        let name = get_jsx_attribute_name(&jsx_attribute)?;
        if !is_configured_attribute(options, name.text()) {
            return None;
        }
        tailwind_class_string(
            self.inner_string_text().ok()?,
            self.value_token().ok()?.text_trimmed_range().start() + TextSize::from(1),
            ClassStringHostKind::JsxString,
        )
        .into()
    }
}

impl TailwindClassStringHost for JsTemplateChunkElement {
    fn tailwind_class_string(&self, options: &TailwindOptions) -> Option<TailwindClassString> {
        for ancestor in self.syntax().ancestors().skip(1) {
            if let Some(template_expression) = JsTemplateExpression::cast_ref(&ancestor) {
                if let Some(AnyJsExpression::JsIdentifierExpression(tag)) =
                    template_expression.tag()
                {
                    let name = tag.name().ok()?.name().ok()?;
                    if is_tailwind_function(options, name.text()) {
                        return Some(tailwind_class_string(
                            self.template_chunk_token().ok()?.token_text(),
                            self.template_chunk_token()
                                .ok()?
                                .text_trimmed_range()
                                .start(),
                            ClassStringHostKind::JsTemplateChunkElement,
                        ));
                    }
                }
                if let Some(AnyJsExpression::JsStaticMemberExpression(tag)) =
                    template_expression.tag()
                    && is_static_member_expression_of_configured_function(&tag, options)
                {
                    return Some(tailwind_class_string(
                        self.template_chunk_token().ok()?.token_text(),
                        self.template_chunk_token()
                            .ok()?
                            .text_trimmed_range()
                            .start(),
                        ClassStringHostKind::JsTemplateChunkElement,
                    ));
                }
            } else if let Some(jsx_attribute) = JsxAttribute::cast_ref(&ancestor) {
                let Some(attribute_name) = get_jsx_attribute_name(&jsx_attribute) else {
                    continue;
                };
                if is_configured_attribute(options, attribute_name.text()) {
                    return Some(tailwind_class_string(
                        self.template_chunk_token().ok()?.token_text(),
                        self.template_chunk_token()
                            .ok()?
                            .text_trimmed_range()
                            .start(),
                        ClassStringHostKind::JsTemplateChunkElement,
                    ));
                }
            }
        }

        None
    }
}

impl TailwindClassStringHost for HtmlAttribute {
    fn tailwind_class_string(&self, options: &TailwindOptions) -> Option<TailwindClassString> {
        let name = self.name().ok()?.value_token().ok()?;
        let is_tailwind_attribute = options.attributes().map_or_else(
            || name.text_trimmed().eq_ignore_ascii_case("class"),
            |attributes| {
                attributes
                    .iter()
                    .any(|attribute| attribute.as_ref().eq_ignore_ascii_case(name.text_trimmed()))
            },
        );
        if !is_tailwind_attribute {
            return None;
        }
        let html_string = self.html_string()?;
        tailwind_class_string(
            html_string.inner_string_text().ok()?,
            html_string.inner_string_range().ok()?.start(),
            ClassStringHostKind::HtmlString,
        )
        .into()
    }
}
