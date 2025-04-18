mod utils;

use biome_js_syntax::{AnyJsModuleItem, AnyJsRoot, AnyJsStatement, JsExpressionStatement};
use biome_js_type_info::Type;

use utils::{
    GlobalsResolver, HardcodedSymbolResolver, assert_type_snapshot, assert_typed_bindings_snapshot,
    get_function_declaration, get_variable_declaration, parse_ts,
};

#[test]
fn infer_flattened_type_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut ty = Type::from_js_function_declaration(&decl);
    ty.resolve(&GlobalsResolver);

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
    ty.resolve(&GlobalsResolver);

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
    function_ty.resolve(&GlobalsResolver);

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
    function_ty.resolve(&GlobalsResolver);

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
    function_ty.resolve(&GlobalsResolver);

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
    expr_ty.resolve(&GlobalsResolver);

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
    expr_ty.resolve(&GlobalsResolver);

    assert_type_snapshot(
        CODE,
        expr_ty,
        "infer_flattened_type_from_static_promise_function",
    )
}

#[test]
fn infer_flattened_type_of_destructured_array_element() {
    const CODE: &str = r#"const [a]: Array<string> = [];"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let resolver = GlobalsResolver;
    let mut bindings = Type::typed_bindings_from_js_variable_declaration(&decl);
    for (_name, binding) in &mut bindings {
        binding.resolve(&resolver);
    }

    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        "infer_flattened_type_of_destructured_array_element",
    );
}

pub fn get_expression_statement(root: &AnyJsRoot) -> JsExpressionStatement {
    let module = root.as_js_module().unwrap();
    module
        .items()
        .into_iter()
        .filter_map(|item| match item {
            AnyJsModuleItem::AnyJsStatement(statement) => Some(statement),
            _ => None,
        })
        .find_map(|statement| match statement {
            AnyJsStatement::JsExpressionStatement(expr) => Some(expr),
            _ => None,
        })
        .expect("cannot find expression statement")
}
