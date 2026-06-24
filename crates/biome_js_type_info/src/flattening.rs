use std::borrow::Cow;
use std::cell::{Cell, RefCell};

use crate::{
    Resolvable, ResolvedTypeId, TypeData, TypeInstance, TypeReference, TypeReferenceQualifier,
    TypeResolver,
};

mod expressions;
mod intersections;
mod unions;

use expressions::flattened_expression;
use intersections::flattened_intersection;

pub const MAX_FLATTEN_DEPTH: usize = 10; // Arbitrary depth, may require tweaking.

/// Depth cap for substituting alias parameters into referenced type data. Past it, a leftover
/// type parameter stays unsubstituted, which the consumer treats as indeterminate, never as wrong.
const MAX_INSTANTIATION_DEPTH: usize = 8;

/// Depth cap for materializing a chain of generic-alias applications (`type W1<T> = W0<T>; ...`).
/// Each level recurses through [`instantiated_generic_alias`], so an unbounded chain would overflow
/// the stack. Past the cap the alias is left opaque, which the consumer treats as indeterminate.
pub const MAX_ALIAS_CHAIN_DEPTH: usize = 32;

impl TypeData {
    /// Flattens the given type.
    ///
    /// Flattening is both an optimisation as well as an aid to make our
    /// reasoning about types easier. It removes unnecessary indirections from
    /// our type structures, and should be performed every time after we perform
    /// type resolution.
    ///
    /// ## Example
    ///
    /// Consider the following example:
    ///
    /// ```ts
    /// const c = 1;
    ///
    /// type A = typeof c;
    /// ```
    ///
    /// After local inference, the inferred type of `A` is:
    ///
    /// ```no_test
    /// TypeData::TypeofValue {
    ///     identifier: "c",
    ///     ty: TypeReference::Unknown
    /// }
    /// ```
    ///
    /// Once we've performed thin type resolution, this becomes:
    ///
    /// ```no_test
    /// TypeData::TypeofValue {
    ///     identifier: "c",
    ///     ty: TypeReference::Resolved(<type ID of literal>)
    /// }
    /// ```
    ///
    /// With flattening, we can reduce this to:
    ///
    /// ```no_test
    /// TypeData::Literal(Literal::Number(1)))
    /// ```
    pub fn flattened(&self, resolver: &mut dyn TypeResolver) -> Option<Self> {
        match self {
            Self::MergedReference(merged) => {
                match (&merged.ty, &merged.value_ty, &merged.namespace_ty) {
                    (Some(ty1), Some(ty2), Some(ty3)) if ty1 == ty2 && ty1 == ty3 => {
                        Some(Self::Reference(ty1.clone()))
                    }
                    (Some(ty1), Some(ty2), None)
                    | (Some(ty1), None, Some(ty2))
                    | (None, Some(ty1), Some(ty2))
                        if ty1 == ty2 =>
                    {
                        Some(Self::Reference(ty1.clone()))
                    }
                    _ => None,
                }
            }
            Self::InstanceOf(instance) => flattened_instance(instance, &instance.ty, resolver),
            Self::Intersection(intersection) => {
                Some(flattened_intersection(intersection, resolver))
            }
            Self::Reference(reference) => match resolver.resolve_and_get(reference) {
                Some(reference) => match reference.as_raw_data() {
                    Self::InstanceOf(instance_of) => Some(
                        reference.apply_module_id_to_data(Self::InstanceOf(instance_of.clone())),
                    ),
                    Self::Reference(target) => Some(Self::Reference(
                        reference.apply_module_id_to_reference(target).into_owned(),
                    )),
                    Self::Unknown => Some(Self::unknown()),
                    _ => None,
                },
                None => None,
            },
            Self::TypeofExpression(expr) => flattened_expression(expr, resolver, 0),
            Self::TypeofType(reference) => resolver
                .resolve_reference(reference.as_ref())
                .map(Self::reference),
            Self::TypeofValue(value) if value.ty.is_known() => {
                resolver.resolve_reference(&value.ty).map(Self::reference)
            }
            _ => None,
        }
    }
}

