// Generated from xtask/codegen/tests/fixtures/global-types/lowering.scalars.d.ts.
// Regenerate with BIOME_GLOBAL_TYPES_UPDATE_FIXTURES=1 cargo test -p xtask_codegen --features global_types --test declaration_lowering declaration_scalar_runtime_fixture_matches_emission
fn scalar_types() -> Box<[crate::TypeData]> {
    Box::new([
        crate::TypeData::Interface(Box::new(crate::Interface {
            name: biome_rowan::Text::new_static("Scalars"),
            type_parameters: Box::default(),
            extends: Box::default(),
            members: Box::new([
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("unchecked")),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("uncertain")),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("impossible")),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("missing")),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("integer")),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("token")),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("enabled")),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("disabled")),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("negative")),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(9)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static("zero")),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(10)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                        "largeInteger",
                    )),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(11)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(biome_rowan::Text::new_static(
                        "negativeInteger",
                    )),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(12)).into(),
                },
            ]),
        })),
        crate::TypeData::AnyKeyword,
        crate::TypeData::UnknownKeyword,
        crate::TypeData::NeverKeyword,
        crate::TypeData::Undefined,
        crate::TypeData::BigInt,
        crate::TypeData::Symbol,
        crate::TypeData::Literal(Box::new(crate::Literal::Boolean(true.into()))),
        crate::TypeData::Literal(Box::new(crate::Literal::Boolean(false.into()))),
        crate::TypeData::Literal(Box::new(crate::Literal::Number(
            crate::literal::NumberLiteral::new(biome_rowan::Text::new_static("-0x2a")),
        ))),
        crate::TypeData::Literal(Box::new(crate::Literal::Number(
            crate::literal::NumberLiteral::new(biome_rowan::Text::new_static("-0")),
        ))),
        crate::TypeData::Literal(Box::new(crate::Literal::BigInt(
            biome_rowan::Text::new_static("9_007_199_254_740_993n"),
        ))),
        crate::TypeData::Literal(Box::new(crate::Literal::BigInt(
            biome_rowan::Text::new_static("-0x2an"),
        ))),
    ])
}
