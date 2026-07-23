//! Relations between a declared return type and inferred return values.
//!
//! This module returns structured relation verdicts and candidate type handles.
//! It deliberately contains no diagnostic policy or rendering.
use crate::TypeDb;
use crate::interned_types::{Literal, TypeData, TypeMemberKind};
use rustc_hash::FxHashSet;

const MAX_RETURN_TYPE_STEPS: usize = 50;

/// Returns whether `ty` prevents a reliable or useful return-type comparison.
///
/// This includes explicit opt-out annotations, non-returning annotations,
/// unresolved inference, and the contextual `this` type.
pub(crate) fn is_escape_hatch(ty: TypeData<'_>) -> bool {
    matches!(
        ty,
        TypeData::AnyKeyword
            | TypeData::VoidKeyword
            | TypeData::UnknownKeyword
            | TypeData::NeverKeyword
            | TypeData::Unknown
            | TypeData::ThisKeyword
    )
}

/// Replaces boolean literal returns with one `boolean` when both values are
/// possible or `boolean` is already present.
fn normalize_boolean_return_types<'db>(
    db: &'db dyn TypeDb,
    mut types: Vec<TypeData<'db>>,
) -> Vec<TypeData<'db>> {
    let has_boolean = types.contains(&TypeData::Boolean);
    let has_true = types.iter().any(|ty| ty.is_boolean_literal(db, true));
    let has_false = types.iter().any(|ty| ty.is_boolean_literal(db, false));
    if !(has_boolean || has_true && has_false) {
        return types;
    }

    let mut seen_boolean = false;
    types.retain(|ty| {
        if matches!(ty, TypeData::Boolean) {
            if seen_boolean {
                return false;
            }
            seen_boolean = true;
            return true;
        }
        !ty.is_boolean_literal(db, true) && !ty.is_boolean_literal(db, false)
    });
    if !seen_boolean {
        types.push(TypeData::Boolean);
    }
    types
}

/// Returns the first type argument of a direct `Promise` instance.
///
/// Other thenables, unions containing promises, and unparameterized promises
/// return `None`.
pub(crate) fn promise_inner<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Option<TypeData<'db>> {
    let TypeData::InstanceOf(instance) = ty else {
        return None;
    };
    if !instance.ty(db).is_promise_class(db) {
        return None;
    }
    instance.type_parameters(db).first().copied()
}

/// Outcome of attempting to collapse a union around an absorbing primitive.
enum PrimitiveUnionCollapse<'db> {
    /// The primitive covers every union variant and replaces the union.
    Collapsed(TypeData<'db>),
    /// The type is not a collapsible union.
    NotApplicable,
    /// At least one type relation could not be determined reliably.
    Indeterminate,
}

/// Collapses a union to its primitive variant when that variant covers every
/// other variant.
fn collapse_union_absorbed_by_primitive<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
) -> PrimitiveUnionCollapse<'db> {
    let TypeData::Union(_) = ty else {
        return PrimitiveUnionCollapse::NotApplicable;
    };
    let mut primitive = None;
    for variant in ty.union_iterator(db) {
        if matches!(
            variant,
            TypeData::String | TypeData::Number | TypeData::Boolean | TypeData::BigInt
        ) {
            if primitive.is_some() {
                return PrimitiveUnionCollapse::NotApplicable;
            }
            primitive = Some(variant);
        }
    }
    let Some(primitive) = primitive else {
        return PrimitiveUnionCollapse::NotApplicable;
    };
    match try_all(ty.union_iterator(db).map(|variant| {
        relation_or(types_match(db, variant, primitive), || {
            is_nonunion_wider(db, primitive, variant)
        })
    })) {
        Some(true) => PrimitiveUnionCollapse::Collapsed(primitive),
        Some(false) => PrimitiveUnionCollapse::NotApplicable,
        None => PrimitiveUnionCollapse::Indeterminate,
    }
}

fn is_intersection_with_type_param(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    matches!(ty, TypeData::Intersection(intersection) if intersection.types(db).iter().any(|ty| matches!(ty, TypeData::Generic(_))))
}

fn is_base_type_of_literal(db: &dyn TypeDb, base: TypeData<'_>, literal: TypeData<'_>) -> bool {
    matches!(literal.literal_base_type(db), Some(literal_base) if literal_base == base)
}

