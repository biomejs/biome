use std::cell::RefCell;
use std::marker::PhantomData;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use biome_analyze::{
    AddVisitor, DiagnosticSignal, FromServices, Phase, Phases, QueryMatch, Queryable, RuleCategory,
    RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic, SignalEntry, SignalRuleKey, Visitor,
    VisitorContext,
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
pub struct SyntaxService<L: Language> {
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
struct SyntaxServiceInner<L: Language> {
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
            .expect("TailwindSyntaxVisitor must parse Tailwind class strings before rule queries")
            .clone()
    }

    pub fn parse_for_visitor(&self, class_string: &TailwindClassString) -> ParsedTailwindSyntax {
        let mut inner = self.inner.borrow_mut();
        let ParseWithInnerResult {
            parse,
            should_emit_diagnostics,
            panic_diagnostic,
        } = parse_with_inner(&mut inner, class_string);
        ParsedTailwindSyntax {
            parse,
            should_emit_diagnostics,
            panic_diagnostic,
        }
    }
}

struct ParseWithInnerResult {
    parse: Rc<TailwindParse>,
    should_emit_diagnostics: bool,
    panic_diagnostic: Option<TailwindParserPanicDiagnostic>,
}

fn parse_with_inner(
    inner: &mut SyntaxServiceInner<TailwindLanguage>,
    class_string: &TailwindClassString,
) -> ParseWithInnerResult {
    if let Some(parse) = inner.parsed.get(&class_string.key) {
        return ParseWithInnerResult {
            parse: parse.clone(),
            should_emit_diagnostics: false,
            panic_diagnostic: None,
        };
    }

    let mut panic_diagnostic = None;
    // we catch panicks here so that we can emit a diagonstic that shows the precise class string that is causing the panic.
    // makes it easier for users to submit bug reports.
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
    ParseWithInnerResult {
        parse,
        should_emit_diagnostics: true,
        panic_diagnostic,
    }
}

pub trait TailwindClassStringHost: AstNode {
    fn tailwind_class_string(&self) -> Option<TailwindClassString>;
}

#[derive(Clone)]
pub struct TailwindSyntax<N> {
    node: N,
    parse: Rc<TailwindParse>,
}

pub struct TailwindSyntaxMatch<L: Language>(SyntaxNode<L>);

impl<L: Language + 'static> QueryMatch for TailwindSyntaxMatch<L> {
    fn text_range(&self) -> TextRange {
        self.0.text_trimmed_range()
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
        let node = N::unwrap_cast(node.0.clone());
        let class_string = node
            .tailwind_class_string()
            .expect("TailwindSyntaxVisitor only emits Tailwind class strings");
        let parse = services
            .get_service::<TwSyntaxService>()
            .expect("TwSyntaxService service is not registered")
            .parse_for_query(&class_string);

        Self { node, parse }
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
        let Some(class_string) = ast_node.tailwind_class_string() else {
            return;
        };
        let Some(service) = ctx.services.get_service::<TwSyntaxService>() else {
            ctx.match_query(TailwindSyntaxMatch(node.clone()));
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
        ctx.match_query(TailwindSyntaxMatch(node.clone()));
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

const DEFAULT_FUNCTIONS: [&str; 10] = [
    "clsx", "tw", "twMerge", "twJoin", "cva", "tv", "cn", "cc", "cnb", "ctl",
];

fn is_default_function(name: &str) -> bool {
    DEFAULT_FUNCTIONS.contains(&name)
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

fn is_call_expression_of_default_function(call_expression: &JsCallExpression) -> bool {
    get_callee_name(call_expression).is_some_and(|name| is_default_function(name.text()))
}

fn is_static_member_expression_of_default_function(
    static_member_expression: &JsStaticMemberExpression,
) -> Option<bool> {
    let mut current = static_member_expression.object().ok()?;
    loop {
        if let Some(identifier) = current.as_js_identifier_expression() {
            let name = identifier.name().ok()?.name().ok()?;
            return Some(is_default_function(name.text()));
        }
        if let Some(static_member) = current.as_js_static_member_expression() {
            current = static_member.object().ok()?;
            continue;
        }
        return Some(false);
    }
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

fn is_class_attribute_name(name: &str) -> bool {
    matches!(name, "class" | "className")
}

fn inspect_string_literal(node: &SyntaxNode<JsLanguage>) -> Option<bool> {
    let mut in_arguments = false;
    let mut in_function = false;
    for ancestor in node.ancestors().skip(1) {
        if let Some(jsx_attribute) = JsxAttribute::cast_ref(&ancestor) {
            let attribute_name = get_jsx_attribute_name(&jsx_attribute)?;
            if is_class_attribute_name(attribute_name.text()) {
                return Some(true);
            }
        }

        if let Some(call_expression) = JsCallExpression::cast_ref(&ancestor) {
            in_function = is_call_expression_of_default_function(&call_expression);
        }

        if JsCallArguments::can_cast(ancestor.kind()) {
            in_arguments = true;
        }

        if in_function && in_arguments {
            return Some(true);
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
    fn tailwind_class_string(&self) -> Option<TailwindClassString> {
        if !inspect_string_literal(self.syntax()).unwrap_or(false) {
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
    fn tailwind_class_string(&self) -> Option<TailwindClassString> {
        if !inspect_string_literal(self.syntax()).unwrap_or(false) {
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
    fn tailwind_class_string(&self) -> Option<TailwindClassString> {
        let jsx_attribute = self
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(JsxAttribute::cast)?;
        let name = get_jsx_attribute_name(&jsx_attribute)?;
        if !is_class_attribute_name(name.text()) {
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
    fn tailwind_class_string(&self) -> Option<TailwindClassString> {
        for ancestor in self.syntax().ancestors().skip(1) {
            if let Some(template_expression) = JsTemplateExpression::cast_ref(&ancestor) {
                if let Some(AnyJsExpression::JsIdentifierExpression(tag)) =
                    template_expression.tag()
                {
                    let name = tag.name().ok()?.name().ok()?;
                    if is_default_function(name.text()) {
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
                    && is_static_member_expression_of_default_function(&tag).unwrap_or(false)
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
                let attribute_name = get_jsx_attribute_name(&jsx_attribute)?;
                if is_class_attribute_name(attribute_name.text()) {
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
    fn tailwind_class_string(&self) -> Option<TailwindClassString> {
        let name = self.name().ok()?.value_token().ok()?;
        if name.text_trimmed() != "class" {
            return None;
        }
        let html_string = self.initializer()?.value().ok()?.as_html_string()?.clone();
        tailwind_class_string(
            html_string.inner_string_text().ok()?,
            html_string.value_token().ok()?.text_trimmed_range().start() + TextSize::from(1),
            ClassStringHostKind::HtmlString,
        )
        .into()
    }
}
