//! Answers narrow Promise-related questions from raw expression types.
//!
//! Callers ask whether an expression, a function return, or an awaited value has
//! a particular Promise shape. Answering that question usually needs only the
//! references and selected members along one path. The classifier follows that
//! path in the raw type graph instead of inferring sibling object members and
//! function parameters.
//!
//! A classification state identifies the module that owns a raw reference, the
//! value or instance side used for member lookup, and the unconsumed member
//! path. Named-member traversal examines only members declared directly on the
//! current type. A member available only from a base type is therefore
//! indeterminate. A pending member path may cross a non-generic concrete class
//! created by `new` without inspecting its constructor arguments.
//! Function-return classification has one narrower inheritance case: an
//! interface with no call signature may follow its single base interface.
//! Traversal visits at most 1024 distinct classification states. Unsupported
//! expression forms, accessors, ambiguous exports, cycles, and an exhausted
//! work limit are indeterminate.

use super::{ImportResolution, ResolutionCtx, find_value_member_type_on_demand};
use crate::db::queries::{
    function_returns_promise, is_array_of_promise_type, is_promise_type, resolved_export_origin,
};
use crate::js_module_info::TsBindingReferenceExt;
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsModuleInfo, JsOwnExport, ModuleDb, ResolvedPath, SymbolFromModuleInfo};
use biome_js_type_info::{
    GlobalTypeId, ImportSymbol, Literal, RawTypeData, RawTypeId, ScopeId, TypeId, TypeMember,
    TypeReference, TypeReferenceQualifier, TypeResolverLevel, TypeofExpression, global_types,
    interned_types::{
        ReturnType, TypeData as InferredTypeData, TypeSubstitution, TypeTransformResult,
    },
};
use biome_rowan::Text;
use rustc_hash::FxHashSet;

const MAX_PROMISE_CLASSIFICATION_STATES: usize = 1024;

/// Result of a narrow Promise-related classification.
///
/// The question may concern the expression itself, a function return, an array
/// element, or a value after `await`. The variant names are shared by all of
/// these questions and must be read as yes, no, or unknown for the requested
/// classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::db) enum PromiseClassification {
    /// The requested Promise shape is present.
    ReturnsPromise,
    /// The requested Promise shape is conclusively absent.
    DoesNotReturnPromise,
    /// Selective traversal cannot answer the question.
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum MemberLookupMode {
    /// Selects static members when the target is a class value.
    Value,
    /// Selects instance members when the target is a class.
    Instance,
    /// Selects a constructed instance after consuming its value-side members.
    Constructed { remaining: usize },
}

impl MemberLookupMode {
    fn prepend_constructed_members(self, count: usize) -> Self {
        match self {
            Self::Constructed { remaining } => Self::Constructed {
                remaining: remaining.saturating_add(count),
            },
            mode @ (Self::Value | Self::Instance) => mode,
        }
    }

    fn after_member(self) -> Self {
        match self {
            Self::Value | Self::Instance | Self::Constructed { remaining: 0 } => Self::Value,
            Self::Constructed { remaining } => Self::Constructed {
                remaining: remaining - 1,
            },
        }
    }

    fn after_namespace_member(self) -> Option<Self> {
        match self {
            Self::Constructed { remaining: 0 } => None,
            Self::Constructed { remaining } => Some(Self::Constructed {
                remaining: remaining - 1,
            }),
            mode @ (Self::Value | Self::Instance) => Some(mode),
        }
    }
}

/// The Promise-related property requested from the current target.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Projection {
    /// Whether an expression evaluates to a Promise-like value.
    Promise,
    /// Whether an instance target represents `Promise` or `PromiseLike`.
    PromiseTarget,
    /// Whether a callable returns a Promise-like value.
    FunctionReturn,
    /// Whether an expression evaluates to an array of Promise-like values.
    ArrayPromise,
    /// Whether a callable returns an array of Promise-like values.
    ArrayFunctionReturn,
    /// Whether awaiting an expression produces an array of Promise-like values.
    AwaitedArrayPromise,
    /// Whether awaiting a callable's return produces an array of Promise-like values.
    AwaitedArrayFunctionReturn,
}

/// A location in the raw type graph that has not yet been classified.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum ClassificationTarget {
    /// A raw type reference owned by the state's current module.
    Reference(TypeReference),
    /// An entry in the current module's raw local type table.
    Local(TypeId),
    /// A symbol imported from another resolved module path.
    Import {
        /// Path used to locate the imported module in the module graph.
        resolved_path: ResolvedPath,
        /// Export selected from the imported module.
        symbol: ImportSymbol,
    },
    /// An export name owned by the state's current module.
    Export(Text),
}

/// One position in the raw projection.
///
/// `members` stores the property names still to consume, in access order. An
/// empty path applies the current projection directly to `target`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct ClassificationState {
    module: ModuleInfo,
    target: ClassificationTarget,
    mode: MemberLookupMode,
    members: Box<[Text]>,
    projection: Projection,
}