/// Returns whether every inferred return differs from the annotation only by
/// widening property or tuple-element literals to their primitive types.
///
/// `None` means a nested comparison was indeterminate or exceeded the relation
/// work budget.
fn is_only_property_literal_widening(
    db: &dyn TypeDb,
    annotation: TypeData<'_>,
    returns: &[TypeData<'_>],
) -> Option<bool> {
    // This limit counts compared structural pairs, including repeated pairs.
    // It bounds total relation work rather than distinct type traversal.
    for inferred in returns {
        let mut stack = vec![(annotation, *inferred)];
        let mut has_widening = false;
        let mut iterations = 0;

        while let Some((annotated, inferred)) = stack.pop() {
            iterations += 1;
            if iterations > MAX_RETURN_TYPE_STEPS {
                return None;
            }

            if let TypeData::Tuple(annotated_tuple) = annotated {
                let TypeData::Tuple(inferred_tuple) = inferred else {
                    return Some(false);
                };
                let annotated_elements = annotated_tuple.elements(db);
                let inferred_elements = inferred_tuple.elements(db);
                if annotated_elements.len() != inferred_elements.len()
                    || annotated_elements.is_empty()
                {
                    return Some(false);
                }
                for (annotated_element, inferred_element) in
                    annotated_elements.iter().zip(inferred_elements)
                {
                    match types_match(db, annotated_element.ty, inferred_element.ty) {
                        Some(true) => continue,
                        None => return None,
                        Some(false) => {}
                    }
                    if is_base_type_of_literal(db, annotated_element.ty, inferred_element.ty) {
                        has_widening = true;
                    } else {
                        stack.push((annotated_element.ty, inferred_element.ty));
                    }
                }
                continue;
            }

            let TypeData::Object(annotated_object) = annotated else {
                return Some(false);
            };
            let annotated_members = annotated_object.members(db);
            if annotated_members.is_empty() {
                return Some(false);
            }
            let inferred_members = match inferred {
                TypeData::Object(object) => object.members(db),
                TypeData::Literal(literal) => match literal.literal(db) {
                    Literal::Object(members) => members,
                    _ => return Some(false),
                },
                _ => return Some(false),
            };
            if inferred_members.is_empty() {
                return Some(false);
            }

            if let Some(index_signature) = annotated_members
                .iter()
                .find_map(|member| member.kind.index_signature_type().map(|_| member))
            {
                let mut index_has_widening = false;
                for member in inferred_members {
                    if member.kind.is_const_asserted() {
                        return Some(false);
                    }
                    match types_match(db, index_signature.ty, member.ty) {
                        Some(true) => continue,
                        None => return None,
                        Some(false) => {}
                    }
                    if is_base_type_of_literal(db, index_signature.ty, member.ty) {
                        index_has_widening = true;
                        continue;
                    }
                    return Some(false);
                }
                if !index_has_widening {
                    return Some(false);
                }
                has_widening = true;
                continue;
            }

            for annotated_member in annotated_members {
                let Some(name) = annotated_member.kind.name() else {
                    continue;
                };
                let Some(inferred_member) = inferred_members
                    .iter()
                    .find(|member| member.kind.has_name(name.text()))
                else {
                    return Some(false);
                };
                if inferred_member.kind.is_const_asserted() {
                    return Some(false);
                }
                match types_match(db, annotated_member.ty, inferred_member.ty) {
                    Some(true) => continue,
                    None => return None,
                    Some(false) => {}
                }
                if is_base_type_of_literal(db, annotated_member.ty, inferred_member.ty) {
                    has_widening = true;
                } else {
                    stack.push((annotated_member.ty, inferred_member.ty));
                }
            }
        }

        if !has_widening {
            return Some(false);
        }
    }
    Some(true)
}

/// Follows generic constraints to the first non-generic type.
///
/// An unconstrained generic is returned unchanged. Cyclic constraints return
/// `None`.
fn resolve_generic_chain<'db>(db: &'db dyn TypeDb, mut ty: TypeData<'db>) -> Option<TypeData<'db>> {
    let mut seen = FxHashSet::default();
    loop {
        let TypeData::Generic(generic) = ty else {
            return Some(ty);
        };
        if !seen.insert(generic) {
            return None;
        }
        let Some(constraint) = generic.constraint(db) else {
            return Some(ty);
        };
        ty = constraint;
    }
}

/// Returns whether a non-union annotation strictly widens an inferred type.
///
/// Exact matches return `Some(false)`. Cyclic generic constraints and
/// structural comparisons that exceed the work budget return `None`.
fn is_nonunion_wider<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
) -> Option<bool> {
    // This limit counts compared structural pairs, including repeated pairs.
    // It bounds total relation work rather than distinct type traversal.
    let mut stack = vec![(annotated, inferred)];
    let mut found_wider = false;
    let mut iterations = 0;

    while let Some((annotated, inferred)) = stack.pop() {
        iterations += 1;
        if iterations > MAX_RETURN_TYPE_STEPS {
            return None;
        }
        let inferred = resolve_generic_chain(db, inferred)?;
        if is_base_type_of_literal(db, annotated, inferred) {
            found_wider = true;
            continue;
        }
        match types_match(db, annotated, inferred) {
            Some(true) => continue,
            None => return None,
            Some(false) => {}
        }

        if matches!(annotated, TypeData::ObjectKeyword) {
            if matches!(inferred, TypeData::InstanceOf(_))
                || inferred.is_strictly_narrower_than_object_keyword(db)?
            {
                found_wider = true;
                continue;
            }
            return Some(false);
        }
        if let (TypeData::InstanceOf(annotated), TypeData::InstanceOf(inferred)) =
            (annotated, inferred)
        {
            match types_match(db, annotated.ty(db), inferred.ty(db)) {
                Some(true) => {}
                Some(false) => return Some(false),
                None => return None,
            }
            let annotated_parameters = annotated.type_parameters(db);
            let inferred_parameters = inferred.type_parameters(db);
            if annotated_parameters.len() != inferred_parameters.len()
                || annotated_parameters.is_empty()
            {
                return Some(false);
            }
            stack.extend(
                annotated_parameters
                    .iter()
                    .zip(inferred_parameters)
                    .map(|(annotated, inferred)| (*annotated, *inferred)),
            );
            continue;
        }
        if matches!(annotated, TypeData::Object(_))
            && matches!(inferred, TypeData::Object(_) | TypeData::Literal(_))
        {
            if !push_object_pairs(db, annotated, inferred, &mut stack) {
                return Some(false);
            }
            continue;
        }
        if let (TypeData::Tuple(annotated), TypeData::Tuple(inferred)) = (annotated, inferred) {
            let annotated_elements = annotated.elements(db);
            let inferred_elements = inferred.elements(db);
            if annotated_elements.len() != inferred_elements.len() || annotated_elements.is_empty()
            {
                return Some(false);
            }
            stack.extend(
                annotated_elements
                    .iter()
                    .zip(inferred_elements)
                    .map(|(annotated, inferred)| (annotated.ty, inferred.ty)),
            );
            continue;
        }
        return Some(false);
    }

    Some(found_wider)
}

