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
    AnyJsExpression, AnyJsLiteralExpression, JsArrayElementList, JsAssignmentExpression,
    JsAssignmentOperator, JsAwaitExpression, JsBinaryExpression, JsBinaryOperator,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsConditionalExpression, JsLanguage,
    JsLiteralMemberName, JsLogicalExpression, JsLogicalOperator, JsObjectMemberList,
    JsParenthesizedExpression, JsPropertyObjectMember, JsSequenceExpression,
    JsStringLiteralExpression, JsSyntaxKind, JsTemplateChunkElement, JsTemplateElement,
    JsTemplateElementList, JsTemplateExpression, JsxAttribute, JsxAttributeInitializerClause,
    JsxExpressionAttributeValue, JsxString, TsAsExpression, TsNonNullAssertionExpression,
    TsSatisfiesExpression, TsTypeAssertionExpression,
};
use biome_languages::JsFileSource;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{
    AstNode, Language, NodeCache, SyntaxKindSet, SyntaxNode, TextLen, TextRange, TextSize,
    TokenText, WalkEvent,
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
    fn tailwind_class_string(
        &self,
        options: &TailwindOptions,
        is_class_attribute: bool,
    ) -> Option<TailwindClassString>;
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
        let Some(class_string) = ast_node.tailwind_class_string(
            ctx.options.tailwind(),
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

const DEFAULT_MERGE_FUNCTIONS: [&str; 8] =
    ["clsx", "tw", "twMerge", "twJoin", "cn", "cc", "cnb", "ctl"];

const DEFAULT_VARIANT_FUNCTIONS: [&str; 2] = ["cva", "tv"];

fn is_merge_function(options: &TailwindOptions, name: &str) -> bool {
    options.merge_functions().map_or_else(
        || DEFAULT_MERGE_FUNCTIONS.contains(&name),
        |functions| functions.iter().any(|function| function.as_ref() == name),
    )
}

fn is_variant_function(options: &TailwindOptions, name: &str) -> bool {
    options.variant_functions().map_or_else(
        || DEFAULT_VARIANT_FUNCTIONS.contains(&name),
        |functions| functions.iter().any(|function| function.as_ref() == name),
    )
}

/// Returns the identifier a callee or template tag is rooted at, so both `tw`
/// and `tw.div.span` yield `tw`.
fn get_root_name(expression: AnyJsExpression) -> Option<TokenText> {
    let mut current = expression;
    loop {
        match current {
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                return identifier.name().ok()?.name().ok();
            }
            AnyJsExpression::JsStaticMemberExpression(member) => {
                current = member.object().ok()?;
            }
            _ => return None,
        }
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

const DEFAULT_ATTRIBUTES: [&str; 2] = ["class", "className"];

fn is_configured_attribute(options: &TailwindOptions, name: &str) -> bool {
    options.attributes().map_or_else(
        || DEFAULT_ATTRIBUTES.contains(&name),
        |attributes| {
            attributes
                .iter()
                .any(|attribute| attribute.as_ref() == name)
        },
    )
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

/// Returns whether the value of `member` is in a position that holds classes in
/// the configuration object of a variant function such as `cva` or `tv`.
fn is_class_configuration_value(
    member: &JsPropertyObjectMember,
    options: &TailwindOptions,
) -> Option<bool> {
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
                if !JsCallArguments::can_cast(child.kind()) {
                    return None;
                }
                let name = get_root_name(call.callee().ok()?)?;
                if !is_variant_function(options, name.text()) {
                    return None;
                }
                // `path` lists keys from `member` outwards, so the first key of the
                // configuration object comes last.
                let mut keys = path.iter().rev();
                return Some(match keys.next()?.text() {
                    "base" | "slots" | "class" | "className" | "variants" => true,
                    "compoundVariants" | "compoundSlots" => keys
                        .next()
                        .is_some_and(|key| matches!(key.text(), "class" | "className")),
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

/// Returns whether the value of `member` is written as classes rather than as a
/// condition.
fn member_value_holds_classes(member: &JsPropertyObjectMember) -> bool {
    member.value().is_ok_and(|value| {
        matches!(
            value.omit_parentheses(),
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(_)
            ) | AnyJsExpression::JsTemplateExpression(_)
                | AnyJsExpression::JsArrayExpression(_)
                | AnyJsExpression::JsObjectExpression(_)
                | AnyJsExpression::JsConditionalExpression(_)
        )
    })
}

fn inspect_string_literal(
    node: &SyntaxNode<JsLanguage>,
    options: &TailwindOptions,
    is_class_attribute: bool,
) -> Option<bool> {
    let mut child = node.clone();
    let mut collection_kind = None;
    let mut is_object_key = false;
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
                    return is_class_configuration_value(&member, options);
                }
                // A key whose value holds classes, like `sm` in `{ sm: "px-2" }`, is a
                // name. Otherwise the object maps classes to conditions, as in
                // `{ "px-2": isActive }`, and the key is a class.
                if is_class_configuration_value(&member, options).unwrap_or(false)
                    && member_value_holds_classes(&member)
                {
                    return None;
                }
                is_object_key = true;
            }
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                let sequence = JsSequenceExpression::cast_ref(&ancestor)?;
                if sequence.right().ok()?.syntax() != &child {
                    return None;
                }
            }
            JsSyntaxKind::JSX_ATTRIBUTE => {
                let attribute = JsxAttribute::cast_ref(&ancestor)?;
                return Some(is_configured_attribute(
                    options,
                    get_jsx_attribute_name(&attribute)?.text(),
                ));
            }
            JsSyntaxKind::JS_CALL_EXPRESSION => {
                let call = JsCallExpression::cast_ref(&ancestor)?;
                if !JsCallArguments::can_cast(child.kind()) {
                    return Some(false);
                }
                let name = get_root_name(call.callee().ok()?)?;
                // Keys of a variant function's configuration object are names, not classes.
                return Some(
                    is_merge_function(options, name.text())
                        || !is_object_key && is_variant_function(options, name.text()),
                );
            }
            JsSyntaxKind::JS_TEMPLATE_EXPRESSION => {
                if collection_kind.is_some() {
                    return None;
                }
                collection_kind = None;
                let template = JsTemplateExpression::cast_ref(&ancestor)?;
                if let Some(tag) = template.tag() {
                    let name = get_root_name(tag)?;
                    return Some(
                        is_merge_function(options, name.text())
                            || is_variant_function(options, name.text()),
                    );
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
    fn tailwind_class_string(
        &self,
        options: &TailwindOptions,
        is_class_attribute: bool,
    ) -> Option<TailwindClassString> {
        if !inspect_string_literal(self.syntax(), options, is_class_attribute).unwrap_or(false) {
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
    fn tailwind_class_string(
        &self,
        options: &TailwindOptions,
        is_class_attribute: bool,
    ) -> Option<TailwindClassString> {
        if !inspect_string_literal(self.syntax(), options, is_class_attribute).unwrap_or(false) {
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
    fn tailwind_class_string(
        &self,
        options: &TailwindOptions,
        _is_class_attribute: bool,
    ) -> Option<TailwindClassString> {
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
    fn tailwind_class_string(
        &self,
        options: &TailwindOptions,
        is_class_attribute: bool,
    ) -> Option<TailwindClassString> {
        if !inspect_string_literal(self.syntax(), options, is_class_attribute).unwrap_or(false) {
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
    fn tailwind_class_string(
        &self,
        options: &TailwindOptions,
        _is_class_attribute: bool,
    ) -> Option<TailwindClassString> {
        let name = self.name().ok()?.value_token().ok()?;
        let is_tailwind_attribute = options.attributes().map_or_else(
            || {
                DEFAULT_ATTRIBUTES
                    .iter()
                    .any(|attribute| attribute.eq_ignore_ascii_case(name.text_trimmed()))
            },
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
