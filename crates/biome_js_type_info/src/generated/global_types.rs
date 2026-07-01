//! This is a generated file. Don't modify it by hand! Run 'just gen-global-types' to re-generate the file.

// Generated from microsoft/TypeScript v6.0.3 (git commit 050880ce59e30b356b686bd3144efe24f875ebc8).

/// Predefined global IDs whose `TypeData` is supplied by this generated module.
pub(crate) const MIGRATED_PREDEFINED_IDS: &[crate::globals::GlobalTypeId] = &[
    crate::globals::ERROR_ID_GLOBAL_TYPE_ID,
    crate::globals::ERROR_CONSTRUCTOR_ID_GLOBAL_TYPE_ID,
    crate::globals::ERROR_CALL_ID_GLOBAL_TYPE_ID,
];

/// Registers all generated global type data into the resolver builder.
pub(crate) fn set_generated_global_type_data(
    builder: &mut crate::globals_builder::GlobalsResolverBuilder,
) {
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