/// Adds corresponding object member types to `stack` for structural
/// comparison.
///
/// Returns `false` when either side has no comparable members or an annotated
/// named member is absent from the inferred object.
fn push_object_pairs<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
    stack: &mut Vec<(TypeData<'db>, TypeData<'db>)>,
) -> bool {
    let Some(annotated_members) = annotated.as_type_members(db) else {
        return false;
    };
    let Some(inferred_members) = inferred.as_type_members(db) else {
        return false;
    };
    if annotated_members.is_empty() || inferred_members.is_empty() {
        return false;
    }

    if let Some(index_signature) = annotated_members.iter().find(|member| {
        matches!(
            member.kind,
            TypeMemberKind::IndexSignature(_) | TypeMemberKind::ConstAssertedIndexSignature(_)
        )
    }) {
        stack.extend(
            inferred_members
                .iter()
                .map(|member| (index_signature.ty, member.ty)),
        );
        return true;
    }

    for annotated_member in annotated_members {
        let Some(name) = annotated_member.kind.name() else {
            continue;
        };
        let Some(inferred_member) = inferred_members
            .iter()
            .find(|member| member.kind.has_name(name.text()))
        else {
            return false;
        };
        stack.push((annotated_member.ty, inferred_member.ty));
    }
    true
}

/// Returns whether `annotated` strictly widens `inferred`, including union
/// coverage.
///
/// `None` means at least one required comparison was indeterminate.
fn is_wider_than<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
) -> Option<bool> {
    let inferred = resolve_generic_chain(db, inferred)?;
    if matches!(annotated, TypeData::Union(_)) {
        return is_union_wider(db, annotated, inferred);
    }
    if matches!(inferred, TypeData::Union(_)) {
        let has_base = try_any(
            inferred
                .union_iterator(db)
                .map(|variant| types_match(db, annotated, variant)),
        )?;
        let all_subsumed = try_all(inferred.union_iterator(db).map(|variant| {
            relation_or(types_match(db, annotated, variant), || {
                Some(is_base_type_of_literal(db, annotated, variant))
            })
        }))?;
        if has_base && all_subsumed {
            return Some(false);
        }
        return Some(
            try_all(inferred.union_iterator(db).map(|variant| {
                relation_or(types_match(db, annotated, variant), || {
                    is_nonunion_wider(db, annotated, variant)
                })
            }))? && try_any(
                inferred
                    .union_iterator(db)
                    .map(|variant| is_nonunion_wider(db, annotated, variant)),
            )?,
        );
    }
    is_nonunion_wider(db, annotated, inferred)
}

/// Returns whether an annotated union covers every inferred return and contains
/// either an unused variant or a variant that widens a return.
fn is_union_wider_than_returns<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    returns: &[TypeData<'db>],
) -> Option<bool> {
    if !try_all(returns.iter().map(|return_ty| {
        try_any(annotated.union_iterator(db).map(|variant| {
            relation_or(types_match(db, variant, *return_ty), || {
                is_nonunion_wider(db, variant, *return_ty)
            })
        }))
    }))? {
        return Some(false);
    }
    let has_extra = try_any(annotated.union_iterator(db).map(|variant| {
        try_any(returns.iter().map(|return_ty| {
            relation_or(types_match(db, variant, *return_ty), || {
                is_nonunion_wider(db, variant, *return_ty)
            })
        }))
        .map(|covered| !covered)
    }))?;
    let has_wider = try_any(returns.iter().map(|return_ty| {
        let has_match = try_any(
            annotated
                .union_iterator(db)
                .map(|variant| types_match(db, variant, *return_ty)),
        )?;
        let has_wider = try_any(
            annotated
                .union_iterator(db)
                .map(|variant| is_nonunion_wider(db, variant, *return_ty)),
        )?;
        Some(!has_match && has_wider)
    }))?;
    Some(has_extra || has_wider)
}