/// Classifies a raw expression's Promise shape without inferring all module tables.
///
/// The projection follows lexical bindings, imports, exports, aliases, `this`,
/// calls, non-generic class instances created by `new`, and own named members.
/// Unsupported expression forms remain indeterminate instead of entering
/// complete expression inference.
pub(in crate::db) fn classify_expression_promise(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    reference: TypeReference,
) -> PromiseClassification {
    classify_expression(db, module, reference, Projection::Promise, true)
}

/// Classifies whether a raw expression is an array of Promise-like values.
pub(in crate::db) fn classify_expression_array_promise(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    reference: TypeReference,
) -> PromiseClassification {
    classify_expression(db, module, reference, Projection::ArrayPromise, true)
}

/// Classifies a raw expression's function return without inferring all module tables.
///
/// The projection follows lexical bindings, imports, exports, aliases, `this`,
/// and own named members. Only the selected function's return reference enters
/// regular inferred-type resolution. Sibling members and function parameters
/// are not resolved.
pub(in crate::db) fn classify_expression_function_return(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    reference: TypeReference,
) -> PromiseClassification {
    classify_expression(db, module, reference, Projection::FunctionReturn, false)
}

fn classify_expression(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    reference: TypeReference,
    projection: Projection,
    allow_new_instance_members: bool,
) -> PromiseClassification {
    use PromiseClassification::{DoesNotReturnPromise, Indeterminate, ReturnsPromise};

    let mut state = ClassificationState {
        module,
        target: ClassificationTarget::Reference(reference),
        mode: MemberLookupMode::Value,
        members: Box::default(),
        projection,
    };
    let mut seen = FxHashSet::default();

    for _ in 0..MAX_PROMISE_CLASSIFICATION_STATES {
        if !seen.insert(state.clone()) {
            return Indeterminate;
        }
        db.unwind_if_revision_cancelled();

        state = match state.target {
            ClassificationTarget::Reference(reference) => match reference {
                TypeReference::Resolved(resolved) => match resolved.level() {
                    TypeResolverLevel::Thin => ClassificationState {
                        target: ClassificationTarget::Local(resolved.id()),
                        ..state
                    },
                    TypeResolverLevel::Global if state.members.is_empty() => {
                        let Some(id) = GlobalTypeId::try_from_type_id(resolved.id()) else {
                            return Indeterminate;
                        };
                        let ty = match state.projection {
                            Projection::Promise => global_types(db).get(id),
                            Projection::PromiseTarget => {
                                let target = global_types(db).get(id);
                                InferredTypeData::instance_of(db, target, Box::default())
                            }
                            Projection::FunctionReturn => {
                                let ty = global_types(db).get(id);
                                let Some(function) = ty.callable_function(db) else {
                                    return DoesNotReturnPromise;
                                };
                                let ReturnType::Type(return_ty) = function.return_type(db) else {
                                    return DoesNotReturnPromise;
                                };
                                *return_ty
                            }
                            Projection::ArrayPromise => global_types(db).get(id),
                            Projection::ArrayFunctionReturn => {
                                let ty = global_types(db).get(id);
                                let Some(function) = ty.callable_function(db) else {
                                    return DoesNotReturnPromise;
                                };
                                let ReturnType::Type(return_ty) = function.return_type(db) else {
                                    return DoesNotReturnPromise;
                                };
                                *return_ty
                            }
                            Projection::AwaitedArrayPromise => global_types(db).get(id),
                            Projection::AwaitedArrayFunctionReturn => {
                                let ty = global_types(db).get(id);
                                let Some(function) = ty.callable_function(db) else {
                                    return DoesNotReturnPromise;
                                };
                                let ReturnType::Type(return_ty) = function.return_type(db) else {
                                    return DoesNotReturnPromise;
                                };
                                let ModuleInfoKind::Js(js_info) = state.module.kind(db) else {
                                    return Indeterminate;
                                };
                                let mut ctx = ResolutionCtx::new(
                                    db,
                                    state.module,
                                    &js_info,
                                    ImportResolution::on_demand(),
                                );
                                let Some(awaited) = ctx.resolve_await_expression(*return_ty) else {
                                    return Indeterminate;
                                };
                                return match is_array_of_promise_type(db, awaited) {
                                    Some(true) => ReturnsPromise,
                                    Some(false) => DoesNotReturnPromise,
                                    None => Indeterminate,
                                };
                            }
                        };
                        let result = match state.projection {
                            Projection::Promise
                            | Projection::PromiseTarget
                            | Projection::FunctionReturn => is_promise_type(db, ty),
                            Projection::ArrayPromise
                            | Projection::ArrayFunctionReturn
                            | Projection::AwaitedArrayPromise => is_array_of_promise_type(db, ty),
                            Projection::AwaitedArrayFunctionReturn => unreachable!(),
                        };
                        return match result {
                            Some(true) => ReturnsPromise,
                            Some(false) => DoesNotReturnPromise,
                            None => Indeterminate,
                        };
                    }
                    TypeResolverLevel::Global
                    | TypeResolverLevel::Full
                    | TypeResolverLevel::Import => return Indeterminate,
                },
                TypeReference::Qualifier(qualifier) => {
                    let mut qualifier_path = qualifier.path.iter();
                    let Some(identifier) = qualifier_path.next() else {
                        return Indeterminate;
                    };
                    let mode = state
                        .mode
                        .prepend_constructed_members(qualifier.path.len().saturating_sub(1));
                    let members: Box<[Text]> = qualifier_path
                        .cloned()
                        .chain(state.members.iter().cloned())
                        .collect();
                    let ModuleInfoKind::Js(js_info) = state.module.kind(db) else {
                        return Indeterminate;
                    };
                    let mut scope = js_info.semantic_model.scope_from_id(qualifier.scope_id);

                    loop {
                        let binding = scope
                            .get_binding_reference(identifier.text())
                            .and_then(|reference| {
                                reference.get_binding_id_for_qualifier(&qualifier)
                            })
                            .and_then(|id| {
                                js_info
                                    .semantic_model
                                    .binding_by_id(id)
                                    .map(|binding| (id, binding))
                            });
                        if let Some((binding_id, binding)) = binding {
                            if !qualifier.type_parameters.is_empty()
                                && matches!(
                                    state.projection,
                                    Projection::FunctionReturn
                                        | Projection::ArrayFunctionReturn
                                        | Projection::AwaitedArrayFunctionReturn
                                )
                            {
                                return Indeterminate;
                            }
                            if matches!(
                                state.projection,
                                Projection::FunctionReturn
                                    | Projection::ArrayFunctionReturn
                                    | Projection::AwaitedArrayFunctionReturn
                            ) && scope
                                .overload_sets()
                                .iter()
                                .any(|set| set.contains(&binding_id))
                            {
                                return Indeterminate;
                            }
                            if binding.is_imported() {
                                let Some(import) = js_info.static_imports.get(identifier.text())
                                else {
                                    return Indeterminate;
                                };
                                break ClassificationState {
                                    module: state.module,
                                    target: ClassificationTarget::Import {
                                        resolved_path: import.resolved_path.clone(),
                                        symbol: import.symbol.clone(),
                                    },
                                    mode,
                                    members: members.clone(),
                                    projection: state.projection,
                                };
                            }
                            let binding_range = binding.syntax().text_trimmed_range();
                            let Some(reference) = js_info.raw_binding_types.get(&binding_range)
                            else {
                                return Indeterminate;
                            };
                            break ClassificationState {
                                module: state.module,
                                target: ClassificationTarget::Reference(reference.clone()),
                                mode,
                                members: members.clone(),
                                projection: state.projection,
                            };
                        }
                        if let Some(parent) = scope.parent() {
                            scope = parent;
                            continue;
                        }

                        if matches!(mode, MemberLookupMode::Constructed { .. }) {
                            return Indeterminate;
                        }
                        let mut ctx = ResolutionCtx::new(
                            db,
                            state.module,
                            &js_info,
                            ImportResolution::on_demand(),
                        );
                        let mut ty = ctx.resolve_qualifier(&qualifier);
                        for member in &members {
                            let Some(member_ty) =
                                find_value_member_type_on_demand(db, ty, member.text())
                            else {
                                return Indeterminate;
                            };
                            ty = member_ty;
                        }
                        return match state.projection {
                            Projection::Promise => match is_promise_type(db, ty) {
                                Some(true) => ReturnsPromise,
                                Some(false) => DoesNotReturnPromise,
                                None => Indeterminate,
                            },
                            Projection::FunctionReturn => {
                                if let InferredTypeData::GlobalType(id) = ty {
                                    ty = global_types(db).get(id);
                                }
                                let result = function_returns_promise(db, ty);
                                match result {
                                    Some(true) => ReturnsPromise,
                                    Some(false) => DoesNotReturnPromise,
                                    None => Indeterminate,
                                }
                            }
                            Projection::ArrayPromise => match is_array_of_promise_type(db, ty) {
                                Some(true) => ReturnsPromise,
                                Some(false) => DoesNotReturnPromise,
                                None => Indeterminate,
                            },
                            Projection::ArrayFunctionReturn => {
                                if let InferredTypeData::GlobalType(id) = ty {
                                    ty = global_types(db).get(id);
                                }
                                let Some(function) = ty.callable_function(db) else {
                                    return DoesNotReturnPromise;
                                };
                                let ReturnType::Type(return_ty) = function.return_type(db) else {
                                    return DoesNotReturnPromise;
                                };
                                match is_array_of_promise_type(db, *return_ty) {
                                    Some(true) => ReturnsPromise,
                                    Some(false) => DoesNotReturnPromise,
                                    None => Indeterminate,
                                }
                            }
                            Projection::AwaitedArrayPromise => {
                                let Some(awaited) = ctx.resolve_await_expression(ty) else {
                                    return Indeterminate;
                                };
                                match is_array_of_promise_type(db, awaited) {
                                    Some(true) => ReturnsPromise,
                                    Some(false) => DoesNotReturnPromise,
                                    None => Indeterminate,
                                }
                            }
                            Projection::AwaitedArrayFunctionReturn => {
                                if let InferredTypeData::GlobalType(id) = ty {
                                    ty = global_types(db).get(id);
                                }
                                let Some(function) = ty.callable_function(db) else {
                                    return DoesNotReturnPromise;
                                };
                                let ReturnType::Type(return_ty) = function.return_type(db) else {
                                    return DoesNotReturnPromise;
                                };
                                let Some(awaited) = ctx.resolve_await_expression(*return_ty) else {
                                    return Indeterminate;
                                };
                                match is_array_of_promise_type(db, awaited) {
                                    Some(true) => ReturnsPromise,
                                    Some(false) => DoesNotReturnPromise,
                                    None => Indeterminate,
                                }
                            }
                            Projection::PromiseTarget => Indeterminate,
                        };
                    }
                }
                TypeReference::Import(import) => ClassificationState {
                    module: state.module,
                    target: ClassificationTarget::Import {
                        resolved_path: import.resolved_path.clone(),
                        symbol: import.symbol.clone(),
                    },
                    mode: state.mode,
                    members: state.members,
                    projection: state.projection,
                },
            },
            ClassificationTarget::Local(type_id) => {
                let ModuleInfoKind::Js(js_info) = state.module.kind(db) else {
                    return Indeterminate;
                };
                let Some(raw) = js_info.raw_types.get(type_id.index()) else {
                    return Indeterminate;
                };
                if matches!(state.mode, MemberLookupMode::Constructed { remaining: 0 })
                    && !(matches!(
                        raw,
                        RawTypeData::Class(_)
                            | RawTypeData::Reference(_)
                            | RawTypeData::TypeofType(_)
                            | RawTypeData::TypeofValue(_)
                    ) || matches!(
                        raw,
                        RawTypeData::TypeofExpression(expression)
                            if matches!(
                                expression.as_ref(),
                                TypeofExpression::StaticMember(_)
                                    | TypeofExpression::OptionalChainStaticMember(_)
                            )
                    ))
                {
                    return Indeterminate;
                }

                match raw {
                    RawTypeData::Function(function) => {
                        if !state.members.is_empty() {
                            return Indeterminate;
                        }
                        if matches!(
                            state.projection,
                            Projection::Promise
                                | Projection::PromiseTarget
                                | Projection::ArrayPromise
                                | Projection::AwaitedArrayPromise
                        ) {
                            return DoesNotReturnPromise;
                        }
                        if function.is_async {
                            match state.projection {
                                Projection::FunctionReturn => return ReturnsPromise,
                                Projection::ArrayFunctionReturn => return DoesNotReturnPromise,
                                Projection::AwaitedArrayFunctionReturn => {}
                                Projection::Promise
                                | Projection::PromiseTarget
                                | Projection::ArrayPromise
                                | Projection::AwaitedArrayPromise => unreachable!(),
                            };
                        }
                        let Some(return_ty) = function.return_type.as_type() else {
                            return DoesNotReturnPromise;
                        };
                        // Only function-return projections can follow a returned call. The next
                        // target is the call expression itself, so map them to their equivalent
                        // expression projections while retaining whether the result is awaited.
                        // Other projections classify values rather than function returns; `None`
                        // leaves them on the regular resolution path.
                        let returned_call_projection = match state.projection {
                            Projection::FunctionReturn => Some(Projection::Promise),
                            Projection::ArrayFunctionReturn => Some(Projection::ArrayPromise),
                            Projection::AwaitedArrayFunctionReturn => {
                                Some(Projection::AwaitedArrayPromise)
                            }
                            Projection::Promise
                            | Projection::PromiseTarget
                            | Projection::ArrayPromise
                            | Projection::AwaitedArrayPromise => None,
                        };
                        if let Some(projection) = returned_call_projection
                            && let Some(returned_call) =
                                returned_call_reference(&js_info, return_ty, function.is_async)
                        {
                            ClassificationState {
                                module: state.module,
                                target: ClassificationTarget::Reference(returned_call),
                                mode: MemberLookupMode::Value,
                                members: Box::default(),
                                projection,
                            }
                        } else {
                            let mut ctx = ResolutionCtx::new(
                                db,
                                state.module,
                                &js_info,
                                ImportResolution::on_demand(),
                            );
                            let ty = ctx.resolve(return_ty);
                            let result = match state.projection {
                                Projection::FunctionReturn => is_promise_type(db, ty),
                                Projection::ArrayFunctionReturn => is_array_of_promise_type(db, ty),
                                Projection::AwaitedArrayFunctionReturn => {
                                    let Some(awaited) = ctx.resolve_await_expression(ty) else {
                                        return Indeterminate;
                                    };
                                    is_array_of_promise_type(db, awaited)
                                }
                                Projection::Promise
                                | Projection::PromiseTarget
                                | Projection::ArrayPromise
                                | Projection::AwaitedArrayPromise => unreachable!(),
                            };
                            if matches!(result, Some(false)) {
                                for parameter in &function.type_parameters {
                                    let parameter = ctx.resolve(parameter);
                                    let TypeTransformResult::Transformed(substituted) = ty
                                        .substitute_type(
                                            db,
                                            TypeSubstitution {
                                                generic: parameter,
                                                replacement: InferredTypeData::Unknown,
                                            },
                                        )
                                    else {
                                        return Indeterminate;
                                    };
                                    if substituted != ty {
                                        return Indeterminate;
                                    }
                                }
                            }
                            return match result {
                                Some(true) => ReturnsPromise,
                                Some(false) => DoesNotReturnPromise,
                                None => Indeterminate,
                            };
                        }
                    }
                    RawTypeData::Reference(reference) => ClassificationState {
                        target: ClassificationTarget::Reference(reference.clone()),
                        ..state
                    },
                    RawTypeData::TypeofType(reference) => ClassificationState {
                        target: ClassificationTarget::Reference((**reference).clone()),
                        ..state
                    },
                    RawTypeData::TypeofValue(value) => {
                        let reference = if value.ty.is_unknown() {
                            TypeReferenceQualifier::from_path(
                                value.scope_id.unwrap_or(ScopeId::GLOBAL),
                                value.identifier.clone(),
                            )
                            .into()
                        } else {
                            value.ty.clone()
                        };
                        ClassificationState {
                            target: ClassificationTarget::Reference(reference),
                            ..state
                        }
                    }
                    RawTypeData::TypeofExpression(expression) => match expression.as_ref() {
                        TypeofExpression::StaticMember(expression)
                        | TypeofExpression::OptionalChainStaticMember(expression) => {
                            ClassificationState {
                                module: state.module,
                                target: ClassificationTarget::Reference(expression.object.clone()),
                                mode: state.mode.prepend_constructed_members(1),
                                members: std::iter::once(expression.member.clone())
                                    .chain(state.members.iter().cloned())
                                    .collect(),
                                projection: state.projection,
                            }
                        }
                        TypeofExpression::This(_)
                            if matches!(state.mode, MemberLookupMode::Constructed { .. }) =>
                        {
                            return Indeterminate;
                        }
                        TypeofExpression::This(expression) => ClassificationState {
                            module: state.module,
                            target: ClassificationTarget::Reference(expression.parent.clone()),
                            mode: MemberLookupMode::Instance,
                            members: state.members,
                            projection: state.projection,
                        },
                        TypeofExpression::Await(_)
                            if matches!(state.projection, Projection::Promise) =>
                        {
                            return DoesNotReturnPromise;
                        }
                        TypeofExpression::Await(expression)
                            if matches!(
                                state.projection,
                                Projection::ArrayPromise | Projection::AwaitedArrayPromise
                            ) =>
                        {
                            ClassificationState {
                                module: state.module,
                                target: ClassificationTarget::Reference(
                                    expression.argument.clone(),
                                ),
                                mode: MemberLookupMode::Value,
                                members: Box::default(),
                                projection: Projection::AwaitedArrayPromise,
                            }
                        }
                        TypeofExpression::Call(expression)
                            if state.members.is_empty()
                                && matches!(
                                    state.projection,
                                    Projection::Promise
                                        | Projection::ArrayPromise
                                        | Projection::AwaitedArrayPromise
                                ) =>
                        {
                            ClassificationState {
                                module: state.module,
                                target: ClassificationTarget::Reference(expression.callee.clone()),
                                mode: MemberLookupMode::Value,
                                members: Box::default(),
                                projection: match state.projection {
                                    Projection::Promise => Projection::FunctionReturn,
                                    Projection::ArrayPromise => Projection::ArrayFunctionReturn,
                                    Projection::AwaitedArrayPromise => {
                                        Projection::AwaitedArrayFunctionReturn
                                    }
                                    Projection::PromiseTarget
                                    | Projection::FunctionReturn
                                    | Projection::ArrayFunctionReturn
                                    | Projection::AwaitedArrayFunctionReturn => unreachable!(),
                                },
                            }
                        }
                        TypeofExpression::New(expression)
                            if allow_new_instance_members && !state.members.is_empty() =>
                        {
                            if matches!(state.mode, MemberLookupMode::Constructed { .. }) {
                                return Indeterminate;
                            }
                            ClassificationState {
                                module: state.module,
                                target: ClassificationTarget::Reference(expression.callee.clone()),
                                mode: MemberLookupMode::Constructed { remaining: 0 },
                                members: state.members,
                                projection: state.projection,
                            }
                        }
                        TypeofExpression::Addition(_)
                        | TypeofExpression::Await(_)
                        | TypeofExpression::BitwiseNot(_)
                        | TypeofExpression::Call(_)
                        | TypeofExpression::CallArgument(_)
                        | TypeofExpression::Parameter(_)
                        | TypeofExpression::Conditional(_)
                        | TypeofExpression::Destructure(_)
                        | TypeofExpression::Index(_)
                        | TypeofExpression::OptionalChainIndex(_)
                        | TypeofExpression::IterableValueOf(_)
                        | TypeofExpression::LogicalAnd(_)
                        | TypeofExpression::LogicalOr(_)
                        | TypeofExpression::New(_)
                        | TypeofExpression::NullishCoalescing(_)
                        | TypeofExpression::Super(_)
                        | TypeofExpression::Typeof(_)
                        | TypeofExpression::UnaryMinus(_) => return Indeterminate,
                    },
                    RawTypeData::InstanceOf(_)
                        if matches!(state.mode, MemberLookupMode::Constructed { .. }) =>
                    {
                        return Indeterminate;
                    }
                    RawTypeData::InstanceOf(instance)
                        if !instance.type_parameters.is_empty()
                            && matches!(
                                state.projection,
                                Projection::FunctionReturn
                                    | Projection::ArrayFunctionReturn
                                    | Projection::AwaitedArrayFunctionReturn
                            ) =>
                    {
                        return Indeterminate;
                    }
                    RawTypeData::InstanceOf(instance) => match state.projection {
                        Projection::FunctionReturn
                        | Projection::ArrayFunctionReturn
                        | Projection::AwaitedArrayFunctionReturn => ClassificationState {
                            module: state.module,
                            target: ClassificationTarget::Reference(instance.ty.clone()),
                            mode: MemberLookupMode::Instance,
                            members: state.members,
                            projection: state.projection,
                        },
                        Projection::Promise if state.members.is_empty() => ClassificationState {
                            module: state.module,
                            target: ClassificationTarget::Reference(instance.ty.clone()),
                            mode: MemberLookupMode::Instance,
                            members: Box::default(),
                            projection: Projection::PromiseTarget,
                        },
                        Projection::Promise => ClassificationState {
                            module: state.module,
                            target: ClassificationTarget::Reference(instance.ty.clone()),
                            mode: MemberLookupMode::Instance,
                            members: state.members,
                            projection: Projection::Promise,
                        },
                        Projection::PromiseTarget => return DoesNotReturnPromise,
                        Projection::ArrayPromise if state.members.is_empty() => {
                            let mut ctx = ResolutionCtx::new(
                                db,
                                state.module,
                                &js_info,
                                ImportResolution::on_demand(),
                            );
                            return match is_array_of_promise_type(
                                db,
                                ctx.resolve_raw_type_id(type_id),
                            ) {
                                Some(true) => ReturnsPromise,
                                Some(false) => DoesNotReturnPromise,
                                None => Indeterminate,
                            };
                        }
                        Projection::ArrayPromise => ClassificationState {
                            module: state.module,
                            target: ClassificationTarget::Reference(instance.ty.clone()),
                            mode: MemberLookupMode::Instance,
                            members: state.members,
                            projection: state.projection,
                        },
                        Projection::AwaitedArrayPromise if state.members.is_empty() => {
                            let mut ctx = ResolutionCtx::new(
                                db,
                                state.module,
                                &js_info,
                                ImportResolution::on_demand(),
                            );
                            let ty = ctx.resolve_raw_type_id(type_id);
                            let Some(awaited) = ctx.resolve_await_expression(ty) else {
                                return Indeterminate;
                            };
                            return match is_array_of_promise_type(db, awaited) {
                                Some(true) => ReturnsPromise,
                                Some(false) => DoesNotReturnPromise,
                                None => Indeterminate,
                            };
                        }
                        Projection::AwaitedArrayPromise => ClassificationState {
                            module: state.module,
                            target: ClassificationTarget::Reference(instance.ty.clone()),
                            mode: MemberLookupMode::Instance,
                            members: state.members,
                            projection: state.projection,
                        },
                    },
                    RawTypeData::Class(class) => {
                        if matches!(state.projection, Projection::PromiseTarget) {
                            return DoesNotReturnPromise;
                        }
                        if matches!(state.mode, MemberLookupMode::Constructed { remaining: 0 })
                            && !class.type_parameters.is_empty()
                        {
                            return Indeterminate;
                        }
                        let Some((name, remaining)) = state.members.split_first() else {
                            return DoesNotReturnPromise;
                        };
                        let Some(member) = find_own_member(&class.members, name, Some(state.mode))
                        else {
                            return Indeterminate;
                        };
                        if member.is_getter() {
                            return Indeterminate;
                        }
                        ClassificationState {
                            module: state.module,
                            target: ClassificationTarget::Reference(member.ty.clone()),
                            mode: state.mode.after_member(),
                            members: remaining.into(),
                            projection: state.projection,
                        }
                    }
                    RawTypeData::Object(object) => {
                        if matches!(state.projection, Projection::PromiseTarget) {
                            return DoesNotReturnPromise;
                        }
                        if state.members.is_empty()
                            && matches!(
                                state.projection,
                                Projection::FunctionReturn
                                    | Projection::ArrayFunctionReturn
                                    | Projection::AwaitedArrayFunctionReturn
                            )
                        {
                            match sole_call_signature(&object.members) {
                                Ok(Some(call_signature)) => ClassificationState {
                                    module: state.module,
                                    target: ClassificationTarget::Reference(
                                        call_signature.ty.clone(),
                                    ),
                                    mode: MemberLookupMode::Value,
                                    members: Box::default(),
                                    projection: state.projection,
                                },
                                Ok(None) => return DoesNotReturnPromise,
                                Err(()) => return Indeterminate,
                            }
                        } else {
                            let Some((name, remaining)) = state.members.split_first() else {
                                return DoesNotReturnPromise;
                            };
                            let Some(member) = find_own_member(&object.members, name, None) else {
                                return Indeterminate;
                            };
                            if member.is_getter() {
                                return Indeterminate;
                            }
                            ClassificationState {
                                module: state.module,
                                target: ClassificationTarget::Reference(member.ty.clone()),
                                mode: state.mode.after_member(),
                                members: remaining.into(),
                                projection: state.projection,
                            }
                        }
                    }
                    RawTypeData::Literal(literal) => match literal.as_ref() {
                        Literal::Object(object) => {
                            let Some((name, remaining)) = state.members.split_first() else {
                                return DoesNotReturnPromise;
                            };
                            let Some(member) = find_own_member(object.members(), name, None) else {
                                return Indeterminate;
                            };
                            if member.is_getter() {
                                return Indeterminate;
                            }
                            ClassificationState {
                                module: state.module,
                                target: ClassificationTarget::Reference(member.ty.clone()),
                                mode: state.mode.after_member(),
                                members: remaining.into(),
                                projection: state.projection,
                            }
                        }
                        Literal::BigInt(_)
                        | Literal::Boolean(_)
                        | Literal::Number(_)
                        | Literal::RegExp(_)
                        | Literal::String(_)
                        | Literal::Template(_) => {
                            return if state.members.is_empty() {
                                DoesNotReturnPromise
                            } else {
                                Indeterminate
                            };
                        }
                    },
                    RawTypeData::Global
                    | RawTypeData::BigInt
                    | RawTypeData::Boolean
                    | RawTypeData::Null
                    | RawTypeData::Number
                    | RawTypeData::String
                    | RawTypeData::Symbol
                    | RawTypeData::Undefined
                    | RawTypeData::Conditional
                    | RawTypeData::Constructor(_)
                    | RawTypeData::Tuple(_)
                    | RawTypeData::ThisKeyword
                    | RawTypeData::NeverKeyword
                    | RawTypeData::ObjectKeyword
                    | RawTypeData::VoidKeyword => {
                        return if state.members.is_empty() {
                            DoesNotReturnPromise
                        } else {
                            Indeterminate
                        };
                    }
                    RawTypeData::Unknown
                    | RawTypeData::ImportNamespace(_)
                    | RawTypeData::Module(_)
                    | RawTypeData::Namespace(_)
                    | RawTypeData::Generic(_)
                    | RawTypeData::Intersection(_)
                    | RawTypeData::Union(_)
                    | RawTypeData::TypeOperator(_)
                    | RawTypeData::MergedReference(_)
                    | RawTypeData::AnyKeyword
                    | RawTypeData::UnknownKeyword => return Indeterminate,
                    RawTypeData::Interface(interface)
                        if matches!(state.projection, Projection::PromiseTarget) =>
                    {
                        if interface.name.text() != "PromiseLike" {
                            return DoesNotReturnPromise;
                        }
                        let mut ctx = ResolutionCtx::new(
                            db,
                            state.module,
                            &js_info,
                            ImportResolution::on_demand(),
                        );
                        let target = ctx.resolve_raw_type_id(type_id);
                        let instance = InferredTypeData::instance_of(db, target, Box::default());
                        return match is_promise_type(db, instance) {
                            Some(true) => ReturnsPromise,
                            Some(false) => DoesNotReturnPromise,
                            None => Indeterminate,
                        };
                    }
                    RawTypeData::Interface(interface) => {
                        if state.members.is_empty()
                            && matches!(
                                state.projection,
                                Projection::FunctionReturn
                                    | Projection::ArrayFunctionReturn
                                    | Projection::AwaitedArrayFunctionReturn
                            )
                        {
                            match sole_call_signature(&interface.members) {
                                Ok(Some(call_signature)) => ClassificationState {
                                    module: state.module,
                                    target: ClassificationTarget::Reference(
                                        call_signature.ty.clone(),
                                    ),
                                    mode: MemberLookupMode::Value,
                                    members: Box::default(),
                                    projection: state.projection,
                                },
                                Ok(None) => match interface.extends.as_ref() {
                                    [extends] => ClassificationState {
                                        module: state.module,
                                        target: ClassificationTarget::Reference(extends.clone()),
                                        mode: state.mode,
                                        members: Box::default(),
                                        projection: state.projection,
                                    },
                                    [] => return DoesNotReturnPromise,
                                    [_, ..] => return Indeterminate,
                                },
                                Err(()) => return Indeterminate,
                            }
                        } else {
                            let Some((name, remaining)) = state.members.split_first() else {
                                return Indeterminate;
                            };
                            let Some(member) = find_own_member(&interface.members, name, None)
                            else {
                                return Indeterminate;
                            };
                            if member.is_getter() {
                                return Indeterminate;
                            }
                            ClassificationState {
                                module: state.module,
                                target: ClassificationTarget::Reference(member.ty.clone()),
                                mode: state.mode.after_member(),
                                members: remaining.into(),
                                projection: state.projection,
                            }
                        }
                    }
                }
            }
            ClassificationTarget::Import {
                resolved_path,
                symbol,
            } => {
                let Some(path) = resolved_path.as_path() else {
                    return DoesNotReturnPromise;
                };
                let Some(module) = db.module_for_path(path) else {
                    return DoesNotReturnPromise;
                };
                let (name, members, mode) = match symbol {
                    ImportSymbol::All => {
                        let Some((name, remaining)) = state.members.split_first() else {
                            return Indeterminate;
                        };
                        let Some(mode) = state.mode.after_namespace_member() else {
                            return Indeterminate;
                        };
                        (name.clone(), remaining.into(), mode)
                    }
                    ImportSymbol::Default => {
                        (Text::new_static("default"), state.members, state.mode)
                    }
                    ImportSymbol::Named(name) => (name, state.members, state.mode),
                };
                ClassificationState {
                    module,
                    target: ClassificationTarget::Export(name),
                    mode,
                    members,
                    projection: state.projection,
                }
            }
            ClassificationTarget::Export(name) => {
                let symbol = SymbolFromModuleInfo::new(db, name.text().to_string(), state.module);
                let (module, name) = match resolved_export_origin(db, symbol) {
                    super::ExportOriginResult::Found { module, name } => (*module, name.clone()),
                    super::ExportOriginResult::Missing
                    | super::ExportOriginResult::Ambiguous
                    | super::ExportOriginResult::Indeterminate => return Indeterminate,
                };
                let ModuleInfoKind::Js(js_info) = module.kind(db) else {
                    return Indeterminate;
                };
                let Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) =
                    js_info.exports.get(name.text())
                else {
                    return Indeterminate;
                };

                match own_export {
                    JsOwnExport::Binding(range) => {
                        let Some(reference) = js_info.raw_binding_types.get(range) else {
                            return Indeterminate;
                        };
                        ClassificationState {
                            module,
                            target: ClassificationTarget::Reference(reference.clone()),
                            mode: state.mode,
                            members: state.members,
                            projection: state.projection,
                        }
                    }
                    JsOwnExport::Type(resolved) => ClassificationState {
                        module,
                        target: ClassificationTarget::Reference(TypeReference::Resolved(
                            RawTypeId::Local(*resolved),
                        )),
                        mode: state.mode,
                        members: state.members,
                        projection: state.projection,
                    },
                    JsOwnExport::Namespace(reexport) => ClassificationState {
                        module,
                        target: ClassificationTarget::Import {
                            resolved_path: reexport.import.resolved_path.clone(),
                            symbol: reexport.import.symbol.clone(),
                        },
                        mode: state.mode,
                        members: state.members,
                        projection: state.projection,
                    },
                }
            }
        };
    }

    Indeterminate
}

