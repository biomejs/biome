use crate::token::JsAstToken;
use biome_js_syntax::{JsSyntaxElement, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use biome_rowan::{SyntaxKind, SyntaxSlot};
use boa_engine::builtins::object::OrdinaryObject;
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::object::builtins::JsArray;
use boa_engine::object::{JsObject, ObjectInitializer};
use boa_engine::property::Attribute;
use boa_engine::{
    Context, Finalize, JsData, JsNativeError, JsResult, JsString, JsValue, NativeFunction, Trace,
    js_string,
};
use std::cell::RefCell;
use std::iter::once;

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

/// The plugin API fields of one node kind, generated in `generated/js_ast.rs`.
///
/// Every node kind shares the same native getter and update functions. The
/// descriptors below carry the per-field data those functions need, which
/// keeps the generated code down to static tables instead of one function per
/// field.
pub(crate) struct JsAstNodeFields {
    pub(crate) kind: JsSyntaxKind,
    /// The Rust node type name, e.g. `JsCallExpression`.
    pub(crate) name: &'static str,
    /// One descriptor per slot, in slot order.
    pub(crate) fields: &'static [JsAstField],
}

pub(crate) struct JsAstField {
    /// The JavaScript property name, e.g. `callee`.
    pub(crate) property: &'static str,
    /// The JavaScript update method name, e.g. `withCallee`.
    pub(crate) updater: &'static str,
    pub(crate) optional: bool,
    pub(crate) value: JsAstFieldValue,
}

pub(crate) enum JsAstFieldValue {
    Token {
        /// The token kinds the slot accepts.
        kinds: &'static [JsSyntaxKind],
        /// The token choices as shown in error messages, e.g. `"?.", "!"`.
        expected: &'static str,
    },
    Node {
        /// The Rust node type name, e.g. `AnyJsExpression`.
        ty: &'static str,
        can_cast: fn(JsSyntaxKind) -> bool,
    },
    List {
        /// The Rust list type name, e.g. `JsCallArgumentList`.
        ty: &'static str,
        can_cast: fn(JsSyntaxKind) -> bool,
    },
}

impl JsAstField {
    fn accepts(&self, kind: JsSyntaxKind) -> bool {
        match &self.value {
            JsAstFieldValue::Token { kinds, .. } => kinds.contains(&kind),
            JsAstFieldValue::Node { can_cast, .. } | JsAstFieldValue::List { can_cast, .. } => {
                can_cast(kind)
            }
        }
    }

    fn is_token(&self) -> bool {
        matches!(self.value, JsAstFieldValue::Token { .. })
    }

    /// The type name a plugin sees for replacement values of this field.
    fn replacement_type(&self) -> String {
        match &self.value {
            JsAstFieldValue::Token { .. } => "JsAstToken".to_owned(),
            JsAstFieldValue::Node { ty, .. } => (*ty).to_owned(),
            JsAstFieldValue::List { ty, .. } => format!("{ty}Node"),
        }
    }
}

/// Captured by the native getter and update functions to identify their field.
#[derive(Clone, Copy)]
struct JsAstFieldRef {
    node: &'static JsAstNodeFields,
    slot: usize,
}

impl Finalize for JsAstFieldRef {}

// SAFETY: The descriptors are static data without values managed by Boa's garbage collector.
unsafe impl Trace for JsAstFieldRef {
    boa_engine::gc::empty_trace!();
}

impl JsAstFieldRef {
    fn field(&self) -> &'static JsAstField {
        &self.node.fields[self.slot]
    }

    fn type_error(&self, message: String) -> boa_engine::JsError {
        JsNativeError::typ().with_message(message).into()
    }

    /// `JsCallExpression.withCallee() for field callee`
    fn update_context(&self) -> String {
        let field = self.field();
        format!(
            "{}.{}() for field {}",
            self.node.name, field.updater, field.property
        )
    }

    fn arity_error(&self) -> boa_engine::JsError {
        let field = self.field();
        let mut message = format!(
            "{} requires exactly one argument. Call node.{}(value) with a {} value.",
            self.update_context(),
            field.updater,
            field.replacement_type()
        );
        if field.optional {
            message.push_str(&format!(
                " To remove this optional field, call node.{}(undefined).",
                field.updater
            ));
        }
        self.type_error(message)
    }

    fn invalid_receiver_error(&self) -> boa_engine::JsError {
        self.type_error(format!(
            "{} was called without a Biome node. Call node.{}(value) on a Biome {} node, not as a standalone function or on a plain object.",
            self.update_context(),
            self.field().updater,
            self.node.name
        ))
    }

    fn wrong_node_error(&self) -> boa_engine::JsError {
        self.type_error(format!(
            "{} was called on the wrong node type. Call node.{}(value) on a Biome {} node.",
            self.update_context(),
            self.field().updater,
            self.node.name
        ))
    }

    fn malformed_node_error(&self) -> boa_engine::JsError {
        self.type_error(format!(
            "{} cannot update this node because its fields are malformed. Check the source syntax or skip this update.",
            self.update_context()
        ))
    }

    fn malformed_field_error(&self) -> boa_engine::JsError {
        let field = self.field();
        self.type_error(format!(
            "Field {}.{} is malformed and cannot be read or updated with {}(). Check the source syntax or skip this update.",
            self.node.name, field.property, field.updater
        ))
    }

    fn missing_list_error(&self) -> boa_engine::JsError {
        self.type_error(format!(
            "{} cannot update this node because its {} list field is unavailable. Check the source syntax or skip this update.",
            self.update_context(),
            self.field().property
        ))
    }

    fn invalid_token_error(&self) -> boa_engine::JsError {
        self.type_error(format!(
            "{} requires a JsAstToken, but the argument is not a Biome token. Pass a token returned by node.token(\"{}\") or factory.token(), not a string or plain object.",
            self.update_context(),
            self.field().property
        ))
    }

    fn wrong_token_error(&self, expected: &str) -> boa_engine::JsError {
        self.type_error(format!(
            "{} received a token that does not match the allowed token choices: {expected}. Pass a matching JsAstToken from node.token(\"{}\") or factory.token().",
            self.update_context(),
            self.field().property
        ))
    }

    fn invalid_node_error(&self) -> boa_engine::JsError {
        self.type_error(format!(
            "{} requires a {}, but the argument is not a Biome node. Pass a matching node from a node field or node.children(), not a plain object or array.",
            self.update_context(),
            self.field().replacement_type()
        ))
    }

    fn wrong_type_error(&self, ty: &str) -> boa_engine::JsError {
        self.type_error(format!(
            "{} received a node that does not match the required type {ty}. Pass a {} from a node field or node.children().",
            self.update_context(),
            self.field().replacement_type()
        ))
    }
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

        Ok(Self::wrap_node_list(node.children(), context))
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
        let index = Self::node_fields(kind)
            .and_then(|node| {
                node.fields
                    .iter()
                    .position(|descriptor| descriptor.is_token() && descriptor.property == field)
            })
            .ok_or_else(|| JsNativeError::typ().with_message(format!(
                "node.token({field:?}) cannot find a token field named {field:?} on a node of kind {kind:?}. Check this node's plugin API type definition and pass a token field name declared for that type."
            )))?;
        match node.slots().nth(index) {
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

    /// The getter shared by every generated node field.
    ///
    /// Returns `undefined` for a receiver that is not a node of the expected
    /// kind and for empty or malformed slots.
    fn get_field(
        this: &JsValue,
        _args: &[JsValue],
        field: &JsAstFieldRef,
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let Some(Self { node }) = Self::from_value(this) else {
            return Ok(JsValue::undefined());
        };
        if node.kind() != field.node.kind {
            return Ok(JsValue::undefined());
        }
        let value = match (&field.field().value, node.slots().nth(field.slot)) {
            (JsAstFieldValue::Token { .. }, Some(SyntaxSlot::Token(token))) => {
                Self::wrap_token(Some(token))
            }
            (JsAstFieldValue::Node { .. }, Some(SyntaxSlot::Node(node))) => {
                Self::from_node(node, context)
            }
            (JsAstFieldValue::List { .. }, Some(SyntaxSlot::Node(list))) => {
                // `children()` skips separator tokens and empty slots.
                Self::wrap_node_list(list.children(), context)
            }
            _ => JsValue::undefined(),
        };
        Ok(value)
    }

    /// The `withX(value)` update method shared by every generated node field.
    ///
    /// Returns a new node with the slot replaced. The replacement keeps the
    /// leading and trailing trivia of the element it replaces.
    fn update_field(
        this: &JsValue,
        args: &[JsValue],
        field: &JsAstFieldRef,
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let [value] = args else {
            return Err(field.arity_error());
        };
        let Self { node } = Self::from_value(this).ok_or_else(|| field.invalid_receiver_error())?;
        if node.kind() != field.node.kind {
            return Err(field.wrong_node_error());
        }
        if node.slots().len() != field.node.fields.len() {
            return Err(field.malformed_node_error());
        }
        let descriptor = field.field();
        let old = node
            .slots()
            .nth(field.slot)
            .and_then(|slot| slot.into_syntax_element());
        match &old {
            Some(element) => {
                if !descriptor.accepts(element.kind()) {
                    return Err(field.malformed_field_error());
                }
            }
            None => {
                if matches!(descriptor.value, JsAstFieldValue::List { .. }) {
                    return Err(field.missing_list_error());
                }
            }
        }

        let replacement = if descriptor.optional && value.is_undefined() {
            None
        } else {
            let element = match &descriptor.value {
                JsAstFieldValue::Token { expected, .. } => {
                    let token = JsAstToken::from_value(value)
                        .ok_or_else(|| field.invalid_token_error())?
                        .token;
                    if !descriptor.accepts(token.kind()) {
                        return Err(field.wrong_token_error(expected));
                    }
                    let token = match &old {
                        Some(JsSyntaxElement::Token(old)) => token
                            .with_leading_trivia_pieces(old.leading_trivia().pieces())
                            .with_trailing_trivia_pieces(old.trailing_trivia().pieces()),
                        _ => token,
                    };
                    JsSyntaxElement::Token(token)
                }
                JsAstFieldValue::Node { ty, .. } | JsAstFieldValue::List { ty, .. } => {
                    let Self { node: mut element } =
                        Self::from_value(value).ok_or_else(|| field.invalid_node_error())?;
                    if !descriptor.accepts(element.kind()) {
                        return Err(field.wrong_type_error(ty));
                    }
                    if let Some(JsSyntaxElement::Node(old)) = &old {
                        if let Some(first) = old.first_token()
                            && let Some(updated) = element
                                .clone()
                                .with_leading_trivia_pieces(first.leading_trivia().pieces())
                        {
                            element = updated;
                        }
                        if let Some(last) = old.last_token()
                            && let Some(updated) = element
                                .clone()
                                .with_trailing_trivia_pieces(last.trailing_trivia().pieces())
                        {
                            element = updated;
                        }
                    }
                    JsSyntaxElement::Node(element)
                }
            };
            Some(element)
        };

        let updated = node.splice_slots(field.slot..=field.slot, once(replacement));
        Ok(Self::from_node(updated, context))
    }

    fn wrap_node_list<I>(nodes: I, context: &mut Context) -> JsValue
    where
        I: IntoIterator<Item = JsSyntaxNode>,
    {
        let nodes = nodes
            .into_iter()
            .map(|node| Self::from_node(node, context))
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

        let prototype = Self::create_prototype(kind, base_prototype, context);
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

    /// Creates the prototype of `kind` with one accessor and one `withX()`
    /// update method per field of the node.
    fn create_prototype(
        kind: JsSyntaxKind,
        base_prototype: JsObject,
        context: &mut Context,
    ) -> JsObject {
        let mut prototype =
            ObjectInitializer::with_native_data_and_proto(OrdinaryObject, base_prototype, context);
        if let Some(node) = Self::node_fields(kind) {
            let fields = node
                .fields
                .iter()
                .enumerate()
                .map(|(slot, field)| (JsAstFieldRef { node, slot }, field));
            for (field_ref, field) in fields.clone() {
                let getter =
                    NativeFunction::from_copy_closure_with_captures(Self::get_field, field_ref)
                        .to_js_function(prototype.context().realm());
                prototype.accessor(
                    JsString::from(field.property),
                    Some(getter),
                    None,
                    Attribute::ENUMERABLE,
                );
            }
            for (field_ref, field) in fields {
                prototype.function(
                    NativeFunction::from_copy_closure_with_captures(Self::update_field, field_ref),
                    JsString::from(field.updater),
                    1,
                );
            }
        }
        prototype.build()
    }
}

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
