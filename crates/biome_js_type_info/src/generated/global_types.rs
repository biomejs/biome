//! This is a generated file. Don't modify it by hand! Run 'just gen-global-types' to re-generate the file.

// Generated from microsoft/TypeScript v6.0.3 (git commit 050880ce59e30b356b686bd3144efe24f875ebc8).

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
    crate::globals::ITERATOR_YIELD_RESULT_ID_GLOBAL_TYPE_ID,
    crate::globals::ITERATOR_RETURN_RESULT_ID_GLOBAL_TYPE_ID,
    crate::globals::ITERATOR_RESULT_ID_GLOBAL_TYPE_ID,
    crate::globals::ITERATOR_ID_GLOBAL_TYPE_ID,
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
                ty: crate::RawTypeId::Local(crate::TypeId::new(84)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("source")),
                ty: crate::globals::GLOBAL_STRING_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("global")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(85)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("ignoreCase")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(86)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("multiline")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(87)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("lastIndex")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("compile")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(89)).into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::REGEXP_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Function(Box::new(crate::Function {
        is_async: false,
        type_parameters: Box::default(),
        name: Some(biome_rowan::Text::new_static("RegExp.exec")),
        parameters: Box::new([]),
        return_type: crate::ReturnType::Type(crate::globals::GLOBAL_INSTANCEOF_REGEXP_ID.into()),
    }));
    builder.set_type_data(crate::globals::REGEXP_EXEC_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Symbol")),
        type_parameters: Box::default(),
        extends: None,
        implements: Box::default(),
        members: Box::new([
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
                ty: crate::RawTypeId::Local(crate::TypeId::new(39)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toDateString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(40)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toTimeString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(41)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toLocaleString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(42)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "toLocaleDateString",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(43)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "toLocaleTimeString",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(44)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("valueOf")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(45)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getTime")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(46)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getFullYear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(47)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCFullYear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(48)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getMonth")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(49)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCMonth")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(50)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getDate")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(51)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCDate")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(52)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getDay")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(53)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCDay")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(54)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getHours")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(55)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCHours")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(56)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getMinutes")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(57)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCMinutes")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(58)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getSeconds")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(59)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("getUTCSeconds")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(60)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "getMilliseconds",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(61)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "getUTCMilliseconds",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(62)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "getTimezoneOffset",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(63)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setTime")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(64)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "setMilliseconds",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(65)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                    "setUTCMilliseconds",
                )),
                ty: crate::RawTypeId::Local(crate::TypeId::new(66)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setSeconds")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(67)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCSeconds")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(68)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setMinutes")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(69)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCMinutes")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(70)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setHours")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(71)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCHours")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(72)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setDate")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(73)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCDate")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(74)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setMonth")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(75)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCMonth")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(76)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setFullYear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(77)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("setUTCFullYear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(78)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toUTCString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(79)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toISOString")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(80)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("toJSON")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(82)).into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::DATE_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Map")),
        type_parameters: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
        ]),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("clear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(25)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("delete")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(27)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("forEach")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(31)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("get")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(34)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("has")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(36)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("set")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(38)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("size")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::MAP_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("Set")),
        type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(11)).into()]),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("add")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("clear")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(14)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("delete")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(16)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("forEach")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(20)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("has")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(22)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("size")),
                ty: crate::globals::GLOBAL_NUMBER_ID.into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::SET_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Class(Box::new(crate::Class {
        name: Some(biome_rowan::Text::new_static("WeakMap")),
        type_parameters: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
        ]),
        extends: None,
        implements: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("delete")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("get")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("has")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("set")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
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
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("IteratorYieldResult"),
        type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(90)).into()]),
        extends: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static("done")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(91)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("value")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(90)).into(),
            },
        ]),
    }));
    builder.set_type_data(
        crate::globals::ITERATOR_YIELD_RESULT_ID_GLOBAL_TYPE_ID,
        data,
    );
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("IteratorReturnResult"),
        type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(92)).into()]),
        extends: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("done")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(93)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("value")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(92)).into(),
            },
        ]),
    }));
    builder.set_type_data(
        crate::globals::ITERATOR_RETURN_RESULT_ID_GLOBAL_TYPE_ID,
        data,
    );
    let data = crate::TypeData::instance_of(crate::TypeInstance {
        ty: crate::RawTypeId::Local(crate::TypeId::new(99)).into(),
        type_parameters: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(94)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(96)).into(),
        ]),
    });
    builder.set_type_data(crate::globals::ITERATOR_RESULT_ID_GLOBAL_TYPE_ID, data);
    let data = crate::TypeData::Interface(Box::new(crate::Interface {
        name: biome_rowan::Text::new_static("Iterator"),
        type_parameters: Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(100)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(102)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(104)).into(),
        ]),
        extends: Box::default(),
        members: Box::new([
            crate::TypeMember {
                kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("next")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(109)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static("return")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(111)).into(),
            },
            crate::TypeMember {
                kind: crate::TypeMemberKind::NamedOptional(biome_rowan::Text::new_static("throw")),
                ty: crate::RawTypeId::Local(crate::TypeId::new(114)).into(),
            },
        ]),
    }));
    builder.set_type_data(crate::globals::ITERATOR_ID_GLOBAL_TYPE_ID, data);
}

