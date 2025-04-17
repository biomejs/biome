use crate::utils::{
    HardcodedSymbolResolver, PromiseResolver, assert_type_snapshot, get_expression_statement,
    get_function_declaration, parse_ts,
};
use biome_js_type_info::Type;

mod utils;

#[test]
fn infer_flattened_type_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut ty = Type::from_js_function_declaration(&decl);
    ty.resolve(&PromiseResolver);

    assert_type_snapshot(
        CODE,
        ty,
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
    let mut ty = Type::from_js_function_declaration(&decl);
    ty.resolve(&PromiseResolver);

    assert_type_snapshot(CODE, ty, "infer_flattened_type_of_async_function")
}

#[test]
fn infer_flattened_type_from_invocation_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}

returnsPromise()"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut function_ty = Type::from_js_function_declaration(&decl);
    function_ty.resolve(&PromiseResolver);

    let expr = get_expression_statement(&root);
    let mut expr_ty = Type::from_any_js_expression(&expr.expression().unwrap());
    expr_ty.resolve(&HardcodedSymbolResolver("returnsPromise", function_ty));

    assert_type_snapshot(
        CODE,
        expr_ty,
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
    let mut function_ty = Type::from_js_function_declaration(&decl);
    function_ty.resolve(&PromiseResolver);

    let expr = get_expression_statement(&root);
    let mut expr_ty = Type::from_any_js_expression(&expr.expression().unwrap());
    expr_ty.resolve(&HardcodedSymbolResolver("returnsPromise", function_ty));

    assert_type_snapshot(
        CODE,
        expr_ty,
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
    let mut function_ty = Type::from_js_function_declaration(&decl);
    function_ty.resolve(&PromiseResolver);

    let expr = get_expression_statement(&root);
    let mut expr_ty = Type::from_any_js_expression(&expr.expression().unwrap());
    expr_ty.resolve(&HardcodedSymbolResolver("returnsPromise", function_ty));

    assert_type_snapshot(
        CODE,
        expr_ty,
        "infer_flattened_type_from_double_chained_invocation_of_promise_returning_function",
    )
}

#[test]
fn infer_flattened_type_from_direct_promise_instance() {
    const CODE: &str = r#"new Promise((resolve) => resolve("value"))"#;

    let root = parse_ts(CODE);
    let expr = get_expression_statement(&root);
    let mut expr_ty = Type::from_any_js_expression(&expr.expression().unwrap());
    expr_ty.resolve(&PromiseResolver);

    assert_type_snapshot(
        CODE,
        expr_ty,
        "infer_flattened_type_from_direct_promise_instance",
    )
}

#[test]
fn infer_flattened_type_from_static_promise_function() {
    const CODE: &str = r#"Promise.resolve("value")"#;

    let root = parse_ts(CODE);
    let expr = get_expression_statement(&root);
    let mut expr_ty = Type::from_any_js_expression(&expr.expression().unwrap());
    expr_ty.resolve(&PromiseResolver);

    assert_type_snapshot(
        CODE,
        expr_ty,
        "infer_flattened_type_from_static_promise_function",
    )
}