fn flattened_instance(
    instance: &TypeInstance,
    reference: &TypeReference,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    let resolved = resolver.resolve_and_get(reference)?;
    match resolved.as_raw_data() {
        TypeData::InstanceOf(resolved_instance) => {
            // Arguments come from the instance, or from the qualifier itself for an unresolved
            // qualifier like `Maybe<string>`.
            let arguments: &[TypeReference] = if !instance.type_parameters.is_empty() {
                &instance.type_parameters
            } else if let TypeReference::Qualifier(qualifier) = reference {
                &qualifier.type_parameters
            } else {
                &[]
            };
            let body_ref = resolved
                .apply_module_id_to_reference(&resolved_instance.ty)
                .into_owned();
            // Own the declared parameters (with the alias's module id applied) before the
            // `resolved` borrow ends.
            let declared: Vec<TypeReference> = resolved_instance
                .type_parameters
                .iter()
                .map(|param| resolved.apply_module_id_to_reference(param).into_owned())
                .collect();

            if !resolver.should_instantiate_generic_qualifiers()
                && !instance.type_parameters.is_empty()
                && declared.is_empty()
            {
                return None;
            }

            // An applied alias expands its body (`Maybe<string>` -> `string | null`).
            // A bare alias stays wrapped in `InstanceOf`; defaults are not applied.
            if !arguments.is_empty() && !declared.is_empty() {
                if !resolver.should_instantiate_generic_qualifiers() {
                    return None;
                }

                if let Some(instantiated) =
                    instantiated_generic_alias(arguments, &declared, &body_ref, resolver)
                {
                    return Some(instantiated);
                }
            }

            Some(TypeData::instance_of(TypeInstance {
                ty: body_ref,
                type_parameters: declared
                    .iter()
                    .enumerate()
                    .map(|(i, param)| {
                        instance
                            .type_parameters
                            .get(i)
                            .cloned()
                            .unwrap_or_else(|| param.clone())
                    })
                    .collect(),
            }))
        }
        TypeData::Reference(reference) if reference.is_known() => {
            Some(TypeData::instance_of(TypeInstance {
                ty: resolved
                    .apply_module_id_to_reference(reference)
                    .into_owned(),
                type_parameters: instance.type_parameters.clone(),
            }))
        }
        TypeData::Reference(_) => Some(TypeData::unknown()),
        _ => resolved
            .should_flatten_instance(instance)
            .then(|| resolved.to_data()),
    }
}

/// Substitutes `declared` parameters with `arguments` throughout `body_ref`. Excess arguments
/// abort; a missing trailing one falls back to its default (`type Pair<A, B = A>`), else aborts.
pub fn instantiated_generic_alias(
    arguments: &[TypeReference],
    declared: &[TypeReference],
    body_ref: &TypeReference,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    // Reject extra arguments outright; never silently drop them.
    if arguments.len() > declared.len() {
        return None;
    }

    // A missing trailing argument falls back to its default, resolved against the params already
    // bound to its left; a parameter with no default aborts.
    let mut substitutions: Vec<(ResolvedTypeId, TypeReference)> =
        Vec::with_capacity(declared.len());
    for (index, param) in declared.iter().enumerate() {
        let argument = match arguments.get(index) {
            Some(argument) => argument.clone(),
            None => {
                let default = parameter_default(param, resolver)?;
                substituted_reference(&default, &substitutions, resolver, 0)
            }
        };
        if let Some(param_id) = resolved_type_parameter(param, resolver) {
            substitutions.push((param_id, argument));
        }
    }
    if substitutions.is_empty() {
        return None;
    }

    // `type Id<T> = T`: body is itself a declared parameter, short-circuit
    // to the argument without re-resolving.
    if let Some(parameter_id) = resolved_type_parameter(body_ref, resolver)
        && let Some((_, argument)) = substitutions
            .iter()
            .find(|(declared, _)| *declared == parameter_id)
    {
        return Some(TypeData::reference(argument.clone()));
    }

    let body = resolver.resolve_and_get(body_ref)?.to_data();
    let substitutions = substitutions_visible_in(&body, &substitutions, resolver);
    if substitutions.is_empty() {
        return Some(body);
    }
    let instantiated =
        substituted_type_data(body.clone(), &substitutions, resolver, 0).unwrap_or(body);
    Some(canonicalize_instantiated_alias(instantiated, resolver))
}

