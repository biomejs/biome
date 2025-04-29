mod utils;

use biome_js_syntax::{AnyJsModuleItem, AnyJsRoot, AnyJsStatement, JsExpressionStatement};
use biome_js_type_info::{GlobalsResolver, TypeData};

use utils::{
    HardcodedSymbolResolver, assert_type_data_snapshot, assert_typed_bindings_snapshot,
    get_function_declaration, get_variable_declaration, parse_ts,
};

#[test]
fn infer_flattened_type_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, &decl);
    let ty = ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        ty,
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
    let ty = TypeData::from_js_function_declaration(&mut resolver, &decl);
    let ty = ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        ty,
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
    let function_ty = TypeData::from_js_function_declaration(&mut resolver, &decl);
    let function_ty = function_ty.inferred(&mut resolver);

    let expr = get_expression_statement(&root);
    let mut resolver = HardcodedSymbolResolver::new("returnsPromise", function_ty, resolver);
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, &expr.expression().unwrap());
    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        expr_ty,
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
    let function_ty = TypeData::from_js_function_declaration(&mut resolver, &decl);
    resolver.run_inference();

    let expr = get_expression_statement(&root);
    let mut resolver = HardcodedSymbolResolver::new("returnsPromise", function_ty, resolver);
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, &expr.expression().unwrap());
    resolver.run_inference();

    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        expr_ty,
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
    let function_ty = TypeData::from_js_function_declaration(&mut resolver, &decl);
    resolver.run_inference();

    let expr = get_expression_statement(&root);
    let mut resolver = HardcodedSymbolResolver::new("returnsPromise", function_ty, resolver);
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, &expr.expression().unwrap());
    resolver.run_inference();

    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        expr_ty,
        &resolver,
        "infer_flattened_type_from_double_chained_invocation_of_promise_returning_function",
    )
}

#[test]
fn infer_flattened_type_from_direct_promise_instance() {
    const CODE: &str = r#"new Promise((resolve) => resolve("value"))"#;

    let root = parse_ts(CODE);
    let expr = get_expression_statement(&root);
    let mut resolver = GlobalsResolver::default();
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, &expr.expression().unwrap());
    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        expr_ty,
        &resolver,
        "infer_flattened_type_from_direct_promise_instance",
    )
}

#[test]
fn infer_flattened_type_from_static_promise_function() {
    const CODE: &str = r#"Promise.resolve("value")"#;

    let root = parse_ts(CODE);
    let expr = get_expression_statement(&root);
    let mut resolver = GlobalsResolver::default();
    let expr_ty = TypeData::from_any_js_expression(&mut resolver, &expr.expression().unwrap());
    resolver.run_inference();

    let expr_ty = expr_ty.inferred(&mut resolver);

    assert_type_data_snapshot(
        CODE,
        expr_ty,
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
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(&mut resolver, &decl);
    resolver.run_inference();

    let bindings: Vec<_> = bindings
        .into_iter()
        .map(|(name, binding)| (name, binding.inferred(&mut resolver)))
        .collect();

    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        &resolver,
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
