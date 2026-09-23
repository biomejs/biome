//! This is a generated file. Don't modify it by hand! Run 'just gen-global-types' to re-generate the file.

// Generated from microsoft/TypeScript v6.0.3 (git commit 050880ce59e30b356b686bd3144efe24f875ebc8).

/// Function identities allocated after the fixed predefined manifest.
pub(crate) mod function_ids {
    pub(crate) const EVAL_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len(),
        ));
    pub(crate) const PARSE_INT_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len() + 1,
        ));
    pub(crate) const PARSE_FLOAT_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len() + 2,
        ));
    pub(crate) const IS_NA_N_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len() + 3,
        ));
    pub(crate) const IS_FINITE_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len() + 4,
        ));
    pub(crate) const DECODE_URI_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len() + 5,
        ));
    pub(crate) const DECODE_URI_COMPONENT_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len() + 6,
        ));
    pub(crate) const ENCODE_URI_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len() + 7,
        ));
    pub(crate) const ENCODE_URI_COMPONENT_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len() + 8,
        ));
    pub(crate) const ESCAPE_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len() + 9,
        ));
    pub(crate) const UNESCAPE_ID_GLOBAL_TYPE_ID: crate::globals::GlobalTypeId =
        crate::globals::GlobalTypeId::new(crate::TypeId::new(
            crate::globals::PREDEFINED_ID_ROWS.len() + 10,
        ));
}

/// Predefined global IDs whose `TypeData` is supplied by this generated module.
pub(crate) const MIGRATED_PREDEFINED_IDS: &[crate::globals::GlobalTypeId] = &[
    crate::globals::ARRAY_ID_GLOBAL_TYPE_ID,
    crate::globals::ARRAY_FILTER_ID_GLOBAL_TYPE_ID,
    crate::globals::ARRAY_FOREACH_ID_GLOBAL_TYPE_ID,
    crate::globals::ARRAY_MAP_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_CONSTRUCTOR_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_CATCH_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_FINALLY_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_THEN_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_ALL_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_ALL_SETTLED_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_ANY_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_RACE_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_REJECT_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_RESOLVE_ID_GLOBAL_TYPE_ID,
    crate::globals::PROMISE_TRY_ID_GLOBAL_TYPE_ID,
    crate::globals::REGEXP_ID_GLOBAL_TYPE_ID,
    crate::globals::REGEXP_EXEC_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_DISPOSE_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID,
    crate::globals::DISPOSABLE_ID_GLOBAL_TYPE_ID,
    crate::globals::DISPOSABLE_DISPOSE_ID_GLOBAL_TYPE_ID,
    crate::globals::ASYNC_DISPOSABLE_ID_GLOBAL_TYPE_ID,
    crate::globals::ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID,
    crate::globals::DATE_ID_GLOBAL_TYPE_ID,
    crate::globals::MAP_ID_GLOBAL_TYPE_ID,
    crate::globals::SET_ID_GLOBAL_TYPE_ID,
    crate::globals::WEAK_MAP_ID_GLOBAL_TYPE_ID,
    crate::globals::ERROR_ID_GLOBAL_TYPE_ID,
    crate::globals::ERROR_CONSTRUCTOR_ID_GLOBAL_TYPE_ID,
    crate::globals::ERROR_CALL_ID_GLOBAL_TYPE_ID,
    crate::globals::MATH_ID_GLOBAL_TYPE_ID,
    crate::globals::ITERATOR_YIELD_RESULT_ID_GLOBAL_TYPE_ID,
    crate::globals::ITERATOR_RETURN_RESULT_ID_GLOBAL_TYPE_ID,
    crate::globals::ITERATOR_RESULT_ID_GLOBAL_TYPE_ID,
    crate::globals::ITERATOR_ID_GLOBAL_TYPE_ID,
    crate::globals::ITERABLE_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_ITERATOR_ID_GLOBAL_TYPE_ID,
    crate::globals::REGEXP_EXEC_ARRAY_ID_GLOBAL_TYPE_ID,
    crate::globals::ARRAY_LIKE_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_TO_STRING_TAG_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_TO_PRIMITIVE_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_MATCH_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_REPLACE_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_SEARCH_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_SPLIT_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_SPECIES_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_HAS_INSTANCE_ID_GLOBAL_TYPE_ID,
    crate::globals::SYMBOL_UNSCOPABLES_ID_GLOBAL_TYPE_ID,
    crate::globals::REGEXP_MATCH_ARRAY_ID_GLOBAL_TYPE_ID,
    crate::globals::ITERATOR_OBJECT_ID_GLOBAL_TYPE_ID,
    crate::globals::MAP_ITERATOR_ID_GLOBAL_TYPE_ID,
    crate::globals::SET_ITERATOR_ID_GLOBAL_TYPE_ID,
    crate::globals::BUILTIN_ITERATOR_RETURN_ID_GLOBAL_TYPE_ID,
    crate::globals::INTL_ID_GLOBAL_TYPE_ID,
    crate::globals::EVAL_ID_GLOBAL_TYPE_ID,
    crate::globals::PARSE_INT_ID_GLOBAL_TYPE_ID,
    crate::globals::PARSE_FLOAT_ID_GLOBAL_TYPE_ID,
    crate::globals::IS_NA_N_ID_GLOBAL_TYPE_ID,
    crate::globals::IS_FINITE_ID_GLOBAL_TYPE_ID,
    crate::globals::DECODE_URI_ID_GLOBAL_TYPE_ID,
    crate::globals::DECODE_URI_COMPONENT_ID_GLOBAL_TYPE_ID,
    crate::globals::ENCODE_URI_ID_GLOBAL_TYPE_ID,
    crate::globals::ENCODE_URI_COMPONENT_ID_GLOBAL_TYPE_ID,
    crate::globals::ESCAPE_ID_GLOBAL_TYPE_ID,
    crate::globals::UNESCAPE_ID_GLOBAL_TYPE_ID,
];

/// Type-only declaration names and their global identities.
pub(crate) const DECLARATION_GLOBALS: &[(&str, crate::RawTypeId)] = &[
    (
        "IteratorYieldResult",
        crate::globals::GLOBAL_ITERATOR_YIELD_RESULT_ID,
    ),
    (
        "IteratorReturnResult",
        crate::globals::GLOBAL_ITERATOR_RETURN_RESULT_ID,
    ),
    ("IteratorResult", crate::globals::GLOBAL_ITERATOR_RESULT_ID),
    ("Iterator", crate::globals::GLOBAL_ITERATOR_ID),
    ("Iterable", crate::globals::GLOBAL_ITERABLE_ID),
    (
        "RegExpExecArray",
        crate::globals::GLOBAL_REGEXP_EXEC_ARRAY_ID,
    ),
    ("ArrayLike", crate::globals::GLOBAL_ARRAY_LIKE_ID),
    (
        "RegExpMatchArray",
        crate::globals::GLOBAL_REGEXP_MATCH_ARRAY_ID,
    ),
    ("IteratorObject", crate::globals::GLOBAL_ITERATOR_OBJECT_ID),
    ("MapIterator", crate::globals::GLOBAL_MAP_ITERATOR_ID),
    ("SetIterator", crate::globals::GLOBAL_SET_ITERATOR_ID),
    (
        "BuiltinIteratorReturn",
        crate::globals::GLOBAL_BUILTIN_ITERATOR_RETURN_ID,
    ),
    ("Disposable", crate::globals::GLOBAL_DISPOSABLE_ID),
];

