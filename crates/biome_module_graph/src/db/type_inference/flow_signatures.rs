//! Selects the signature that a call point takes when its callee has several
//! signatures or a generic one, as TypeScript's overload resolution selects
//! it before deciding what the call does to the flow.
//!
//! TypeScript considers the call only when some signature returns a type
//! predicate or an assertion, or has the return type annotation `never`.
//! It then tries the signatures in order, infers the type arguments of a
//! generic one from the arguments, and takes the first signature that
//! accepts them under its subtype relation, or under assignability when
//! none does.
//!
//! TypeScript checks the arguments with their types at the call, and a test
//! that Biome doesn't decompose can make those narrower than Biome's, as a
//! guard whose type predicate TypeScript infers does. So Biome decides only
//! where every such narrowing leads TypeScript to a signature that narrows
//! alike: it skips a signature only when the signature rejects the arguments
//! whatever narrowing gives them, and takes the first one that accepts them
//! under the subtype relation, which then accepts any narrower types too,
//! when each signature before it that it can't skip narrows the same way.
//! Otherwise the selection ends without a signature, and the call narrows
//! nothing. Assignability never selects a signature, because TypeScript
//! tries it only once every signature fails the subtype relation, which
//! narrowing can change for any signature that Biome can't skip.
//!
//! The type arguments that the call infers can be wider than TypeScript's
//! for the same reason, and so can a literal type that TypeScript keeps but
//! Biome can't tell from one that it widens. A wider target of a type
//! predicate still narrows soundly where the call is truthy, but removes too
//! much where it is falsy, so such a call narrows only where it is truthy
//! (see [`EffectSignature::narrows_falsy`]).
//!
//! A read of an argument at the call must not depend on what the call itself
//! does, as it would in a loop that brings the call back before the read, so
//! the read narrows only by the call points that end before the call starts
//! (see [`ResolutionCtx::argument_type_at_call`]).

use super::ResolutionCtx;
use crate::db::queries::{
    ResolvedCallArgument, call_signatures, callback_predicate_binding, generic_substitutions,
    rejects_every_narrowing, satisfies_constraint, signature_accepts_arguments,
};
use crate::db::type_inference::{
    apply_substitutions, apply_substitutions_to_root_body, resolve_local_type_on_demand,
};
use crate::js_module_info::flow::FlowCall;
use biome_js_type_info::interned_types::{
    FunctionParameter, InternedFunction, InternedGenericTypeParameter, Literal, ReturnType,
    TypeData, TypeSubstitution,
};
use biome_js_type_info::{CallArgumentType, TypeOperator, TypeReference, TypeofCallExpression};
use biome_rowan::TextRange;

/// Limits the types visited to find a type parameter at the top level of a
/// type predicate's target, or a primitive type in a constraint.
const MAX_TOP_LEVEL_STEPS: usize = 64;

/// The signature that a call point takes, with its type parameters replaced
/// by the type arguments that the call infers.
pub(super) struct EffectSignature<'db> {
    pub(super) function: InternedFunction<'db>,
    /// Whether the call also narrows where it is falsy, which needs the type
    /// arguments that TypeScript infers. A type argument that may be wider
    /// gives a target that may be wider, which narrows soundly only where the
    /// call is truthy, as the target of an assertion does.
    pub(super) narrows_falsy: bool,
}

/// The outcome of inferring the type arguments of one signature for a call.
enum Instantiation<'db> {
    /// The signature with its type parameters replaced by the inferred type
    /// arguments, or the signature itself when it has none.
    Bound(EffectSignature<'db>),
    /// A type argument that TypeScript infers too violates its type
    /// parameter's constraint. TypeScript then uses the constraint instead,
    /// and the signature doesn't accept the argument that the type argument
    /// came from.
    Rejected,
    /// Biome can't infer the type arguments that TypeScript infers, or can't
    /// tell whether they satisfy their constraints.
    Undetermined,
}

