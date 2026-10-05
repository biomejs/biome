use crate::ast::JsAstNode;
use biome_js_semantic::{
    Binding, GlobalReference, JsDeclarationKind, Reference, Scope, SemanticModel,
    UnresolvedReference,
};
use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;
use biome_js_syntax::{AnyJsIdentifierReference, JsSyntaxNode};
use biome_rowan::AstNode;
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::object::builtins::JsArray;
use boa_engine::object::{JsObject, ObjectInitializer};
use boa_engine::property::Attribute;
use boa_engine::{
    Context, Finalize, JsData, JsNativeError, JsResult, JsValue, NativeFunction, Trace, js_string,
};
use std::rc::Rc;

/// Declares a Boa class wrapping an owned semantic handle and its JavaScript methods.
///
/// Each method entry is `(name, argument_count, |handle, args, context| { ... })`.
/// The count excludes `this`: it sets the function's `length` and the exact number
/// of arguments accepted. The receiver and argument count are checked before the
/// body runs. `handle` is an `Rc` to the native value; the body returns `JsResult<JsValue>`.
macro_rules! semantic_object {
    ($wrapper:ident, $native:ty, $name:literal, $(($method:literal, $arity:literal, |$value:ident, $args:ident, $context:ident| $body:block)),+ $(,)?) => {
        #[derive(Clone, JsData)]
        pub(crate) struct $wrapper(Rc<$native>);

        impl Finalize for $wrapper {}

        // SAFETY: Semantic handles own Rust model data, which contains no Boa-managed values.
        unsafe impl Trace for $wrapper {
            boa_engine::gc::empty_trace!();
        }

        impl $wrapper {
            pub(crate) fn wrap(value: $native, context: &mut Context) -> JsValue {
                let prototype = context.get_global_class::<Self>().map(|class| class.prototype());
                JsObject::from_proto_and_data(prototype, Self(Rc::new(value))).into()
            }

            fn from_value(value: &JsValue) -> JsResult<Rc<$native>> {
                let object = value.as_object().ok_or_else(|| JsNativeError::typ()
                    .with_message(concat!("Call this method on a ", $name, " returned by Biome.")))?;
                let native = object.downcast_ref::<Self>().ok_or_else(|| JsNativeError::typ()
                    .with_message(concat!("Call this method on a ", $name, " returned by Biome.")))?;
                Ok(Rc::clone(&native.0))
            }
        }

        impl Class for $wrapper {
            const NAME: &'static str = $name;

            fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
                $(class.method(
                    js_string!($method), $arity,
                    NativeFunction::from_fn_ptr(|this, $args, $context| {
                        let $value = Self::from_value(this)?;
                        let _ = &$context;
                        if $args.len() != $arity {
                            return Err(JsNativeError::typ().with_message(format!(
                                "{}.{}() requires {} argument(s).", $name, $method, $arity,
                            )).into());
                        }
                        $body
                    }),
                );)+
                Ok(())
            }

            fn data_constructor(_new_target: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<Self> {
                Err(JsNativeError::typ().with_message(concat!(
                    $name, " has no public constructor. Obtain semantic information through context.model in a semantic rule.",
                )).into())
            }
        }
    };
}