/// Value declaration names and their global identities.
pub(crate) const VALUE_GLOBALS: &[(&str, crate::globals::GlobalTypeId)] = &[
    ("eval", crate::globals::EVAL_ID_GLOBAL_TYPE_ID),
    ("parseInt", crate::globals::PARSE_INT_ID_GLOBAL_TYPE_ID),
    ("parseFloat", crate::globals::PARSE_FLOAT_ID_GLOBAL_TYPE_ID),
    ("isNaN", crate::globals::IS_NA_N_ID_GLOBAL_TYPE_ID),
    ("isFinite", crate::globals::IS_FINITE_ID_GLOBAL_TYPE_ID),
    ("decodeURI", crate::globals::DECODE_URI_ID_GLOBAL_TYPE_ID),
    (
        "decodeURIComponent",
        crate::globals::DECODE_URI_COMPONENT_ID_GLOBAL_TYPE_ID,
    ),
    ("encodeURI", crate::globals::ENCODE_URI_ID_GLOBAL_TYPE_ID),
    (
        "encodeURIComponent",
        crate::globals::ENCODE_URI_COMPONENT_ID_GLOBAL_TYPE_ID,
    ),
    ("escape", crate::globals::ESCAPE_ID_GLOBAL_TYPE_ID),
    ("unescape", crate::globals::UNESCAPE_ID_GLOBAL_TYPE_ID),
];