/// Re-runs union absorption so a substituted `Maybe<any>` collapses to `any`, not `any | null`.
/// A union body is canonicalized directly; otherwise each reference one level deep is.
fn canonicalize_instantiated_alias(
    instantiated: TypeData,
    resolver: &mut dyn TypeResolver,
) -> TypeData {
    if let TypeData::Union(union) = instantiated {
        return TypeData::union_of(resolver, union.into_types().into_boxed_slice());
    }

    // Canonicalize each direct reference that resolves to a union, then write them back.
    // One level only, no recursion.
    let mut data = instantiated;
    let originals: Vec<TypeReference> = {
        let collected = RefCell::new(Vec::new());
        data.update_all_references(|reference| collected.borrow_mut().push(reference.clone()));
        collected.into_inner()
    };

    let mut rewritten: Option<Vec<TypeReference>> = None;
    for (index, original) in originals.iter().enumerate() {
        let canonical = canonicalized_union_reference(original, resolver);
        if let Some(buffer) = rewritten.as_mut() {
            buffer.push(canonical);
        } else if canonical != *original {
            let mut buffer = Vec::with_capacity(originals.len());
            buffer.extend(originals[..index].iter().cloned());
            buffer.push(canonical);
            rewritten = Some(buffer);
        }
    }
    let Some(rewritten) = rewritten else {
        return data;
    };

    let index = Cell::new(0usize);
    data.update_all_references(|reference| {
        let position = index.get();
        index.set(position + 1);
        *reference = rewritten[position].clone();
    });
    debug_assert_eq!(
        index.get(),
        rewritten.len(),
        "collect and write-back passes must visit the same references",
    );
    data
}

/// Re-registers `reference` with a canonicalized union when it resolves to
/// a [`TypeData::Union`]; otherwise returns it unchanged.
fn canonicalized_union_reference(
    reference: &TypeReference,
    resolver: &mut dyn TypeResolver,
) -> TypeReference {
    let Some(TypeData::Union(union)) = resolver
        .resolve_and_get(reference)
        .map(|resolved| resolved.to_data())
    else {
        return reference.clone();
    };
    let canonical = TypeData::union_of(resolver, union.into_types().into_boxed_slice());
    TypeReference::Resolved(resolver.register_and_resolve(canonical))
}

/// The unit of work fed to [`run_substitution`]: a whole type body, or a single reference.
enum SubstitutionTask {
    Data(TypeData),
    Reference(TypeReference),
}

enum SubstitutionOutcome {
    Data(Option<TypeData>),
    Reference(TypeReference),
}

/// What a finished [`SubstitutionFrame`] produces once all of its references are substituted.
enum SubstitutionFinalize {
    /// The entry body: write the substitutions back if any changed, otherwise `None`.
    RootData(TypeData),
    /// A resolved reference's body: write back and intern if any changed, else the original reference.
    NestedData {
        original: TypeReference,
        data: TypeData,
    },
    /// A qualifier's arguments: rebuild the qualifier if any changed, else the original reference.
    Qualifier {
        original: TypeReference,
        qualifier: Box<TypeReferenceQualifier>,
    },
}

/// References being substituted at a fixed depth, with the substitutions visible to them.
struct SubstitutionFrame {
    references: Box<[TypeReference]>,
    out: Vec<TypeReference>,
    changed: bool,
    depth: usize,
    substitutions: Vec<(ResolvedTypeId, TypeReference)>,
    finalize: SubstitutionFinalize,
}

enum SubstitutionStep {
    Done(TypeReference),
    Descend(SubstitutionFrame),
}

fn substituted_type_data(
    data: TypeData,
    substitutions: &[(ResolvedTypeId, TypeReference)],
    resolver: &mut dyn TypeResolver,
    depth: usize,
) -> Option<TypeData> {
    match run_substitution(SubstitutionTask::Data(data), substitutions, resolver, depth) {
        SubstitutionOutcome::Data(data) => data,
        SubstitutionOutcome::Reference(_) => unreachable!("a data task yields a data outcome"),
    }
}

/// Substitutes type parameters within a single reference. Depth is bounded
/// by [`MAX_INSTANTIATION_DEPTH`].
fn substituted_reference(
    reference: &TypeReference,
    substitutions: &[(ResolvedTypeId, TypeReference)],
    resolver: &mut dyn TypeResolver,
    depth: usize,
) -> TypeReference {
    match run_substitution(
        SubstitutionTask::Reference(reference.clone()),
        substitutions,
        resolver,
        depth,
    ) {
        SubstitutionOutcome::Reference(reference) => reference,
        SubstitutionOutcome::Data(_) => unreachable!("a reference task yields a reference outcome"),
    }
}