/// Returns whether an annotated union covers an inferred type but contains an
/// eligible variant not represented by that type.
///
/// Generic variants covered by another annotated variant do not count as
/// additional width.
fn is_union_wider<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
) -> Option<bool> {
    if !try_all(inferred.union_iterator(db).map(|inferred| {
        try_any(annotated.union_iterator(db).map(|annotated| {
            relation_or(types_match(db, annotated, inferred), || {
                is_nonunion_wider(db, annotated, inferred)
            })
        }))
    }))? {
        return Some(false);
    }

    let mut eligible = Vec::new();
    for variant in annotated.union_iterator(db) {
        let TypeData::Generic(generic) = variant else {
            eligible.push(variant);
            continue;
        };
        let Some(constraint) = generic.constraint(db) else {
            eligible.push(variant);
            continue;
        };
        let covered = try_any(annotated.union_iterator(db).map(|other| {
            if other == variant {
                Some(false)
            } else {
                relation_or(types_match(db, other, constraint), || {
                    is_nonunion_wider(db, other, constraint)
                })
            }
        }))?;
        if !covered {
            eligible.push(variant);
        }
    }
    try_any(eligible.iter().map(|annotated| {
        try_any(inferred.union_iterator(db).map(|inferred| {
            relation_or(types_match(db, *annotated, inferred), || {
                is_nonunion_wider(db, *annotated, inferred)
            })
        }))
        .map(|covered| !covered)
    }))
}

/// Compares type identity after removing unparameterized instance wrappers.
///
/// Generic parameters compare by name. `None` means wrapper traversal exceeded
/// the relation work budget.
fn types_match<'db>(
    db: &'db dyn TypeDb,
    mut left: TypeData<'db>,
    mut right: TypeData<'db>,
) -> Option<bool> {
    for _ in 0..MAX_RETURN_TYPE_STEPS {
        if left == right {
            return Some(true);
        }
        if let (TypeData::Generic(left_generic), TypeData::Generic(right_generic)) = (left, right) {
            return Some(left_generic.name(db) == right_generic.name(db));
        }
        if let (TypeData::InstanceOf(left_instance), TypeData::InstanceOf(right_instance)) =
            (left, right)
            && left_instance.type_parameters(db).is_empty()
            && right_instance.type_parameters(db).is_empty()
        {
            left = left_instance.ty(db);
            right = right_instance.ty(db);
            continue;
        }
        if let (TypeData::Generic(generic), TypeData::InstanceOf(instance)) = (left, right)
            && instance.type_parameters(db).is_empty()
        {
            return Some(
                matches!(instance.ty(db), TypeData::Generic(other) if generic.name(db) == other.name(db)),
            );
        }
        if let (TypeData::InstanceOf(instance), TypeData::Generic(generic)) = (left, right)
            && instance.type_parameters(db).is_empty()
        {
            return Some(
                matches!(instance.ty(db), TypeData::Generic(other) if generic.name(db) == other.name(db)),
            );
        }
        return Some(false);
    }
    None
}

/// Combines tri-state predicates while preserving a proven `true` result and
/// propagating uncertainty when neither predicate proves the relation.
fn relation_or(first: Option<bool>, second: impl FnOnce() -> Option<bool>) -> Option<bool> {
    match first {
        Some(true) => Some(true),
        Some(false) => second(),
        None => match second() {
            Some(true) => Some(true),
            Some(false) | None => None,
        },
    }
}

/// Applies three-valued universal quantification.
///
/// A proven `false` dominates indeterminate values. Otherwise, any
/// indeterminate value makes the result indeterminate.
fn try_all(results: impl IntoIterator<Item = Option<bool>>) -> Option<bool> {
    let mut unknown = false;
    for result in results {
        match result {
            Some(false) => return Some(false),
            Some(true) => {}
            None => unknown = true,
        }
    }
    (!unknown).then_some(true)
}

/// Applies three-valued existential quantification.
///
/// A proven `true` dominates indeterminate values. Otherwise, any indeterminate
/// value makes the result indeterminate.
fn try_any(results: impl IntoIterator<Item = Option<bool>>) -> Option<bool> {
    let mut unknown = false;
    for result in results {
        match result {
            Some(true) => return Some(true),
            Some(false) => {}
            None => unknown = true,
        }
    }
    (!unknown).then_some(false)
}

/// Selects a strict subset of annotated union variants that covers the inferred
/// returns without requiring a structural widening suggestion.
fn narrowed_type_candidates<'db>(
    db: &'db dyn TypeDb,
    annotation: TypeData<'db>,
    returns: &[TypeData<'db>],
) -> NarrowedTypeCandidates<'db> {
    let variants = annotation.union_iterator(db);
    let variant_count = variants.len();
    let mut covered = Vec::new();
    for variant in variants {
        let Some(is_covered) = try_any(returns.iter().map(|return_ty| {
            relation_or(types_match(db, variant, *return_ty), || {
                is_nonunion_wider(db, variant, *return_ty)
            })
        })) else {
            return NarrowedTypeCandidates::Indeterminate;
        };
        if is_covered {
            covered.push(variant);
        }
    }
    if covered.is_empty() || covered.len() == variant_count {
        return NarrowedTypeCandidates::Unavailable;
    }
    let Some(has_widening) = try_any(covered.iter().map(|variant| {
        try_any(returns.iter().map(|return_ty| {
            let matches = types_match(db, *variant, *return_ty)?;
            let wider = is_nonunion_wider(db, *variant, *return_ty)?;
            Some(!matches && wider)
        }))
    })) else {
        return NarrowedTypeCandidates::Indeterminate;
    };
    if has_widening
        && !(covered.len() == 1 && returns.len() == 1 && returns[0].is_literal_of_primitive(db))
    {
        return NarrowedTypeCandidates::Unavailable;
    }
    NarrowedTypeCandidates::Available(covered.into_boxed_slice())
}

