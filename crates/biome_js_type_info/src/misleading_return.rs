//! Policy and evidence extraction for `noMisleadingReturnType` diagnostics.

use crate::TypeDb;
use crate::interned_types::{InternedClass, Literal, TypeData, TypeMember, TypeMemberKind};
use rustc_hash::FxHashSet;

const MAX_RETURN_TYPE_STEPS: usize = 50;
const MAX_RETURN_TYPE_DEPTH: usize = 50;
const MAX_RETURN_TYPE_DESCRIPTION_LENGTH: usize = 80;
const RETURN_TYPE_SEPARATOR: &str = " | ";

fn is_escape_hatch(ty: TypeData<'_>) -> bool {
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

fn promise_inner<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Option<TypeData<'db>> {
    let TypeData::InstanceOf(instance) = ty else {
        return None;
    };
    if !instance.ty(db).is_promise_class(db) {
        return None;
    }
    instance
        .type_parameters(db)
        .first()
        .copied()
        .filter(|ty| !is_escape_hatch(*ty))
}

fn union_variants<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Vec<TypeData<'db>> {
    match ty {
        TypeData::Union(union) => union.types(db).to_vec(),
        ty => Vec::from([ty]),
    }
}

fn collapse_union_absorbed_by_primitive<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
) -> Result<Option<TypeData<'db>>, ()> {
    let TypeData::Union(_) = ty else {
        return Ok(None);
    };
    let variants = union_variants(db, ty);
    let mut primitive = None;
    for variant in &variants {
        if matches!(
            variant,
            TypeData::String | TypeData::Number | TypeData::Boolean | TypeData::BigInt
        ) {
            if primitive.is_some() {
                return Ok(None);
            }
            primitive = Some(*variant);
        }
    }
    let Some(primitive) = primitive else {
        return Ok(None);
    };
    Ok(try_all(variants.iter().map(|variant| {
        relation_or(types_match(db, *variant, primitive), || {
            is_nonunion_wider(db, primitive, *variant)
        })
    }))
    .ok_or(())?
    .then_some(primitive))
}

fn includes_object_keyword(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    matches!(ty, TypeData::ObjectKeyword)
        || matches!(ty, TypeData::Union(_))
            && union_variants(db, ty)
                .iter()
                .any(|variant| matches!(variant, TypeData::ObjectKeyword))
}

fn includes_undefined(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    matches!(ty, TypeData::Undefined | TypeData::VoidKeyword)
        || matches!(ty, TypeData::Union(_))
            && union_variants(db, ty)
                .iter()
                .any(|variant| matches!(variant, TypeData::Undefined | TypeData::VoidKeyword))
}

fn is_any_contaminated(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    match ty {
        TypeData::AnyKeyword => true,
        TypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| matches!(ty, TypeData::AnyKeyword)),
        TypeData::Intersection(intersection) => intersection
            .types(db)
            .iter()
            .any(|ty| matches!(ty, TypeData::AnyKeyword)),
        _ => false,
    }
}

fn is_intersection_with_type_param(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    matches!(ty, TypeData::Intersection(intersection) if intersection.types(db).iter().any(|ty| matches!(ty, TypeData::Generic(_))))
}

fn is_literal_of_primitive(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    match ty {
        TypeData::Literal(literal) => matches!(
            literal.literal(db),
            Literal::BigInt(_)
                | Literal::Boolean(_)
                | Literal::Number(_)
                | Literal::String(_)
                | Literal::Template(_)
        ),
        TypeData::Union(union) if union.types(db).len() == 1 => {
            is_literal_of_primitive(db, union.types(db)[0])
        }
        _ => false,
    }
}

fn is_base_type_of_literal(db: &dyn TypeDb, base: TypeData<'_>, literal: TypeData<'_>) -> bool {
    matches!(literal.literal_base_type(db), Some(literal_base) if literal_base == base)
}