/// Registers all generated global type data into the resolver builder.
pub(crate) fn set_generated_global_type_data(
    builder: &mut crate::globals_builder::GlobalsResolverBuilder,
) {
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Array")),
        type_parameters: Box::new([crate::globals::GLOBAL_T_ID.into()]),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("filter")),
                ty: crate::globals::GLOBAL_ARRAY_FILTER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("forEach")),
                ty: crate::globals::GLOBAL_ARRAY_FOREACH_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("map")),
                ty: crate::globals::GLOBAL_ARRAY_MAP_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("length")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("from")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedStatic(
                    crate::globals::GLOBAL_SYMBOL_SPECIES_ID.into(),
                ),
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::ARRAY_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Array.prototype.filter")),
        parameters: Box::new([crate::FunctionParameter::Pattern(
            crate::PatternFunctionParameter {
                bindings: Box::default(),
                ty: crate::globals::GLOBAL_CONDITIONAL_CALLBACK_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_ARRAY_T_ID.into()),
    }));
    builder.set_type_data(crate::globals::ARRAY_FILTER_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Array.prototype.forEach")),
        parameters: Box::new([crate::FunctionParameter::Pattern(
            crate::PatternFunctionParameter {
                bindings: Box::default(),
                ty: crate::globals::GLOBAL_VOID_CALLBACK_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
    }));
    builder.set_type_data(crate::globals::ARRAY_FOREACH_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::new([crate::globals::GLOBAL_U_ID.into()]),
        name: Some(biome_rowan::Text::new_static("Array.prototype.map")),
        parameters: Box::new([crate::FunctionParameter::Pattern(
            crate::PatternFunctionParameter {
                bindings: Box::default(),
                ty: crate::globals::GLOBAL_MAP_CALLBACK_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_ARRAY_U_ID.into()),
    }));
    builder.set_type_data(crate::globals::ARRAY_MAP_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Promise")),
        type_parameters: Box::new([crate::globals::GLOBAL_T_ID.into()]),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::globals::GLOBAL_PROMISE_CONSTRUCTOR_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("catch")),
                ty: crate::globals::GLOBAL_PROMISE_CATCH_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("finally")),
                ty: crate::globals::GLOBAL_PROMISE_FINALLY_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("then")),
                ty: crate::globals::GLOBAL_PROMISE_THEN_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("all")),
                ty: crate::globals::GLOBAL_PROMISE_ALL_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                    "allSettled",
                )),
                ty: crate::globals::GLOBAL_PROMISE_ALL_SETTLED_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("any")),
                ty: crate::globals::GLOBAL_PROMISE_ANY_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("race")),
                ty: crate::globals::GLOBAL_PROMISE_RACE_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("reject")),
                ty: crate::globals::GLOBAL_PROMISE_REJECT_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("resolve")),
                ty: crate::globals::GLOBAL_PROMISE_RESOLVE_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("try")),
                ty: crate::globals::GLOBAL_PROMISE_TRY_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::PROMISE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.constructor")),
        parameters: Box::new([crate::FunctionParameter::Pattern(
            crate::PatternFunctionParameter {
                bindings: Box::default(),
                ty: crate::globals::GLOBAL_VOID_CALLBACK_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_CONSTRUCTOR_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.prototype.catch")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_CATCH_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.prototype.finally")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_FINALLY_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.prototype.then")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_THEN_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.all")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_ALL_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.allSettled")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_ALL_SETTLED_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.any")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_ANY_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.race")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_RACE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.reject")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_REJECT_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.resolve")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_RESOLVE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Promise.try")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(crate::globals::PROMISE_TRY_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("RegExp")),
        type_parameters: Box::default(),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("exec")),
                ty: crate::globals::GLOBAL_REGEXP_EXEC_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("test")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("source")),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("global")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("ignoreCase")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("multiline")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("lastIndex")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("compile")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("flags")),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("sticky")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("unicode")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedValue(
                    crate::globals::GLOBAL_SYMBOL_MATCH_ID.into(),
                ),
                ty: crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedValue(
                    crate::globals::GLOBAL_SYMBOL_REPLACE_ID.into(),
                ),
                ty: crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedValue(
                    crate::globals::GLOBAL_SYMBOL_SEARCH_ID.into(),
                ),
                ty: crate::RawTypeId::Local(crate::TypeId::new(14)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedValue(
                    crate::globals::GLOBAL_SYMBOL_SPLIT_ID.into(),
                ),
                ty: crate::RawTypeId::Local(crate::TypeId::new(16)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(19)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(20)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::CallSignature,
                ty: crate::RawTypeId::Local(crate::TypeId::new(21)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::CallSignature,
                ty: crate::RawTypeId::Local(crate::TypeId::new(22)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::CallSignature,
                ty: crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedStatic(
                    crate::globals::GLOBAL_SYMBOL_SPECIES_ID.into(),
                ),
                ty: crate::globals::GLOBAL_REGEXP_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::REGEXP_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("exec")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("string"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::RawTypeId::Local(crate::TypeId::new(2)).into()),
    }));
    builder.set_type_data(crate::globals::REGEXP_EXEC_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Symbol")),
        type_parameters: Box::default(),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("iterator")),
                ty: crate::globals::GLOBAL_SYMBOL_ITERATOR_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::CallSignature,
                ty: crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("for")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("keyFor")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                    "hasInstance",
                )),
                ty: crate::globals::GLOBAL_SYMBOL_HAS_INSTANCE_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                    "isConcatSpreadable",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("match")),
                ty: crate::globals::GLOBAL_SYMBOL_MATCH_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("replace")),
                ty: crate::globals::GLOBAL_SYMBOL_REPLACE_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("search")),
                ty: crate::globals::GLOBAL_SYMBOL_SEARCH_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("species")),
                ty: crate::globals::GLOBAL_SYMBOL_SPECIES_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("split")),
                ty: crate::globals::GLOBAL_SYMBOL_SPLIT_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                    "toPrimitive",
                )),
                ty: crate::globals::GLOBAL_SYMBOL_TO_PRIMITIVE_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                    "toStringTag",
                )),
                ty: crate::globals::GLOBAL_SYMBOL_TO_STRING_TAG_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                    "unscopables",
                )),
                ty: crate::globals::GLOBAL_SYMBOL_UNSCOPABLES_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                    "asyncIterator",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("dispose")),
                ty: crate::globals::GLOBAL_SYMBOL_DISPOSE_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                    "asyncDispose",
                )),
                ty: crate::globals::GLOBAL_SYMBOL_ASYNC_DISPOSE_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::SYMBOL_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_DISPOSE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("Disposable"),
        type_parameters: Box::default(),
        extends: Box::default(),
        members: Box::new([crate::TypeMember {
            kind: crate::TypeMemberKind::ComputedValue(
                crate::globals::GLOBAL_SYMBOL_DISPOSE_ID.into(),
            ),
            ty: crate::globals::GLOBAL_DISPOSABLE_DISPOSE_ID.into(),
        }]),
    }));
    builder.set_type_data(crate::globals::DISPOSABLE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: None,
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
    }));
    builder.set_type_data(crate::globals::DISPOSABLE_DISPOSE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("AsyncDisposable"),
        type_parameters: Box::default(),
        extends: Box::default(),
        members: Box::new([crate::TypeMember {
            kind: crate::TypeMemberKind::ComputedValue(
                crate::globals::GLOBAL_SYMBOL_ASYNC_DISPOSE_ID.into(),
            ),
            ty: crate::globals::GLOBAL_ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID.into(),
        }]),
    }));
    builder.set_type_data(crate::globals::ASYNC_DISPOSABLE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: true,
        type_parameters: Box::default(),
        name: None,
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_PROMISE_ID.into()),
    }));
    builder.set_type_data(
        crate::globals::ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID,
        data,
    );
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Date")),
        type_parameters: Box::default(),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toDateString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toTimeString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toLocaleString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(70)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "toLocaleDateString",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(72)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "toLocaleTimeString",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(74)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("valueOf")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getTime")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getFullYear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCFullYear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getMonth")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCMonth")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getDate")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCDate")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getDay")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(14)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCDay")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(15)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getHours")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(16)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCHours")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getMinutes")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCMinutes")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(19)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getSeconds")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(20)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCSeconds")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(21)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "getMilliseconds",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(22)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "getUTCMilliseconds",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "getTimezoneOffset",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setTime")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(25)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "setMilliseconds",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(26)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "setUTCMilliseconds",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(27)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setSeconds")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(28)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCSeconds")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(29)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setMinutes")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(30)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCMinutes")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(31)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setHours")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(32)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCHours")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(33)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setDate")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(34)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCDate")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(35)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setMonth")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(36)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCMonth")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(37)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setFullYear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(38)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCFullYear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(39)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toUTCString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(40)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toISOString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(41)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toJSON")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(43)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedValue(
                    crate::globals::GLOBAL_SYMBOL_TO_PRIMITIVE_ID.into(),
                ),
                ty: crate::RawTypeId::Local(crate::TypeId::new(85)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(87)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(89)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(90)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::CallSignature,
                ty: crate::RawTypeId::Local(crate::TypeId::new(91)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                    "prototype",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(86)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("parse")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(92)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("UTC")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(93)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("now")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(94)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(96)).into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::DATE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Map")),
        type_parameters: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
        ]),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("clear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("delete")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("forEach")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("get")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("has")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("set")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(14)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("size")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedValue(
                    crate::globals::GLOBAL_SYMBOL_ITERATOR_ID.into(),
                ),
                ty: crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("entries")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("keys")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(20)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("values")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(22)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedValue(
                    crate::globals::GLOBAL_SYMBOL_TO_STRING_TAG_ID.into(),
                ),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(30)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(33)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedStatic(
                    crate::globals::GLOBAL_SYMBOL_SPECIES_ID.into(),
                ),
                ty: crate::globals::GLOBAL_MAP_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::MAP_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Set")),
        type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("add")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("clear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("delete")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("forEach")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("has")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("size")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedValue(
                    crate::globals::GLOBAL_SYMBOL_ITERATOR_ID.into(),
                ),
                ty: crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("entries")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(15)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("keys")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(16)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("values")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedValue(
                    crate::globals::GLOBAL_SYMBOL_TO_STRING_TAG_ID.into(),
                ),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(27)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedStatic(
                    crate::globals::GLOBAL_SYMBOL_SPECIES_ID.into(),
                ),
                ty: crate::globals::GLOBAL_SET_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::SET_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("WeakMap")),
        type_parameters: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
        ]),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("delete")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("get")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("has")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("set")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedValue(
                    crate::globals::GLOBAL_SYMBOL_TO_STRING_TAG_ID.into(),
                ),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(21)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::WEAK_MAP_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Error")),
        type_parameters: Box::default(),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("name")),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("message")),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static("stack")),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Constructor,
                ty: crate::globals::GLOBAL_ERROR_CONSTRUCTOR_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::CallSignature,
                ty: crate::globals::GLOBAL_ERROR_CALL_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                    "prototype",
                )),
                ty: crate::globals::GLOBAL_INSTANCEOF_ERROR_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::ERROR_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Constructor(Box::new(crate::Constructor {
        type_parameters: Box::default(),
        parameters: Box::new([crate::ConstructorParameter {
            parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("message"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: true,
                is_rest: false,
            }),
            accessibility: None,
        }]),
        return_type: Some(crate::globals::GLOBAL_ERROR_ID.into()),
    }));
    builder.set_type_data(crate::globals::ERROR_CONSTRUCTOR_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("Error")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("message"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: true,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_ERROR_ID.into()),
    }));
    builder.set_type_data(crate::globals::ERROR_CALL_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Math")),
        type_parameters: Box::default(),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("E")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("LN10")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("LN2")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("LOG2E")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("LOG10E")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("PI")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("SQRT1_2")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("SQRT2")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("abs")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("acos")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("asin")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("atan")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("atan2")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("ceil")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("cos")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("exp")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("floor")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("log")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("max")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("min")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("pow")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("random")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(14)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("round")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(15)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("sin")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(16)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("sqrt")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("tan")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("clz32")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(19)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("imul")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(20)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("sign")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(21)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("log10")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(22)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("log2")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("log1p")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("expm1")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(25)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("cosh")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(26)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("sinh")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(27)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("tanh")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(28)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("acosh")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(29)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("asinh")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(30)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("atanh")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(31)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("hypot")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(32)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("trunc")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(33)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("fround")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(34)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static("cbrt")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(35)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::ComputedStatic(
                    crate::globals::GLOBAL_SYMBOL_TO_STRING_TAG_ID.into(),
                ),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::MATH_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("IteratorYieldResult"),
        type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
        extends: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static("done")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("value")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            },
        ]),
    }));
    builder.set_type_data(
        crate::globals::ITERATOR_YIELD_RESULT_ID_GLOBAL_TYPE_ID,
        data,
    );
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("IteratorReturnResult"),
        type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
        extends: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("done")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("value")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            },
        ]),
    }));
    builder.set_type_data(
        crate::globals::ITERATOR_RETURN_RESULT_ID_GLOBAL_TYPE_ID,
        data,
    );
    let data = crate::TypeData::instance_of(crate::TypeInstance {
        ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
        type_parameters: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
        ]),
    });
    builder.set_type_data(crate::globals::ITERATOR_RESULT_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("Iterator"),
        type_parameters: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
        ]),
        extends: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("next")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static("return")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static("throw")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::ITERATOR_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("Iterable"),
        type_parameters: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
        ]),
        extends: Box::default(),
        members: Box::new([crate::TypeMember {
            kind: crate::TypeMemberKind::ComputedValue(
                crate::globals::GLOBAL_SYMBOL_ITERATOR_ID.into(),
            ),
            ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
        }]),
    }));
    builder.set_type_data(crate::globals::ITERABLE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_ITERATOR_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("RegExpExecArray"),
        type_parameters: Box::default(),
        extends: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("index")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("input")),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("0")),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::REGEXP_EXEC_ARRAY_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("ArrayLike"),
        type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
        extends: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("length")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::IndexSignature(
                    crate::globals::GLOBAL_NUMBER_ID.into(),
                ),
                ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::ARRAY_LIKE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_TO_STRING_TAG_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_TO_PRIMITIVE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_MATCH_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_REPLACE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_SEARCH_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_SPLIT_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_SPECIES_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_HAS_INSTANCE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Symbol;
    builder.set_type_data(crate::globals::SYMBOL_UNSCOPABLES_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("RegExpMatchArray"),
        type_parameters: Box::default(),
        extends: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static("index")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static("input")),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("0")),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::REGEXP_MATCH_ARRAY_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("IteratorObject"),
        type_parameters: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
        ]),
        extends: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
        ]),
        members: Box::new([crate::TypeMember {
            kind: crate::TypeMemberKind::ComputedValue(
                crate::globals::GLOBAL_SYMBOL_ITERATOR_ID.into(),
            ),
            ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
        }]),
    }));
    builder.set_type_data(crate::globals::ITERATOR_OBJECT_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("MapIterator"),
        type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
        extends: Box::new([crate::RawTypeId::Local(crate::TypeId::new(3)).into()]),
        members: Box::new([crate::TypeMember {
            kind: crate::TypeMemberKind::ComputedValue(
                crate::globals::GLOBAL_SYMBOL_ITERATOR_ID.into(),
            ),
            ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
        }]),
    }));
    builder.set_type_data(crate::globals::MAP_ITERATOR_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("SetIterator"),
        type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
        extends: Box::new([crate::RawTypeId::Local(crate::TypeId::new(3)).into()]),
        members: Box::new([crate::TypeMember {
            kind: crate::TypeMemberKind::ComputedValue(
                crate::globals::GLOBAL_SYMBOL_ITERATOR_ID.into(),
            ),
            ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
        }]),
    }));
    builder.set_type_data(crate::globals::SET_ITERATOR_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::instance_of(crate::TypeInstance {
        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
        type_parameters: Box::default(),
    });
    builder.set_type_data(
        crate::globals::BUILTIN_ITERATOR_RETURN_ID_GLOBAL_TYPE_ID,
        data,
    );
    let data = crate::TypeData::from(crate::Object {
        prototype: None,
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("Collator")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(48)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("NumberFormat")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(76)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("DateTimeFormat")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(104)).into(),
            },
        ]),
        has_unknown_members: false,
    });
    builder.set_type_data(crate::globals::INTL_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("eval")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("x"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::RawTypeId::Local(crate::TypeId::new(0)).into()),
    }));
    builder.set_type_data(crate::globals::EVAL_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("parseInt")),
        parameters: Box::new([
            crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("string"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: false,
                is_rest: false,
            }),
            crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("radix"),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                is_optional: true,
                is_rest: false,
            }),
        ]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
    }));
    builder.set_type_data(crate::globals::PARSE_INT_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("parseFloat")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("string"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
    }));
    builder.set_type_data(crate::globals::PARSE_FLOAT_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("isNaN")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("number"),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::RawTypeId::Local(crate::TypeId::new(0)).into()),
    }));
    builder.set_type_data(crate::globals::IS_NA_N_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("isFinite")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("number"),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::RawTypeId::Local(crate::TypeId::new(0)).into()),
    }));
    builder.set_type_data(crate::globals::IS_FINITE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("decodeURI")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("encodedURI"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
    }));
    builder.set_type_data(crate::globals::DECODE_URI_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("decodeURIComponent")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("encodedURIComponent"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
    }));
    builder.set_type_data(crate::globals::DECODE_URI_COMPONENT_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("encodeURI")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("uri"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
    }));
    builder.set_type_data(crate::globals::ENCODE_URI_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("encodeURIComponent")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("uriComponent"),
                ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
    }));
    builder.set_type_data(crate::globals::ENCODE_URI_COMPONENT_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("escape")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("string"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
    }));
    builder.set_type_data(crate::globals::ESCAPE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("unescape")),
        parameters: Box::new([crate::FunctionParameter::Named(
            crate::NamedFunctionParameter {
                name: biome_rowan::Text::new_static("string"),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
                is_optional: false,
                is_rest: false,
            },
        )]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
    }));
    builder.set_type_data(crate::globals::UNESCAPE_ID_GLOBAL_TYPE_ID, data);
}

