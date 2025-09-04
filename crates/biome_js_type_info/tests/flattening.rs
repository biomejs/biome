mod utils;

use biome_js_type_info::{GlobalsResolver, ScopeId, TypeData, TypeResolver};

use utils::{
    HardcodedSymbolResolver, assert_type_data_snapshot, assert_typed_bindings_snapshot,
    get_expression, get_function_declaration, get_interface_declaration, get_variable_declaration,
    parse_ts,
};

#[test]
fn infer_flattened_type_of_typeof_expression() {
    const CODE: &str = r#"const foo = "foo";

typeof foo
"#;

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

    let expr = get_expression(&root);
    let mut resolver = HardcodedSymbolResolver::new("foo", var_ty, resolver);
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_flattened_type_of_typeof_expression",
    )
}

#[test]
fn infer_flattened_type_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    let ty = ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_flattened_type_of_promise_returning_function",
    )
}

#[test]
fn infer_flattened_type_of_async_function() {
    const CODE: &str = r#"async function returnsPromise(): Promise<string> {
	return "value";
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    let ty = ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_flattened_type_of_async_function",
    )
}

#[test]
fn infer_flattened_type_from_invocation_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}

returnsPromise()"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let function_ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    let function_ty = function_ty.inferred(&mut resolver);

    let expr = get_expression(&root);
    let mut resolver = HardcodedSymbolResolver::new("returnsPromise", function_ty, resolver);
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_flattened_type_from_invocation_of_promise_returning_function",
    )
}

#[test]
fn infer_flattened_type_from_chained_invocation_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}

returnsPromise().then(() => {})"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let function_ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    resolver.run_inference();

    let expr = get_expression(&root);
    let mut resolver = HardcodedSymbolResolver::new("returnsPromise", function_ty, resolver);
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    resolver.run_inference();

    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_flattened_type_from_chained_invocation_of_promise_returning_function",
    )
}

#[test]
fn infer_flattened_type_from_double_chained_invocation_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}

returnsPromise().then(() => {}).finally(() => {})"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let function_ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    resolver.run_inference();

    let expr = get_expression(&root);
    let mut resolver = HardcodedSymbolResolver::new("returnsPromise", function_ty, resolver);
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    resolver.run_inference();

    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_flattened_type_from_double_chained_invocation_of_promise_returning_function",
    )
}

#[test]
fn infer_flattened_type_from_direct_promise_instance() {
    const CODE: &str = r#"new Promise((resolve) => resolve("value"))"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = GlobalsResolver::default();
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_flattened_type_from_direct_promise_instance",
    )
}

#[test]
fn infer_flattened_type_from_static_promise_function() {
    const CODE: &str = r#"Promise.resolve("value")"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = GlobalsResolver::default();
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    resolver.run_inference();

    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_flattened_type_from_static_promise_function",
    )
}

#[test]
fn infer_flattened_type_of_destructured_array_element() {
    const CODE: &str = r#"const [a]: Array<string> = [];"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    resolver.run_inference();

    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        &resolver,
        "infer_flattened_type_of_destructured_array_element",
    );
}

#[test]
fn infer_flattened_type_of_destructured_interface_field() {
    const CODE: &str = r#"interface Foo {
    foo(): string;
}

function bar({ foo }: Foo) {
}"#;

    let root = parse_ts(CODE);
    let decl = get_interface_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let interface_ty =
        TypeData::from_ts_interface_declaration(&mut resolver, ScopeId::GLOBAL, &decl)
            .expect("interface must be inferred");
    resolver.run_inference();

    let function_decl = get_function_declaration(&root);
    let mut resolver = HardcodedSymbolResolver::new("Foo", interface_ty, resolver);
    let function_decl =
        TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &function_decl);
    resolver.run_inference();

    let expr_ty = function_decl.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_flattened_type_of_destructured_interface_field",
    )
}