/// The type argument that one argument infers for a type parameter.
struct Binding<'db> {
    generic: InternedGenericTypeParameter<'db>,
    inferred: TypeData<'db>,
    /// Whether TypeScript infers the same type argument, or one that narrows
    /// the same way (see [`ResolutionCtx::instantiate_signature`]).
    exact: bool,
}

/// The arguments of a call point with their types at the call.
struct CallPointArguments<'db> {
    types: Vec<ResolvedCallArgument<'db>>,
    /// Whether narrowing gives the argument at each position a type other
    /// than its ordinary type. Narrowing can make a literal type fresh, as
    /// `y === "a"` does for `y: string`, so the literal types of such an
    /// argument aren't those of its declaration.
    narrowed: Vec<bool>,
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
    /// their type arguments are inferred ([`Self::instantiate_signature`]).
    /// The first signature that accepts the arguments under TypeScript's
    /// subtype relation ([`signature_accepts_arguments`]) is selected, when
    /// each signature before it rejects the arguments whatever narrowing
    /// gives them ([`rejects_every_narrowing`]), or has the same type
    /// predicate or assertion. So with `declare function g(v: number): v is 1`
    /// and `declare function g(v: string): v is "a"`, a call `g(x)` with
    /// `x: "a" | "b"` takes the second signature, because neither `"a"` nor
    /// `"b"` is a number. With `x: string | number` and the signatures
    /// `(v: string): v is "a"` and `(v: string | number): v is string`, the
    /// call narrows nothing: a test that Biome doesn't decompose could narrow
    /// `x` to `string`, which the first signature accepts.
    ///
    /// Returns `None`, so that the call narrows nothing, when no signature
    /// returns a type predicate or an assertion or has the return type
    /// `never`, when a spread argument is passed, when a signature that
    /// Biome can't skip and that narrows differently comes before the one
    /// that accepts the arguments, and when no signature accepts them. A
    /// signature with a `this` parameter is undetermined among several,
    /// because TypeScript also checks the object that the callee is read
    /// from. A single generic signature needs only its type arguments,
    /// because TypeScript takes the only signature even when it reports an
    /// error for an argument.
    pub(super) fn select_effect_signature(
        &mut self,
        call: &FlowCall,
        raw: &TypeofCallExpression,
        callee: TypeData<'db>,
    ) -> Option<EffectSignature<'db>> {
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
            return match self.instantiate_signature(call, raw, *signature, &arguments) {
                Instantiation::Bound(selected) => Some(selected),
                Instantiation::Rejected | Instantiation::Undetermined => None,
            };
        }
        // The signatures before the accepting one that TypeScript may select
        // for narrower argument types.
        let mut possible: Vec<EffectSignature<'db>> = Vec::new();
        for signature in signatures {
            if signature
                .parameters(db)
                .iter()
                .any(|parameter| parameter.is_this())
            {
                return None;
            }
            let selected = match self.instantiate_signature(call, raw, signature, &arguments) {
                Instantiation::Bound(selected) => selected,
                Instantiation::Rejected => continue,
                Instantiation::Undetermined => return None,
            };
            if signature_accepts_arguments(db, selected.function, &arguments.types) {
                // Which of the possible signatures TypeScript selects doesn't
                // matter when they all narrow alike.
                if !possible.is_empty() {
                    let effect = effect_of(db, selected.function);
                    if effect.is_none()
                        || possible
                            .iter()
                            .any(|other| effect_of(db, other.function) != effect)
                    {
                        return None;
                    }
                }
                let narrows_falsy =
                    selected.narrows_falsy && possible.iter().all(|other| other.narrows_falsy);
                return Some(EffectSignature {
                    function: selected.function,
                    narrows_falsy,
                });
            }
            if !rejects_every_narrowing(db, selected.function, &arguments.types) {
                possible.push(selected);
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
    ) -> Option<CallPointArguments<'db>> {
        let mut types = Vec::with_capacity(raw.arguments.len());
        let mut narrowed = Vec::with_capacity(raw.arguments.len());
        for (index, argument) in raw.arguments.iter().enumerate() {
            let CallArgumentType::Argument(reference) = argument else {
                return None;
            };
            let (argument, is_narrowed) = match call.arguments.get(index) {
                Some(argument) if argument.subject.is_some() => {
                    let ty = self.argument_type_at_call(argument.range, call.range)?;
                    let is_narrowed = self.raw_operand_at(argument.range) != Some(ty);
                    (ResolvedCallArgument::Argument(ty), is_narrowed)
                }
                _ => {
                    let ty = self.resolve(reference);
                    let argument = match self.literal_argument(reference, ty) {
                        Some(literal_ty) => {
                            ResolvedCallArgument::WidenedArgument { ty, literal_ty }
                        }
                        None => ResolvedCallArgument::Argument(ty),
                    };
                    (argument, false)
                }
            };
            types.push(argument);
            narrowed.push(is_narrowed);
        }
        Some(CallPointArguments { types, narrowed })
    }

    /// Returns the type of the argument at `range` at the call at `call`: its
    /// narrowed type where narrowing changes it, and its ordinary type
    /// otherwise.
    ///
    /// The narrowing applies only the call points that end before the call
    /// starts, together with the other tests and writes along the paths to
    /// the argument. A call point after it reaches the argument only through
    /// a loop, as does the call itself, whose effect needs the argument's
    /// type, so leaving those out keeps the selection free of cycles.
    ///
    /// The type can be wider than TypeScript's, in such a loop and wherever
    /// Biome lacks a narrowing that TypeScript applies, so the selection
    /// relies on it only as [`Self::select_effect_signature`] and
    /// [`Self::instantiate_signature`] describe.
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

    /// Returns the type at the call `call` of the object that its callee,
    /// whose collected type is `raw_callee`, reads its function from: the
    /// type of a subject read as an argument is, or the ordinary type of
    /// another object. Returns `None` for a callee that reads no member.
    fn receiver_at_call(
        &mut self,
        call: &FlowCall,
        raw_callee: &TypeReference,
    ) -> Option<TypeData<'db>> {
        match call.receiver {
            Some(receiver) => self.argument_type_at_call(receiver.range, call.range),
            None => self.callee_receiver(raw_callee),
        }
    }

    /// Infers the type arguments of `signature` from `arguments`, the
    /// arguments of the call point `call` whose collected call is `raw`, as
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
    /// The first two kinds widen literal types as TypeScript does (see
    /// [`Self::widen_type_argument`]).
    ///
    /// TypeScript infers from the argument's type at the call, which can be
    /// narrower than Biome's (see [`Self::argument_type_at_call`]). A type
    /// argument is TypeScript's when it comes from a callback's type
    /// predicate, or from an argument whose type is a single literal type,
    /// `null`, or `undefined`, which narrowing can't change. One inferred
    /// from the argument that the signature's type predicate or assertion
    /// narrows, when the target is the type parameter itself, as in
    /// `isNonNull<T>(v: T | null): v is T`, may be wider, but narrows the
    /// argument as TypeScript's does. Any other type argument may be wider
    /// than TypeScript's, so the call then narrows only where it is truthy
    /// ([`EffectSignature::narrows_falsy`]).
    ///
    /// Each inferred type must satisfy its type parameter's constraint (see
    /// [`satisfies_constraint`]), read with the type arguments of the object
    /// that the callee is read from, at the call (see
    /// [`Self::receiver_at_call`]). A type that may be wider than
    /// TypeScript's can violate a constraint that TypeScript's satisfies, so
    /// only a violation by TypeScript's type argument rejects the signature.
    /// The object's type can be wider than TypeScript's too, where Biome
    /// lacks a narrowing of the object, and a constraint that it satisfies,
    /// as `S extends T` in `xs.every(isString)`, can then fail in TypeScript,
    /// which selects another signature.
    ///
    /// Returns [`Instantiation::Undetermined`] when a parameter of any other
    /// kind refers to a type parameter, when a type parameter is inferred
    /// from two arguments with different types or from two arguments either
    /// of which may be wider than TypeScript's, or from none while a
    /// parameter or the target of the type predicate or assertion refers to
    /// it, when a constraint can't be checked, and when a type argument that
    /// may be wider than TypeScript's violates a constraint or satisfies one
    /// that names another such type argument. Returns
    /// [`Instantiation::Rejected`] when a type argument that is TypeScript's
    /// violates its constraint.
    fn instantiate_signature(
        &mut self,
        call: &FlowCall,
        raw: &TypeofCallExpression,
        signature: InternedFunction<'db>,
        arguments: &CallPointArguments<'db>,
    ) -> Instantiation<'db> {
        let db = self.db;
        let type_parameters = signature.type_parameters(db);
        if type_parameters.is_empty() {
            return Instantiation::Bound(EffectSignature {
                function: signature,
                narrows_falsy: true,
            });
        }
        let mut generics = Vec::with_capacity(type_parameters.len());
        for type_parameter in type_parameters {
            let Some(generic) = type_parameter.as_generic_reference(db) else {
                return Instantiation::Undetermined;
            };
            generics.push(generic);
        }
        let tested = tested_type_parameter(db, signature);
        let mut bindings: Vec<Binding<'db>> = Vec::new();
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
            let Some(argument) = arguments.types.get(index) else {
                continue;
            };
            let is_callback = matches!(shape, ParameterShape::Other);
            // An argument infers its declared type, which flattening reads
            // through annotations and `typeof`, as `"a"` for `lit` with
            // `declare const lit: "a"`.
            let argument_ty = if is_callback {
                argument.ty()
            } else {
                self.flow_baseline(argument.ty())
                    .unwrap_or_else(|| argument.ty())
            };
            let (generic, inferred) = match shape {
                ParameterShape::Plain => continue,
                ParameterShape::Other => {
                    // A callback's type predicate is never a literal type
                    // that TypeScript widens, and narrowing doesn't change
                    // the callback's signature.
                    match callback_predicate_binding(db, signature, ty, argument_ty) {
                        Some(binding) => binding,
                        None => return Instantiation::Undetermined,
                    }
                }
                ParameterShape::Bare(generic) => (generic, argument_ty),
                ParameterShape::Nullable(generic, nullish) => {
                    match without_members(db, argument_ty, &nullish) {
                        Some(inferred) => (generic, inferred),
                        None => return Instantiation::Undetermined,
                    }
                }
            };
            let binding = if is_callback {
                Binding {
                    generic,
                    inferred,
                    exact: true,
                }
            } else {
                let reference = match raw.arguments.get(index) {
                    Some(CallArgumentType::Argument(reference)) => Some(reference),
                    Some(CallArgumentType::Spread(_)) | None => None,
                };
                let narrowed = arguments.narrowed.get(index).copied().unwrap_or(true);
                let (inferred, is_widened_as_typescript) =
                    self.widen_type_argument(signature, generic, inferred, reference, narrowed);
                let is_typescript_argument =
                    tested == Some((index, generic)) || is_unit_type(db, argument_ty);
                Binding {
                    generic,
                    inferred,
                    exact: is_widened_as_typescript && is_typescript_argument,
                }
            };
            match bindings.iter().find(|bound| bound.generic == generic) {
                Some(bound)
                    if bound.inferred != binding.inferred || !bound.exact || !binding.exact =>
                {
                    return Instantiation::Undetermined;
                }
                Some(_) => {}
                None => bindings.push(binding),
            }
        }
        // TypeScript infers `unknown` for a type parameter that no argument
        // infers, which changes nothing that the call does when no parameter
        // refers to it and neither does the target of the signature's type
        // predicate or assertion.
        for generic in &generics {
            if !bindings.iter().any(|bound| bound.generic == *generic)
                && signature_refers_to(db, signature, *generic)
            {
                return Instantiation::Undetermined;
            }
        }
        let substitutions: Vec<TypeSubstitution<'db>> = bindings
            .iter()
            .flat_map(|bound| generic_substitutions(db, bound.generic, bound.inferred))
            .collect();
        for bound in &bindings {
            let mut receiver = || self.receiver_at_call(call, &raw.callee);
            let satisfies = satisfies_constraint(
                db,
                bound.generic,
                bound.inferred,
                &mut receiver,
                &substitutions,
            );
            match satisfies {
                Some(true) => {
                    // A constraint that names a type argument that may be
                    // wider than TypeScript's, as `T` in `S extends T`, can
                    // be narrower in TypeScript and reject the type argument.
                    if bound.generic.constraint(db).is_some_and(|constraint| {
                        bindings
                            .iter()
                            .filter(|other| !other.exact)
                            .any(|other| refers_to(db, constraint, other.generic))
                    }) {
                        return Instantiation::Undetermined;
                    }
                }
                Some(false) if bound.exact => return Instantiation::Rejected,
                Some(false) | None => return Instantiation::Undetermined,
            }
        }
        match apply_substitutions_to_root_body(db, TypeData::Function(signature), &substitutions)
            .callable_function(db)
        {
            Some(function) => Instantiation::Bound(EffectSignature {
                function,
                narrows_falsy: bindings.iter().all(|bound| bound.exact),
            }),
            None => Instantiation::Undetermined,
        }
    }

    /// Returns `inferred`, the type that an argument whose collected type is
    /// `reference` infers for `generic`, a type parameter of `signature`, with
    /// its literal types widened as TypeScript widens them, and whether Biome
    /// can tell that TypeScript widens them so.
    ///
    /// TypeScript keeps the literal types when `generic` is at the top level
    /// of the target of the type predicate or the assertion, also through a
    /// union or an intersection, as `K` in `v is K & string`, when it is a
    /// `const` type parameter, and when its constraint includes a primitive
    /// or literal type, as `K extends string` does. Otherwise it widens the
    /// fresh ones, those of literal expressions and of the `const` variables
    /// without a type annotation that the argument names, so `"a"` becomes
    /// `string`, and keeps the others, as that of a variable declared `"a"`.
    ///
    /// When Biome can't tell which of these applies, it widens every literal
    /// type and returns false: for a constraint or a target that it can't
    /// classify, for an argument whose narrowing changes its type
    /// (`narrowed`), because `y === "a"` gives `y: string` a fresh literal
    /// type, and for an argument whose literal types it can't trace, such as
    /// a call.
    fn widen_type_argument(
        &mut self,
        signature: InternedFunction<'db>,
        generic: InternedGenericTypeParameter<'db>,
        inferred: TypeData<'db>,
        reference: Option<&TypeReference>,
        narrowed: bool,
    ) -> (TypeData<'db>, bool) {
        let db = self.db;
        let widened = self.widen_all_literals(inferred, false);
        if widened == inferred {
            return (inferred, true);
        }
        let keeps = if generic.is_const(db) {
            Some(true)
        } else {
            match (
                is_at_top_level(db, signature, generic),
                has_primitive_constraint(db, generic),
            ) {
                (Some(true), _) | (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                (None, _) | (_, None) => None,
            }
        };
        match (keeps, reference) {
            (Some(true), _) => (inferred, true),
            (Some(false), Some(reference)) if !narrowed => {
                self.widen_known_fresh_literals(reference, inferred)
            }
            (Some(false) | None, _) => (widened, false),
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

/// What a signature's type predicate or assertion does to a call's operand.
#[derive(Eq, PartialEq)]
struct SignatureEffect<'db> {
    asserts: bool,
    /// The position of the argument that it names, not counting a `this`
    /// parameter, or `None` for `this`.
    position: Option<usize>,
    target: TypeData<'db>,
}

/// Returns what the type predicate or the assertion of `signature` does, or
/// `None` for a signature without one, or one that names no parameter.
fn effect_of<'db>(
    db: &'db dyn crate::ModuleDb,
    signature: InternedFunction<'db>,
) -> Option<SignatureEffect<'db>> {
    let (asserts, name, target) = match signature.return_type(db) {
        ReturnType::Predicate(predicate) => (false, &predicate.parameter_name, predicate.ty),
        ReturnType::Asserts(assertion) => (true, &assertion.parameter_name, assertion.ty),
        ReturnType::Type(_) => return None,
    };
    let position = if name.text() == "this" {
        None
    } else {
        Some(parameter_position(db, signature, name.text())?)
    };
    Some(SignatureEffect {
        asserts,
        position,
        target,
    })
}