pub(crate) static ARRAY_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 14]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("T"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_LIKE_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
                name: Some(biome_rowan::Text::new_static("from")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("arrayLike"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                ),
            })),
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("U"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("v"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("k"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                ),
            })),
            crate::TypeData::AnyKeyword,
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(4)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                ]),
                name: Some(biome_rowan::Text::new_static("from")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("arrayLike"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("mapfn"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("thisArg"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                ),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERABLE_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            ])))),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
                name: Some(biome_rowan::Text::new_static("from")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("iterable"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                ]),
                name: Some(biome_rowan::Text::new_static("from")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("iterable"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("mapfn"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("thisArg"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                ),
            })),
            crate::TypeData::from(crate::Object {
                prototype: None,
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
                    },
                ]),
                has_unknown_members: false,
            }),
        ]
    });
pub(crate) static REGEXP_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 25]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::Boolean,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("test")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("string"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                ),
            })),
            crate::TypeData::ThisKeyword,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("compile")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("pattern"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("flags"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                ),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_REGEXP_MATCH_ARRAY_ID.into(),
                type_parameters: Box::default(),
            }),
            crate::TypeData::Null,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
            ])))),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.match]")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("string"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.replace]")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("string"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("replaceValue"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::AnyKeyword,
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(9)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("substring"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("args"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                        is_optional: false,
                        is_rest: true,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.replace]")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("string"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("replacer"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::from(crate::Object {
                prototype: None,
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
                    },
                ]),
                has_unknown_members: false,
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.search]")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("string"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
                type_parameters: Box::new([crate::globals::GLOBAL_STRING_ID.into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.split]")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("string"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("limit"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(15)).into(),
                ),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_REGEXP_ID.into(),
                type_parameters: Box::default(),
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
                crate::globals::GLOBAL_STRING_ID.into(),
            ])))),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([crate::ConstructorParameter {
                    parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("pattern"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    accessibility: None,
                }]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(17)).into()),
            })),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("pattern"),
                            ty: crate::globals::GLOBAL_STRING_ID.into(),
                            is_optional: false,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("flags"),
                            ty: crate::globals::GLOBAL_STRING_ID.into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                ]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(17)).into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("pattern"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("pattern"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("flags"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
                ),
            })),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("pattern"),
                            ty: crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
                            is_optional: false,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("flags"),
                            ty: crate::globals::GLOBAL_STRING_ID.into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                ]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(17)).into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("pattern"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("flags"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
                ),
            })),
        ]
    });
pub(crate) static REGEXP_EXEC_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 3]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_REGEXP_EXEC_ARRAY_ID.into(),
                type_parameters: Box::default(),
            }),
            crate::TypeData::Null,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            ])))),
        ]
    });
pub(crate) static SYMBOL_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 7]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_STRING_ID.into(),
                crate::globals::GLOBAL_NUMBER_ID.into(),
            ])))),
            crate::TypeData::Symbol,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("description"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: true,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("for")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                ),
            })),
            crate::TypeData::Undefined,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_STRING_ID.into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("keyFor")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("sym"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
                ),
            })),
        ]
    });
