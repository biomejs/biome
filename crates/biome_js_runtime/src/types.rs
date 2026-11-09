use crate::ast::JsAstNode;
use crate::js_object::js_object;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, JsCallExpression, JsNewExpression, JsSyntaxNode,
};
use biome_rowan::AstNode;
use boa_engine::class::Class;
use boa_engine::{Context, JsNativeError, JsResult, JsString, JsValue, js_string};
use std::rc::Rc;

/// A property of an inferred type that JavaScript code can query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypePredicate {
    Any,
    Unknown,
    Never,
    Primitive,
    ObjectLike,
    Array,
    Tuple,
    Function,
    Callable,
    PromiseInstance,
    ArrayOfPromise,
    Nullish,
    AlwaysTruthy,
    AlwaysFalsy,
    StringLike,
    NumberLike,
    BooleanLike,
    BigIntLike,
}

/// The high-level classifications offered by the typed service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeClassification {
    Promise,
    Thenable,
    ArrayOfPromises,
    PromiseReturningFunction,
}

/// The outcome of a [`TypeClassification`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeClassificationResult {
    Match,
    NoMatch,
    Indeterminate,
}

/// A lazily resolved inferred type exposed to JavaScript.
///
/// Implementations must not require a lifetime from the type database: Boa
/// stores them in garbage-collected objects that outlive individual queries.
pub trait TypeView: 'static {
    /// Renders the type as TypeScript-like source.
    fn display(&self) -> String;

    /// Returns the short category name of the type, such as `string` or `union`.
    fn kind(&self) -> &'static str;

    /// Evaluates `predicate`. Returns `None` when inference cannot decide.
    fn evaluate(&self, predicate: TypePredicate) -> Option<bool>;
}

/// Resolves type information for the currently analyzed file.
///
/// The provider is implemented by the plugin loader, which has access to the
/// module graph. The runtime only sees owned values and opaque [`TypeView`]s.
pub trait TypeInferenceProvider: 'static {
    /// Returns the inferred type of an expression, if inference produced one.
    fn type_of_expression(&self, expression: &JsSyntaxNode) -> Option<Rc<dyn TypeView>>;

    /// Returns the inferred type of the binding `name` visible at `node`.
    fn type_of_named_value(&self, node: &JsSyntaxNode, name: &str) -> Option<Rc<dyn TypeView>>;

    /// Returns the inferred return type of a function-like node.
    fn return_type_of_function(&self, function: &JsSyntaxNode) -> Option<Rc<dyn TypeView>>;

    /// Returns the inferred return type of a class or object member.
    ///
    /// When the member is not a function, the member's own type is returned.
    fn return_type_of_member(&self, member: &JsSyntaxNode, name: &str) -> Option<Rc<dyn TypeView>>;

    /// Returns the expected callable type of an argument at `index`.
    fn expected_argument_type(&self, call: &JsSyntaxNode, index: usize)
    -> Option<Rc<dyn TypeView>>;

    /// Classifies `expression` without resolving unrelated members.
    fn classify(
        &self,
        expression: &JsSyntaxNode,
        classification: TypeClassification,
    ) -> TypeClassificationResult;
}

/// Prefix of the error raised when a rule requests type information for a file
/// that wasn't indexed with type inference.
pub const TYPE_INFERENCE_UNAVAILABLE: &str = "Type information is unavailable for this file";

js_object!(
    JsTypeService,
    Rc<dyn TypeInferenceProvider>,
    "TypeService",
    "Obtain type information through context.types in a rule that requests the types capability.",
    ("typeOf", 1, |service, args, context| {
        let node = ast_node(&args[0])?;
        if AnyJsExpression::cast_ref(&node).is_none() {
            return Err(JsNativeError::typ()
                .with_message(
                    "TypeService.typeOf() requires an expression node. Pass an expression from the analyzed source.",
                )
                .into());
        }
        Ok(optional_type(service.type_of_expression(&node), context))
    }),
    ("typeOfNamed", 2, |service, args, context| {
        let node = ast_node(&args[0])?;
        let name = string_arg(&args[1], "name")?;
        Ok(optional_type(
            service.type_of_named_value(&node, &name),
            context,
        ))
    }),
    ("returnTypeOf", 1, |service, args, context| {
        let node = ast_node(&args[0])?;
        if AnyJsFunction::cast_ref(&node).is_none() {
            return Err(JsNativeError::typ()
                .with_message(
                    "TypeService.returnTypeOf() requires a function node. Pass a function declaration, expression, or arrow function.",
                )
                .into());
        }
        Ok(optional_type(
            service.return_type_of_function(&node),
            context,
        ))
    }),
    ("returnTypeOfMember", 2, |service, args, context| {
        let node = ast_node(&args[0])?;
        let name = string_arg(&args[1], "name")?;
        Ok(optional_type(
            service.return_type_of_member(&node, &name),
            context,
        ))
    }),
    ("expectedArgumentType", 2, |service, args, context| {
        let node = ast_node(&args[0])?;
        if JsCallExpression::cast_ref(&node).is_none() && JsNewExpression::cast_ref(&node).is_none()
        {
            return Err(JsNativeError::typ()
                .with_message(
                    "TypeService.expectedArgumentType() requires a call or new expression node.",
                )
                .into());
        }
        let index = index_arg(&args[1], context)?;
        Ok(optional_type(
            service.expected_argument_type(&node, index),
            context,
        ))
    }),
    ("classifiesAs", 2, |service, args, context| {
        let node = ast_node(&args[0])?;
        if AnyJsExpression::cast_ref(&node).is_none() {
            return Err(JsNativeError::typ()
                .with_message(
                    "TypeService.classifiesAs() requires an expression node. Pass an expression from the analyzed source.",
                )
                .into());
        }
        let classification = classification_arg(&args[1])?;
        Ok(classification_result(
            service.classify(&node, classification),
        ))
    }),
);

