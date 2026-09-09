use crate::token::JsAstToken;
use biome_js_syntax::{JsLanguage, JsSyntaxElement, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use biome_rowan::{AstNode, SyntaxKind, SyntaxSlot};
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::object::builtins::JsArray;
use boa_engine::object::{JsObject, ObjectInitializer};
use boa_engine::property::Attribute;
use boa_engine::{
    Context, Finalize, JsData, JsNativeError, JsResult, JsString, JsValue, NativeFunction, Trace,
    js_string,
};
use std::cell::RefCell;

#[derive(Clone, Debug, JsData)]
pub(crate) struct JsAstNode {
    pub(crate) node: JsSyntaxNode,
}

#[derive(Debug, Default, JsData)]
struct JsAstPrototypeCache {
    prototypes: RefCell<Vec<Option<JsObject>>>,
}

impl Finalize for JsAstPrototypeCache {}

// SAFETY: The implementation marks every GC-managed object stored in the cache.
unsafe impl Trace for JsAstPrototypeCache {
    boa_engine::gc::custom_trace!(this, mark, {
        for prototype in this.prototypes.borrow().iter().flatten() {
            mark(prototype);
        }
    });
}

impl Finalize for JsAstNode {}

// SAFETY: `JsAstNode` only contains Rowan data and no values managed by Boa's garbage collector.
unsafe impl Trace for JsAstNode {
    boa_engine::gc::empty_trace!();
}

impl JsAstNode {
    pub(crate) fn register(context: &mut Context) -> JsResult<()> {
        context.register_global_class::<Self>()?;

        // Remove the class from the global object so we can't access it from JS side anymore.
        context
            .global_object()
            .delete_property_or_throw(js_string!(Self::NAME), context)?;

        Ok(())
    }

    pub(crate) fn from_node(node: JsSyntaxNode, context: &mut Context) -> JsValue {
        let base_prototype = context
            .get_global_class::<Self>()
            .expect("the JsAstNode class must be registered before loading the plugin")
            .prototype();

        let prototype = Self::prototype_for_kind(node.kind(), base_prototype, context);

        ObjectInitializer::with_native_data_and_proto(Self { node }, prototype, context)
            .build()
            .into()
    }

    pub(crate) fn from_value(this: &JsValue) -> Option<Self> {
        let object = this.as_object()?;
        let node = object.downcast_ref::<Self>()?;

        Some(node.clone())
    }

    fn get_kind(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("Cannot read node.kind because this value is not a node. Use the node passed to run() or a node obtained by traversing it.")
                .into());
        };

        Ok(JsString::from(format!("{:?}", node.kind())).into())
    }

    fn get_text(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("Cannot read node.text because this value is not a node. Use the node passed to run() or a node obtained by traversing it.")
                .into());
        };

        Ok(JsString::from(node.text_trimmed().to_string()).into())
    }

    fn get_parent(this: &JsValue, _args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("Cannot read node.parent because this value is not a node. Use the node passed to run() or a node obtained by traversing it.")
                .into());
        };

        Ok(node.parent().map_or_else(JsValue::undefined, |parent| {
            Self::from_node(parent, context)
        }))
    }

    fn ancestors(this: &JsValue, _args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("ancestors() was called on a value that is not a node. Call node.ancestors() on the node passed to run() or a node obtained by traversing it.")
                .into());
        };

        let ancestors = node
            .ancestors()
            .skip(1)
            .map(|ancestor| Self::from_node(ancestor, context))
            .collect::<Vec<_>>();

        Ok(JsArray::from_iter(ancestors, context).into())
    }

    fn children(this: &JsValue, _args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("children() was called on a value that is not a node. Call node.children() on the node passed to run() or a node obtained by traversing it.")
                .into());
        };

        let children = node
            .children()
            .map(|child| Self::from_node(child, context))
            .collect::<Vec<_>>();

        Ok(JsArray::from_iter(children, context).into())
    }

    fn token(this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("token() was called on a value that is not a node. Call node.token(field) on the node passed to run() or a node obtained by traversing it.")
                .into());
        };
        let [field] = args else {
            return Err(JsNativeError::typ()
                .with_message("node.token() received the wrong number of arguments. Pass exactly one token field name as a string, using node.token(field).")
                .into());
        };
        let field = field.as_string().ok_or_else(|| {
            JsNativeError::typ().with_message("The field argument to node.token() is not a string. Pass the token field name as a string value, not a String object.")
        })?;
        let field = field.to_std_string().map_err(|_| {
            JsNativeError::typ().with_message("The field argument to node.token() contains an incomplete Unicode character. Check the string's \\u escapes and supply a token field name with complete Unicode characters.")
        })?;
        let kind = node.kind();
        let (_, index) = Self::token_fields(kind)
            .iter()
            .find(|(name, _)| *name == field)
            .ok_or_else(|| JsNativeError::typ().with_message(format!(
                "node.token({field:?}) cannot find a token field named {field:?} on a node of kind {kind:?}. Check this node's plugin API type definition and pass a token field name declared for that type."
            )))?;
        match node.slots().nth(*index) {
            Some(SyntaxSlot::Token(token)) => Ok(JsAstToken::from_token(token, context)),
            Some(SyntaxSlot::Node(_)) => Err(JsNativeError::typ()
                .with_message(format!(
                    "node.token({field:?}) found a node where a token was expected on {kind:?}. Check the source syntax before reading this field."
                ))
                .into()),
            Some(SyntaxSlot::Empty { .. }) | None => Ok(JsValue::undefined()),
        }
    }

    fn children_with_tokens(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Err(JsNativeError::typ()
                .with_message("childrenWithTokens() was called on a value that is not a node. Call node.childrenWithTokens() on the node passed to run() or a node obtained by traversing it.")
                .into());
        };
        let children = node
            .children_with_tokens()
            .map(|child| match child {
                JsSyntaxElement::Node(node) => Self::from_node(node, context),
                JsSyntaxElement::Token(token) => JsAstToken::from_token(token, context),
            })
            .collect::<Vec<_>>();
        Ok(JsArray::from_iter(children, context).into())
    }

    pub(crate) fn wrap_optional_node<N>(node: Option<N>, context: &mut Context) -> JsValue
    where
        N: AstNode<Language = JsLanguage>,
    {
        match node {
            Some(node) => Self::from_node(node.into_syntax(), context),
            None => JsValue::undefined(),
        }
    }

    pub(crate) fn wrap_node_list<I, N>(nodes: I, context: &mut Context) -> JsValue
    where
        I: IntoIterator<Item = N>,
        N: AstNode<Language = JsLanguage>,
    {
        let nodes = nodes
            .into_iter()
            .map(|node| Self::from_node(node.into_syntax(), context))
            .collect::<Vec<_>>();

        JsArray::from_iter(nodes, context).into()
    }

    pub(crate) fn wrap_token(token: Option<JsSyntaxToken>) -> JsValue {
        token.map_or_else(JsValue::undefined, |token| {
            JsString::from(token.text_trimmed().to_string()).into()
        })
    }

    fn prototype_for_kind(
        kind: JsSyntaxKind,
        base_prototype: JsObject,
        context: &mut Context,
    ) -> JsObject {
        let index = usize::from(kind.to_raw().0);

        if let Some(prototype) = context
            .get_data::<JsAstPrototypeCache>()
            .and_then(|cache| cache.prototypes.borrow().get(index).cloned().flatten())
        {
            return prototype;
        }

        let prototype = Self::create_generated_prototype(kind, base_prototype, context);
        let cache = context
            .get_data::<JsAstPrototypeCache>()
            .expect("the AST prototype cache is initialized with the class");

        let mut prototypes = cache.prototypes.borrow_mut();
        if prototypes.len() <= index {
            prototypes.resize(index + 1, None);
        }

        prototypes[index] = Some(prototype.clone());
        prototype
    }
}

