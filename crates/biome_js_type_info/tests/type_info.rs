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
    assert_eq!(id.module_id(), ModuleId::new(5));

    let id = id.with_module_id(ModuleId::new(7));
    assert_eq!(id.level(), TypeResolverLevel::Global);
    assert_eq!(id.id(), TypeId::new(3));
    assert_eq!(id.module_id(), ModuleId::new(7));
}
