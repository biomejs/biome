//! Selects the signature that a call point takes when its callee has several
//! signatures or a generic one, as TypeScript's overload resolution selects
//! it before deciding what the call does to the flow.
//!
//! TypeScript considers the call only when some signature returns a type
//! predicate or an assertion, or has the return type annotation `never`.
//! It then tries the signatures in order, infers the type arguments of a
//! generic one from the arguments, and takes the first signature that
//! accepts them under its subtype relation, or under assignability when
//! none does. Biome follows those steps, but decides only where it can
//! tell what TypeScript does: a signature whose type arguments it can't
//! infer, or that may or may not accept the arguments, ends the selection
//! without a signature, so the call narrows nothing.
//!
//! The arguments have their types at the call, as TypeScript checks them:
//! the narrowed type of a read where narrowing changes it. A read there must
//! not depend on what the call itself does, as it would in a loop that
//! brings the call back before the read, so the read narrows only by the
//! call points that end before the call starts (see
//! [`ResolutionCtx::argument_type_at_call`]).

use super::ResolutionCtx;
use crate::db::queries::{
    ArgumentAcceptance, ArgumentRelation, ResolvedCallArgument, call_signatures,
    callback_predicate_binding, generic_substitutions, satisfies_constraint,
    signature_accepts_arguments,
};
use crate::db::type_inference::{
    apply_substitutions, apply_substitutions_to_root_body, resolve_local_type_on_demand,
};
use crate::js_module_info::flow::FlowCall;
use biome_js_type_info::interned_types::{
    InternedFunction, InternedGenericTypeParameter, ReturnType, TypeData, TypeSubstitution,
};
use biome_js_type_info::{
    CallArgumentType, TypeReference, TypeofCallExpression, base_literal_type,
};
use biome_rowan::TextRange;

/// The outcome of inferring the type arguments of one signature for a call.
enum Instantiation<'db> {
    /// The signature with its type parameters replaced by the inferred type
    /// arguments, or the signature itself when it has none.
    Bound(InternedFunction<'db>),
    /// An inferred type argument violates its type parameter's constraint.
    /// TypeScript then uses the constraint instead, and the signature
    /// doesn't accept the argument that the type argument came from.
    Rejected,
    /// Biome can't infer the type arguments that TypeScript infers.
    Undetermined,
}

/// How the type of a parameter refers to the type parameters of its
/// signature, which decides what an argument infers for them.
enum ParameterShape<'db> {
    /// The type refers to no type parameter of the signature.
    Plain,
    /// The type is a type parameter, as `T` in `isOf<T>(v: unknown, sample: T)`,
    /// which infers the argument's type.
    Bare(InternedGenericTypeParameter<'db>),
    /// The type is a union of a type parameter with `null` or `undefined`,
    /// as `T | null` in `isNonNull<T>(v: T | null)`, which infers the members
    /// of the argument's type other than those.
    Nullable(InternedGenericTypeParameter<'db>, Vec<TypeData<'db>>),
    /// Any other type that refers to a type parameter, such as a callback
    /// type whose type predicate targets one.
    Other,
}