/// Iterative (explicit-frame) substitution of alias parameters, bounded by
/// [`MAX_INSTANTIATION_DEPTH`]. Frames run depth-first so references are interned in a stable
/// order, keeping type ids deterministic.
fn run_substitution(
    task: SubstitutionTask,
    substitutions: &[(ResolvedTypeId, TypeReference)],
    resolver: &mut dyn TypeResolver,
    depth: usize,
) -> SubstitutionOutcome {
    // Collects the references a type holds, in traversal order. `update_all_references` takes `Fn`,
    // so the buffer is shared through a `RefCell`.
    fn collect_references(data: &mut TypeData) -> Box<[TypeReference]> {
        let collected = RefCell::new(Vec::new());
        data.update_all_references(|reference| collected.borrow_mut().push(reference.clone()));
        collected.into_inner().into_boxed_slice()
    }

    // Writes the substituted references back into `data`, in the same order they were collected
    // (`data` is unchanged between the two traversals).
    fn write_back(mut data: TypeData, out: &[TypeReference]) -> TypeData {
        let index = Cell::new(0usize);
        data.update_all_references(|reference| {
            let position = index.get();
            index.set(position + 1);
            *reference = out[position].clone();
        });
        data
    }

    // Substitutes a single reference: a finished reference, or a frame to descend into.
    fn step(
        reference: &TypeReference,
        substitutions: &[(ResolvedTypeId, TypeReference)],
        resolver: &mut dyn TypeResolver,
        depth: usize,
    ) -> SubstitutionStep {
        if let Some(parameter_id) = resolved_type_parameter(reference, resolver)
            && let Some((_, argument)) = substitutions
                .iter()
                .find(|(declared, _)| *declared == parameter_id)
        {
            return SubstitutionStep::Done(argument.clone());
        }

        if depth >= MAX_INSTANTIATION_DEPTH {
            return SubstitutionStep::Done(reference.clone());
        }

        // Nested generic application (`type Wrap<T> = Maybe<T>`): substitute the qualifier's args.
        if let TypeReference::Qualifier(qualifier) = reference
            && !qualifier.type_parameters.is_empty()
        {
            return SubstitutionStep::Descend(SubstitutionFrame {
                out: Vec::with_capacity(qualifier.type_parameters.len()),
                references: qualifier.type_parameters.clone(),
                changed: false,
                depth: depth + 1,
                substitutions: substitutions.to_vec(),
                finalize: SubstitutionFinalize::Qualifier {
                    original: reference.clone(),
                    qualifier: qualifier.clone(),
                },
            });
        }

        let Some(mut data) = resolver
            .resolve_and_get(reference)
            .map(|data| data.to_data())
        else {
            return SubstitutionStep::Done(reference.clone());
        };
        let visible = substitutions_visible_in(&data, substitutions, resolver);
        if visible.is_empty() {
            return SubstitutionStep::Done(reference.clone());
        }
        let references = collect_references(&mut data);
        SubstitutionStep::Descend(SubstitutionFrame {
            out: Vec::with_capacity(references.len()),
            references,
            changed: false,
            depth: depth + 1,
            substitutions: visible.into_owned(),
            finalize: SubstitutionFinalize::NestedData {
                original: reference.clone(),
                data,
            },
        })
    }

    // Finishes a frame: the resulting reference, or `RootData`'s `Option<TypeData>`.
    fn finalize(frame: SubstitutionFrame, resolver: &mut dyn TypeResolver) -> SubstitutionOutcome {
        match frame.finalize {
            SubstitutionFinalize::RootData(data) => {
                SubstitutionOutcome::Data(frame.changed.then(|| write_back(data, &frame.out)))
            }
            SubstitutionFinalize::NestedData { original, data } => {
                SubstitutionOutcome::Reference(if frame.changed {
                    let substituted = write_back(data, &frame.out);
                    TypeReference::Resolved(resolver.register_and_resolve(substituted))
                } else {
                    original
                })
            }
            SubstitutionFinalize::Qualifier {
                original,
                qualifier,
            } => SubstitutionOutcome::Reference(if frame.changed {
                TypeReference::Qualifier(Box::new(
                    qualifier.with_type_parameters(frame.out.into_boxed_slice()),
                ))
            } else {
                original
            }),
        }
    }

    let mut stack: Vec<SubstitutionFrame> = Vec::new();
    match task {
        SubstitutionTask::Data(mut data) => {
            let references = collect_references(&mut data);
            stack.push(SubstitutionFrame {
                out: Vec::with_capacity(references.len()),
                references,
                changed: false,
                depth,
                substitutions: substitutions.to_vec(),
                finalize: SubstitutionFinalize::RootData(data),
            });
        }
        SubstitutionTask::Reference(reference) => {
            match step(&reference, substitutions, resolver, depth) {
                SubstitutionStep::Done(result) => return SubstitutionOutcome::Reference(result),
                SubstitutionStep::Descend(frame) => stack.push(frame),
            }
        }
    }

    // The reference a just-finished child frame produced, waiting to be slotted into its parent.
    let mut pending: Option<TypeReference> = None;
    loop {
        if let Some(reference) = pending.take() {
            let frame = stack.last_mut().expect("parent frame");
            frame.changed |= reference != frame.references[frame.out.len()];
            frame.out.push(reference);
        }

        let complete = {
            let frame = stack.last().expect("active frame");
            frame.out.len() >= frame.references.len()
        };
        if complete {
            let outcome = finalize(stack.pop().expect("active frame"), resolver);
            if stack.is_empty() {
                return outcome;
            }
            match outcome {
                SubstitutionOutcome::Reference(reference) => pending = Some(reference),
                SubstitutionOutcome::Data(_) => {
                    unreachable!("only the bottom frame finalizes to data")
                }
            }
            continue;
        }

        let outcome = {
            let frame = stack.last().expect("active frame");
            step(
                &frame.references[frame.out.len()],
                &frame.substitutions,
                resolver,
                frame.depth,
            )
        };
        match outcome {
            SubstitutionStep::Done(result) => {
                let frame = stack.last_mut().expect("active frame");
                frame.changed |= result != frame.references[frame.out.len()];
                frame.out.push(result);
            }
            SubstitutionStep::Descend(frame) => stack.push(frame),
        }
    }
}