/// Returns the position of the parameter of `signature` named `name`, not
/// counting a `this` parameter.
fn parameter_position<'db>(
    db: &'db dyn crate::ModuleDb,
    signature: InternedFunction<'db>,
    name: &str,
) -> Option<usize> {
    signature
        .parameters(db)
        .iter()
        .filter(|parameter| !parameter.is_this())
        .position(|parameter| {
            matches!(parameter, FunctionParameter::Named(named) if named.name.text() == name)
        })
}

/// Returns the position of the argument that the type predicate or the
/// assertion of `signature` narrows, not counting a `this` parameter, with
/// the type parameter that the predicate's target is, as `v` and `T` in
/// `isNonNull<T>(v: T | null): v is T`. Returns `None` when the target is
/// another type or the predicate names `this`.
fn tested_type_parameter<'db>(
    db: &'db dyn crate::ModuleDb,
    signature: InternedFunction<'db>,
) -> Option<(usize, InternedGenericTypeParameter<'db>)> {
    let (name, target) = match signature.return_type(db) {
        ReturnType::Predicate(predicate) => (&predicate.parameter_name, predicate.ty),
        ReturnType::Asserts(assertion) => (&assertion.parameter_name, assertion.ty),
        ReturnType::Type(_) => return None,
    };
    let generic = target.as_generic_reference(db)?;
    Some((parameter_position(db, signature, name.text())?, generic))
}

