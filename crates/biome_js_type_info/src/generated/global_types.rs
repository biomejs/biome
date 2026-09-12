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
        members: Box::new([crate::TypeMember {
            kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("exec")),
            ty: crate::globals::GLOBAL_REGEXP_EXEC_ID.into(),
        }]),
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
        members: Box::new([]),
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
    ])
}