impl<'db> ResolutionCtx<'db, '_> {
    /// Returns the signature that the call point `call`, whose collected call
    /// is `raw`, takes when `callee`, the type of its callee, has several
    /// signatures or a generic one, with its type parameters replaced by the
    /// type arguments that the call infers.
    ///
    /// The signatures are tried in the order of [`call_signatures`], after
    /// their type arguments are inferred ([`Self::instantiate_signature`]),
    /// first with TypeScript's subtype relation and then, when no signature
    /// applies, with assignability, as [`signature_accepts_arguments`]
    /// decides. The first signature that accepts the arguments is selected.
    /// So with `declare function f(x: string, y: string): x is "a"` and
    /// `declare function f(x: string, y: unknown): x is string`, a call
    /// `f(x, y)` with `y: any` takes the second signature, because `any` is
    /// a subtype only of `any` and `unknown`.
    ///
    /// Returns `None`, so that the call narrows nothing, when no signature
    /// returns a type predicate or an assertion or has the return type
    /// `never`, when a spread argument is passed, when Biome can't tell
    /// whether a signature that TypeScript tries before the selected one
    /// accepts the arguments, and when no signature accepts them. A signature
    /// with a `this` parameter is undetermined among several, because
    /// TypeScript also checks the object that the callee is read from. A
    /// single generic signature needs only its type arguments, because
    /// TypeScript takes the only signature even when it reports an error for
    /// an argument.
    pub(super) fn select_effect_signature(
        &mut self,
        call: &FlowCall,
        raw: &TypeofCallExpression,
        callee: TypeData<'db>,
    ) -> Option<InternedFunction<'db>> {
        let db = self.db;
        let signatures = call_signatures(db, callee)?;
        if !signatures
            .iter()
            .any(|signature| has_effect_return(db, *signature))
        {
            return None;
        }
        let arguments = self.call_point_arguments(call, raw)?;
        if let [signature] = signatures.as_slice() {
            return match self.instantiate_signature(*signature, &arguments, &raw.callee) {
                Instantiation::Bound(instantiated) => Some(instantiated),
                Instantiation::Rejected | Instantiation::Undetermined => None,
            };
        }
        // TypeScript tries every signature with the subtype relation first,
        // and only when none applies, every signature with assignability.
        let mut candidates = Vec::with_capacity(signatures.len());
        for signature in signatures {
            if signature
                .parameters(db)
                .iter()
                .any(|parameter| parameter.is_this())
            {
                return None;
            }
            let instantiated = match self.instantiate_signature(signature, &arguments, &raw.callee)
            {
                Instantiation::Bound(instantiated) => instantiated,
                Instantiation::Rejected => continue,
                Instantiation::Undetermined => return None,
            };
            match signature_accepts_arguments(
                db,
                instantiated,
                &arguments,
                ArgumentRelation::Subtype,
            ) {
                ArgumentAcceptance::Accepts => return Some(instantiated),
                ArgumentAcceptance::Rejects => candidates.push(instantiated),
                ArgumentAcceptance::Undetermined => return None,
            }
        }
        for candidate in candidates {
            match signature_accepts_arguments(
                db,
                candidate,
                &arguments,
                ArgumentRelation::Assignability,
            ) {
                ArgumentAcceptance::Accepts => return Some(candidate),
                ArgumentAcceptance::Rejects => {}
                ArgumentAcceptance::Undetermined => return None,
            }
        }
        None
    }

    /// Returns the arguments of the call point `call`, whose collected call
    /// is `raw`, with their types at the call. An argument that reads a
    /// subject has its type there, as [`Self::argument_type_at_call`] gives
    /// it, and any other argument its ordinary type, with the literal types
    /// of the properties of an object or array expression kept apart, as for
    /// overload selection of a call's return type.
    ///
    /// Returns `None` when the call passes a spread argument, which can fill
    /// any number of parameters.
    fn call_point_arguments(
        &mut self,
        call: &FlowCall,
        raw: &TypeofCallExpression,
    ) -> Option<Vec<ResolvedCallArgument<'db>>> {
        let mut arguments = Vec::with_capacity(raw.arguments.len());
        for (index, argument) in raw.arguments.iter().enumerate() {
            let CallArgumentType::Argument(reference) = argument else {
                return None;
            };
            let argument = match call.arguments.get(index) {
                Some(argument) if argument.subject.is_some() => ResolvedCallArgument::Argument(
                    self.argument_type_at_call(argument.range, call.range)?,
                ),
                _ => {
                    let ty = self.resolve(reference);
                    match self.literal_argument(reference, ty) {
                        Some(literal_ty) => {
                            ResolvedCallArgument::WidenedArgument { ty, literal_ty }
                        }
                        None => ResolvedCallArgument::Argument(ty),
                    }
                }
            };
            arguments.push(argument);
        }
        Some(arguments)
    }

    /// Returns the type of the argument at `range` at the call at `call`: its
    /// narrowed type where narrowing changes it, and its ordinary type
    /// otherwise.
    ///
    /// The narrowing applies only the call points that end before the call
    /// starts, together with the other tests and writes along the paths to
    /// the argument. A call point after it reaches the argument only through
    /// a loop, as does the call itself, whose effect needs the argument's
    /// type, so leaving those out keeps the selection free of cycles. The
    /// argument's type is then wider than TypeScript's in such a loop, which
    /// can only make the selection undetermined or select a signature that
    /// also accepts TypeScript's type.
    fn argument_type_at_call(
        &mut self,
        range: TextRange,
        call: TextRange,
    ) -> Option<TypeData<'db>> {
        let bound = self
            .calls_before
            .map_or(call.start(), |bound| bound.min(call.start()));
        let previous = self.calls_before.replace(bound);
        let narrowed = self.flow_operand_at(range);
        self.calls_before = previous;
        narrowed.or_else(|| self.raw_operand_at(range))
    }

    /// Infers the type arguments of `signature` from `arguments`, as
    /// TypeScript does when overload resolution tries it, and returns the
    /// signature with its type parameters replaced.
    ///
    /// Each type parameter must be inferred from exactly one kind of
    /// parameter:
    ///
    /// - a parameter whose type is the type parameter itself, as
    ///   `sample: T`, infers the argument's type;
    /// - a parameter whose type is a union of the type parameter with `null`
    ///   or `undefined`, as `v: T | null`, infers the members of the
    ///   argument's type other than those;
    /// - a callback parameter whose type predicate targets the type
    ///   parameter infers the target of the argument's type predicate (see
    ///   [`callback_predicate_binding`]).
    ///
    /// TypeScript widens a literal type that the first two kinds infer, as
    /// `"a"` to `string`, unless the type parameter is at the top level of
    /// the signature's type predicate or assertion, as in `v is T`, so Biome
    /// widens it there too. Widening a literal type that TypeScript keeps,
    /// as for a variable whose declared type is a literal type, gives a wider
    /// target, never a narrower one.
    ///
    /// Each inferred type must satisfy its type parameter's constraint (see
    /// [`satisfies_constraint`]), read with the type arguments of the object
    /// that the callee, whose collected type is `raw_callee`, is read from.
    ///
    /// Returns [`Instantiation::Undetermined`] when a parameter of any other
    /// kind refers to a type parameter, when a type parameter is inferred
    /// from two arguments with different types, or from none while a
    /// parameter or the target of the type predicate or assertion refers to
    /// it, and when a constraint can't be checked. Returns
    /// [`Instantiation::Rejected`] when an inferred type violates its
    /// constraint.
    fn instantiate_signature(
        &mut self,
        signature: InternedFunction<'db>,
        arguments: &[ResolvedCallArgument<'db>],
        raw_callee: &TypeReference,
    ) -> Instantiation<'db> {
        let db = self.db;
        let type_parameters = signature.type_parameters(db);
        if type_parameters.is_empty() {
            return Instantiation::Bound(signature);
        }
        let mut generics = Vec::with_capacity(type_parameters.len());
        for type_parameter in type_parameters {
            let Some(generic) = type_parameter.as_generic_reference(db) else {
                return Instantiation::Undetermined;
            };
            generics.push(generic);
        }
        let mut bindings: Vec<(InternedGenericTypeParameter<'db>, TypeData<'db>)> = Vec::new();
        let parameters = signature
            .parameters(db)
            .iter()
            .filter(|parameter| !parameter.is_this());
        for (index, parameter) in parameters.enumerate() {
            let ty = parameter.ty();
            let shape = parameter_shape(db, ty, &generics);
            if matches!(shape, ParameterShape::Plain) {
                continue;
            }
            if parameter.is_rest() {
                return Instantiation::Undetermined;
            }
            let Some(argument) = arguments.get(index) else {
                continue;
            };
            let argument = argument.ty();
            let is_callback = matches!(shape, ParameterShape::Other);
            let binding = match shape {
                ParameterShape::Plain => continue,
                ParameterShape::Bare(generic) => Some((generic, argument)),
                ParameterShape::Nullable(generic, nullish) => {
                    without_members(db, argument, &nullish).map(|inferred| (generic, inferred))
                }
                ParameterShape::Other => callback_predicate_binding(db, signature, ty, argument),
            };
            // TypeScript widens a literal type that it infers from an
            // argument unless the type parameter is at the top level of the
            // type predicate or the assertion. A callback's type predicate is
            // never such a literal.
            let binding = binding.map(|(generic, inferred)| {
                if is_callback || is_at_top_level(db, signature, generic) {
                    (generic, inferred)
                } else {
                    (generic, base_literal_type(db, inferred))
                }
            });
            let Some((generic, inferred)) = binding else {
                return Instantiation::Undetermined;
            };
            match bindings.iter().find(|(bound, _)| *bound == generic) {
                Some((_, existing)) if *existing != inferred => {
                    return Instantiation::Undetermined;
                }
                Some(_) => {}
                None => bindings.push((generic, inferred)),
            }
        }
        // TypeScript infers `unknown` for a type parameter that no argument
        // infers, which changes nothing that the call does when no parameter
        // refers to it and neither does the target of the signature's type
        // predicate or assertion.
        for generic in &generics {
            if !bindings.iter().any(|(bound, _)| bound == generic)
                && signature_refers_to(db, signature, *generic)
            {
                return Instantiation::Undetermined;
            }
        }
        let substitutions: Vec<TypeSubstitution<'db>> = bindings
            .iter()
            .flat_map(|(generic, inferred)| generic_substitutions(db, *generic, *inferred))
            .collect();
        for (generic, inferred) in &bindings {
            let mut receiver = || self.callee_receiver(raw_callee);
            match satisfies_constraint(db, *generic, *inferred, &mut receiver, &substitutions) {
                Some(true) => {}
                Some(false) => return Instantiation::Rejected,
                None => return Instantiation::Undetermined,
            }
        }
        match apply_substitutions_to_root_body(db, TypeData::Function(signature), &substitutions)
            .callable_function(db)
        {
            Some(instantiated) => Instantiation::Bound(instantiated),
            None => Instantiation::Undetermined,
        }
    }
}