/// The relation between a declared return type and the inferred return values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnTypeVerdict {
    /// The declared type has a strict widening relation to the inferred returns.
    Wider,
    /// The declared and inferred return types are equivalent.
    Equal,
    /// At least one inferred return is not covered by the declared type.
    Incompatible,
    /// Resolution, a cycle, or a traversal budget prevented a reliable answer.
    Indeterminate,
}

/// Candidate declared-type variants that cover the inferred returns.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NarrowedTypeCandidates<'db> {
    /// Candidate variants were computed successfully.
    Available(Box<[TypeData<'db>]>),
    /// No strict subset of the declared variants is suitable.
    Unavailable,
    /// Candidate traversal could not produce a reliable result.
    Indeterminate,
}

/// Structured result of comparing declared and inferred return types.
pub struct ReturnTypeRelation<'db> {
    db: &'db dyn TypeDb,
    declared: TypeData<'db>,
    inferred: Box<[TypeData<'db>]>,
    verdict: ReturnTypeVerdict,
    narrowed: NarrowedTypeCandidates<'db>,
    only_property_literal_widening: Option<bool>,
}

impl<'db> ReturnTypeRelation<'db> {
    pub fn db(&self) -> &'db dyn TypeDb {
        self.db
    }

    pub fn declared(&self) -> TypeData<'db> {
        self.declared
    }

    pub fn inferred(&self) -> &[TypeData<'db>] {
        &self.inferred
    }

    pub fn verdict(&self) -> ReturnTypeVerdict {
        self.verdict
    }

    pub fn narrowed(&self) -> &NarrowedTypeCandidates<'db> {
        &self.narrowed
    }

    /// Returns whether widening is limited to object-property literal values.
    ///
    /// `Some(true)` confirms that specific widening, `Some(false)` confirms a
    /// different relation, and `None` means the overall comparison was
    /// indeterminate.
    pub fn is_only_property_literal_widening(&self) -> Option<bool> {
        self.only_property_literal_widening
    }

    pub fn has_single_primitive_literal_return(&self) -> bool {
        self.inferred.len() == 1 && self.inferred[0].is_literal_of_primitive(self.db)
    }

    /// Returns whether the declaration suppresses a reliable or useful
    /// relation verdict.
    pub fn declared_is_escape_hatch(&self) -> bool {
        is_escape_hatch(self.declared)
    }

    pub fn inferred_is_empty(&self) -> bool {
        self.inferred.is_empty()
    }

    pub fn has_any_contaminated_inferred(&self) -> bool {
        self.inferred
            .iter()
            .any(|ty| ty.is_any_contaminated(self.db))
    }

    pub fn declared_union_contains_unknown(&self) -> bool {
        matches!(self.declared, TypeData::Union(_))
            && self
                .declared
                .union_iterator(self.db)
                .any(|ty| matches!(ty, TypeData::UnknownKeyword | TypeData::Unknown))
    }

    pub fn has_undefined_mismatch(&self) -> bool {
        self.declared.includes_undefined(self.db)
            && !self
                .inferred
                .iter()
                .any(|ty| ty.includes_undefined(self.db))
    }

    pub fn inferred_has_generic_intersection(&self) -> bool {
        self.inferred
            .iter()
            .any(|ty| is_intersection_with_type_param(self.db, *ty))
    }

    pub fn includes_object_return(&self) -> bool {
        self.inferred
            .iter()
            .any(|ty| ty.includes_object_keyword(self.db))
    }

    /// Returns whether an inferred object return is wider than the declaration.
    ///
    /// `Some(true)` confirms a wider return, `Some(false)` proves no inferred
    /// return is wider, and `None` means unresolved data, a cycle, or budget
    /// exhaustion prevented a reliable answer.
    pub fn object_has_wider_return(&self) -> Option<bool> {
        complete_any(
            self.inferred
                .iter()
                .map(|ty| is_wider_than(self.db, self.declared, *ty)),
        )
    }
}

fn indeterminate_relation<'db>(
    db: &'db dyn TypeDb,
    declared: TypeData<'db>,
    inferred: Box<[TypeData<'db>]>,
) -> ReturnTypeRelation<'db> {
    ReturnTypeRelation {
        db,
        declared,
        inferred,
        verdict: ReturnTypeVerdict::Indeterminate,
        narrowed: NarrowedTypeCandidates::Indeterminate,
        only_property_literal_widening: None,
    }
}