/// Supporting types in dependency order; local references address this table.
pub(crate) fn generated_local_types() -> Box<[crate::TypeData]> {
    Box::new([
        crate::TypeData::from(crate::GenericTypeParameter {
            name: biome_rowan::Text::new_static("K"),
            constraint: crate::TypeReference::unknown(),
            default: crate::TypeReference::unknown(),
        }),
        crate::TypeData::from(crate::GenericTypeParameter {
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
                    ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    is_optional: false,
                    is_rest: false,
                },
            )]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
            ),
        })),
        crate::TypeData::Undefined,
        crate::TypeData::Union(Box::new(crate::Union(Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
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
                crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
            ),
        })),
        crate::TypeData::Boolean,
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
                crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
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
                crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
            ),
        })),
        crate::TypeData::from(crate::GenericTypeParameter {
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
                    ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                    is_optional: false,
                    is_rest: false,
                },
            )]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
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
                    ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                    is_optional: false,
                    is_rest: false,
                },
            )]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(15)).into(),
            ),
        })),
        crate::TypeData::instance_of(crate::TypeInstance {
            ty: crate::globals::GLOBAL_SET_ID.into(),
            type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(11)).into()]),
        }),
        crate::TypeData::Function(Box::new(crate::Function {
            is_async: false,
            type_parameters: Box::default(),
            name: None,
            parameters: Box::new([
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("value"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                    is_optional: false,
                    is_rest: false,
                }),
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("value2"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                    is_optional: false,
                    is_rest: false,
                }),
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("set"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
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
                    ty: crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
                    is_optional: false,
                    is_rest: false,
                }),
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("thisArg"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(19)).into(),
                    is_optional: true,
                    is_rest: false,
                }),
            ]),
            return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
        })),
        crate::TypeData::Boolean,
        crate::TypeData::Function(Box::new(crate::Function {
            is_async: false,
            type_parameters: Box::default(),
            name: Some(biome_rowan::Text::new_static("has")),
            parameters: Box::new([crate::FunctionParameter::Named(
                crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("value"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                    is_optional: false,
                    is_rest: false,
                },
            )]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(21)).into(),
            ),
        })),
        crate::TypeData::from(crate::GenericTypeParameter {
            name: biome_rowan::Text::new_static("K"),
            constraint: crate::TypeReference::unknown(),
            default: crate::TypeReference::unknown(),
        }),
        crate::TypeData::from(crate::GenericTypeParameter {
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
                    ty: crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
                    is_optional: false,
                    is_rest: false,
                },
            )]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(26)).into(),
            ),
        })),
        crate::TypeData::instance_of(crate::TypeInstance {
            ty: crate::globals::GLOBAL_MAP_ID.into(),
            type_parameters: Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
            ]),
        }),
        crate::TypeData::Function(Box::new(crate::Function {
            is_async: false,
            type_parameters: Box::default(),
            name: None,
            parameters: Box::new([
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("value"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
                    is_optional: false,
                    is_rest: false,
                }),
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("key"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
                    is_optional: false,
                    is_rest: false,
                }),
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("map"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(28)).into(),
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
                    ty: crate::RawTypeId::Local(crate::TypeId::new(29)).into(),
                    is_optional: false,
                    is_rest: false,
                }),
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("thisArg"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(30)).into(),
                    is_optional: true,
                    is_rest: false,
                }),
            ]),
            return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
        })),
        crate::TypeData::Undefined,
        crate::TypeData::Union(Box::new(crate::Union(Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(32)).into(),
        ])))),
        crate::TypeData::Function(Box::new(crate::Function {
            is_async: false,
            type_parameters: Box::default(),
            name: Some(biome_rowan::Text::new_static("get")),
            parameters: Box::new([crate::FunctionParameter::Named(
                crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("key"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
                    is_optional: false,
                    is_rest: false,
                },
            )]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(33)).into(),
            ),
        })),
        crate::TypeData::Boolean,
        crate::TypeData::Function(Box::new(crate::Function {
            is_async: false,
            type_parameters: Box::default(),
            name: Some(biome_rowan::Text::new_static("has")),
            parameters: Box::new([crate::FunctionParameter::Named(
                crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("key"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
                    is_optional: false,
                    is_rest: false,
                },
            )]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(35)).into(),
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
                    ty: crate::RawTypeId::Local(crate::TypeId::new(23)).into(),
                    is_optional: false,
                    is_rest: false,
                }),
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("value"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(24)).into(),
                    is_optional: false,
                    is_rest: false,
                }),
            ]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(37)).into(),
            ),
        })),
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
                    ty: crate::RawTypeId::Local(crate::TypeId::new(81)).into(),
                    is_optional: true,
                    is_rest: false,
                },
            )]),
            return_type: crate::ReturnType::Type(crate::globals::GLOBAL_STRING_ID.into()),
        })),
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
                crate::RawTypeId::Local(crate::TypeId::new(83)).into(),
            ),
        })),
        crate::TypeData::Boolean,
        crate::TypeData::Boolean,
        crate::TypeData::Boolean,
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
                crate::RawTypeId::Local(crate::TypeId::new(88)).into(),
            ),
        })),
        crate::TypeData::from(crate::GenericTypeParameter {
            name: biome_rowan::Text::new_static("TYield"),
            constraint: crate::TypeReference::unknown(),
            default: crate::TypeReference::unknown(),
        }),
        crate::TypeData::Literal(Box::new(crate::Literal::Boolean(false.into()))),
        crate::TypeData::from(crate::GenericTypeParameter {
            name: biome_rowan::Text::new_static("TReturn"),
            constraint: crate::TypeReference::unknown(),
            default: crate::TypeReference::unknown(),
        }),
        crate::TypeData::Literal(Box::new(crate::Literal::Boolean(true.into()))),
        crate::TypeData::from(crate::GenericTypeParameter {
            name: biome_rowan::Text::new_static("T"),
            constraint: crate::TypeReference::unknown(),
            default: crate::TypeReference::unknown(),
        }),
        crate::TypeData::AnyKeyword,
        crate::TypeData::from(crate::GenericTypeParameter {
            name: biome_rowan::Text::new_static("TReturn"),
            constraint: crate::TypeReference::unknown(),
            default: crate::RawTypeId::Local(crate::TypeId::new(95)).into(),
        }),
        crate::TypeData::instance_of(crate::TypeInstance {
            ty: crate::globals::GLOBAL_ITERATOR_YIELD_RESULT_ID.into(),
            type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(94)).into()]),
        }),
        crate::TypeData::instance_of(crate::TypeInstance {
            ty: crate::globals::GLOBAL_ITERATOR_RETURN_RESULT_ID.into(),
            type_parameters: Box::new([crate::RawTypeId::Local(crate::TypeId::new(96)).into()]),
        }),
        crate::TypeData::Union(Box::new(crate::Union(Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(97)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(98)).into(),
        ])))),
        crate::TypeData::from(crate::GenericTypeParameter {
            name: biome_rowan::Text::new_static("T"),
            constraint: crate::TypeReference::unknown(),
            default: crate::TypeReference::unknown(),
        }),
        crate::TypeData::AnyKeyword,
        crate::TypeData::from(crate::GenericTypeParameter {
            name: biome_rowan::Text::new_static("TReturn"),
            constraint: crate::TypeReference::unknown(),
            default: crate::RawTypeId::Local(crate::TypeId::new(101)).into(),
        }),
        crate::TypeData::AnyKeyword,
        crate::TypeData::from(crate::GenericTypeParameter {
            name: biome_rowan::Text::new_static("TNext"),
            constraint: crate::TypeReference::unknown(),
            default: crate::RawTypeId::Local(crate::TypeId::new(103)).into(),
        }),
        crate::TypeData::from(crate::Tuple(Box::new([]))),
        crate::TypeData::from(crate::Tuple(Box::new([crate::TupleElementType {
            ty: crate::RawTypeId::Local(crate::TypeId::new(104)).into(),
            name: None,
            is_optional: false,
            is_rest: false,
        }]))),
        crate::TypeData::Union(Box::new(crate::Union(Box::new([
            crate::RawTypeId::Local(crate::TypeId::new(105)).into(),
            crate::RawTypeId::Local(crate::TypeId::new(106)).into(),
        ])))),
        crate::TypeData::instance_of(crate::TypeInstance {
            ty: crate::globals::GLOBAL_ITERATOR_RESULT_ID.into(),
            type_parameters: Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(100)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(102)).into(),
            ]),
        }),
        crate::TypeData::Function(Box::new(crate::Function {
            is_async: false,
            type_parameters: Box::default(),
            name: Some(biome_rowan::Text::new_static("next")),
            parameters: Box::new([crate::FunctionParameter::Pattern(
                crate::PatternFunctionParameter {
                    bindings: Box::default(),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(107)).into(),
                    is_optional: false,
                    is_rest: true,
                },
            )]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(108)).into(),
            ),
        })),
        crate::TypeData::instance_of(crate::TypeInstance {
            ty: crate::globals::GLOBAL_ITERATOR_RESULT_ID.into(),
            type_parameters: Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(100)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(102)).into(),
            ]),
        }),
        crate::TypeData::Function(Box::new(crate::Function {
            is_async: false,
            type_parameters: Box::default(),
            name: Some(biome_rowan::Text::new_static("return")),
            parameters: Box::new([crate::FunctionParameter::Named(
                crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("value"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(102)).into(),
                    is_optional: true,
                    is_rest: false,
                },
            )]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(110)).into(),
            ),
        })),
        crate::TypeData::AnyKeyword,
        crate::TypeData::instance_of(crate::TypeInstance {
            ty: crate::globals::GLOBAL_ITERATOR_RESULT_ID.into(),
            type_parameters: Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(100)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(102)).into(),
            ]),
        }),
        crate::TypeData::Function(Box::new(crate::Function {
            is_async: false,
            type_parameters: Box::default(),
            name: Some(biome_rowan::Text::new_static("throw")),
            parameters: Box::new([crate::FunctionParameter::Named(
                crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("e"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(112)).into(),
                    is_optional: true,
                    is_rest: false,
                },
            )]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(113)).into(),
            ),
        })),
    ])
}