fn is_only_property_literal_widening(
    db: &dyn TypeDb,
    annotation: TypeData<'_>,
    returns: &[TypeData<'_>],
) -> Option<bool> {
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

fn type_members<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Option<&'db [TypeMember<'db>]> {
    match ty {
        TypeData::Object(object) => Some(object.members(db)),
        TypeData::Literal(literal) => match literal.literal(db) {
            Literal::Object(members) => Some(members),
            _ => None,
        },
        _ => None,
    }
}

fn is_strictly_narrower_than_object_keyword(db: &dyn TypeDb, ty: TypeData<'_>) -> Option<bool> {
    match ty {
        TypeData::Object(object) => Some(!object.members(db).is_empty()),
        TypeData::InstanceOf(instance) => match instance.ty(db) {
            TypeData::Class(class) => {
                class_has_instance_shape(db, class, &mut FxHashSet::default(), 0)
            }
            _ => Some(true),
        },
        TypeData::Tuple(_) | TypeData::Function(_) => Some(true),
        TypeData::Literal(literal) => match literal.literal(db) {
            Literal::RegExp(_) => Some(true),
            Literal::Object(members) => Some(!members.is_empty()),
            _ => Some(false),
        },
        _ => Some(false),
    }
}

fn class_has_instance_shape<'db>(
    db: &'db dyn TypeDb,
    class: InternedClass<'db>,
    seen: &mut FxHashSet<TypeData<'db>>,
    depth: usize,
) -> Option<bool> {
    let ty = TypeData::Class(class);
    if depth >= MAX_RETURN_TYPE_DEPTH || !seen.insert(ty) {
        return None;
    }
    if class
        .members(db)
        .iter()
        .any(|member| !member.kind.is_static())
    {
        return Some(true);
    }
    match class.extends(db) {
        None => Some(false),
        Some(base) => match base {
            TypeData::Class(base) => class_has_instance_shape(db, base, seen, depth + 1),
            TypeData::InstanceOf(instance) => match instance.ty(db) {
                TypeData::Class(base) => class_has_instance_shape(db, base, seen, depth + 1),
                _ => Some(true),
            },
            _ => Some(true),
        },
    }
}

fn is_nonunion_wider<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
) -> Option<bool> {
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

        match (annotated, inferred) {
            (TypeData::ObjectKeyword, TypeData::InstanceOf(_)) => found_wider = true,
            (TypeData::InstanceOf(annotated), TypeData::InstanceOf(inferred)) => {
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
            }
            (TypeData::Object(_), TypeData::Object(_) | TypeData::Literal(_)) => {
                if !push_object_pairs(db, annotated, inferred, &mut stack) {
                    return Some(false);
                }
            }
            (TypeData::ObjectKeyword, inferred)
                if is_strictly_narrower_than_object_keyword(db, inferred)? =>
            {
                found_wider = true;
            }
            (TypeData::Tuple(annotated), TypeData::Tuple(inferred)) => {
                let annotated_elements = annotated.elements(db);
                let inferred_elements = inferred.elements(db);
                if annotated_elements.len() != inferred_elements.len()
                    || annotated_elements.is_empty()
                {
                    return Some(false);
                }
                stack.extend(
                    annotated_elements
                        .iter()
                        .zip(inferred_elements)
                        .map(|(annotated, inferred)| (annotated.ty, inferred.ty)),
                );
            }
            _ => return Some(false),
        }
    }

    Some(found_wider)
}