/// Returns whether narrowing can't change `ty`, the type of an argument: a
/// single literal type of a primitive, `null`, or `undefined`. Narrowing
/// such a type gives the type itself or no value.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only literal types, `null`, and `undefined` are unit types."
)]
fn is_unit_type<'db>(db: &'db dyn crate::ModuleDb, ty: TypeData<'db>) -> bool {
    match resolve_local_type_on_demand(db, ty) {
        TypeData::Null | TypeData::Undefined => true,
        TypeData::Literal(literal) => matches!(
            literal.literal(db),
            Literal::BigInt(_) | Literal::Boolean(_) | Literal::Number(_) | Literal::String(_)
        ),
        _ => false,
    }
}

/// Returns whether a parameter of `signature`, or the target of its type
/// predicate or assertion, refers to `generic`.
fn signature_refers_to<'db>(
    db: &'db dyn crate::ModuleDb,
    signature: InternedFunction<'db>,
    generic: InternedGenericTypeParameter<'db>,
) -> bool {
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
        .any(|ty| refers_to(db, ty, generic))
}

/// Returns whether `ty` refers to `generic`.
fn refers_to<'db>(
    db: &'db dyn crate::ModuleDb,
    ty: TypeData<'db>,
    generic: InternedGenericTypeParameter<'db>,
) -> bool {
    let substitutions = generic_substitutions(db, generic, TypeData::Unknown);
    apply_substitutions(db, ty, &substitutions) != ty
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
/// member of a union or an intersection at the top level, as `K` in
/// `v is (K & string) | null`. TypeScript keeps a literal type that it
/// infers for such a type parameter.
///
/// Returns `None` when the target has more than [`MAX_TOP_LEVEL_STEPS`]
/// such members, or a conditional type, whose branches TypeScript also
/// counts and Biome doesn't read here.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only unions and intersections have members at the top level."
)]
fn is_at_top_level<'db>(
    db: &'db dyn crate::ModuleDb,
    signature: InternedFunction<'db>,
    generic: InternedGenericTypeParameter<'db>,
) -> Option<bool> {
    let target = match signature.return_type(db) {
        ReturnType::Predicate(predicate) => predicate.ty,
        ReturnType::Asserts(assertion) => assertion.ty,
        ReturnType::Type(_) => return Some(false),
    };
    let mut pending = Vec::from([target]);
    for _ in 0..MAX_TOP_LEVEL_STEPS {
        let Some(ty) = pending.pop() else {
            return Some(false);
        };
        if ty.as_generic_reference(db) == Some(generic) {
            return Some(true);
        }
        match resolve_local_type_on_demand(db, ty) {
            TypeData::Union(union) => pending.extend(union.types(db).iter().copied()),
            TypeData::Intersection(intersection) => {
                pending.extend(intersection.types(db).iter().copied());
            }
            TypeData::Extends(_) => return None,
            resolved if resolved.as_generic_reference(db) == Some(generic) => return Some(true),
            _ => {}
        }
    }
    None
}