/// Finds an own named member visible through the requested side of a class.
///
/// `mode` is omitted for object types, whose members are always selected as
/// instance properties. Accessor dereferencing remains the caller's concern.
fn find_own_member<'a>(
    members: &'a [TypeMember],
    name: &Text,
    mode: Option<MemberLookupMode>,
) -> Option<&'a TypeMember> {
    members.iter().find(|member| {
        member.has_name(name.text())
            && mode.is_none_or(|mode| match mode {
                MemberLookupMode::Value | MemberLookupMode::Constructed { remaining: 1.. } => {
                    member.is_static()
                }
                MemberLookupMode::Instance | MemberLookupMode::Constructed { remaining: 0 } => {
                    !member.is_static()
                }
            })
    })
}

fn sole_call_signature(members: &[TypeMember]) -> Result<Option<&TypeMember>, ()> {
    let mut call_signatures = members
        .iter()
        .filter(|member| member.kind.is_call_signature());
    let call_signature = call_signatures.next();
    if call_signatures.next().is_some() {
        Err(())
    } else {
        Ok(call_signature)
    }
}

fn returned_call_reference(
    js_info: &JsModuleInfo,
    return_ty: &TypeReference,
    is_async: bool,
) -> Option<TypeReference> {
    let TypeReference::Resolved(resolved) = return_ty else {
        return None;
    };
    if resolved.level() != TypeResolverLevel::Thin {
        return None;
    }
    let raw = js_info.raw_types.get(resolved.id().index())?;

    let (return_ty, raw) = if is_async {
        let RawTypeData::InstanceOf(instance) = raw else {
            return None;
        };
        let TypeReference::Qualifier(qualifier) = &instance.ty else {
            return None;
        };
        if !qualifier.is_promise() {
            return None;
        }
        let return_ty = qualifier.type_parameters.first()?;
        let TypeReference::Resolved(resolved) = return_ty else {
            return None;
        };
        if resolved.level() != TypeResolverLevel::Thin {
            return None;
        }
        let raw = js_info.raw_types.get(resolved.id().index())?;
        (return_ty, raw)
    } else {
        (return_ty, raw)
    };
    matches!(
        raw,
        RawTypeData::TypeofExpression(expression)
            if matches!(expression.as_ref(), TypeofExpression::Call(_))
    )
    .then(|| return_ty.clone())
}
