use biome_rowan::Text;

use crate::{
    IndexedAccessType, Literal, MappedType, Object, TypeData, TypeMember, TypeMemberKind,
    TypeOperator, TypeOperatorType, TypeReference, TypeResolver, type_data::literal::StringLiteral,
};

/// Resolves a [`TypeReference`] to owned, concrete (non-`Reference`)
/// [`TypeData`]. Returns `None` when the reference cannot be resolved.
fn resolve_to_owned(reference: &TypeReference, resolver: &dyn TypeResolver) -> Option<TypeData> {
    // `resolve_and_get` already chases `TypeData::Reference` chains
    // internally with its own depth guard.
    resolver.resolve_and_get(reference).map(|r| r.to_data())
}

// #region keyof

/// Flattens `keyof T` to a union of string literal types when `T` is a
/// known interface or object type.
pub(super) fn flattened_keyof(
    op: &TypeOperatorType,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    if !matches!(op.operator, TypeOperator::Keyof) {
        return None;
    }

    let data = resolve_to_owned(&op.ty, resolver)?;
    keys_to_union(&data, resolver)
}

/// Collects member names from a type and returns them as a union of
/// string literal types.
fn keys_to_union(data: &TypeData, resolver: &mut dyn TypeResolver) -> Option<TypeData> {
    let members = type_members(data)?;
    let keys: Box<[TypeReference]> = members
        .iter()
        .filter_map(member_key_name)
        .map(|name| {
            let literal = TypeData::from(Literal::String(StringLiteral::from(name)));
            resolver.reference_to_owned_data(literal)
        })
        .collect();
    if keys.is_empty() {
        return Some(TypeData::NeverKeyword);
    }
    Some(TypeData::union_of(resolver, keys))
}

// #endregion

// #region indexed access

/// Flattens `T[K]` (indexed access type) to the resolved member type.
pub(super) fn flattened_indexed_access(
    access: &IndexedAccessType,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    let object_data = resolve_to_owned(&access.object_type, resolver)?;
    let index_data = resolve_to_owned(&access.index_type, resolver)?;

    resolve_indexed_access(&object_data, &index_data, resolver)
}

/// Resolves an indexed access given concrete object and index data.
fn resolve_indexed_access(
    object_data: &TypeData,
    index_data: &TypeData,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    // String literal index — direct member lookup.
    if let TypeData::Literal(literal) = index_data
        && let Literal::String(string_lit) = literal.as_ref()
    {
        let name = Text::new_owned(string_lit.as_str().into());
        return find_member_type_by_name(object_data, &name);
    }

    // Union index — look up each variant independently.
    if let TypeData::Union(union) = index_data {
        let variants: Vec<_> = union.types().to_vec();
        let mut types = Vec::new();
        for variant_ref in &variants {
            let Some(variant_data) = resolve_to_owned(variant_ref, resolver) else {
                continue;
            };
            // Each variant should be a string literal after resolution;
            // handle it inline rather than recursing.
            if let TypeData::Literal(lit) = &variant_data
                && let Literal::String(s) = lit.as_ref()
            {
                let name = Text::new_owned(s.as_str().into());
                if let Some(result) = find_member_type_by_name(object_data, &name) {
                    let ref_ = resolver.reference_to_owned_data(result);
                    types.push(ref_);
                }
            }
        }
        if types.is_empty() {
            return None;
        }
        return Some(TypeData::union_of(resolver, types.into_boxed_slice()));
    }

    None
}

/// Finds a member's type by name in an object or interface.
///
/// Expects concrete (non-reference) data — callers must resolve first.
fn find_member_type_by_name(data: &TypeData, name: &Text) -> Option<TypeData> {
    type_members(data)?
        .iter()
        .find(|m| member_has_name(m, name))
        .map(|m| TypeData::Reference(m.ty.clone()))
}

// #endregion

// #region mapped type