js_object!(
    JsTypeInfo,
    Rc<dyn TypeView>,
    "TypeInfo",
    "Obtain a type through context.types in a rule that requests the types capability.",
    ("toString", 0, |view, args, context| {
        Ok(JsValue::from(JsString::from(view.display())))
    }),
    ("kind", 0, |view, args, context| {
        Ok(JsValue::from(JsString::from(view.kind())))
    }),
    ("isAny", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::Any)))
    }),
    ("isUnknown", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::Unknown)))
    }),
    ("isNever", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::Never)))
    }),
    ("isPrimitive", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::Primitive)))
    }),
    ("isObjectLike", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::ObjectLike)))
    }),
    ("isArray", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::Array)))
    }),
    ("isTuple", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::Tuple)))
    }),
    ("isFunction", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::Function)))
    }),
    ("isCallable", 0, |view, args, context| {
        Ok(optional_bool(view.evaluate(TypePredicate::Callable)))
    }),
    ("isPromise", 0, |view, args, context| {
        Ok(optional_bool(view.evaluate(TypePredicate::PromiseInstance)))
    }),
    ("isArrayOfPromise", 0, |view, args, context| {
        Ok(optional_bool(view.evaluate(TypePredicate::ArrayOfPromise)))
    }),
    ("isNullish", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::Nullish)))
    }),
    ("isAlwaysTruthy", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::AlwaysTruthy)))
    }),
    ("isAlwaysFalsy", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::AlwaysFalsy)))
    }),
    ("isStringLike", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::StringLike)))
    }),
    ("isNumberLike", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::NumberLike)))
    }),
    ("isBooleanLike", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::BooleanLike)))
    }),
    ("isBigIntLike", 0, |view, args, context| {
        Ok(boolean(view.evaluate(TypePredicate::BigIntLike)))
    }),
);

fn ast_node(value: &JsValue) -> JsResult<JsSyntaxNode> {
    JsAstNode::from_value(value)
        .map(|node| node.node)
        .ok_or_else(|| {
            JsNativeError::typ()
                .with_message("Pass a node from the analyzed source.")
                .into()
        })
}

fn string_arg(value: &JsValue, name: &str) -> JsResult<String> {
    value
        .as_string()
        .ok_or_else(|| {
            JsNativeError::typ().with_message(format!("The {name} argument must be a string."))
        })?
        .to_std_string()
        .map_err(|_| {
            JsNativeError::typ()
                .with_message(format!(
                    "The {name} argument contains an incomplete Unicode character."
                ))
                .into()
        })
}

fn index_arg(value: &JsValue, context: &mut Context) -> JsResult<usize> {
    let index = value.to_number(context)?;
    if !index.is_finite() || index < 0.0 || index.fract() != 0.0 {
        return Err(JsNativeError::typ()
            .with_message("The argument index must be a non-negative integer.")
            .into());
    }
    Ok(index as usize)
}

fn classification_arg(value: &JsValue) -> JsResult<TypeClassification> {
    let name = string_arg(value, "classification")?;
    match name.as_str() {
        "promise" => Ok(TypeClassification::Promise),
        "thenable" => Ok(TypeClassification::Thenable),
        "arrayOfPromises" => Ok(TypeClassification::ArrayOfPromises),
        "promiseReturningFunction" => Ok(TypeClassification::PromiseReturningFunction),
        _ => Err(JsNativeError::typ()
            .with_message(
                "The classification must be \"promise\", \"thenable\", \"arrayOfPromises\", or \"promiseReturningFunction\".",
            )
            .into()),
    }
}

fn classification_result(value: TypeClassificationResult) -> JsValue {
    JsValue::from(JsString::from(match value {
        TypeClassificationResult::Match => "match",
        TypeClassificationResult::NoMatch => "noMatch",
        TypeClassificationResult::Indeterminate => "indeterminate",
    }))
}

fn optional_type(view: Option<Rc<dyn TypeView>>, context: &mut Context) -> JsValue {
    view.map_or_else(JsValue::undefined, |view| JsTypeInfo::wrap(view, context))
}

fn optional_bool(value: Option<bool>) -> JsValue {
    value.map_or_else(JsValue::undefined, JsValue::from)
}

fn boolean(value: Option<bool>) -> JsValue {
    JsValue::from(value.unwrap_or(false))
}

pub(crate) fn register_types(context: &mut Context) -> JsResult<()> {
    fn register<C: Class>(context: &mut Context) -> JsResult<()> {
        context.register_global_class::<C>()?;
        context
            .global_object()
            .delete_property_or_throw(js_string!(C::NAME), context)?;
        Ok(())
    }
    register::<JsTypeService>(context)?;
    register::<JsTypeInfo>(context)
}
