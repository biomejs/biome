use biome_js_type_info::*;

#[test]
fn test_resolved_type_id() {
    let id = ResolvedTypeId::new(TypeResolverLevel::Global, TypeId::new(3));
    assert_eq!(id.level(), TypeResolverLevel::Global);
    assert_eq!(id.id(), TypeId::new(3));
    assert_eq!(id.module_id(), ModuleId::new(0));

    let id = id.with_module_id(ModuleId::new(5));
    assert_eq!(id.level(), TypeResolverLevel::Global);
    assert_eq!(id.id(), TypeId::new(3));
    assert_eq!(id.module_id(), ModuleId::new(0)); // Module ID shouldn't be applied to global level.

    let id = ResolvedTypeId::new(TypeResolverLevel::Thin, TypeId::new(3));
    let id = id.with_module_id(ModuleId::new(5));
    assert_eq!(id.level(), TypeResolverLevel::Thin);
    assert_eq!(id.id(), TypeId::new(3));
    assert_eq!(id.module_id(), ModuleId::new(5));

    let id = id.with_module_id(ModuleId::new(7));
    assert_eq!(id.level(), TypeResolverLevel::Thin);
    assert_eq!(id.id(), TypeId::new(3));
    assert_eq!(id.module_id(), ModuleId::new(7));
}

#[test]
fn verify_type_sizes() {
    assert_eq!(
        std::mem::size_of::<ResolvedTypeId>(),
        8,
        "`ResolvedTypeId` should not be bigger than 8 bytes"
    );

    #[cfg(target_pointer_width = "64")]
    assert_eq!(
        std::mem::size_of::<TypeData>(),
        16,
        "`TypeData` should not be bigger than 16 bytes"
    );

    assert_eq!(
        std::mem::size_of::<TypeReference>(),
        16,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeReferenceQualifier>(),
        56,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeImportQualifier>(),
        40,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeMember>(),
        40,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofExpression>(),
        48,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofAdditionExpression>(),
        32,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofAwaitExpression>(),
        16,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofBitwiseNotExpression>(),
        16,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofCallExpression>(),
        32,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofDestructureExpression>(),
        40,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofNewExpression>(),
        32,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofStaticMemberExpression>(),
        32,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofThisOrSuperExpression>(),
        16,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofTypeofExpression>(),
        16,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeofUnaryMinusExpression>(),
        16,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeOperatorType>(),
        24,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeOperator>(),
        1,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<Function>(),
        80,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<Union>(),
        16,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<GenericTypeParameter>(),
        48,
        "The size shouldn't go higher"
    );

    assert_eq!(
        std::mem::size_of::<TypeInstance>(),
        32,
        "The size shouldn't go higher"
    );
}
