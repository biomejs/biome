//! Class context detection shared by parsed and unparsed Tailwind queries.

use std::marker::PhantomData;

use biome_analyze::{
    AddVisitor, Phases, QueryMatch, Queryable, ServiceBag, Visitor, VisitorContext,
    options::TailwindOptions,
};
use biome_html_syntax::HtmlAttribute;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsxAttribute};
use biome_rowan::{
    AstNode, Language, SyntaxNode, TextRange, TokenText, WalkEvent, declare_node_union,
};

/// Syntax nodes that identify where Tailwind classes are supplied.
/// Recognition uses attribute and function names without resolving bindings.
pub trait TailwindClassContextNode: AstNode {
    /// Returns how this node supplies classes, or `None` if it doesn't supply any.
    fn tailwind_class_context(&self, options: &TailwindOptions) -> Option<ClassContextKind>;

    fn is_tailwind_class_context(&self, options: &TailwindOptions) -> bool {
        self.tailwind_class_context(options).is_some()
    }
}

/// How a class context supplies classes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassContextKind {
    /// Class attributes and merge functions such as `clsx`. Object keys are
    /// classes and object values are conditions.
    Classes,
    /// Variant functions such as `cva` and `tv`. Object arguments are
    /// configurations whose values hold classes.
    Variants,
}

declare_node_union! {
    pub AnyJsClassContext = JsxAttribute | JsCallExpression
}

impl TailwindClassContextNode for AnyJsClassContext {
    fn tailwind_class_context(&self, options: &TailwindOptions) -> Option<ClassContextKind> {
        match self {
            Self::JsxAttribute(attribute) => attribute.tailwind_class_context(options),
            Self::JsCallExpression(call) => call.tailwind_class_context(options),
        }
    }
}

impl TailwindClassContextNode for JsxAttribute {
    fn tailwind_class_context(&self, options: &TailwindOptions) -> Option<ClassContextKind> {
        get_jsx_attribute_name(self)
            .is_some_and(|name| is_configured_attribute(options, name.text()))
            .then_some(ClassContextKind::Classes)
    }
}

impl TailwindClassContextNode for JsCallExpression {
    fn tailwind_class_context(&self, options: &TailwindOptions) -> Option<ClassContextKind> {
        let name = get_root_name(self.callee().ok()?)?;
        if is_merge_function(options, name.text()) {
            Some(ClassContextKind::Classes)
        } else if is_variant_function(options, name.text()) {
            Some(ClassContextKind::Variants)
        } else {
            None
        }
    }
}

impl TailwindClassContextNode for HtmlAttribute {
    fn tailwind_class_context(&self, options: &TailwindOptions) -> Option<ClassContextKind> {
        let name = self.name().ok()?.value_token().ok()?;
        let name = name.text_trimmed();
        options
            .attributes()
            .map_or_else(
                || {
                    DEFAULT_ATTRIBUTES
                        .iter()
                        .any(|attribute| attribute.eq_ignore_ascii_case(name))
                },
                |attributes| {
                    attributes
                        .iter()
                        .any(|attribute| attribute.as_ref().eq_ignore_ascii_case(name))
                },
            )
            .then_some(ClassContextKind::Classes)
    }
}

/// Matches class attributes and utility calls without parsing their class text.
/// Rules receive the context node and inspect its values or arguments.
#[derive(Clone)]
pub struct TailwindClassContext<N> {
    node: N,
    kind: ClassContextKind,
}

impl<N> TailwindClassContext<N> {
    pub fn node(&self) -> &N {
        &self.node
    }

    pub fn kind(&self) -> ClassContextKind {
        self.kind
    }
}

impl<N: AstNode + 'static> QueryMatch for TailwindClassContext<N> {
    fn text_range(&self) -> TextRange {
        self.node.range()
    }
}

impl<N: TailwindClassContextNode + 'static> Queryable for TailwindClassContext<N> {
    type Input = Self;
    type Output = Self;
    type Language = N::Language;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, || ClassContextVisitor::<N>(PhantomData));
    }

    fn unwrap_match(_: &ServiceBag, context: &Self::Input) -> Self::Output {
        context.clone()
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
            && let Some(kind) = context.tailwind_class_context(ctx.options.tailwind())
        {
            ctx.match_query(TailwindClassContext {
                node: context,
                kind,
            });
        }
    }
}

/// Returns whether the value of `key` holds classes when `key` is a top-level
/// key in the configuration of a variant function such as `cva` or `tv`.
pub fn is_class_configuration_key(key: &str) -> bool {
    matches!(key, "base" | "slots" | "class" | "className" | "variants")
}

/// Returns whether `key` is a top-level key in the configuration of a variant
/// function whose entries hold classes in their `class` and `className` values.
pub fn is_compound_configuration_key(key: &str) -> bool {
    matches!(key, "compoundVariants" | "compoundSlots")
}

const DEFAULT_MERGE_FUNCTIONS: [&str; 8] =
    ["clsx", "tw", "twMerge", "twJoin", "cn", "cc", "cnb", "ctl"];

const DEFAULT_VARIANT_FUNCTIONS: [&str; 2] = ["cva", "tv"];

pub(crate) fn is_merge_function(options: &TailwindOptions, name: &str) -> bool {
    options.merge_functions().map_or_else(
        || DEFAULT_MERGE_FUNCTIONS.contains(&name),
        |functions| functions.iter().any(|function| function.as_ref() == name),
    )
}

pub(crate) fn is_variant_function(options: &TailwindOptions, name: &str) -> bool {
    options.variant_functions().map_or_else(
        || DEFAULT_VARIANT_FUNCTIONS.contains(&name),
        |functions| functions.iter().any(|function| function.as_ref() == name),
    )
}

/// Returns the identifier a callee or template tag is rooted at, so both `tw`
/// and `tw.div.span` yield `tw`.
pub(crate) fn get_root_name(expression: AnyJsExpression) -> Option<TokenText> {
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