pub(crate) static MATH_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 36]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("abs")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("acos")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("asin")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("atan")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("atan2")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("y"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("ceil")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("cos")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("exp")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("floor")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("log")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
                type_parameters: Box::new([crate::globals::GLOBAL_NUMBER_ID.into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("max")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("values"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                        is_optional: false,
                        is_rest: true,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("min")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("values"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                        is_optional: false,
                        is_rest: true,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("pow")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("y"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("random")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("round")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("sin")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("sqrt")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("tan")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("clz32")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("imul")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("y"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("sign")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("log10")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("log2")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("log1p")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("expm1")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("cosh")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("sinh")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("tanh")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("acosh")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("asinh")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("atanh")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("hypot")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("values"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                        is_optional: false,
                        is_rest: true,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("trunc")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("fround")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("cbrt")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
        ]
    });
pub(crate) static WEAK_MAP_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 25]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::ObjectKeyword,
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("K"),
                constraint: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("V"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::Boolean,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("delete")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                ),
            })),
            crate::TypeData::Undefined,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
            ])))),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("get")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("has")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                ),
            })),
            crate::TypeData::ThisKeyword,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("set")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
                ),
            })),
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("K"),
                constraint: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                default: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            }),
            crate::TypeData::AnyKeyword,
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("V"),
                constraint: crate::TypeReference::unknown(),
                default: crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
            }),
            crate::TypeData::from(crate::Tuple {
                elements: Box::new([
                    crate::TupleElementType {
                        ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                        name: None,
                        is_optional: false,
                        is_rest: false,
                    },
                    crate::TupleElementType {
                        ty: crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
                        name: None,
                        is_optional: false,
                        is_rest: false,
                    },
                ]),
                is_inferred_array: false,
            }),
            crate::TypeData::TypeOperator(Box::new(crate::TypeOperatorType {
                operator: crate::TypeOperator::Readonly,
                ty: crate::RawTypeId::Local(crate::TypeId::new(14)).into(),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(15)).into()]),
            }),
            crate::TypeData::TypeOperator(Box::new(crate::TypeOperatorType {
                operator: crate::TypeOperator::Readonly,
                ty: crate::RawTypeId::Local(crate::TypeId::new(16)).into(),
            })),
            crate::TypeData::Null,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
            ])))),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_WEAK_MAP_ID.into(),
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
                ]),
            }),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
                ]),
                parameters: Box::new([crate::ConstructorParameter {
                    parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("entries"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(19)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    accessibility: None,
                }]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(20)).into()),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERABLE_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(15)).into()]),
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(22)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
            ])))),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
                ]),
                parameters: Box::new([crate::ConstructorParameter {
                    parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("iterable"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    accessibility: None,
                }]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(20)).into()),
            })),
        ]
    });
pub(crate) static SET_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 28]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("T"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::ThisKeyword,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("add")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("clear")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
            })),
            crate::TypeData::Boolean,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("delete")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                ),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_SET_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value2"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("set"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
            })),
            crate::TypeData::AnyKeyword,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("forEach")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("callbackfn"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("thisArg"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("has")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                ),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_SET_ITERATOR_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.iterator]")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                ),
            })),
            crate::TypeData::from(crate::Tuple {
                elements: Box::new([
                    crate::TupleElementType {
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        name: None,
                        is_optional: false,
                        is_rest: false,
                    },
                    crate::TupleElementType {
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        name: None,
                        is_optional: false,
                        is_rest: false,
                    },
                ]),
                is_inferred_array: false,
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_SET_ITERATOR_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(13)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("entries")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(14)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("keys")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("values")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                ),
            })),
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("T"),
                constraint: crate::TypeReference::unknown(),
                default: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(18)).into()]),
            }),
            crate::TypeData::TypeOperator(Box::new(crate::TypeOperatorType {
                operator: crate::TypeOperator::Readonly,
                ty: crate::RawTypeId::Local(crate::TypeId::new(19)).into(),
            })),
            crate::TypeData::Null,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(20)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(21)).into(),
            ])))),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_SET_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(18)).into()]),
            }),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(18)).into()]),
                parameters: Box::new([crate::ConstructorParameter {
                    parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("values"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(22)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    accessibility: None,
                }]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(23)).into()),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERABLE_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(25)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(21)).into(),
            ])))),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
                parameters: Box::new([crate::ConstructorParameter {
                    parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("iterable"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(26)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    accessibility: None,
                }]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(6)).into()),
            })),
        ]
    });
pub(crate) static MAP_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 34]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("K"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("V"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("clear")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
            })),
            crate::TypeData::Boolean,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("delete")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                ),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_MAP_ID.into(),
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                ]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("map"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
            })),
            crate::TypeData::AnyKeyword,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("forEach")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("callbackfn"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("thisArg"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
            })),
            crate::TypeData::Undefined,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
            ])))),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("get")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("has")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                ),
            })),
            crate::TypeData::ThisKeyword,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("set")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
                ),
            })),
            crate::TypeData::from(crate::Tuple {
                elements: Box::new([
                    crate::TupleElementType {
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                        name: None,
                        is_optional: false,
                        is_rest: false,
                    },
                    crate::TupleElementType {
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        name: None,
                        is_optional: false,
                        is_rest: false,
                    },
                ]),
                is_inferred_array: false,
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_MAP_ITERATOR_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(15)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.iterator]")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(16)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("entries")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(16)).into(),
                ),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_MAP_ITERATOR_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("keys")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(19)).into(),
                ),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_MAP_ITERATOR_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(1)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("values")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(21)).into(),
                ),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_MAP_ID.into(),
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                ]),
            }),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(23)).into()),
            })),
            crate::TypeData::TypeOperator(Box::new(crate::TypeOperatorType {
                operator: crate::TypeOperator::Readonly,
                ty: crate::RawTypeId::Local(crate::TypeId::new(15)).into(),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(25)).into()]),
            }),
            crate::TypeData::TypeOperator(Box::new(crate::TypeOperatorType {
                operator: crate::TypeOperator::Readonly,
                ty: crate::RawTypeId::Local(crate::TypeId::new(26)).into(),
            })),
            crate::TypeData::Null,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(27)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(28)).into(),
            ])))),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                ]),
                parameters: Box::new([crate::ConstructorParameter {
                    parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("entries"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(29)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    accessibility: None,
                }]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(5)).into()),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERABLE_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(25)).into()]),
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(31)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(28)).into(),
            ])))),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                ]),
                parameters: Box::new([crate::ConstructorParameter {
                    parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("iterable"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(32)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    accessibility: None,
                }]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(5)).into()),
            })),
        ]
    });
