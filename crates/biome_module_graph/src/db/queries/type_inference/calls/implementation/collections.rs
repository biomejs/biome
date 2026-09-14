use super::*;
use biome_js_type_info::interned_types::TypeMemberKind;

const MAX_COLLECTION_STEPS: usize = 64;

#[derive(Clone, Copy)]
pub(super) enum CollectionKind {
    ArrayLike,
    Iterable,
}

impl CollectionKind {
    pub(super) fn for_target<'db>(
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> Option<Self> {
        let InferredTypeData::Class(class) =
            resolve_local_type_on_demand(db, ty).expand_canonical_global(db)
        else {
            return None;
        };
        if !class.is_builtin(db) {
            return None;
        }
        match class.name(db).as_ref()?.text() {
            "ArrayLike" => Some(Self::ArrayLike),
            "Iterable" => Some(Self::Iterable),
            _ => None,
        }
    }
}

/// Returns the element type of an iterable or array-like source. An array-like
/// object with only a length has no evidence for its element type and yields
/// `Unknown`. Exhausting the traversal budget also yields `Unknown`; `None`
/// means the source does not expose the requested collection protocol.
pub(super) fn element_type<'db>(
    db: &'db dyn ModuleDb,
    source: InferredTypeData<'db>,
    kind: CollectionKind,
) -> Option<InferredTypeData<'db>> {
    let mut budget = MAX_COLLECTION_STEPS;
    let result = element_type_with_budget(db, source, kind, &mut budget);
    if budget == 0 {
        Some(InferredTypeData::Unknown)
    } else {
        result
    }
}

