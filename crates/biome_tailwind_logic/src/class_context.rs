//! Class context detection shared by parsed and unparsed Tailwind queries.

use std::marker::PhantomData;

use biome_analyze::{
    AddVisitor, Phases, QueryMatch, Queryable, ServiceBag, Visitor, VisitorContext,
};
use biome_html_syntax::HtmlAttribute;
use biome_js_syntax::{JsCallExpression, JsxAttribute};
use biome_rowan::{
    AstNode, Language, SyntaxNode, TextRange, TokenText, WalkEvent, declare_node_union,
};

/// Syntax nodes that identify where Tailwind classes are supplied.
/// Recognition uses attribute and function names without resolving bindings.
pub trait TailwindClassContextNode: AstNode {
    fn is_tailwind_class_context(&self) -> bool;
}

declare_node_union! {
    pub AnyJsClassContext = JsxAttribute | JsCallExpression
}

impl TailwindClassContextNode for AnyJsClassContext {
    fn is_tailwind_class_context(&self) -> bool {
        match self {
            Self::JsxAttribute(attribute) => attribute.is_tailwind_class_context(),
            Self::JsCallExpression(call) => call.is_tailwind_class_context(),
        }
    }
}

impl TailwindClassContextNode for JsxAttribute {
    fn is_tailwind_class_context(&self) -> bool {
        get_jsx_attribute_name(self).is_some_and(|name| is_class_attribute_name(name.text()))
    }
}

impl TailwindClassContextNode for JsCallExpression {
    fn is_tailwind_class_context(&self) -> bool {
        is_call_expression_of_default_function(self)
    }
}

impl TailwindClassContextNode for HtmlAttribute {
    fn is_tailwind_class_context(&self) -> bool {
        self.name()
            .ok()
            .and_then(|name| name.value_token().ok())
            .is_some_and(|name| name.text_trimmed().eq_ignore_ascii_case("class"))
    }
}

/// Matches class attributes and utility calls without parsing their class text.
/// Rules receive the context node and inspect its values or arguments.
#[derive(Clone)]
pub struct TailwindClassContext<N>(N);

impl<N: AstNode + 'static> QueryMatch for TailwindClassContext<N> {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl<N: TailwindClassContextNode + 'static> Queryable for TailwindClassContext<N> {
    type Input = Self;
    type Output = N;
    type Language = N::Language;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, || ClassContextVisitor::<N>(PhantomData));
    }

    fn unwrap_match(_: &ServiceBag, context: &Self::Input) -> Self::Output {
        context.0.clone()
    }
}

struct ClassContextVisitor<N>(PhantomData<N>);

impl<N: TailwindClassContextNode + 'static> Visitor for ClassContextVisitor<N> {
    type Language = N::Language;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        if let WalkEvent::Enter(node) = event
            && let Some(context) = N::cast_ref(node)
            && context.is_tailwind_class_context()
        {
            ctx.match_query(TailwindClassContext(context));
        }
    }
}

const DEFAULT_FUNCTIONS: [&str; 10] = [
    "clsx", "tw", "twMerge", "twJoin", "cva", "tv", "cn", "cc", "cnb", "ctl",
];

pub(crate) fn is_default_function(name: &str) -> bool {
    DEFAULT_FUNCTIONS.contains(&name)
}

pub(crate) fn get_callee_name(call_expression: &JsCallExpression) -> Option<TokenText> {
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

pub(crate) fn is_class_attribute_name(name: &str) -> bool {
    matches!(name, "class" | "className")
}