fn exact_returns_match(
    db: &dyn TypeDb,
    declared: TypeData<'_>,
    inferred: &[TypeData<'_>],
) -> Option<bool> {
    try_all(
        inferred
            .iter()
            .map(|inferred| types_match(db, declared, *inferred)),
    )
}

fn union_covers_returns(
    db: &dyn TypeDb,
    declared: TypeData<'_>,
    inferred: &[TypeData<'_>],
) -> Option<bool> {
    try_all(inferred.iter().map(|inferred| {
        try_any(declared.union_iterator(db).map(|variant| {
            relation_or(types_match(db, variant, *inferred), || {
                is_nonunion_wider(db, variant, *inferred)
            })
        }))
    }))
}

/// Returns whether any result is true, but only when every result is known.
fn complete_any(results: impl IntoIterator<Item = Option<bool>>) -> Option<bool> {
    let mut any = false;
    for result in results {
        any |= result?;
    }
    Some(any)
}

/// Compares a declared return type with the inferred return values.
///
/// Boolean literal returns are normalized before comparison. Unresolved data,
/// cycles, or the relation's traversal limits produce
/// [`ReturnTypeVerdict::Indeterminate`] and indeterminate narrowing candidates.
pub fn compare_declared_return_type<'db>(
    db: &'db dyn TypeDb,
    declared: TypeData<'db>,
    inferred: &[TypeData<'db>],
) -> ReturnTypeRelation<'db> {
    compare_declared_return_type_owned(db, declared, inferred.to_vec())
}

/// Owned-input variant of [`compare_declared_return_type`].
pub(crate) fn compare_declared_return_type_owned<'db>(
    db: &'db dyn TypeDb,
    declared: TypeData<'db>,
    inferred: Vec<TypeData<'db>>,
) -> ReturnTypeRelation<'db> {
    if is_escape_hatch(declared) {
        return indeterminate_relation(db, declared, inferred.into());
    }
    let declared = match collapse_union_absorbed_by_primitive(db, declared) {
        PrimitiveUnionCollapse::Collapsed(primitive) => primitive,
        PrimitiveUnionCollapse::NotApplicable => declared,
        PrimitiveUnionCollapse::Indeterminate => {
            return indeterminate_relation(db, declared, inferred.into());
        }
    };
    let inferred = normalize_boolean_return_types(db, inferred).into_boxed_slice();

    if inferred.is_empty()
        || inferred.iter().any(|ty| ty.is_any_contaminated(db))
        || matches!(declared, TypeData::Union(_))
            && declared
                .union_iterator(db)
                .any(|ty| matches!(ty, TypeData::UnknownKeyword | TypeData::Unknown))
        || declared.includes_undefined(db) && !inferred.iter().any(|ty| ty.includes_undefined(db))
        || inferred
            .iter()
            .any(|ty| is_intersection_with_type_param(db, *ty))
    {
        return indeterminate_relation(db, declared, inferred);
    }

    let only_property_literal_widening = is_only_property_literal_widening(db, declared, &inferred);
    let narrowed = narrowed_type_candidates(db, declared, &inferred);
    let verdict = if matches!(declared, TypeData::Union(_)) {
        match is_union_wider_than_returns(db, declared, &inferred) {
            Some(true) => ReturnTypeVerdict::Wider,
            Some(false) => match union_covers_returns(db, declared, &inferred) {
                Some(true) => ReturnTypeVerdict::Equal,
                Some(false) => ReturnTypeVerdict::Incompatible,
                None => ReturnTypeVerdict::Indeterminate,
            },
            None => ReturnTypeVerdict::Indeterminate,
        }
    } else if matches!(declared, TypeData::ObjectKeyword) {
        if inferred.iter().any(|ty| ty.includes_object_keyword(db)) {
            ReturnTypeVerdict::Equal
        } else {
            match complete_any(inferred.iter().map(|ty| is_wider_than(db, declared, *ty))) {
                Some(true) => ReturnTypeVerdict::Wider,
                Some(false) => ReturnTypeVerdict::Incompatible,
                None => ReturnTypeVerdict::Indeterminate,
            }
        }
    } else {
        match try_all(inferred.iter().map(|ty| is_wider_than(db, declared, *ty))) {
            Some(true) => ReturnTypeVerdict::Wider,
            Some(false) => match exact_returns_match(db, declared, &inferred) {
                Some(true) => ReturnTypeVerdict::Equal,
                Some(false) => ReturnTypeVerdict::Incompatible,
                None => ReturnTypeVerdict::Indeterminate,
            },
            None => ReturnTypeVerdict::Indeterminate,
        }
    };

    ReturnTypeRelation {
        db,
        declared,
        inferred,
        verdict,
        narrowed,
        only_property_literal_widening,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interned_types::{
        InternedClass, InternedGenericTypeParameter, InternedLiteral, InternedObject,
        InternedTuple, InternedTypeInstance, InternedUnion, TupleElementType, TypeMember,
    };
    use biome_rowan::Text;
    use salsa::plumbing::FromId;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(
            &self,
            _path: &camino::Utf8Path,
        ) -> Option<biome_db::ParsedSource> {
            None
        }
    }

    #[salsa::db]
    impl TypeDb for TestDb {}

    #[test]
    fn iterates_union_variants_without_expanding_nonunions() {
        let db = TestDb::default();
        let union = TypeData::Union(InternedUnion::new(
            &db,
            Box::new([TypeData::String, TypeData::Number]) as Box<[_]>,
        ));

        let mut variants = union.union_iterator(&db);
        assert_eq!(variants.len(), 2);
        assert_eq!(variants.next(), Some(TypeData::String));
        assert_eq!(variants.len(), 1);
        assert_eq!(variants.next(), Some(TypeData::Number));
        assert_eq!(variants.next(), None);

        assert_eq!(
            TypeData::Boolean.union_iterator(&db).collect::<Vec<_>>(),
            [TypeData::Boolean]
        );
    }

    fn text(value: &'static str) -> Text {
        Text::new_static(value)
    }

    fn string_literal<'db>(db: &'db TestDb) -> TypeData<'db> {
        TypeData::Literal(InternedLiteral::new(
            db,
            Literal::String(text("value").into()),
        ))
    }

    fn generic_chain<'db>(
        db: &'db TestDb,
        length: usize,
        terminal: TypeData<'db>,
    ) -> TypeData<'db> {
        (0..length).fold(terminal, |constraint, index| {
            TypeData::Generic(InternedGenericTypeParameter::new(
                db,
                Some(constraint),
                None,
                Text::new_owned(format!("T{index}").into_boxed_str()),
            ))
        })
    }

    fn tuple<'db>(db: &'db TestDb, ty: TypeData<'db>) -> TypeData<'db> {
        TypeData::Tuple(InternedTuple::new(
            db,
            Box::new([TupleElementType {
                ty,
                name: None,
                is_optional: false,
                is_rest: false,
            }]) as Box<[_]>,
        ))
    }

    fn object<'db>(db: &'db TestDb, ty: TypeData<'db>) -> TypeData<'db> {
        TypeData::Object(InternedObject::new(
            db,
            None,
            Box::new([TypeMember {
                kind: TypeMemberKind::Named(text("value")),
                ty,
            }]) as Box<[_]>,
        ))
    }

    fn indexed_object<'db>(db: &'db TestDb, ty: TypeData<'db>) -> TypeData<'db> {
        TypeData::Object(InternedObject::new(
            db,
            None,
            Box::new([TypeMember {
                kind: TypeMemberKind::IndexSignature(TypeData::String),
                ty,
            }]) as Box<[_]>,
        ))
    }

    fn instance<'db>(db: &'db TestDb, ty: TypeData<'db>) -> TypeData<'db> {
        TypeData::InstanceOf(InternedTypeInstance::new(
            db,
            TypeData::ObjectKeyword,
            Box::new([ty]) as Box<[_]>,
        ))
    }

    fn named_class<'db>(db: &'db TestDb, name: &'static str) -> TypeData<'db> {
        TypeData::Class(InternedClass::new(
            db,
            Box::default(),
            None,
            Box::default(),
            Box::default(),
            Some(text(name)),
            false,
        ))
    }

    fn extending_class_chain<'db>(db: &'db TestDb, distinct_types: usize) -> InternedClass<'db> {
        let leaf = InternedClass::new(
            db,
            Box::default(),
            None,
            Box::default(),
            Box::default(),
            Some(text("Leaf")),
            false,
        );
        (1..distinct_types).fold(leaf, |base, _| {
            InternedClass::new(
                db,
                Box::default(),
                Some(TypeData::Class(base)),
                Box::default(),
                Box::default(),
                None,
                false,
            )
        })
    }

    #[test]
    fn resolves_long_generic_chains_repeatedly() {
        let db = TestDb::default();
        let chain = generic_chain(&db, 65, TypeData::String);

        assert_eq!(resolve_generic_chain(&db, chain), Some(TypeData::String));
        assert_eq!(resolve_generic_chain(&db, chain), Some(TypeData::String));
    }

    #[test]
    fn object_widening_requires_complete_comparisons() {
        assert_eq!(complete_any([Some(true), None]), None);
        assert_eq!(complete_any([Some(false), None]), None);
        assert_eq!(complete_any([Some(false), Some(true)]), Some(true));
    }

    #[test]
    fn generic_cycles_do_not_resolve() {
        let db = TestDb::default();
        let self_reference =
            InternedGenericTypeParameter::from_id(unsafe { salsa::Id::from_index(0) });
        let self_cycle = TypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            Some(TypeData::Generic(self_reference)),
            None,
            text("SelfCycle"),
        ));

        assert_eq!(resolve_generic_chain(&db, self_cycle), None);
        assert_eq!(resolve_generic_chain(&db, self_cycle), None);

        let db = TestDb::default();
        let second_reference =
            InternedGenericTypeParameter::from_id(unsafe { salsa::Id::from_index(1) });
        let first = TypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            Some(TypeData::Generic(second_reference)),
            None,
            text("First"),
        ));
        let TypeData::Generic(first_reference) = first else {
            unreachable!();
        };
        InternedGenericTypeParameter::new(
            &db,
            Some(TypeData::Generic(first_reference)),
            None,
            text("Second"),
        );

        assert_eq!(resolve_generic_chain(&db, first), None);
        assert_eq!(resolve_generic_chain(&db, first), None);
    }

    #[test]
    fn long_generic_chains_resolve_for_structural_consumers() {
        let db = TestDb::default();
        let literal = string_literal(&db);

        for (annotated, inferred) in [
            (
                object(&db, TypeData::String),
                object(&db, generic_chain(&db, 65, literal)),
            ),
            (
                tuple(&db, TypeData::String),
                tuple(&db, generic_chain(&db, 65, literal)),
            ),
            (
                instance(&db, TypeData::String),
                instance(&db, generic_chain(&db, 65, literal)),
            ),
            (
                indexed_object(&db, TypeData::String),
                object(&db, generic_chain(&db, 65, literal)),
            ),
        ] {
            assert_eq!(is_nonunion_wider(&db, annotated, inferred), Some(true));
            assert_eq!(is_nonunion_wider(&db, annotated, inferred), Some(true));
        }
    }

    #[test]
    fn generic_cycles_suppress_structural_comparisons() {
        let db = TestDb::default();
        let cycle_reference =
            InternedGenericTypeParameter::from_id(unsafe { salsa::Id::from_index(0) });
        let cycle = TypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            Some(TypeData::Generic(cycle_reference)),
            None,
            text("Cycle"),
        ));

        for (annotated, inferred) in [
            (object(&db, TypeData::String), object(&db, cycle)),
            (tuple(&db, TypeData::String), tuple(&db, cycle)),
            (instance(&db, TypeData::String), instance(&db, cycle)),
            (indexed_object(&db, TypeData::String), object(&db, cycle)),
        ] {
            assert_eq!(is_nonunion_wider(&db, annotated, inferred), None);
            assert_eq!(is_nonunion_wider(&db, annotated, inferred), None);
        }
    }

    #[test]
    fn structural_return_relations_observe_step_limit() {
        let db = TestDb::default();
        let literal = string_literal(&db);

        for steps in [
            MAX_RETURN_TYPE_STEPS - 1,
            MAX_RETURN_TYPE_STEPS,
            MAX_RETURN_TYPE_STEPS + 1,
        ] {
            let (property_annotation, property_inferred) = (1..steps).fold(
                (object(&db, TypeData::String), object(&db, literal)),
                |(annotated, inferred), _| (object(&db, annotated), object(&db, inferred)),
            );
            let expected = (steps <= MAX_RETURN_TYPE_STEPS).then_some(true);
            assert_eq!(
                is_only_property_literal_widening(&db, property_annotation, &[property_inferred],),
                expected,
                "property-only widening steps {steps}"
            );
            let (annotated, inferred) = (2..steps).fold(
                (object(&db, TypeData::String), object(&db, literal)),
                |(annotated, inferred), _| (object(&db, annotated), object(&db, inferred)),
            );
            assert_eq!(
                is_nonunion_wider(&db, annotated, inferred),
                expected,
                "structural widening steps {steps}"
            );
            let relation = compare_declared_return_type(&db, annotated, &[inferred]);
            assert_eq!(
                relation.verdict(),
                if steps <= MAX_RETURN_TYPE_STEPS {
                    ReturnTypeVerdict::Wider
                } else {
                    ReturnTypeVerdict::Indeterminate
                },
                "public relation steps {steps}"
            );
        }
    }

    #[test]
    fn equality_and_class_shape_observe_step_limit() {
        let db = TestDb::default();

        for (steps, expected_class_shape) in [
            (MAX_RETURN_TYPE_STEPS - 1, Some(false)),
            (MAX_RETURN_TYPE_STEPS, Some(false)),
            (MAX_RETURN_TYPE_STEPS + 1, None),
        ] {
            let left = (1..steps).fold(named_class(&db, "Left"), |ty, _| {
                TypeData::InstanceOf(InternedTypeInstance::new(&db, ty, Box::default()))
            });
            let right = (1..steps).fold(named_class(&db, "Right"), |ty, _| {
                TypeData::InstanceOf(InternedTypeInstance::new(&db, ty, Box::default()))
            });
            assert_eq!(
                types_match(&db, left, right),
                (steps <= MAX_RETURN_TYPE_STEPS).then_some(false),
                "equality steps {steps}"
            );

            let class_instance = TypeData::InstanceOf(InternedTypeInstance::new(
                &db,
                TypeData::Class(extending_class_chain(&db, steps)),
                Box::default(),
            ));
            assert_eq!(
                class_instance.is_strictly_narrower_than_object_keyword(&db),
                expected_class_shape,
                "class shape steps {steps}"
            );
        }
    }

    #[test]
    fn generic_union_extra_variant_remains_wider() {
        let db = TestDb::default();
        let generic = TypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            None,
            None,
            text("T"),
        ));
        let annotation = TypeData::Union(InternedUnion::new(
            &db,
            Box::new([generic, TypeData::Null]) as Box<[_]>,
        ));

        assert_eq!(
            is_union_wider_than_returns(&db, annotation, &[generic]),
            Some(true)
        );
        let relation = compare_declared_return_type(&db, annotation, &[generic]);
        assert_eq!(relation.verdict(), ReturnTypeVerdict::Wider);
        assert_eq!(
            relation.narrowed(),
            &NarrowedTypeCandidates::Available(Box::new([generic]))
        );
    }
}