/// Flattens a mapped type `{ [K in KeysType]: ValueType }` to a concrete
/// object type when the keys can be statically resolved.
pub(super) fn flattened_mapped_type(
    mapped: &MappedType,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    let keys = resolve_keys(&mapped.keys_type, resolver)?;

    if keys.is_empty() {
        return Some(TypeData::from(Object {
            prototype: None,
            members: [].into(),
        }));
    }

    let mut members = Vec::new();
    for key in &keys {
        let final_keys = if let Some(name_type_ref) = &mapped.name_type {
            remap_key(key, &mapped.property_name, name_type_ref, resolver)?
        } else {
            vec![key.clone()]
        };
        for remapped_key in final_keys {
            members.push(TypeMember {
                kind: TypeMemberKind::Named(remapped_key),
                ty: mapped.value_type.clone(),
            });
        }
    }

    Some(TypeData::from(Object {
        prototype: None,
        members: members.into_boxed_slice(),
    }))
}

/// Resolves a keys_type reference to a list of string key names.
fn resolve_keys(keys_ref: &TypeReference, resolver: &mut dyn TypeResolver) -> Option<Vec<Text>> {
    let data = resolve_to_owned(keys_ref, resolver)?;
    resolve_keys_from_data(&data, resolver)
}

/// Extracts string key names from concrete type data.
fn resolve_keys_from_data(data: &TypeData, resolver: &mut dyn TypeResolver) -> Option<Vec<Text>> {
    match data {
        // `keyof T` — resolve T, then extract its member names.
        TypeData::TypeOperator(op) if matches!(op.operator, TypeOperator::Keyof) => {
            let resolved = resolve_to_owned(&op.ty, resolver)?;
            extract_key_names(&resolved)
        }
        // Union of string literals.
        TypeData::Union(union) => {
            let variants: Vec<_> = union.types().to_vec();
            let mut keys = Vec::new();
            for variant_ref in &variants {
                let variant = resolve_to_owned(variant_ref, resolver)?;
                if let TypeData::Literal(lit) = &variant
                    && let Literal::String(s) = lit.as_ref()
                {
                    keys.push(Text::new_owned(s.as_str().into()));
                } else {
                    return None;
                }
            }
            Some(keys)
        }
        // Single string literal.
        TypeData::Literal(lit) => {
            if let Literal::String(s) = lit.as_ref() {
                Some(vec![Text::new_owned(s.as_str().into())])
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Extracts key names from an interface or object.
fn extract_key_names(data: &TypeData) -> Option<Vec<Text>> {
    Some(
        type_members(data)?
            .iter()
            .filter_map(member_key_name)
            .collect(),
    )
}

/// Flattens a generic mapped type by directly evaluating it with
/// generic→concrete substitutions applied during resolution.
///
/// Instead of producing an intermediate `MappedType` with substituted
/// references (which would require registering new types), this function
/// resolves the keys and value types inline, applying the substitutions
/// at each resolution step.
pub(crate) fn flattened_mapped_type_with_substitutions(
    mapped: &MappedType,
    substitutions: &[(TypeReference, TypeReference)],
    resolver: &dyn TypeResolver,
) -> Option<TypeData> {
    let keys = resolve_keys_with_subs(&mapped.keys_type, substitutions, resolver)?;

    if keys.is_empty() {
        return Some(TypeData::from(Object {
            prototype: None,
            members: [].into(),
        }));
    }

    let mut members = Vec::new();
    for key in &keys {
        let final_keys = if let Some(name_type_ref) = &mapped.name_type {
            remap_key(key, &mapped.property_name, name_type_ref, resolver)?
        } else {
            vec![key.clone()]
        };
        for remapped_key in final_keys {
            members.push(TypeMember {
                kind: TypeMemberKind::Named(remapped_key),
                ty: mapped.value_type.clone(),
            });
        }
    }

    Some(TypeData::from(Object {
        prototype: None,
        members: members.into_boxed_slice(),
    }))
}

/// Resolves a reference, applying generic substitutions before resolution.
fn resolve_with_subs(
    reference: &TypeReference,
    substitutions: &[(TypeReference, TypeReference)],
    resolver: &dyn TypeResolver,
) -> Option<TypeData> {
    // Check if the reference itself is a generic param (direct or via InstanceOf wrapper).
    if let Some(concrete_ref) = find_substitution(reference, substitutions, resolver) {
        return resolve_to_owned(&concrete_ref, resolver);
    }

    // Resolve normally, then check if the result contains substitutable refs.
    let data = resolve_to_owned(reference, resolver)?;
    Some(apply_subs_to_data(data, substitutions, resolver))
}

/// Applies substitutions to a resolved TypeData. Only handles the types
/// that appear in mapped type key/value positions.
fn apply_subs_to_data(
    data: TypeData,
    substitutions: &[(TypeReference, TypeReference)],
    resolver: &dyn TypeResolver,
) -> TypeData {
    match &data {
        TypeData::TypeOperator(op) if matches!(op.operator, TypeOperator::Keyof) => {
            // keyof T — substitute T if it's a generic param (direct or via InstanceOf wrapper)
            if let Some(concrete_ref) = find_substitution(&op.ty, substitutions, resolver) {
                return TypeData::TypeOperator(Box::new(TypeOperatorType {
                    operator: TypeOperator::Keyof,
                    ty: concrete_ref,
                }));
            }
            data
        }
        _ => data,
    }
}

/// Resolves keys from a reference, applying substitutions.
fn resolve_keys_with_subs(
    keys_ref: &TypeReference,
    substitutions: &[(TypeReference, TypeReference)],
    resolver: &dyn TypeResolver,
) -> Option<Vec<Text>> {
    let data = resolve_with_subs(keys_ref, substitutions, resolver)?;

    match &data {
        // `keyof T` where T has been substituted — resolve T, extract member names.
        TypeData::TypeOperator(op) if matches!(op.operator, TypeOperator::Keyof) => {
            let resolved = resolve_with_subs(&op.ty, substitutions, resolver)?;
            extract_key_names(&resolved)
        }
        // Union of string literals.
        TypeData::Union(union) => {
            let variants: Vec<_> = union.types().to_vec();
            let mut keys = Vec::new();
            for variant_ref in &variants {
                let variant = resolve_with_subs(variant_ref, substitutions, resolver)?;
                if let TypeData::Literal(lit) = &variant
                    && let Literal::String(s) = lit.as_ref()
                {
                    keys.push(Text::new_owned(s.as_str().into()));
                } else {
                    return None;
                }
            }
            Some(keys)
        }
        // Single string literal.
        TypeData::Literal(lit) => {
            if let Literal::String(s) = lit.as_ref() {
                Some(vec![Text::new_owned(s.as_str().into())])
            } else {
                None
            }
        }
        _ => None,
    }
}

// #endregion

// #region key remapping

/// Remaps a key through a name_type expression.
///
/// Returns `None` if remapping fails, or `Some(vec![...])` with the
/// remapped key names. An empty vec means the key is filtered out
/// (e.g., mapped to `never`).
fn remap_key(
    key: &Text,
    property_name: &Text,
    name_type_ref: &TypeReference,
    resolver: &dyn TypeResolver,
) -> Option<Vec<Text>> {
    let data = resolve_to_owned(name_type_ref, resolver)?;
    remap_key_impl(key, property_name, &data, resolver)
}

/// Remaps a key given concrete name_type data.
fn remap_key_impl(
    key: &Text,
    property_name: &Text,
    name_type_data: &TypeData,
    resolver: &dyn TypeResolver,
) -> Option<Vec<Text>> {
    match name_type_data {
        TypeData::Literal(lit) => match lit.as_ref() {
            Literal::Template(template) => {
                let substituted = substitute_template(template.text(), property_name, key);
                Some(vec![Text::new_owned(substituted.into())])
            }
            Literal::String(s) => Some(vec![Text::new_owned(s.as_str().into())]),
            _ => None,
        },
        // Conditional type approximation: conditionals are stored as
        // unions of (true_branch | false_branch). Iterate variants,
        // skipping `never`.
        TypeData::Union(union) => {
            let variants: Vec<_> = union.types().to_vec();
            let mut results = Vec::new();
            for variant_ref in &variants {
                let Some(variant) = resolve_to_owned(variant_ref, resolver) else {
                    continue;
                };
                if matches!(&variant, TypeData::NeverKeyword) {
                    continue;
                }
                // Handle the variant directly — it must be a literal or
                // another terminal form at this point.
                if let Some(remapped) = remap_key_impl(key, property_name, &variant, resolver) {
                    results.extend(remapped);
                }
            }
            if results.is_empty() {
                None
            } else {
                Some(results)
            }
        }
        TypeData::NeverKeyword => Some(vec![]),
        _ => None,
    }
}

// #endregion

// #region substitution helpers

/// Checks if a reference matches a generic parameter in the substitution list.
///
/// Handles both direct matches (`reference == generic_ref`) and indirect
/// matches where `reference` resolves to `InstanceOf(generic_ref, [])` —
/// which happens because `T` in type positions like `keyof T` is parsed
/// as a `TsReferenceType`, producing `InstanceOf(Qualifier("T"), [])`.
fn find_substitution(
    reference: &TypeReference,
    substitutions: &[(TypeReference, TypeReference)],
    resolver: &dyn TypeResolver,
) -> Option<TypeReference> {
    // Direct match.
    for (generic_ref, concrete_ref) in substitutions {
        if reference == generic_ref {
            return Some(concrete_ref.clone());
        }
    }

    // Indirect match: reference resolves to InstanceOf(generic_ref, []).
    // This happens because `T` in type positions like `keyof T` is parsed
    // as a `TsReferenceType`, producing `InstanceOf(Qualifier("T"), [])`.
    let resolved = resolver.resolve_and_get(reference)?;
    if let TypeData::InstanceOf(instance) = resolved.as_raw_data()
        && instance.type_parameters.is_empty()
    {
        for (generic_ref, concrete_ref) in substitutions {
            if &instance.ty == generic_ref {
                return Some(concrete_ref.clone());
            }
        }
    }

    None
}

// #endregion

// #region helpers

/// Returns the member slice for types that have members (interface, object),
/// or `None` for everything else.
fn type_members(data: &TypeData) -> Option<&[TypeMember]> {
    match data {
        TypeData::Interface(i) => Some(&i.members),
        TypeData::Object(o) => Some(&o.members),
        _ => None,
    }
}

/// Extracts the string name from a TypeMember if it has one.
fn member_key_name(member: &TypeMember) -> Option<Text> {
    match &member.kind {
        TypeMemberKind::Named(name)
        | TypeMemberKind::Getter(name)
        | TypeMemberKind::NamedStatic(name) => Some(name.clone()),
        _ => None,
    }
}

fn member_has_name(member: &TypeMember, name: &Text) -> bool {
    match &member.kind {
        TypeMemberKind::Named(n) | TypeMemberKind::Getter(n) | TypeMemberKind::NamedStatic(n) => {
            n == name
        }
        _ => false,
    }
}

/// Substitutes a template literal variable with a concrete value.
///
/// Given a template like `` `get_${K}` ``, property_name `K`, and key
/// `thing`, produces `get_thing`.
fn substitute_template(template: &str, property_name: &Text, key: &Text) -> String {
    let inner = template.trim_start_matches('`').trim_end_matches('`');
    let pattern = format!("${{{}}}", property_name.text());
    inner.replace(&pattern, key.text())
}

// #endregion