fn element_type_with_budget<'db>(
    db: &'db dyn ModuleDb,
    source: InferredTypeData<'db>,
    kind: CollectionKind,
    budget: &mut usize,
) -> Option<InferredTypeData<'db>> {
    *budget = budget.checked_sub(1)?;
    let source = resolve_local_type_on_demand(db, source).expand_structural_global(db);
    if source.is_indeterminate()
        || matches!(
            source,
            InferredTypeData::ObjectKeyword
                | InferredTypeData::Conditional
                | InferredTypeData::Global
        )
    {
        return Some(InferredTypeData::Unknown);
    }
    if let InferredTypeData::Object(object) = source
        && object.has_unknown_members(db)
    {
        return Some(InferredTypeData::Unknown);
    }
    if matches!(kind, CollectionKind::Iterable)
        && let Some(method) = computed_iterator_method(db, source, budget)
    {
        return Some(
            iterator_element_type(db, method, budget).unwrap_or(InferredTypeData::Unknown),
        );
    }
    if matches!(kind, CollectionKind::ArrayLike)
        && matches!(source, InferredTypeData::Intersection(_))
        && let Some(length) = find_member_type_on_demand(db, source, "length")
        && ArgumentTypeCompatibility::new(InferredTypeData::Number, length).is_satisfied(db)
        && let Some(element) = numeric_element_type(db, source, budget)
    {
        return Some(element);
    }
    let class_base = match source {
        InferredTypeData::Class(class) => class.extends(db),
        _ => None,
    };
    let bases = match source {
        InferredTypeData::Interface(interface) => interface.extends(db).as_ref(),
        InferredTypeData::Class(_) => class_base.as_slice(),
        InferredTypeData::Intersection(intersection) => intersection.types(db).as_ref(),
        _ => &[],
    };
    for base in bases {
        if let Some(element) = element_type_with_budget(db, *base, kind, budget) {
            return if matches!(kind, CollectionKind::ArrayLike) {
                numeric_element_type(db, source, budget).or(Some(element))
            } else {
                Some(element)
            };
        }
    }
    match source {
        InferredTypeData::AnyKeyword => Some(source),
        InferredTypeData::Unknown | InferredTypeData::UnknownKeyword => {
            Some(InferredTypeData::Unknown)
        }
        InferredTypeData::String => Some(InferredTypeData::String),
        InferredTypeData::Literal(literal)
            if matches!(literal.literal(db), InferredLiteral::String(_)) =>
        {
            Some(InferredTypeData::String)
        }
        InferredTypeData::Tuple(tuple) => {
            let mut elements = Vec::new();
            for element in tuple.elements(db) {
                *budget = budget.checked_sub(1)?;
                elements.push(if element.is_rest {
                    element_type_with_budget(db, element.ty, kind, budget)?
                } else if element.is_optional {
                    InferredTypeData::union_from_types(
                        db,
                        vec![element.ty, InferredTypeData::Undefined],
                    )
                } else {
                    element.ty
                });
            }
            Some(InferredTypeData::union_from_types(db, elements))
        }
        InferredTypeData::Union(union) => {
            let mut elements = Vec::new();
            for source in union.types(db) {
                elements.push(element_type_with_budget(db, *source, kind, budget)?);
            }
            Some(InferredTypeData::union_from_types(db, elements))
        }
        InferredTypeData::InstanceOf(instance) => {
            let target = resolve_local_type_on_demand(db, instance.ty(db));
            let arguments = instance.type_parameters(db);
            let name = builtin_collection_name(db, target);
            let is_readonly_array = name == Some("ReadonlyArray");
            let is_collection = target.is_array_class(db)
                || is_readonly_array
                || matches!(
                    (kind, CollectionKind::for_target(db, target)),
                    (CollectionKind::ArrayLike, Some(CollectionKind::ArrayLike))
                        | (CollectionKind::Iterable, Some(CollectionKind::Iterable))
                )
                || (matches!(kind, CollectionKind::Iterable)
                    && (target == resolve_local_type_on_demand(db, InferredTypeData::set_class())
                        || matches!(name, Some("ReadonlySet" | "IterableIterator" | "Generator"))));
            if is_collection {
                return Some(
                    arguments
                        .first()
                        .copied()
                        .unwrap_or(InferredTypeData::Unknown),
                );
            }
            if matches!(kind, CollectionKind::Iterable)
                && (target == resolve_local_type_on_demand(db, InferredTypeData::map_class())
                    || name == Some("ReadonlyMap"))
            {
                return Some(InferredTypeData::Tuple(InferredTuple::new(
                    db,
                    (0..2)
                        .map(|index| InferredTupleElementType {
                            ty: arguments
                                .get(index)
                                .copied()
                                .unwrap_or(InferredTypeData::Unknown),
                            name: None,
                            is_optional: false,
                            is_rest: false,
                        })
                        .collect::<Box<[_]>>(),
                )));
            }
            let substitutions = substitutions_for_instance(db, target, arguments, &[]);
            let target = apply_substitutions_to_root_body(db, target, &substitutions);
            element_type_with_budget(db, target, kind, budget)
        }
        InferredTypeData::Generic(parameter) => parameter
            .constraint(db)
            .and_then(|constraint| element_type_with_budget(db, constraint, kind, budget))
            .or(Some(InferredTypeData::Unknown)),
        InferredTypeData::TypeOperator(operator)
            if operator.operator(db) == biome_js_type_info::TypeOperator::Readonly =>
        {
            element_type_with_budget(db, operator.ty(db), kind, budget)
        }
        InferredTypeData::TypeOperator(_) => Some(InferredTypeData::Unknown),
        InferredTypeData::TypeofType(inner) => {
            element_type_with_budget(db, inner.ty(db), kind, budget)
        }
        InferredTypeData::TypeofValue(inner) => {
            element_type_with_budget(db, inner.ty(db), kind, budget)
        }
        _ if matches!(kind, CollectionKind::ArrayLike) => {
            let length = find_member_type_on_demand(db, source, "length")?;
            if !ArgumentTypeCompatibility::new(InferredTypeData::Number, length).is_satisfied(db) {
                return None;
            }
            numeric_element_type(db, source, budget).or(Some(InferredTypeData::Unknown))
        }
        _ => None,
    }
}

fn iterator_element_type<'db>(
    db: &'db dyn ModuleDb,
    iterator: InferredTypeData<'db>,
    budget: &mut usize,
) -> Option<InferredTypeData<'db>> {
    let iterator = resolve_callable_function(db, iterator)?;
    let ReturnType::Type(iterator) = iterator.return_type(db) else {
        return None;
    };
    iterator_type_element(db, *iterator, budget)
}