/// Returns whether `signature` returns a type predicate or an assertion, or
/// has the return type `never`, the signatures that TypeScript considers
/// when it decides what a call does to the flow. Whether a `never` comes
/// from an annotation is checked for the selected signature only.
fn has_effect_return<'db>(db: &'db dyn crate::ModuleDb, signature: InternedFunction<'db>) -> bool {
    match signature.return_type(db) {
        ReturnType::Predicate(_) | ReturnType::Asserts(_) => true,
        ReturnType::Type(ty) => resolve_local_type_on_demand(db, *ty) == TypeData::NeverKeyword,
    }
}

/// Returns whether a parameter of `signature`, or the target of its type
/// predicate or assertion, refers to `generic`.
fn signature_refers_to<'db>(
    db: &'db dyn crate::ModuleDb,
    signature: InternedFunction<'db>,
    generic: InternedGenericTypeParameter<'db>,
) -> bool {
    let substitutions = generic_substitutions(db, generic, TypeData::Unknown);
    let refers_to = |ty: TypeData<'db>| apply_substitutions(db, ty, &substitutions) != ty;
    let target = match signature.return_type(db) {
        ReturnType::Predicate(predicate) => Some(predicate.ty),
        ReturnType::Asserts(assertion) => Some(assertion.ty),
        ReturnType::Type(_) => None,
    };
    signature
        .parameters(db)
        .iter()
        .filter(|parameter| !parameter.is_this())
        .map(|parameter| parameter.ty())
        .chain(target)
        .any(refers_to)
}

