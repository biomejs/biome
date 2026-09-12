fn types() -> Box<[crate::TypeData]> {
    Box::new([
        crate::TypeData::Interface(
            Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Catalog"),
                type_parameters: Box::default(),
                extends: Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                ]),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(
                            biome_rowan::Text::new_static("item"),
                        ),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(
                            biome_rowan::Text::new_static("selected"),
                        ),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(
                            biome_rowan::Text::new_static("state"),
                        ),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(
                            biome_rowan::Text::new_static("empty"),
                        ),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(
                            biome_rowan::Text::new_static("label"),
                        ),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(
                            biome_rowan::Text::new_static("find"),
                        ),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(18)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(
                            biome_rowan::Text::new_static("update"),
                        ),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(20)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(
                            biome_rowan::Text::new_static("count"),
                        ),
                        ty: crate::globals::GLOBAL_NUMBER_ID.into(),
                    },
                ]),
            }),
        ),
        crate::TypeData::Interface(
            Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Named"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(
                            biome_rowan::Text::new_static("name"),
                        ),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                ]),
            }),
        ),
        crate::TypeData::Interface(
            Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Tagged"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(
                            biome_rowan::Text::new_static("tag"),
                        ),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(21)).into(),
                    },
                ]),
            }),
        ),
        crate::TypeData::Interface(
            Box::new(crate::Interface {
                name: biome_rowan::Text::new_static("Item"),
                type_parameters: Box::default(),
                extends: Box::default(),
                members: Box::new([
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::NamedOptional(
                            biome_rowan::Text::new_static("owner"),
                        ),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    },
                    crate::TypeMember {
                        kind: crate::TypeMemberKind::Named(
                            biome_rowan::Text::new_static("title"),
                        ),
                        ty: crate::globals::GLOBAL_STRING_ID.into(),
                    },
                ]),
            }),
        ),
        crate::TypeData::Null,
        crate::TypeData::Union(
            Box::new(
                crate::Union(
                    Box::new([
                        crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                        crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                    ]),
                ),
            ),
        ),
        crate::TypeData::Boolean,
        crate::TypeData::Literal(
            Box::new(
                crate::Literal::String(biome_rowan::Text::new_static("pending").into()),
            ),
        ),
        crate::TypeData::Union(
            Box::new(
                crate::Union(
                    Box::new([
                        crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                        crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                    ]),
                ),
            ),
        ),
        crate::TypeData::Null,
        crate::TypeData::Literal(
            Box::new(
                crate::Literal::String(biome_rowan::Text::new_static("ready").into()),
            ),
        ),
        crate::TypeData::Null,
        crate::TypeData::Union(
            Box::new(
                crate::Union(
                    Box::new([
                        crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                        crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                    ]),
                ),
            ),
        ),
        crate::TypeData::Boolean,
        crate::TypeData::Literal(
            Box::new(
                crate::Literal::String(biome_rowan::Text::new_static("pending").into()),
            ),
        ),
        crate::TypeData::Union(
            Box::new(
                crate::Union(
                    Box::new([
                        crate::RawTypeId::Local(crate::TypeId::new(13)).into(),
                        crate::RawTypeId::Local(crate::TypeId::new(14)).into(),
                    ]),
                ),
            ),
        ),
        crate::TypeData::Null,
        crate::TypeData::Union(
            Box::new(
                crate::Union(
                    Box::new([
                        crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                        crate::RawTypeId::Local(crate::TypeId::new(16)).into(),
                    ]),
                ),
            ),
        ),
        crate::TypeData::Function(
            Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: Some(biome_rowan::Text::new_static("find")),
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("candidate"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("state"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(15)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::RawTypeId::Local(crate::TypeId::new(17)).into(),
                ),
            }),
        ),
        crate::TypeData::Literal(
            Box::new(
                crate::Literal::String(biome_rowan::Text::new_static("ready").into()),
            ),
        ),
        crate::TypeData::Function(
            Box::new(crate::Function {
                is_async: false,
                type_parameters: Box::default(),
                name: None,
                parameters: Box::new([
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("item"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                        is_optional: false,
                        is_rest: false,
                    }),
                    crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                        name: biome_rowan::Text::new_static("label"),
                        ty: crate::RawTypeId::Local(crate::TypeId::new(19)).into(),
                        is_optional: true,
                        is_rest: false,
                    }),
                ]),
                return_type: crate::ReturnType::Type(
                    crate::globals::GLOBAL_VOID_ID.into(),
                ),
            }),
        ),
        crate::TypeData::Literal(
            Box::new(crate::Literal::String(biome_rowan::Text::new_static("tag").into())),
        ),
    ])
}