fn push_object_pairs<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
    stack: &mut Vec<(TypeData<'db>, TypeData<'db>)>,
) -> bool {
    let Some(annotated_members) = type_members(db, annotated) else {
        return false;
    };
    let Some(inferred_members) = type_members(db, inferred) else {
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

fn is_wider_than<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
) -> Option<bool> {
    let inferred = resolve_generic_chain(db, inferred)?;
    match (annotated, inferred) {
        (TypeData::String, TypeData::String)
        | (TypeData::Number, TypeData::Number)
        | (TypeData::Boolean, TypeData::Boolean)
        | (TypeData::BigInt, TypeData::BigInt) => Some(false),
        (TypeData::Union(_), _) => is_union_wider(db, annotated, inferred),
        (_, TypeData::Union(_)) => {
            let variants = union_variants(db, inferred);
            let has_base = try_any(
                variants
                    .iter()
                    .map(|variant| types_match(db, annotated, *variant)),
            )?;
            let all_subsumed = try_all(variants.iter().map(|variant| {
                relation_or(types_match(db, annotated, *variant), || {
                    Some(is_base_type_of_literal(db, annotated, *variant))
                })
            }))?;
            if has_base && all_subsumed {
                return Some(false);
            }
            Some(
                try_all(variants.iter().map(|variant| {
                    relation_or(types_match(db, annotated, *variant), || {
                        is_nonunion_wider(db, annotated, *variant)
                    })
                }))? && try_any(
                    variants
                        .iter()
                        .map(|variant| is_nonunion_wider(db, annotated, *variant)),
                )?,
            )
        }
        _ => is_nonunion_wider(db, annotated, inferred),
    }
}

fn is_union_wider_than_returns<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    returns: &[TypeData<'db>],
) -> Option<bool> {
    let variants = union_variants(db, annotated);
    if !try_all(returns.iter().map(|return_ty| {
        try_any(variants.iter().map(|variant| {
            relation_or(types_match(db, *variant, *return_ty), || {
                is_nonunion_wider(db, *variant, *return_ty)
            })
        }))
    }))? {
        return Some(false);
    }
    let has_extra = try_any(variants.iter().map(|variant| {
        try_any(returns.iter().map(|return_ty| {
            relation_or(types_match(db, *variant, *return_ty), || {
                is_nonunion_wider(db, *variant, *return_ty)
            })
        }))
        .map(|covered| !covered)
    }))?;
    let has_wider = try_any(returns.iter().map(|return_ty| {
        let has_match = try_any(
            variants
                .iter()
                .map(|variant| types_match(db, *variant, *return_ty)),
        )?;
        let has_wider = try_any(
            variants
                .iter()
                .map(|variant| is_nonunion_wider(db, *variant, *return_ty)),
        )?;
        Some(!has_match && has_wider)
    }))?;
    Some(has_extra || has_wider)
}

fn is_union_wider<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
) -> Option<bool> {
    let annotated_variants = union_variants(db, annotated);
    let inferred_variants = union_variants(db, inferred);
    if !try_all(inferred_variants.iter().map(|inferred| {
        try_any(annotated_variants.iter().map(|annotated| {
            relation_or(types_match(db, *annotated, *inferred), || {
                is_nonunion_wider(db, *annotated, *inferred)
            })
        }))
    }))? {
        return Some(false);
    }

    let mut eligible = Vec::new();
    for variant in &annotated_variants {
        let TypeData::Generic(generic) = variant else {
            eligible.push(*variant);
            continue;
        };
        let Some(constraint) = generic.constraint(db) else {
            eligible.push(*variant);
            continue;
        };
        let covered = try_any(annotated_variants.iter().map(|other| {
            if other == variant {
                Some(false)
            } else {
                relation_or(types_match(db, *other, constraint), || {
                    is_nonunion_wider(db, *other, constraint)
                })
            }
        }))?;
        if !covered {
            eligible.push(*variant);
        }
    }
    try_any(eligible.iter().map(|annotated| {
        try_any(inferred_variants.iter().map(|inferred| {
            relation_or(types_match(db, *annotated, *inferred), || {
                is_nonunion_wider(db, *annotated, *inferred)
            })
        }))
        .map(|covered| !covered)
    }))
}

fn types_match<'db>(
    db: &'db dyn TypeDb,
    mut left: TypeData<'db>,
    mut right: TypeData<'db>,
) -> Option<bool> {
    for _ in 0..MAX_RETURN_TYPE_STEPS {
        if left == right {
            return Some(true);
        }
        match (left, right) {
            (TypeData::Generic(left_generic), TypeData::Generic(right_generic)) => {
                return Some(left_generic.name(db) == right_generic.name(db));
            }
            (TypeData::InstanceOf(left_instance), TypeData::InstanceOf(right_instance))
                if left_instance.type_parameters(db).is_empty()
                    && right_instance.type_parameters(db).is_empty() =>
            {
                left = left_instance.ty(db);
                right = right_instance.ty(db);
            }
            (TypeData::Generic(generic), TypeData::InstanceOf(instance))
                if instance.type_parameters(db).is_empty() =>
            {
                return Some(
                    matches!(instance.ty(db), TypeData::Generic(other) if generic.name(db) == other.name(db)),
                );
            }
            (TypeData::InstanceOf(instance), TypeData::Generic(generic))
                if instance.type_parameters(db).is_empty() =>
            {
                return Some(
                    matches!(instance.ty(db), TypeData::Generic(other) if generic.name(db) == other.name(db)),
                );
            }
            _ => return Some(false),
        }
    }
    None
}

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

