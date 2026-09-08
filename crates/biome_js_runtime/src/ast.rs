use std::cell::RefCell;

use boa_engine::builtins::object::OrdinaryObject;
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::object::builtins::JsArray;
use boa_engine::object::{JsObject, ObjectInitializer};
use boa_engine::property::Attribute;
use boa_engine::{
    Context, Finalize, JsData, JsNativeError, JsResult, JsString, JsValue, NativeFunction, Trace,
    js_string,
};

use biome_js_syntax::{JsLanguage, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use biome_rowan::{AstNode, SyntaxKind};
use biome_text_size::TextRange;

#[derive(Debug, JsData)]
pub(crate) struct JsAstNode {
    node: JsSyntaxNode,
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

    pub(crate) fn text_range(value: &JsValue) -> Option<TextRange> {
        let object = value.as_object()?;
        let node = object.downcast_ref::<Self>()?;

        Some(node.node.text_trimmed_range())
    }

    fn from_this(this: &JsValue) -> Option<JsSyntaxNode> {
        let object = this.as_object()?;
        let node = object.downcast_ref::<Self>()?;

        Some(node.node.clone())
    }

    fn get_kind(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let Some(node) = Self::from_this(this) else {
            return Err(JsNativeError::typ()
                .with_message("AST getter called with an invalid receiver")
                .into());
        };

        Ok(JsString::from(format!("{:?}", node.kind())).into())
    }

    fn get_text(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let Some(node) = Self::from_this(this) else {
            return Err(JsNativeError::typ()
                .with_message("AST getter called with an invalid receiver")
                .into());
        };

        Ok(JsString::from(node.text_trimmed().to_string()).into())
    }

    fn wrap_optional_node<N>(node: Option<N>, context: &mut Context) -> JsValue
    where
        N: AstNode<Language = JsLanguage>,
    {
        match node {
            Some(node) => Self::from_node(node.into_syntax(), context),
            None => JsValue::undefined(),
        }
    }

    fn wrap_node_list<I, N>(nodes: I, context: &mut Context) -> JsValue
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

    fn wrap_token(token: Option<JsSyntaxToken>) -> JsValue {
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
                    let Some(syntax) = Self::from_this(this) else {
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

#[path = "generated/js_ast.rs"]
mod js_ast;

impl Class for JsAstNode {
    const NAME: &'static str = "__JsAstNode";

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        let kind =
            NativeFunction::from_fn_ptr(Self::get_kind).to_js_function(class.context().realm());
        let text =
            NativeFunction::from_fn_ptr(Self::get_text).to_js_function(class.context().realm());

        class
            .accessor(js_string!("kind"), Some(kind), None, Attribute::ENUMERABLE)
            .accessor(js_string!("text"), Some(text), None, Attribute::ENUMERABLE);

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
            .with_message("AST nodes cannot be constructed from JavaScript")
            .into())
    }
}
