mod utils;

use biome_js_type_info::{
    GlobalsResolver, ScopeId, TypeData, TypeResolver, reference_to_falsy_subset_of,
};

use utils::{assert_type_data_snapshot, get_variable_declaration, parse_ts};

#[test]
fn test_reference_to_falsy_subset_of() {
    const CODE: &str = r#"let foo: undefined | null | number = 1;"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    resolver.run_inference();

    let (var_name, var_ty) = bindings.into_vec().remove(0);
    assert_eq!(var_name.text(), "foo");
    let var_ty = resolver
        .resolve_and_get(&var_ty)
        .expect("must resolve")
        .to_data();

    let falsy_subset_of_ty = reference_to_falsy_subset_of(&var_ty, &mut resolver);
    let falsy_subset_of_ty = resolver
        .resolve_and_get(&falsy_subset_of_ty.unwrap())
        .expect("must resolve")
        .to_data();

    assert_type_data_snapshot(
        CODE,
        &falsy_subset_of_ty,
        &resolver,
        "test_reference_to_falsy_subset_of",
    )
}