pub(crate) static DATE_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 97]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toString")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toDateString")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toTimeString")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toLocaleString")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toLocaleDateString")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toLocaleTimeString")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("valueOf")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getTime")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getFullYear")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getUTCFullYear")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getMonth")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getUTCMonth")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getDate")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getUTCDate")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getDay")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getUTCDay")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getHours")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getUTCHours")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getMinutes")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getUTCMinutes")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getSeconds")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getUTCSeconds")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getMilliseconds")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getUTCMilliseconds")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("getTimezoneOffset")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setTime")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("time"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setMilliseconds")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("ms"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setUTCMilliseconds")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("ms"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setSeconds")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("sec"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("ms"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setUTCSeconds")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("sec"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("ms"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setMinutes")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("min"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("sec"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("ms"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setUTCMinutes")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("min"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("sec"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("ms"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setHours")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("hours"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("min"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("sec"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("ms"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setUTCHours")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("hours"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("min"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("sec"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("ms"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setDate")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("date"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setUTCDate")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("date"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setMonth")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("month"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("date"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setUTCMonth")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("month"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("date"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setFullYear")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("year"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("month"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("date"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("setUTCFullYear")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("year"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("month"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("date"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toUTCString")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toISOString")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::AnyKeyword,
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toJSON")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("key"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(42)).into(),
                        is_optional: true,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
                type_parameters: Box::new([crate::globals::GLOBAL_STRING_ID.into()]),
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_STRING_ID.into(),
                crate::RawTypeId::Local(crate::TypeId::new(44)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("best fit").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("lookup").into(),
            ))),
            crate::TypeData::Undefined,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(46)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(47)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(48)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("long").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("short").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("narrow").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(50)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(51)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(52)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(48)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("numeric").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("2-digit").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(54)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(55)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(48)).into(),
            ])))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(54)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(55)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(50)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(51)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(52)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(48)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("shortOffset").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("longOffset").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("shortGeneric").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("longGeneric").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(51)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(50)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(58)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(59)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(60)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(61)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(48)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("basic").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(46)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(63)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(48)).into(),
            ])))),
            crate::TypeData::Boolean,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(65)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(48)).into(),
            ])))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_STRING_ID.into(),
                crate::RawTypeId::Local(crate::TypeId::new(48)).into(),
            ])))),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.DateTimeFormatOptions"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "localeMatcher",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(49)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "weekday",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(53)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "era",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(53)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "year",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(56)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "month",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(57)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "day",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(56)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "hour",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(56)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "minute",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(56)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "second",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(56)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "timeZoneName",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(62)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "formatMatcher",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(64)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "hour12",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(66)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "timeZone",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(67)).into(),
                    },
                ]),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toLocaleString")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("locales"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(45)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("options"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(68)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::from(crate::Object {
                prototype: None,
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(69)).into(),
                    },
                ]),
                has_unknown_members: false,
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toLocaleDateString")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("locales"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(45)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("options"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(68)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::from(crate::Object {
                prototype: None,
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(71)).into(),
                    },
                ]),
                has_unknown_members: false,
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("toLocaleTimeString")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("locales"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(45)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("options"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(68)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::from(crate::Object {
                prototype: None,
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(73)).into(),
                    },
                ]),
                has_unknown_members: false,
            }),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("default").into(),
            ))),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.toPrimitive]")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("hint"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(75)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("string").into(),
            ))),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.toPrimitive]")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("hint"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(77)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::from(crate::Object {
                prototype: None,
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(76)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(78)).into(),
                    },
                ]),
                has_unknown_members: false,
            }),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("number").into(),
            ))),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.toPrimitive]")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("hint"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(80)).into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::from(crate::Object {
                prototype: None,
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(76)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(78)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(81)).into(),
                    },
                ]),
                has_unknown_members: false,
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_STRING_ID.into(),
                crate::globals::GLOBAL_NUMBER_ID.into(),
            ])))),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.toPrimitive]")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("hint"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(83)).into(),
                ),
            })),
            crate::TypeData::from(crate::Object {
                prototype: None,
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(76)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(78)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(81)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(84)).into(),
                    },
                ]),
                has_unknown_members: false,
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_DATE_ID.into(),
                type_parameters: Box::default(),
            }),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(86)).into()),
            })),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_NUMBER_ID.into(),
                crate::globals::GLOBAL_STRING_ID.into(),
            ])))),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([crate::ConstructorParameter {
                    parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(88)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    accessibility: None,
                }]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(86)).into()),
            })),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("year"),
                            ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                            is_optional: false,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("monthIndex"),
                            ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                            is_optional: false,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("date"),
                            ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("hours"),
                            ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("minutes"),
                            ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("seconds"),
                            ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("ms"),
                            ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                ]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(86)).into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("parse")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("s"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("UTC")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("year"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("monthIndex"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("date"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("hours"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("minutes"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("seconds"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("ms"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("now")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_NUMBER_ID.into(),
                crate::globals::GLOBAL_STRING_ID.into(),
                crate::RawTypeId::Local(crate::TypeId::new(86)).into(),
            ])))),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([crate::ConstructorParameter {
                    parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(95)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    accessibility: None,
                }]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(86)).into()),
            })),
        ]
    });
pub(crate) static ITERATOR_YIELD_RESULT_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 2]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("TYield"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::Literal(Box::new(crate::Literal::Boolean(false.into()))),
        ]
    });
pub(crate) static ITERATOR_RETURN_RESULT_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 2]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("TReturn"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::Literal(Box::new(crate::Literal::Boolean(true.into()))),
        ]
    });
pub(crate) static ITERATOR_RESULT_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 6]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("T"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::AnyKeyword,
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("TReturn"),
                constraint: crate::TypeReference::unknown(),
                default: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERATOR_YIELD_RESULT_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERATOR_RETURN_RESULT_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(2)).into()]),
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
        ]
    });
pub(crate) static ITERATOR_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 11]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("T"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::AnyKeyword,
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("TReturn"),
                constraint: crate::TypeReference::unknown(),
                default: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            }),
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("TNext"),
                constraint: crate::TypeReference::unknown(),
                default: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            }),
            crate::TypeData::from(crate::Tuple {
                elements: Box::new([]),
                is_inferred_array: false,
            }),
            crate::TypeData::from(crate::Tuple {
                elements: Box::new([crate::TupleElementType {
                    ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                    name: None,
                    is_optional: false,
                    is_rest: false,
                }]),
                is_inferred_array: false,
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
            ])))),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERATOR_RESULT_ID.into(),
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                ]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("next")),
                parameters: Box::new([crate::FunctionParameter::Pattern(
                    crate::PatternFunctionParameter {
                        bindings: Box::default(),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                        is_optional: false,
                        is_rest: true,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("return")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                        is_optional: true,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("throw")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("e"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: true,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                ),
            })),
        ]
    });
pub(crate) static ITERABLE_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 6]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("T"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::AnyKeyword,
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("TReturn"),
                constraint: crate::TypeReference::unknown(),
                default: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            }),
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("TNext"),
                constraint: crate::TypeReference::unknown(),
                default: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERATOR_ID.into(),
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                ]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.iterator]")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                ),
            })),
        ]
    });
pub(crate) static REGEXP_EXEC_ARRAY_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 1]> =
    std::sync::LazyLock::new(|| {
        [crate::TypeData::instance_of(crate::TypeInstance {
            ty: crate::globals::GLOBAL_ARRAY_ID.into(),
            type_parameters: Box::new([crate::globals::GLOBAL_STRING_ID.into()]),
        })]
    });
pub(crate) static ARRAY_LIKE_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 1]> =
    std::sync::LazyLock::new(|| {
        [crate::TypeData::from(crate::GenericTypeParameter {
            is_const: false,
            name: biome_rowan::Text::new_static("T"),
            constraint: crate::TypeReference::unknown(),
            default: crate::TypeReference::unknown(),
        })]
    });
pub(crate) static REGEXP_MATCH_ARRAY_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 1]> =
    std::sync::LazyLock::new(|| {
        [crate::TypeData::instance_of(crate::TypeInstance {
            ty: crate::globals::GLOBAL_ARRAY_ID.into(),
            type_parameters: Box::new([crate::globals::GLOBAL_STRING_ID.into()]),
        })]
    });
pub(crate) static ITERATOR_OBJECT_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 8]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("T"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::UnknownKeyword,
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("TReturn"),
                constraint: crate::TypeReference::unknown(),
                default: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            }),
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("TNext"),
                constraint: crate::TypeReference::unknown(),
                default: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERATOR_ID.into(),
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                ]),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERATOR_OBJECT_ID.into(),
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                ]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.iterator]")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
                ),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_DISPOSABLE_ID.into(),
                type_parameters: Box::default(),
            }),
        ]
    });