fn iterator_type_element<'db>(
    db: &'db dyn ModuleDb,
    iterator: InferredTypeData<'db>,
    budget: &mut usize,
) -> Option<InferredTypeData<'db>> {
    *budget = budget.checked_sub(1)?;
    let iterator = resolve_local_type_on_demand(db, iterator);
    if let InferredTypeData::InstanceOf(instance) = iterator {
        if matches!(
            builtin_collection_name(db, instance.ty(db)),
            Some("Iterator" | "IterableIterator" | "Generator")
        ) {
            return Some(
                instance
                    .type_parameters(db)
                    .first()
                    .copied()
                    .unwrap_or(InferredTypeData::Unknown),
            );
        }
        let target = resolve_local_type_on_demand(db, instance.ty(db));
        let substitutions =
            substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
        let target = apply_substitutions_to_root_body(db, target, &substitutions);
        return iterator_type_element(db, target, budget);
    }
    let next = find_member_type_on_demand(db, iterator, "next")?;
    let next = resolve_callable_function(db, next)?;
    let ReturnType::Type(result) = next.return_type(db) else {
        return None;
    };
    iterator_result_element(db, *result, budget)
}

fn computed_iterator_method<'db>(
    db: &'db dyn ModuleDb,
    source: InferredTypeData<'db>,
    budget: &mut usize,
) -> Option<InferredTypeData<'db>> {
    *budget = budget.checked_sub(1)?;
    let source = resolve_local_type_on_demand(db, source);
    let (members, bases) = match source {
        InferredTypeData::Class(class) => (
            class.members(db),
            class.extends(db).into_iter().collect::<Vec<_>>(),
        ),
        InferredTypeData::Interface(interface) => {
            (interface.members(db), interface.extends(db).to_vec())
        }
        InferredTypeData::Object(object) => (
            object.members(db),
            object.prototype(db).into_iter().collect(),
        ),
        InferredTypeData::Literal(literal) => {
            let InferredLiteral::Object(members) = literal.literal(db) else {
                return None;
            };
            (members, Vec::new())
        }
        _ => return None,
    };
    for member in members {
        *budget = budget.checked_sub(1)?;
        if !member.kind.is_static() && member.kind.computed_name() == Some("Symbol.iterator") {
            return Some(member.ty);
        }
    }
    for base in bases {
        if let Some(method) = computed_iterator_method(db, base, budget) {
            return Some(method);
        }
    }
    None
}

fn iterator_result_element<'db>(
    db: &'db dyn ModuleDb,
    result: InferredTypeData<'db>,
    budget: &mut usize,
) -> Option<InferredTypeData<'db>> {
    *budget = budget.checked_sub(1)?;
    let result = resolve_local_type_on_demand(db, result);
    if let InferredTypeData::InstanceOf(instance) = result {
        if builtin_collection_name(db, instance.ty(db)) == Some("IteratorResult") {
            return Some(
                instance
                    .type_parameters(db)
                    .first()
                    .copied()
                    .unwrap_or(InferredTypeData::Unknown),
            );
        }
        let target = resolve_local_type_on_demand(db, instance.ty(db));
        let substitutions =
            substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
        let target = apply_substitutions_to_root_body(db, target, &substitutions);
        return iterator_result_element(db, target, budget);
    }
    if let InferredTypeData::Union(union) = result {
        let mut elements = Vec::new();
        for variant in union.types(db) {
            elements.push(iterator_result_element(db, *variant, budget)?);
        }
        return Some(InferredTypeData::union_from_types(db, elements));
    }
    if let Some(InferredTypeData::Literal(done)) = find_member_type_on_demand(db, result, "done")
        && matches!(done.literal(db), InferredLiteral::Boolean(value) if value.as_bool())
    {
        return Some(InferredTypeData::NeverKeyword);
    }
    find_member_type_on_demand(db, result, "value")
}

fn numeric_element_type<'db>(
    db: &'db dyn ModuleDb,
    source: InferredTypeData<'db>,
    budget: &mut usize,
) -> Option<InferredTypeData<'db>> {
    let mut elements = Vec::new();
    collect_numeric_elements(db, source, budget, &mut elements)
        .map(|signature| signature.ty)
        .or_else(|| {
            (!elements.is_empty()).then(|| {
                InferredTypeData::union_from_types(
                    db,
                    elements.into_iter().map(|(_, ty)| ty).collect(),
                )
            })
        })
}