/// Returns how `ty`, the type of a parameter, refers to `generics`, the type
/// parameters of its signature.
fn parameter_shape<'db>(
    db: &'db dyn crate::ModuleDb,
    ty: TypeData<'db>,
    generics: &[InternedGenericTypeParameter<'db>],
) -> ParameterShape<'db> {
    if let Some(generic) = ty.as_generic_reference(db) {
        return if generics.contains(&generic) {
            ParameterShape::Bare(generic)
        } else {
            ParameterShape::Other
        };
    }
    if let TypeData::Union(union) = ty {
        let mut generic = None;
        let mut nullish = Vec::new();
        let mut supported = true;
        for member in union.types(db) {
            match member.as_generic_reference(db) {
                Some(member) if generic.is_none() && generics.contains(&member) => {
                    generic = Some(member);
                }
                Some(_) => supported = false,
                None if matches!(member, TypeData::Null | TypeData::Undefined) => {
                    nullish.push(*member);
                }
                None => supported = false,
            }
        }
        if let (Some(generic), true) = (generic, supported) {
            return ParameterShape::Nullable(generic, nullish);
        }
    }
    if ty.may_refer_to_free_generic(db) {
        ParameterShape::Other
    } else {
        ParameterShape::Plain
    }
}

/// Returns the members of `ty` other than those in `removed`, or `None` when
/// none remains.
fn without_members<'db>(
    db: &'db dyn crate::ModuleDb,
    ty: TypeData<'db>,
    removed: &[TypeData<'db>],
) -> Option<TypeData<'db>> {
    let members = if let TypeData::Union(union) = ty {
        union.types(db).to_vec()
    } else {
        Vec::from([ty])
    };
    let remaining: Vec<_> = members
        .into_iter()
        .filter(|member| !removed.contains(member))
        .collect();
    (!remaining.is_empty()).then(|| TypeData::union_from_types(db, remaining))
}

/// Returns whether `generic` is at the top level of the type that the type
/// predicate or the assertion of `signature` targets: the type itself, or a
/// member of it when it is a union. TypeScript keeps a literal type that it
/// infers for such a type parameter.
fn is_at_top_level<'db>(
    db: &'db dyn crate::ModuleDb,
    signature: InternedFunction<'db>,
    generic: InternedGenericTypeParameter<'db>,
) -> bool {
    let target = match signature.return_type(db) {
        ReturnType::Predicate(predicate) => predicate.ty,
        ReturnType::Asserts(assertion) => assertion.ty,
        ReturnType::Type(_) => return false,
    };
    let is_generic = |ty: TypeData<'db>| ty.as_generic_reference(db) == Some(generic);
    if let TypeData::Union(union) = target {
        union.types(db).iter().copied().any(is_generic)
    } else {
        is_generic(target)
    }
}
