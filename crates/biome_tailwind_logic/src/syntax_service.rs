use crate::class_context::{
    TailwindClassContextNode, get_callee_name, is_class_attribute_name, is_default_function,
};
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
    AnyJsExpression, JsArrayElementList, JsAssignmentExpression, JsAssignmentOperator,
    JsAwaitExpression, JsBinaryExpression, JsBinaryOperator, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsConditionalExpression, JsLanguage, JsLiteralMemberName,
    JsLogicalExpression, JsLogicalOperator, JsObjectMemberList, JsParenthesizedExpression,
    JsPropertyObjectMember, JsSequenceExpression, JsStaticMemberExpression,
    JsStringLiteralExpression, JsSyntaxKind, JsTemplateChunkElement, JsTemplateElement,
    JsTemplateElementList, JsTemplateExpression, JsxAttribute, JsxAttributeInitializerClause,
    JsxExpressionAttributeValue, JsxString, TsAsExpression, TsNonNullAssertionExpression,
    TsSatisfiesExpression, TsTypeAssertionExpression,
};
use biome_languages::JsFileSource;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{
    AstNode, AstSeparatedList, Language, NodeCache, SyntaxKindSet, SyntaxNode, TextLen, TextRange,
    TextSize, TokenText, WalkEvent,
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
    fn tailwind_class_string(&self, is_class_attribute: bool) -> Option<TailwindClassString>;
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
            .tailwind_class_string(
                services
                    .get_service::<JsFileSource>()
                    .is_some_and(|source| source.as_embedding_kind().is_class_attribute()),
            )
            // SAFETY: The visitor emits matches only for nodes that host a Tailwind class string.
            .expect("TailwindSyntaxVisitor only emits Tailwind class strings");
        let parse = services
            .get_service::<TwSyntaxService>()
            // SAFETY: TailwindSyntaxServices requires this service before the rule can run.
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
        let Some(class_string) = ast_node.tailwind_class_string(
            ctx.services
                .get_service::<JsFileSource>()
                .is_some_and(|source| source.as_embedding_kind().is_class_attribute()),
        ) else {
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

const CLASS_CONFIGURATION_WRAPPER_KINDS: SyntaxKindSet<JsLanguage> = JsObjectMemberList::KIND_SET
    .union(JsArrayElementList::KIND_SET)
    .union(JsCallArguments::KIND_SET)
    .union(JsCallArgumentList::KIND_SET)
    .union(JsParenthesizedExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(TsAsExpression::KIND_SET)
    .union(TsSatisfiesExpression::KIND_SET)
    .union(TsNonNullAssertionExpression::KIND_SET)
    .union(TsTypeAssertionExpression::KIND_SET);

const CLASS_STRING_WRAPPER_KINDS: SyntaxKindSet<JsLanguage> = CLASS_CONFIGURATION_WRAPPER_KINDS
    .union(JsTemplateElementList::KIND_SET)
    .union(JsTemplateElement::KIND_SET)
    .union(JsxExpressionAttributeValue::KIND_SET)
    .union(JsxAttributeInitializerClause::KIND_SET);

fn is_class_preserving_spread(
    collection_kind: Option<JsSyntaxKind>,
    parent_kind: JsSyntaxKind,
) -> bool {
    match collection_kind {
        Some(JsSyntaxKind::JS_ARRAY_EXPRESSION) => matches!(
            parent_kind,
            JsSyntaxKind::JS_ARRAY_ELEMENT_LIST | JsSyntaxKind::JS_CALL_ARGUMENT_LIST
        ),
        Some(JsSyntaxKind::JS_OBJECT_EXPRESSION) => {
            parent_kind == JsSyntaxKind::JS_OBJECT_MEMBER_LIST
        }
        _ => false,
    }
}

fn is_class_configuration_value(member: &JsPropertyObjectMember) -> Option<bool> {
    let mut path = Vec::new();
    let mut collection_kind = None;
    let mut child = member.syntax().clone();
    for ancestor in member.syntax().ancestors() {
        match ancestor.kind() {
            JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => {
                let member = JsPropertyObjectMember::cast_ref(&ancestor)?;
                path.push(member.name().ok()?.name()?);
            }
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                let conditional = JsConditionalExpression::cast_ref(&ancestor)?;
                if conditional.test().ok()?.syntax() == &child {
                    return None;
                }
            }
            JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                let logical = JsLogicalExpression::cast_ref(&ancestor)?;
                if logical.operator().ok()? == JsLogicalOperator::LogicalAnd
                    && logical.left().ok()?.syntax() == &child
                {
                    return None;
                }
            }
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                let sequence = JsSequenceExpression::cast_ref(&ancestor)?;
                if sequence.right().ok()?.syntax() != &child {
                    return None;
                }
            }
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                let assignment = JsAssignmentExpression::cast_ref(&ancestor)?;
                if assignment.right().ok()?.syntax() != &child
                    || !matches!(
                        assignment.operator().ok()?,
                        JsAssignmentOperator::Assign
                            | JsAssignmentOperator::LogicalAndAssign
                            | JsAssignmentOperator::LogicalOrAssign
                            | JsAssignmentOperator::NullishCoalescingAssign
                    )
                {
                    return None;
                }
            }
            JsSyntaxKind::JS_CALL_EXPRESSION => {
                let call = JsCallExpression::cast_ref(&ancestor)?;
                let name = get_callee_name(&call)?;
                let config_index = match name.text() {
                    "cva" => 1,
                    "tv" => 0,
                    _ => return None,
                };
                let config = call
                    .arguments()
                    .ok()?
                    .args()
                    .iter()
                    .nth(config_index)?
                    .ok()?;
                if !config.syntax().text_range().contains_range(member.range()) {
                    return None;
                }
                return Some(match path.as_slice() {
                    [base] => name.text() == "tv" && base.text() == "base",
                    [_, slots] if name.text() == "tv" && slots.text() == "slots" => true,
                    [_, _, variants] if variants.text() == "variants" => true,
                    [_, _, _, variants] => name.text() == "tv" && variants.text() == "variants",
                    [class, compound_variants] | [_, class, compound_variants] => {
                        (path.len() == 2 || name.text() == "tv")
                            && is_class_attribute_name(class.text())
                            && (compound_variants.text() == "compoundVariants"
                                || name.text() == "tv"
                                    && compound_variants.text() == "compoundSlots")
                    }
                    _ => false,
                });
            }
            JsSyntaxKind::JS_ARRAY_EXPRESSION | JsSyntaxKind::JS_OBJECT_EXPRESSION => {
                collection_kind = Some(ancestor.kind());
            }
            JsSyntaxKind::JS_SPREAD => {
                if !is_class_preserving_spread(collection_kind, ancestor.parent()?.kind()) {
                    return None;
                }
            }
            kind if CLASS_CONFIGURATION_WRAPPER_KINDS.matches(kind) => {}
            _ => return None,
        }
        child = ancestor;
    }
    None
}