fn literal_text(db: &dyn TypeDb, ty: TypeData<'_>) -> Option<String> {
    let TypeData::Literal(literal) = ty else {
        return None;
    };
    match literal.literal(db) {
        Literal::String(value) => Some(format!("\"{}\"", value.as_str())),
        Literal::Number(value) => Some(value.as_str().to_string()),
        Literal::Boolean(value) => Some(value.as_bool().to_string()),
        _ => None,
    }
}

fn renderable_variant(db: &dyn TypeDb, ty: TypeData<'_>) -> Option<String> {
    match ty {
        TypeData::String => Some("string".into()),
        TypeData::Number => Some("number".into()),
        TypeData::Boolean => Some("boolean".into()),
        TypeData::BigInt => Some("bigint".into()),
        _ => literal_text(db, ty),
    }
}

fn clean_literal_text(text: &str) -> bool {
    !text.contains("...") && !text.contains("__internal") && !text.contains("typeof import(")
}

fn join_description(parts: Vec<String>) -> Option<String> {
    if parts.is_empty() || parts.iter().any(|part| !clean_literal_text(part)) {
        return None;
    }
    let description = parts.join(RETURN_TYPE_SEPARATOR);
    (description.len() <= MAX_RETURN_TYPE_DESCRIPTION_LENGTH).then_some(description)
}

