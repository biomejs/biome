use biome_js_type_info::{ModuleId, ResolvedTypeId, TypeId, TypeResolverLevel};

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

    let id = ResolvedTypeId::new(TypeResolverLevel::Module, TypeId::new(3));
    let id = id.with_module_id(ModuleId::new(5));
    assert_eq!(id.level(), TypeResolverLevel::Module);
    assert_eq!(id.id(), TypeId::new(3));
    assert_eq!(id.module_id(), ModuleId::new(5));

    let id = id.with_module_id(ModuleId::new(7));
    assert_eq!(id.level(), TypeResolverLevel::Module);
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
        std::mem::size_of::<biome_js_type_info::TypeData>(),
        16,
        "`TypeData` should not be bigger than 16 bytes"
    );
}