/// Returns whether the constraint of `generic` includes a primitive type, a
/// literal type, or a `keyof` type, also as a member of a union or an
/// intersection, which makes TypeScript keep the literal types that it
/// infers for `generic`. A type parameter in the constraint, as `T` in
/// `S extends T`, doesn't count, as in TypeScript.
///
/// Returns `None` when Biome can't tell, as for a constraint that it can't
/// resolve or that has more than [`MAX_TOP_LEVEL_STEPS`] members.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Any type that the listed arms don't classify leaves the answer unknown."
)]
fn has_primitive_constraint<'db>(
    db: &'db dyn crate::ModuleDb,
    generic: InternedGenericTypeParameter<'db>,
) -> Option<bool> {
    let Some(constraint) = generic.constraint(db) else {
        return Some(false);
    };
    let mut pending = Vec::from([constraint]);
    let mut is_known = true;
    for _ in 0..MAX_TOP_LEVEL_STEPS {
        let Some(ty) = pending.pop() else {
            return is_known.then_some(false);
        };
        if ty.as_generic_reference(db).is_some() {
            continue;
        }
        match resolve_local_type_on_demand(db, ty).expand_canonical_global(db) {
            TypeData::Union(union) => pending.extend(union.types(db).iter().copied()),
            TypeData::Intersection(intersection) => {
                pending.extend(intersection.types(db).iter().copied());
            }
            TypeData::BigInt
            | TypeData::Boolean
            | TypeData::Null
            | TypeData::Number
            | TypeData::String
            | TypeData::Symbol
            | TypeData::Undefined
            | TypeData::VoidKeyword => return Some(true),
            TypeData::Literal(literal)
                if !matches!(literal.literal(db), Literal::Object(_) | Literal::RegExp(_)) =>
            {
                return Some(true);
            }
            TypeData::TypeOperator(operator) if operator.operator(db) == TypeOperator::Keyof => {
                return Some(true);
            }
            resolved if resolved.as_generic_reference(db).is_some() => {}
            TypeData::AnyKeyword
            | TypeData::UnknownKeyword
            | TypeData::NeverKeyword
            | TypeData::ObjectKeyword
            | TypeData::Object(_)
            | TypeData::Interface(_)
            | TypeData::Class(_)
            | TypeData::Function(_)
            | TypeData::Tuple(_)
            | TypeData::InstanceOf(_)
            | TypeData::Literal(_) => {}
            _ => is_known = false,
        }
    }
    None
}