fn inspect_string_literal(node: &SyntaxNode<JsLanguage>, is_class_attribute: bool) -> Option<bool> {
    let mut child = node.clone();
    let mut collection_kind = None;
    for ancestor in node.ancestors().skip(1) {
        match ancestor.kind() {
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                let conditional = JsConditionalExpression::cast_ref(&ancestor)?;
                if conditional.test().ok()?.syntax() == &child {
                    return None;
                }
            }
            JsSyntaxKind::JS_BINARY_EXPRESSION => {
                let binary = JsBinaryExpression::cast_ref(&ancestor)?;
                if collection_kind.is_some() || binary.operator().ok()? != JsBinaryOperator::Plus {
                    return None;
                }
                collection_kind = None;
            }
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                let assignment = JsAssignmentExpression::cast_ref(&ancestor)?;
                if assignment.right().ok()?.syntax() != &child {
                    return None;
                }
                match assignment.operator().ok()? {
                    JsAssignmentOperator::Assign
                    | JsAssignmentOperator::LogicalAndAssign
                    | JsAssignmentOperator::LogicalOrAssign
                    | JsAssignmentOperator::NullishCoalescingAssign => {}
                    JsAssignmentOperator::AddAssign if collection_kind.is_none() => {}
                    _ => return None,
                }
            }
            JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                let logical = JsLogicalExpression::cast_ref(&ancestor)?;
                if logical.operator().ok()? == JsLogicalOperator::LogicalAnd
                    && logical.left().ok()?.syntax() == &child
                {
                    return None;
                }
            }
            JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => {
                let member = JsPropertyObjectMember::cast_ref(&ancestor)?;
                if member.name().ok()?.syntax() != &child {
                    return is_class_configuration_value(&member);
                }
                if is_class_configuration_value(&member).unwrap_or(false) {
                    return None;
                }
            }
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                let sequence = JsSequenceExpression::cast_ref(&ancestor)?;
                if sequence.right().ok()?.syntax() != &child {
                    return None;
                }
            }
            JsSyntaxKind::JSX_ATTRIBUTE => {
                let attribute = JsxAttribute::cast_ref(&ancestor)?;
                return Some(attribute.is_tailwind_class_context());
            }
            JsSyntaxKind::JS_CALL_EXPRESSION => {
                let call = JsCallExpression::cast_ref(&ancestor)?;
                return Some(
                    JsCallArguments::can_cast(child.kind()) && call.is_tailwind_class_context(),
                );
            }
            JsSyntaxKind::JS_TEMPLATE_EXPRESSION => {
                if collection_kind.is_some() {
                    return None;
                }
                collection_kind = None;
                let template = JsTemplateExpression::cast_ref(&ancestor)?;
                if let Some(tag) = template.tag() {
                    return match tag {
                        AnyJsExpression::JsIdentifierExpression(tag) => {
                            Some(is_default_function(tag.name().ok()?.name().ok()?.text()))
                        }
                        AnyJsExpression::JsStaticMemberExpression(tag) => {
                            is_static_member_expression_of_default_function(&tag)
                        }
                        _ => None,
                    };
                }
            }
            JsSyntaxKind::JS_EXPRESSION_TEMPLATE_ROOT => return Some(is_class_attribute),
            JsSyntaxKind::JS_ARRAY_EXPRESSION | JsSyntaxKind::JS_OBJECT_EXPRESSION => {
                collection_kind = Some(ancestor.kind());
            }
            JsSyntaxKind::JS_SPREAD => {
                if !is_class_preserving_spread(collection_kind, ancestor.parent()?.kind()) {
                    return None;
                }
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_NAME => {
                if collection_kind.is_some() {
                    return None;
                }
            }
            kind if CLASS_STRING_WRAPPER_KINDS.matches(kind) => {}
            _ => return None,
        }
        child = ancestor;
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
    fn tailwind_class_string(&self, is_class_attribute: bool) -> Option<TailwindClassString> {
        if !inspect_string_literal(self.syntax(), is_class_attribute).unwrap_or(false) {
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
    fn tailwind_class_string(&self, is_class_attribute: bool) -> Option<TailwindClassString> {
        if !inspect_string_literal(self.syntax(), is_class_attribute).unwrap_or(false) {
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
    fn tailwind_class_string(&self, _is_class_attribute: bool) -> Option<TailwindClassString> {
        let jsx_attribute = self
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(JsxAttribute::cast)?;
        if !jsx_attribute.is_tailwind_class_context() {
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
    fn tailwind_class_string(&self, is_class_attribute: bool) -> Option<TailwindClassString> {
        if !inspect_string_literal(self.syntax(), is_class_attribute).unwrap_or(false) {
            return None;
        }
        let token = self.template_chunk_token().ok()?;
        Some(tailwind_class_string(
            token.token_text(),
            token.text_trimmed_range().start(),
            ClassStringHostKind::JsTemplateChunkElement,
        ))
    }
}

impl TailwindClassStringHost for HtmlAttribute {
    fn tailwind_class_string(&self, _is_class_attribute: bool) -> Option<TailwindClassString> {
        if !self.is_tailwind_class_context() {
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