struct NumericIndexType<'db> {
    ty: InferredTypeData<'db>,
    is_number_key: bool,
}

fn collect_numeric_elements<'db>(
    db: &'db dyn ModuleDb,
    source: InferredTypeData<'db>,
    budget: &mut usize,
    elements: &mut Vec<(u32, InferredTypeData<'db>)>,
) -> Option<NumericIndexType<'db>> {
    *budget = budget.checked_sub(1)?;
    let source = resolve_local_type_on_demand(db, source);
    if let InferredTypeData::InstanceOf(instance) = source {
        let target = resolve_local_type_on_demand(db, instance.ty(db));
        let substitutions =
            substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
        let target = apply_substitutions_to_root_body(db, target, &substitutions);
        return collect_numeric_elements(db, target, budget, elements);
    }
    if let InferredTypeData::Intersection(intersection) = source {
        let mut signatures = Vec::new();
        for part in intersection.types(db) {
            let mut part_elements = Vec::new();
            if let Some(signature) = collect_numeric_elements(db, *part, budget, &mut part_elements)
            {
                signatures.push(signature);
            }
            for (index, ty) in part_elements {
                if let Some((_, previous)) = elements.iter_mut().find(|(key, _)| *key == index) {
                    *previous = InferredTypeData::intersection_from_types(db, vec![*previous, ty]);
                } else {
                    elements.push((index, ty));
                }
            }
        }
        return (!signatures.is_empty()).then(|| NumericIndexType {
            is_number_key: signatures.iter().any(|signature| signature.is_number_key),
            ty: InferredTypeData::intersection_from_types(
                db,
                signatures
                    .into_iter()
                    .map(|signature| signature.ty)
                    .collect(),
            ),
        });
    }
    let members = match source {
        InferredTypeData::Class(class) => class.members(db),
        InferredTypeData::Interface(interface) => interface.members(db),
        InferredTypeData::Object(object) => object.members(db),
        InferredTypeData::Literal(literal) => {
            let InferredLiteral::Object(object) = literal.literal(db) else {
                return None;
            };
            object
        }
        _ => return None,
    };
    let mut string_signature = None;
    for member in members {
        *budget = budget.checked_sub(1)?;
        if let Some(key) = member.kind.index_signature_type() {
            let key = resolve_local_type_on_demand(db, key).expand_structural_global(db);
            if key == InferredTypeData::Number {
                return Some(NumericIndexType {
                    ty: member.ty,
                    is_number_key: true,
                });
            }
            if key == InferredTypeData::String && string_signature.is_none() {
                string_signature = Some(NumericIndexType {
                    ty: member.ty,
                    is_number_key: false,
                });
            }
        }
        if !member.kind.is_static()
            && let Some(name) = member.kind.name()
            && let Ok(index) = name.text().parse::<u32>()
            && index.to_string() == name.text()
            && !elements.iter().any(|(key, _)| *key == index)
        {
            let ty = if matches!(
                member.kind,
                TypeMemberKind::Getter(_) | TypeMemberKind::ConstAssertedGetter(_)
            ) {
                find_member_type_on_demand(db, source, name.text())
                    .unwrap_or(InferredTypeData::Unknown)
            } else {
                member.ty
            };
            let ty = if member.kind.is_optional() {
                InferredTypeData::union_from_types(db, vec![ty, InferredTypeData::Undefined])
            } else {
                ty
            };
            elements.push((index, ty));
        }
    }
    let class_base = match source {
        InferredTypeData::Class(class) => class.extends(db),
        _ => None,
    };
    let bases = match source {
        InferredTypeData::Interface(interface) => interface.extends(db).as_ref(),
        InferredTypeData::Class(_) => class_base.as_slice(),
        _ => &[],
    };
    for base in bases {
        if let Some(element) = collect_numeric_elements(db, *base, budget, elements) {
            if element.is_number_key {
                return Some(element);
            }
            string_signature = string_signature.or(Some(element));
        }
    }
    string_signature
}

fn builtin_collection_name<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<&'db str> {
    let InferredTypeData::Class(class) =
        resolve_local_type_on_demand(db, ty).expand_canonical_global(db)
    else {
        return None;
    };
    class
        .is_builtin(db)
        .then_some(class.name(db).as_ref()?.text())
}