pub(crate) static MAP_ITERATOR_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 6]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("T"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_BUILTIN_ITERATOR_RETURN_ID.into(),
                type_parameters: Box::default(),
            }),
            crate::TypeData::UnknownKeyword,
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERATOR_OBJECT_ID.into(),
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                ]),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_MAP_ITERATOR_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.iterator]")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                ),
            })),
        ]
    });
pub(crate) static SET_ITERATOR_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 6]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::from(crate::GenericTypeParameter {
                is_const: false,
                name: biome_rowan::Text::new_static("T"),
                constraint: crate::TypeReference::unknown(),
                default: crate::TypeReference::unknown(),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_BUILTIN_ITERATOR_RETURN_ID.into(),
                type_parameters: Box::default(),
            }),
            crate::TypeData::UnknownKeyword,
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ITERATOR_OBJECT_ID.into(),
                type_parameters: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                ]),
            }),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_SET_ITERATOR_ID.into(),
                type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(0)).into()]),
            }),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("[Symbol.iterator]")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                ),
            })),
        ]
    });
pub(crate) static BUILTIN_ITERATOR_RETURN_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 1]> =
    std::sync::LazyLock::new(|| [crate::TypeData::UnknownKeyword]);
pub(crate) static INTL_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 105]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_ARRAY_ID.into(),
                type_parameters: Box::new([crate::globals::GLOBAL_STRING_ID.into()]),
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_STRING_ID.into(),
                crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("sort").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("search").into(),
            ))),
            crate::TypeData::Undefined,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("lookup").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("best fit").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Boolean,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("upper").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("lower").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("false").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("base").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("accent").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("case").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("variant").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(15)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(16)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("big5han").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("compat").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("default").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("dict").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("direct").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("ducet").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("emoji").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("eor").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("gb2312").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("phonebk").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("phonetic").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("pinyin").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("reformed").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("searchjl").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("stroke").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("trad").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("unihan").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("zhuyin").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(20)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(21)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(22)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(25)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(26)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(27)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(28)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(29)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(30)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(31)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(32)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(33)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(34)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(35)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(36)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(37)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.CollatorOptions"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "usage",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "localeMatcher",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "numeric",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "caseFirst",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(14)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "sensitivity",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(19)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "collation",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(38)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "ignorePunctuation",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                    },
                ]),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("compare")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("x"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("y"),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_NUMBER_ID.into()),
            })),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.ResolvedCollatorOptions"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("locale")),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("usage")),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "sensitivity",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "ignorePunctuation",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "collation",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "caseFirst",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "numeric",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
                    },
                ]),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("resolvedOptions")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(41)).into(),
                ),
            })),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.Collator"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "compare",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(40)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "resolvedOptions",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(42)).into(),
                    },
                ]),
            })),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("locales"),
                            ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("options"),
                            ty: crate::RawTypeId::Local(crate::TypeId::new(39)).into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                ]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(43)).into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("locales"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("options"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(39)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(43)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("supportedLocalesOf")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("locales"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("options"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(39)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                ),
            })),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.CollatorConstructor"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Constructor,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(44)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(45)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "supportedLocalesOf",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(46)).into(),
                    },
                ]),
            })),
            crate::TypeData::Class(Box::new(crate::Class {
                name: Some(biome_rowan::Text::new_static("Intl.Collator")),
                type_parameters: Box::default(),
                extends: None,
                implements: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Constructor,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(44)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(45)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                            "supportedLocalesOf",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(46)).into(),
                    },
                ]),
            })),
            crate::TypeData::NeverKeyword,
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.NumberFormatOptionsStyleRegistry"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "decimal",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(49)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "percent",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(49)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "currency",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(49)).into(),
                    },
                ]),
            })),
            crate::TypeData::TypeOperator(Box::new(crate::TypeOperatorType {
                operator: crate::TypeOperator::Keyof,
                ty: crate::RawTypeId::Local(crate::TypeId::new(50)).into(),
            })),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(51)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_STRING_ID.into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static(
                    "Intl.NumberFormatOptionsCurrencyDisplayRegistry",
                ),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("code")),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(49)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("symbol")),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(49)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("name")),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(49)).into(),
                    },
                ]),
            })),
            crate::TypeData::TypeOperator(Box::new(crate::TypeOperatorType {
                operator: crate::TypeOperator::Keyof,
                ty: crate::RawTypeId::Local(crate::TypeId::new(54)).into(),
            })),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(55)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.NumberFormatOptionsUseGroupingRegistry"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([]),
            })),
            crate::TypeData::TypeOperator(Box::new(crate::TypeOperatorType {
                operator: crate::TypeOperator::Keyof,
                ty: crate::RawTypeId::Local(crate::TypeId::new(57)).into(),
            })),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("true").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(58)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(59)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
            ])))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(60)).into(),
            ])))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(61)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_NUMBER_ID.into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.NumberFormatOptions"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "localeMatcher",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "style",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(52)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "currency",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(53)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "currencyDisplay",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(56)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "useGrouping",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(62)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "minimumIntegerDigits",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(63)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "minimumFractionDigits",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(63)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "maximumFractionDigits",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(63)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "minimumSignificantDigits",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(63)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "maximumSignificantDigits",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(63)).into(),
                    },
                ]),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("format")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("value"),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                        is_optional: false,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Literal(Box::new(crate::Literal::Boolean(false.into()))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(58)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(66)).into(),
            ])))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(67)).into(),
            ])))),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.ResolvedNumberFormatOptions"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("locale")),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "numberingSystem",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("style")),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(51)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "currency",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "currencyDisplay",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(55)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "minimumIntegerDigits",
                        )),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "minimumFractionDigits",
                        )),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "maximumFractionDigits",
                        )),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "minimumSignificantDigits",
                        )),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "maximumSignificantDigits",
                        )),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "useGrouping",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(68)).into(),
                    },
                ]),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("resolvedOptions")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(69)).into(),
                ),
            })),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.NumberFormat"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("format")),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(65)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "resolvedOptions",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(70)).into(),
                    },
                ]),
            })),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("locales"),
                            ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("options"),
                            ty: crate::RawTypeId::Local(crate::TypeId::new(64)).into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                ]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(71)).into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("locales"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("options"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(64)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(71)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("supportedLocalesOf")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("locales"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("options"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(64)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                ),
            })),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.NumberFormatConstructor"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Constructor,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(72)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(73)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "supportedLocalesOf",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(74)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "prototype",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(71)).into(),
                    },
                ]),
            })),
            crate::TypeData::Class(Box::new(crate::Class {
                name: Some(biome_rowan::Text::new_static("Intl.NumberFormat")),
                type_parameters: Box::default(),
                extends: None,
                implements: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Constructor,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(72)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(73)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                            "supportedLocalesOf",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(74)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                            "prototype",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(71)).into(),
                    },
                ]),
            })),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("long").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("short").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("narrow").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(78)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(79)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(80)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("numeric").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("2-digit").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(82)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(83)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(82)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(83)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(78)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(79)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(80)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("shortOffset").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("longOffset").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("shortGeneric").into(),
            ))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("longGeneric").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(79)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(78)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(86)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(87)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(88)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(89)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Literal(Box::new(crate::Literal::String(
                biome_rowan::Text::new_static("basic").into(),
            ))),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(91)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
            ])))),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.DateTimeFormatOptions"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "localeMatcher",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(77)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "weekday",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(81)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "era",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(81)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "year",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(84)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "month",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(85)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "day",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(84)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "hour",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(84)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "minute",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(84)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "second",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(84)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "timeZoneName",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(90)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "formatMatcher",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(92)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "hour12",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "timeZone",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(53)).into(),
                    },
                ]),
            })),
            crate::TypeData::instance_of(crate::TypeInstance {
                ty: crate::globals::GLOBAL_DATE_ID.into(),
                type_parameters: Box::default(),
            }),
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(94)).into(),
                crate::globals::GLOBAL_NUMBER_ID.into(),
            ])))),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("format")),
                parameters: Box::new([crate::FunctionParameter::Named(
                    crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("date"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(95)).into(),
                        is_optional: true,
                        is_rest: false,
                    },
                )]),
                return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
            })),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.ResolvedDateTimeFormatOptions"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("locale")),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "calendar",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "numberingSystem",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "timeZone",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "hour12",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "weekday",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "era",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "year",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "month",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "day",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "hour",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "minute",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "second",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static(
                            "timeZoneName",
                        )),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                ]),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("resolvedOptions")),
                parameters: Box::new([]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(97)).into(),
                ),
            })),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.DateTimeFormat"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("format")),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(96)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "resolvedOptions",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(98)).into(),
                    },
                ]),
            })),
            crate::TypeData::Constructor(Box::new(crate::Constructor {
                type_parameters: Box::default(),
                parameters: Box::new([
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("locales"),
                            ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                    crate::ConstructorParameter {
                        parameter: crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                            name: biome_rowan::Text::new_static("options"),
                            ty: crate::RawTypeId::Local(crate::TypeId::new(93)).into(),
                            is_optional: true,
                            is_rest: false,
                        }),
                        accessibility: None,
                    },
                ]),
                return_type: Some(crate::RawTypeId::Local(crate::TypeId::new(99)).into()),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("locales"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("options"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(93)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(99)).into(),
                ),
            })),
            crate::TypeData::Function(Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("supportedLocalesOf")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("locales"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("options"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(93)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                ),
            })),
            crate::TypeData::Interface(Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Intl.DateTimeFormatConstructor"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Constructor,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(100)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(101)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "supportedLocalesOf",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(102)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                            "prototype",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(99)).into(),
                    },
                ]),
            })),
            crate::TypeData::Class(Box::new(crate::Class {
                name: Some(biome_rowan::Text::new_static("Intl.DateTimeFormat")),
                type_parameters: Box::default(),
                extends: None,
                implements: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Constructor,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(100)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::CallSignature,
                        ty: crate::RawTypeId::Local(crate::TypeId::new(101)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                            "supportedLocalesOf",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(102)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedStatic(biome_rowan::Text::new_static(
                            "prototype",
                        )),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(99)).into(),
                    },
                ]),
            })),
        ]
    });