macro_rules! cast_js_ast_node {
    ($node:expr, $node_type:path) => {{
        // SAFETY: Generated call sites use this macro only after matching the node's syntax kind.
        unsafe { <$node_type>::new_unchecked($node) }
    }};
}

macro_rules! register_js_ast_fields {
    (
        $prototype:ident,
        $node_kind:path,
        $node_type:path,
        $(
            ($property:literal, |$node:ident, $context:ident| $value:expr)
        ),* $(,)?
    ) => {
        $(
            let getter = NativeFunction::from_fn_ptr(
                |this: &JsValue, _args: &[JsValue], js_context: &mut Context| {
                    let $context = js_context;
                    let _ = &$context;
                    let Some(Self { node: syntax }) = Self::from_value(this) else {
                        return Ok(JsValue::undefined());
                    };
                    if syntax.kind() != $node_kind {
                        return Ok(JsValue::undefined());
                    }
                    let $node = cast_js_ast_node!(syntax, $node_type);
                    Ok($value)
                },
            )
            .to_js_function($prototype.context().realm());
            $prototype.accessor(
                js_string!($property),
                Some(getter),
                None,
                Attribute::ENUMERABLE,
            );
        )*
    };
}

pub(crate) use cast_js_ast_node;
pub(crate) use register_js_ast_fields;

impl Class for JsAstNode {
    const NAME: &'static str = "__JsAstNode";

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        let kind =
            NativeFunction::from_fn_ptr(Self::get_kind).to_js_function(class.context().realm());
        let text =
            NativeFunction::from_fn_ptr(Self::get_text).to_js_function(class.context().realm());
        let parent =
            NativeFunction::from_fn_ptr(Self::get_parent).to_js_function(class.context().realm());

        class
            .accessor(js_string!("kind"), Some(kind), None, Attribute::ENUMERABLE)
            .accessor(js_string!("text"), Some(text), None, Attribute::ENUMERABLE)
            .accessor(js_string!("parent"), Some(parent), None, Attribute::empty())
            .method(
                js_string!("ancestors"),
                0,
                NativeFunction::from_fn_ptr(Self::ancestors),
            )
            .method(
                js_string!("children"),
                0,
                NativeFunction::from_fn_ptr(Self::children),
            )
            .method(
                js_string!("token"),
                1,
                NativeFunction::from_fn_ptr(Self::token),
            )
            .method(
                js_string!("childrenWithTokens"),
                0,
                NativeFunction::from_fn_ptr(Self::children_with_tokens),
            );

        if !class.context().has_data::<JsAstPrototypeCache>() {
            let _ = class.context().insert_data(JsAstPrototypeCache::default());
        }

        Ok(())
    }

    fn data_constructor(
        _new_target: &JsValue,
        _args: &[JsValue],
        _context: &mut Context,
    ) -> JsResult<Self> {
        Err(JsNativeError::typ()
            .with_message("Nodes do not have a public constructor. Use the node passed to run() or a node obtained by traversing it, and create updated nodes with the field update methods defined for its type, such as withValueToken().")
            .into())
    }
}
