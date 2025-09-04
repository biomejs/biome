mod utils;

use biome_js_type_info::{GlobalsResolver, ScopeId, TypeData};

use utils::{
    assert_type_data_snapshot, assert_typed_bindings_snapshot, get_expression,
    get_function_declaration, get_variable_declaration, parse_ts,
};

#[test]
fn infer_type_of_identifier() {
    const CODE: &str = r#"foo"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_identifier");
}

#[test]
fn infer_type_of_object_member_expression() {
    const CODE: &str = r#"foo.bar"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_type_of_object_member_expression",
    );
}

#[test]
fn infer_type_of_typeof_expression() {
    const CODE: &str = r#"typeof foo"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_typeof_expression");
}

#[test]
fn infer_type_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    assert_type_data_snapshot(
        CODE,
        &ty,
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
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_async_function");
}

#[test]
fn infer_type_of_array() {
    const CODE: &str = r#"const array: Array<string> = [];"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    assert_typed_bindings_snapshot(CODE, &bindings, &resolver, "infer_type_of_array");
}

#[test]
fn infer_type_of_destructured_array_element() {
    const CODE: &str = r#"const [a]: Array<string> = [];"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
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
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    assert_type_data_snapshot(
        CODE,
        &ty,
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
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    assert_typed_bindings_snapshot(CODE, &bindings, &resolver, "infer_type_of_literal");
}

#[test]
fn infer_type_of_binary_expression_eq() {
    const CODE: &str = r#"const a = 1 === 1"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
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
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        &resolver,
        "infer_type_of_binary_expression_ne",
    );
}

#[test]
fn infer_type_of_dynamic_import() {
    const CODE: &str = r#"const a = import("some-module");"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    assert_typed_bindings_snapshot(CODE, &bindings, &resolver, "infer_type_of_dynamic_import");
}