semantic_object!(
    JsSemanticModel,
    SemanticModel,
    "SemanticModel",
    ("binding", 1, |model, args, context| {
        let node = model_node(&model, &args[0])?;
        let reference = reference_node(node)?;
        Ok(optional_binding(model.binding(&reference), context))
    }),
    ("asBinding", 1, |model, args, context| {
        let node = model_node(&model, &args[0])?;
        let binding = binding_node(node)?;
        Ok(optional_binding(
            model.as_binding_by_range(binding.syntax().text_trimmed_range()),
            context,
        ))
    }),
    ("allBindings", 0, |model, args, context| {
        Ok(array(model.all_bindings(), JsBinding::wrap, context))
    }),
    ("allExportedBindings", 0, |model, args, context| {
        Ok(array(
            model.all_exported_bindings(),
            JsBinding::wrap,
            context,
        ))
    }),
    ("hasExports", 0, |model, args, context| {
        Ok(model.has_exports().into())
    }),
    ("scopes", 0, |model, args, context| {
        Ok(array(model.scopes(), JsScope::wrap, context))
    }),
    ("globalScope", 0, |model, args, context| {
        Ok(JsScope::wrap(model.global_scope(), context))
    }),
    ("scope", 1, |model, args, context| {
        let node = model_node(&model, &args[0])?;
        if node.text_trimmed_range().is_empty() {
            return Ok(JsValue::undefined());
        }
        Ok(JsScope::wrap(model.scope(&node), context))
    }),
    ("scopeHoistedTo", 1, |model, args, context| {
        let node = model_node(&model, &args[0])?;
        Ok(optional_scope(model.scope_hoisted_to(&node), context))
    }),
    ("allGlobalReferences", 0, |model, args, context| {
        Ok(array(
            model.all_global_references(),
            JsGlobalReference::wrap,
            context,
        ))
    }),
    ("allUnresolvedReferences", 0, |model, args, context| {
        Ok(array(
            model.all_unresolved_references(),
            JsUnresolvedReference::wrap,
            context,
        ))
    }),
    ("isGlobalReference", 1, |model, args, context| {
        let node = reference_node(model_node(&model, &args[0])?)?;
        Ok(model.is_global_reference(&node).into())
    }),
    ("isUnresolvedReference", 1, |model, args, context| {
        let node = reference_node(model_node(&model, &args[0])?)?;
        Ok(model.is_unresolved_reference(&node).into())
    }),
    ("isImported", 1, |model, args, context| {
        let node = model_node(&model, &args[0])?;
        let binding = if let Some(reference) = AnyJsIdentifierReference::cast_ref(&node) {
            model.binding(&reference)
        } else {
            let binding = binding_node(node)?;
            model.as_binding_by_range(binding.syntax().text_trimmed_range())
        };
        Ok(binding.is_some_and(|binding| binding.is_imported()).into())
    }),
    ("isExported", 1, |model, args, context| {
        let node = model_node(&model, &args[0])?;
        let exported = if let Some(reference) = AnyJsIdentifierReference::cast_ref(&node) {
            model.is_exported(&reference).unwrap_or(false)
        } else {
            model.is_exported(&binding_node(node)?)
        };
        Ok(exported.into())
    }),
);

semantic_object!(
    JsBinding,
    Binding,
    "Binding",
    ("syntax", 0, |binding, args, context| {
        Ok(JsAstNode::from_node(binding.syntax(), context))
    }),
    ("declarationKind", 0, |binding, args, context| {
        let kind = match binding.declaration_kind() {
            JsDeclarationKind::Class => js_string!("class"),
            JsDeclarationKind::Enum => js_string!("enum"),
            JsDeclarationKind::Function => js_string!("function"),
            JsDeclarationKind::Generic => js_string!("generic"),
            JsDeclarationKind::HoistedValue => js_string!("hoistedValue"),
            JsDeclarationKind::Import => js_string!("import"),
            JsDeclarationKind::ImportType => js_string!("importType"),
            JsDeclarationKind::Interface => js_string!("interface"),
            JsDeclarationKind::Module => js_string!("module"),
            JsDeclarationKind::Namespace => js_string!("namespace"),
            JsDeclarationKind::Type => js_string!("type"),
            JsDeclarationKind::Unknown => js_string!("unknown"),
            JsDeclarationKind::Using => js_string!("using"),
            JsDeclarationKind::Value => js_string!("value"),
        };
        Ok(kind.into())
    }),
    ("exports", 0, |binding, args, context| {
        Ok(array(binding.exports(), JsAstNode::from_node, context))
    }),
    ("exportRanges", 0, |binding, args, context| {
        Ok(array(
            binding.export_ranges().iter().copied(),
            |range, context| {
                ObjectInitializer::new(context)
                    .property(
                        js_string!("start"),
                        u32::from(range.start()),
                        Attribute::ENUMERABLE,
                    )
                    .property(
                        js_string!("end"),
                        u32::from(range.end()),
                        Attribute::ENUMERABLE,
                    )
                    .build()
                    .into()
            },
            context,
        ))
    }),
    ("scope", 0, |binding, args, context| {
        Ok(JsScope::wrap(binding.scope(), context))
    }),
    ("allReferences", 0, |binding, args, context| {
        Ok(array(binding.all_references(), JsReference::wrap, context))
    }),
    ("allReads", 0, |binding, args, context| {
        Ok(array(binding.all_reads(), JsReference::wrap, context))
    }),
    ("allWrites", 0, |binding, args, context| {
        Ok(array(binding.all_writes(), JsReference::wrap, context))
    }),
    ("isImported", 0, |binding, args, context| {
        Ok(binding.is_imported().into())
    }),
    ("isExported", 0, |binding, args, context| {
        Ok(binding.is_exported().into())
    }),
);

semantic_object!(
    JsReference,
    Reference,
    "Reference",
    ("syntax", 0, |reference, args, context| {
        Ok(JsAstNode::from_node(reference.syntax(), context))
    }),
    ("binding", 0, |reference, args, context| {
        Ok(optional_binding(reference.binding(), context))
    }),
    ("scope", 0, |reference, args, context| {
        Ok(JsScope::wrap(reference.scope(), context))
    }),
    ("isRead", 0, |reference, args, context| {
        Ok(reference.is_read().into())
    }),
    ("isWrite", 0, |reference, args, context| {
        Ok(reference.is_write().into())
    }),
);

