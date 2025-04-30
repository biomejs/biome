mod utils;

use biome_js_type_info::GlobalsResolver;
use biome_js_type_info::TypeData;

use utils::assert_typed_bindings_snapshot;
use utils::get_variable_declaration;
use utils::{assert_type_data_snapshot, get_function_declaration, parse_ts};

#[test]
fn infer_type_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, &decl);
    assert_type_data_snapshot(
        CODE,
        ty,
        &resolver,
        "infer_type_of_promise_returning_function",
    );
}

#[test]
fn infer_type_of_async_function() {
    const CODE: &str = r#"async function returnsPromise(): Promise<string> {
	return "value";
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, &decl);
    assert_type_data_snapshot(CODE, ty, &resolver, "infer_type_of_async_function");
}

#[test]
fn infer_type_of_array() {
    const CODE: &str = r#"const array: Array<string> = [];"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(&mut resolver, &decl);
    assert_typed_bindings_snapshot(CODE, &bindings, &resolver, "infer_type_of_array");
}

#[test]
fn infer_type_of_destructured_array_element() {
    const CODE: &str = r#"const [a]: Array<string> = [];"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(&mut resolver, &decl);
    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        &resolver,
        "infer_type_of_destructured_array_element",
    );
}

#[test]
fn infer_type_of_function_with_destructured_arguments() {
    const CODE: &str = r#"function destruct({ a, b }: { a: number, b: string }, [first, ...rest]: Array<boolean>) {}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, &decl);
    assert_type_data_snapshot(
        CODE,
        ty,
        &resolver,
        "infer_type_of_function_with_destructured_arguments",
    );
}

#[test]
fn infer_type_of_literal() {
    const CODE: &str = r#"const a = 123.45;"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(&mut resolver, &decl);
    assert_typed_bindings_snapshot(CODE, &bindings, &resolver, "infer_type_of_literal");
}

#[test]
fn infer_type_of_binary_expression_eq() {
    const CODE: &str = r#"const a = 1 === 1"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(&mut resolver, &decl);
    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        &resolver,
        "infer_type_of_binary_expression_eq",
    );
}

#[test]
fn infer_type_of_binary_expression_ne() {
    const CODE: &str = r#"const a = 0 !== 1"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(&mut resolver, &decl);
    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        &resolver,
        "infer_type_of_binary_expression_ne",
    );
}