/// Drops substitutions whose parameter is shadowed by `data`'s own declared parameters
/// (`<T>() => T` inside an alias body re-binds `T`); returns the input unchanged otherwise.
fn substitutions_visible_in<'a>(
    data: &TypeData,
    substitutions: &'a [(ResolvedTypeId, TypeReference)],
    resolver: &dyn TypeResolver,
) -> Cow<'a, [(ResolvedTypeId, TypeReference)]> {
    // Only binders shadow. `InstanceOf` is excluded: its `type_parameters` are arguments, not a
    // re-binding, and we lack the binder-unique ids to tell a declaration from an application.
    let parameters = match data {
        TypeData::Class(class) => &class.type_parameters,
        TypeData::Constructor(constructor) => &constructor.type_parameters,
        TypeData::Function(function) => &function.type_parameters,
        TypeData::Interface(interface) => &interface.type_parameters,
        _ => return Cow::Borrowed(substitutions),
    };
    let shadowed: Vec<ResolvedTypeId> = parameters
        .iter()
        .filter_map(|parameter| resolved_type_parameter(parameter, resolver))
        .collect();
    if shadowed.is_empty() {
        return Cow::Borrowed(substitutions);
    }
    // Build the filtered list only when something is actually dropped.
    let mut filtered: Option<Vec<(ResolvedTypeId, TypeReference)>> = None;
    for (index, substitution) in substitutions.iter().enumerate() {
        let is_shadowed = shadowed.contains(&substitution.0);
        if is_shadowed && filtered.is_none() {
            filtered = Some(substitutions[..index].to_vec());
        } else if !is_shadowed && let Some(buffer) = filtered.as_mut() {
            buffer.push(substitution.clone());
        }
    }
    match filtered {
        Some(buffer) => Cow::Owned(buffer),
        None => Cow::Borrowed(substitutions),
    }
}

/// Returns the [`ResolvedTypeId`] of the [`TypeData::Generic`] this reference denotes, unwrapping
/// the `InstanceOf` layer added around parameter references. Bounded by [`MAX_FLATTEN_DEPTH`].
fn resolved_type_parameter(
    reference: &TypeReference,
    resolver: &dyn TypeResolver,
) -> Option<ResolvedTypeId> {
    let mut resolved_id = resolver.resolve_reference(reference)?;
    for _ in 0..MAX_FLATTEN_DEPTH {
        let resolved = resolver.get_by_resolved_id(resolved_id)?;
        match resolved.as_raw_data() {
            TypeData::Generic(_) => return Some(resolved_id),
            TypeData::Reference(reference) => {
                let inner = resolved.apply_module_id_to_reference(reference);
                resolved_id = resolver.resolve_reference(inner.as_ref())?;
            }
            TypeData::InstanceOf(instance) if instance.type_parameters.is_empty() => {
                let inner = resolved.apply_module_id_to_reference(&instance.ty);
                resolved_id = resolver.resolve_reference(inner.as_ref())?;
            }
            _ => return None,
        }
    }
    None
}

/// Returns the declared default of a generic type parameter (`type Maybe<T = string>`), or
/// `None` when the reference is not a generic parameter or has no known default.
fn parameter_default(
    parameter: &TypeReference,
    resolver: &dyn TypeResolver,
) -> Option<TypeReference> {
    let resolved_id = resolved_type_parameter(parameter, resolver)?;
    let resolved = resolver.get_by_resolved_id(resolved_id)?;
    let TypeData::Generic(generic) = resolved.as_raw_data() else {
        return None;
    };
    generic.default.is_known().then(|| {
        resolved
            .apply_module_id_to_reference(&generic.default)
            .into_owned()
    })
}