fn render_inferred(db: &dyn TypeDb, returns: &[TypeData<'_>]) -> Option<String> {
    join_description(
        returns
            .iter()
            .map(|ty| literal_text(db, *ty))
            .collect::<Option<Vec<_>>>()?,
    )
}

fn render_narrowed<'db>(
    db: &'db dyn TypeDb,
    annotation: TypeData<'db>,
    returns: &[TypeData<'db>],
) -> Result<Option<String>, ()> {
    let variants = union_variants(db, annotation);
    let mut covered = Vec::new();
    for variant in &variants {
        if try_any(returns.iter().map(|return_ty| {
            relation_or(types_match(db, *variant, *return_ty), || {
                is_nonunion_wider(db, *variant, *return_ty)
            })
        }))
        .ok_or(())?
        {
            covered.push(*variant);
        }
    }
    if covered.is_empty() || covered.len() == variants.len() {
        return Ok(None);
    }
    let has_widening = try_any(covered.iter().map(|variant| {
        try_any(returns.iter().map(|return_ty| {
            let matches = types_match(db, *variant, *return_ty)?;
            let wider = is_nonunion_wider(db, *variant, *return_ty)?;
            Some(!matches && wider)
        }))
    }))
    .ok_or(())?;
    if has_widening
        && !(covered.len() == 1 && returns.len() == 1 && is_literal_of_primitive(db, returns[0]))
    {
        return Ok(None);
    }
    Ok(covered
        .iter()
        .map(|ty| renderable_variant(db, *ty))
        .collect::<Option<Vec<_>>>()
        .and_then(join_description))
}

#[derive(Clone, Copy, Debug, Default)]
/// Evidence collected from return expressions for the user-visible diagnostic.
pub struct ReturnTypeEvidence {
    /// At least one return expression contains a const assertion.
    pub has_any_const: bool,
    /// Number of object-widening casts observed in return expressions.
    pub object_wide_casts: usize,
    /// At least one return is narrower than the declared object type.
    pub has_narrower_than_object: bool,
    /// At least one assertion intentionally pins the declared return type.
    pub has_pinning_assertion: bool,
    /// Prefer showing the inferred replacement over a generic widening hint.
    pub prefer_inferred_suggestion: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Data used to render a `noMisleadingReturnType` diagnostic.
pub struct MisleadingReturnType {
    /// A concise inferred return-type replacement when one can be rendered.
    pub suggestion: Option<String>,
}

pub(crate) fn check_misleading_return_type<'db>(
    db: &'db dyn TypeDb,
    mut annotation: TypeData<'db>,
    returns: Vec<TypeData<'db>>,
    evidence: ReturnTypeEvidence,
    is_async: bool,
) -> Option<MisleadingReturnType> {
    if is_escape_hatch(annotation) {
        return None;
    }
    if is_async {
        annotation = promise_inner(db, annotation).unwrap_or(annotation);
    }
    annotation = collapse_union_absorbed_by_primitive(db, annotation)
        .ok()?
        .unwrap_or(annotation);

    let return_types = normalize_boolean_return_types(db, returns);
    if return_types.is_empty() {
        return None;
    }
    if return_types.len() == 1
        && !evidence.has_any_const
        && !evidence.has_pinning_assertion
        && is_literal_of_primitive(db, return_types[0])
        && !matches!(annotation, TypeData::Union(_))
    {
        return None;
    }
    if !evidence.has_any_const
        && evidence.object_wide_casts == return_types.len()
        && matches!(annotation, TypeData::ObjectKeyword)
    {
        return None;
    }
    if matches!(annotation, TypeData::Boolean)
        && return_types
            .iter()
            .any(|ty| ty.is_boolean_literal(db, true))
        && return_types
            .iter()
            .any(|ty| ty.is_boolean_literal(db, false))
    {
        return None;
    }
    if return_types.iter().any(|ty| is_any_contaminated(db, *ty)) {
        return None;
    }
    if matches!(annotation, TypeData::Union(_))
        && union_variants(db, annotation)
            .iter()
            .any(|ty| matches!(ty, TypeData::UnknownKeyword | TypeData::Unknown))
    {
        return None;
    }
    if includes_undefined(db, annotation)
        && !return_types.iter().any(|ty| includes_undefined(db, *ty))
    {
        return None;
    }
    if return_types
        .iter()
        .any(|ty| is_intersection_with_type_param(db, *ty))
    {
        return None;
    }
    if !evidence.has_any_const && is_only_property_literal_widening(db, annotation, &return_types)?
    {
        return None;
    }

    let is_misleading = if matches!(annotation, TypeData::Union(_)) {
        is_union_wider_than_returns(db, annotation, &return_types)?
    } else if matches!(annotation, TypeData::ObjectKeyword) {
        !return_types
            .iter()
            .any(|ty| includes_object_keyword(db, *ty))
            && evidence.object_wide_casts == 0
            && (evidence.has_narrower_than_object
                || return_types
                    .iter()
                    .map(|ty| is_wider_than(db, annotation, *ty))
                    .collect::<Option<Vec<_>>>()?
                    .into_iter()
                    .any(|is_wider| is_wider))
    } else {
        try_all(
            return_types
                .iter()
                .map(|ty| is_wider_than(db, annotation, *ty)),
        )?
    };
    if !is_misleading {
        return None;
    }

    let suggestion = if evidence.has_any_const || evidence.prefer_inferred_suggestion {
        render_inferred(db, &return_types)
    } else {
        render_narrowed(db, annotation, &return_types)
            .ok()?
            .or_else(|| render_inferred(db, &return_types))
    };
    Some(MisleadingReturnType { suggestion })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interned_types::{
        InternedGenericTypeParameter, InternedLiteral, InternedObject, InternedTuple,
        InternedTypeInstance, InternedUnion, TupleElementType,
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
        );
        (1..distinct_types).fold(leaf, |base, _| {
            InternedClass::new(
                db,
                Box::default(),
                Some(TypeData::Class(base)),
                Box::default(),
                Box::default(),
                None,
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
            assert_eq!(
                check_misleading_return_type(
                    &db,
                    annotated,
                    vec![inferred],
                    ReturnTypeEvidence {
                        has_any_const: true,
                        ..ReturnTypeEvidence::default()
                    },
                    false,
                )
                .is_some(),
                steps <= MAX_RETURN_TYPE_STEPS,
                "diagnostic steps {steps}"
            );
        }
    }

    #[test]
    fn equality_and_class_shape_observe_step_limit() {
        let db = TestDb::default();

        for steps in [
            MAX_RETURN_TYPE_STEPS - 1,
            MAX_RETURN_TYPE_STEPS,
            MAX_RETURN_TYPE_STEPS + 1,
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

            assert_eq!(
                class_has_instance_shape(
                    &db,
                    extending_class_chain(&db, steps),
                    &mut FxHashSet::default(),
                    0,
                ),
                (steps <= MAX_RETURN_TYPE_DEPTH).then_some(false),
                "class shape steps {steps}"
            );
        }
    }

    #[test]
    fn generic_union_extra_variant_remains_misleading() {
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
        assert!(render_narrowed(&db, annotation, &[generic]).is_ok());
        assert!(
            check_misleading_return_type(
                &db,
                annotation,
                vec![generic],
                ReturnTypeEvidence::default(),
                false,
            )
            .is_some()
        );
    }
}