semantic_object!(
    JsScope,
    Scope,
    "Scope",
    ("syntax", 0, |scope, args, context| {
        Ok(JsAstNode::from_node(scope.syntax(), context))
    }),
    ("isGlobalScope", 0, |scope, args, context| {
        Ok(scope.is_global_scope().into())
    }),
    ("parent", 0, |scope, args, context| {
        Ok(optional_scope(scope.parent(), context))
    }),
    ("children", 0, |scope, args, context| {
        Ok(array(scope.children(), JsScope::wrap, context))
    }),
    ("ancestors", 0, |scope, args, context| {
        Ok(array(scope.ancestors(), JsScope::wrap, context))
    }),
    ("bindings", 0, |scope, args, context| {
        Ok(array(scope.bindings(), JsBinding::wrap, context))
    }),
    ("getBinding", 1, |scope, args, context| {
        let name = args[0].as_string().ok_or_else(|| {
            JsNativeError::typ()
                .with_message("Scope.getBinding() requires an identifier name as a string.")
        })?;
        let name = name.to_std_string().map_err(|_| JsNativeError::typ()
            .with_message("The binding name contains an incomplete Unicode character. Supply a complete identifier name."))?;
        Ok(optional_binding(scope.get_binding(&name), context))
    }),
);

semantic_object!(
    JsGlobalReference,
    GlobalReference,
    "GlobalReference",
    ("syntax", 0, |reference, args, context| {
        Ok(JsAstNode::from_node(reference.syntax(), context))
    }),
    ("isRead", 0, |reference, args, context| {
        Ok(reference.is_read().into())
    }),
    ("isWrite", 0, |reference, args, context| {
        Ok(reference.is_write().into())
    }),
);

semantic_object!(
    JsUnresolvedReference,
    UnresolvedReference,
    "UnresolvedReference",
    ("syntax", 0, |reference, args, context| {
        Ok(JsAstNode::from_node(reference.syntax(), context))
    }),
);

fn model_node(model: &SemanticModel, value: &JsValue) -> JsResult<JsSyntaxNode> {
    let node = JsAstNode::from_value(value)
        .ok_or_else(|| {
            JsNativeError::typ()
                .with_message("Pass a node from the source associated with this semantic model.")
        })?
        .node;
    if node.ancestors().last().as_ref() != Some(model.root().syntax()) {
        return Err(JsNativeError::typ().with_message(
            "The node belongs to a different source than this semantic model. Use a node from the model's original source.",
        ).into());
    }
    Ok(node)
}

fn reference_node(node: JsSyntaxNode) -> JsResult<AnyJsIdentifierReference> {
    AnyJsIdentifierReference::cast(node).ok_or_else(|| JsNativeError::typ()
        .with_message("Pass a JsReferenceIdentifier, JsIdentifierAssignment, or JsxReferenceIdentifier node.").into())
}

fn binding_node(node: JsSyntaxNode) -> JsResult<AnyJsIdentifierBinding> {
    AnyJsIdentifierBinding::cast(node).ok_or_else(|| JsNativeError::typ()
        .with_message("Pass an identifier binding, type parameter name, or literal enum member name node.").into())
}

fn optional_binding(binding: Option<Binding>, context: &mut Context) -> JsValue {
    binding.map_or_else(JsValue::undefined, |binding| {
        JsBinding::wrap(binding, context)
    })
}

fn optional_scope(scope: Option<Scope>, context: &mut Context) -> JsValue {
    scope.map_or_else(JsValue::undefined, |scope| JsScope::wrap(scope, context))
}

fn array<T>(
    items: impl IntoIterator<Item = T>,
    wrap: fn(T, &mut Context) -> JsValue,
    context: &mut Context,
) -> JsValue {
    let values = items
        .into_iter()
        .map(|item| wrap(item, context))
        .collect::<Vec<_>>();
    JsArray::from_iter(values, context).into()
}

pub(crate) fn register_semantic(context: &mut Context) -> JsResult<()> {
    fn register<C: Class>(context: &mut Context) -> JsResult<()> {
        context.register_global_class::<C>()?;
        context
            .global_object()
            .delete_property_or_throw(js_string!(C::NAME), context)?;
        Ok(())
    }
    register::<JsSemanticModel>(context)?;
    register::<JsBinding>(context)?;
    register::<JsReference>(context)?;
    register::<JsScope>(context)?;
    register::<JsGlobalReference>(context)?;
    register::<JsUnresolvedReference>(context)
}
