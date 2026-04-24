mod utils;

use biome_js_type_info::{
    ConditionalType, FunctionParameter, GlobalsResolver, ScopeId, TypeData, TypeResolver,
    reference_to_falsy_subset_of,
};

use utils::{
    assert_type_data_snapshot, get_function_declaration, get_variable_declaration, parse_ts,
};

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

/// Helper: builds a function from `CODE`, resolves its single parameter's
/// type, and returns the conditional semantics of that parameter type.
fn conditional_type_of_first_param(code: &str) -> ConditionalType {
    let root = parse_ts(code);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    resolver.run_inference();

    let function = match &ty {
        TypeData::Function(function) => function,
        _ => panic!("expected function type, got: {ty:?}"),
    };
    let param = function
        .parameters
        .first()
        .expect("expected at least one parameter");
    let param_ref = match param {
        FunctionParameter::Named(named) => named.ty.clone(),
        FunctionParameter::Pattern(pattern) => pattern.ty.clone(),
    };

    let resolved = resolver
        .resolve_and_get(&param_ref)
        .expect("parameter type must resolve");
    ConditionalType::from_resolved_data(resolved, &resolver)
}

#[test]
fn optional_object_param_is_not_truthy() {
    const CODE: &str = r#"function foo(user?: { name: string }) {}"#;
    let conditional = conditional_type_of_first_param(CODE);
    assert!(
        !conditional.is_truthy(),
        "optional object parameter should not be classified as truthy, got {conditional:?}"
    );
    assert!(
        !conditional.is_nullish(),
        "optional object parameter should not be classified as nullish, got {conditional:?}"
    );
}

#[test]
fn optional_object_keyword_param_is_not_truthy() {
    const CODE: &str = r#"function foo(value?: object) {}"#;
    let conditional = conditional_type_of_first_param(CODE);
    assert!(
        !conditional.is_truthy(),
        "optional `object` keyword parameter should not be classified as truthy, got {conditional:?}"
    );
    assert!(
        !conditional.is_nullish(),
        "optional `object` keyword parameter should not be classified as nullish, got {conditional:?}"
    );
}

#[test]
fn optional_string_param_is_not_truthy() {
    // Primitive sanity check — this already works; keep as a regression guard.
    const CODE: &str = r#"function foo(name?: string) {}"#;
    let conditional = conditional_type_of_first_param(CODE);
    assert!(
        !conditional.is_truthy(),
        "optional primitive parameter should not be classified as truthy, got {conditional:?}"
    );
}