pub(crate) static EVAL_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 1]> =
    std::sync::LazyLock::new(|| [crate::TypeData::AnyKeyword]);
pub(crate) static IS_NA_N_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 1]> =
    std::sync::LazyLock::new(|| [crate::TypeData::Boolean]);
pub(crate) static IS_FINITE_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 1]> =
    std::sync::LazyLock::new(|| [crate::TypeData::Boolean]);
pub(crate) static ENCODE_URI_COMPONENT_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 2]> =
    std::sync::LazyLock::new(|| {
        [
            crate::TypeData::Boolean,
            crate::TypeData::Union(Box::new(crate::Union(Box::new([
                crate::globals::GLOBAL_STRING_ID.into(),
                crate::globals::GLOBAL_NUMBER_ID.into(),
                crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            ])))),
        ]
    });

/// Supporting types in dependency order, indexed relative to their owning global.
pub(crate) fn generated_local_types(
    owner: crate::globals::GlobalTypeId,
) -> &'static [crate::TypeData] {
    match owner {
        crate::globals::ARRAY_ID_GLOBAL_TYPE_ID => &*ARRAY_LOCAL_TYPES,
        crate::globals::REGEXP_ID_GLOBAL_TYPE_ID => &*REGEXP_LOCAL_TYPES,
        crate::globals::REGEXP_EXEC_ID_GLOBAL_TYPE_ID => &*REGEXP_EXEC_LOCAL_TYPES,
        crate::globals::SYMBOL_ID_GLOBAL_TYPE_ID => &*SYMBOL_LOCAL_TYPES,
        crate::globals::MATH_ID_GLOBAL_TYPE_ID => &*MATH_LOCAL_TYPES,
        crate::globals::WEAK_MAP_ID_GLOBAL_TYPE_ID => &*WEAK_MAP_LOCAL_TYPES,
        crate::globals::SET_ID_GLOBAL_TYPE_ID => &*SET_LOCAL_TYPES,
        crate::globals::MAP_ID_GLOBAL_TYPE_ID => &*MAP_LOCAL_TYPES,
        crate::globals::DATE_ID_GLOBAL_TYPE_ID => &*DATE_LOCAL_TYPES,
        crate::globals::ITERATOR_YIELD_RESULT_ID_GLOBAL_TYPE_ID => {
            &*ITERATOR_YIELD_RESULT_LOCAL_TYPES
        }
        crate::globals::ITERATOR_RETURN_RESULT_ID_GLOBAL_TYPE_ID => {
            &*ITERATOR_RETURN_RESULT_LOCAL_TYPES
        }
        crate::globals::ITERATOR_RESULT_ID_GLOBAL_TYPE_ID => &*ITERATOR_RESULT_LOCAL_TYPES,
        crate::globals::ITERATOR_ID_GLOBAL_TYPE_ID => &*ITERATOR_LOCAL_TYPES,
        crate::globals::ITERABLE_ID_GLOBAL_TYPE_ID => &*ITERABLE_LOCAL_TYPES,
        crate::globals::REGEXP_EXEC_ARRAY_ID_GLOBAL_TYPE_ID => &*REGEXP_EXEC_ARRAY_LOCAL_TYPES,
        crate::globals::ARRAY_LIKE_ID_GLOBAL_TYPE_ID => &*ARRAY_LIKE_LOCAL_TYPES,
        crate::globals::REGEXP_MATCH_ARRAY_ID_GLOBAL_TYPE_ID => &*REGEXP_MATCH_ARRAY_LOCAL_TYPES,
        crate::globals::ITERATOR_OBJECT_ID_GLOBAL_TYPE_ID => &*ITERATOR_OBJECT_LOCAL_TYPES,
        crate::globals::MAP_ITERATOR_ID_GLOBAL_TYPE_ID => &*MAP_ITERATOR_LOCAL_TYPES,
        crate::globals::SET_ITERATOR_ID_GLOBAL_TYPE_ID => &*SET_ITERATOR_LOCAL_TYPES,
        crate::globals::BUILTIN_ITERATOR_RETURN_ID_GLOBAL_TYPE_ID => {
            &*BUILTIN_ITERATOR_RETURN_LOCAL_TYPES
        }
        crate::globals::INTL_ID_GLOBAL_TYPE_ID => &*INTL_LOCAL_TYPES,
        crate::globals::EVAL_ID_GLOBAL_TYPE_ID => &*EVAL_LOCAL_TYPES,
        crate::globals::IS_NA_N_ID_GLOBAL_TYPE_ID => &*IS_NA_N_LOCAL_TYPES,
        crate::globals::IS_FINITE_ID_GLOBAL_TYPE_ID => &*IS_FINITE_LOCAL_TYPES,
        crate::globals::ENCODE_URI_COMPONENT_ID_GLOBAL_TYPE_ID => {
            &*ENCODE_URI_COMPONENT_LOCAL_TYPES
        }
        _ => &[],
    }
}
