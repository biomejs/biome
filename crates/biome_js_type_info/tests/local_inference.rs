mod utils;

use biome_js_semantic::ScopeId;
use biome_js_type_info::{GlobalsResolver, TypeData, TypeResolver};

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
fn infer_type_of_regex() {
    const CODE: &str = r#"/ab+c/"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_regex");
}

#[test]
fn infer_type_of_regex_with_flags() {
    const CODE: &str = r#"/ab+c/gi"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_regex_with_flags");
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
fn infer_type_of_const_assertion() {
    const CODE: &str = r#""value" as const"#;

    let syntax_tree = parse_ts(CODE);
    let expression = get_expression(&syntax_tree);
    let mut resolver = GlobalsResolver::default();
    let expression_type =
        TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expression);
    assert_eq!(expression_type.to_string(), "string: value");
}

#[test]
fn const_assertion_marks_object_property_as_const_asserted() {
    const CODE: &str =
        r#"({ value: "x" as const, nested: { flag: true as const }, paren: ("y" as const) })"#;

    let syntax_tree = parse_ts(CODE);
    let expression = get_expression(&syntax_tree);
    let mut resolver = GlobalsResolver::default();
    let expression_type =
        TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expression);
    let TypeData::Object(object) = expression_type else {
        panic!("expected object type");
    };

    let value = object
        .members
        .iter()
        .find(|member| member.has_name("value"))
        .expect("value member");
    assert!(value.is_const_asserted());
    let value_type = resolver
        .resolve_and_get(&value.ty)
        .expect("value type")
        .to_data();
    assert_eq!(value_type.to_string(), "string: x");

    let nested = object
        .members
        .iter()
        .find(|member| member.has_name("nested"))
        .expect("nested member");
    assert!(!nested.is_const_asserted());
    let nested_type = resolver
        .resolve_and_get(&nested.ty)
        .expect("nested type")
        .to_data();
    let TypeData::Object(nested_object) = nested_type else {
        panic!("expected nested object type");
    };
    let flag = nested_object
        .members
        .iter()
        .find(|member| member.has_name("flag"))
        .expect("flag member");
    assert!(flag.is_const_asserted());
    let flag_type = resolver
        .resolve_and_get(&flag.ty)
        .expect("flag type")
        .to_data();
    assert_eq!(flag_type.to_string(), "bool: true");

    let parenthesized = object
        .members
        .iter()
        .find(|member| member.has_name("paren"))
        .expect("paren member");
    assert!(parenthesized.is_const_asserted());
}

#[test]
fn const_assertion_marks_nested_object_members_as_const_asserted() {
    const CODE: &str = r#"({ value: { inner: "x" } } as const)"#;

    let syntax_tree = parse_ts(CODE);
    let expression = get_expression(&syntax_tree);
    let mut resolver = GlobalsResolver::default();
    let expression_type =
        TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expression);
    let TypeData::Object(object) = expression_type else {
        panic!("expected object type");
    };

    let value = object
        .members
        .iter()
        .find(|member| member.has_name("value"))
        .expect("value member");
    assert!(value.is_const_asserted());
    let value_type = resolver
        .resolve_and_get(&value.ty)
        .expect("value type")
        .to_data();
    let TypeData::Object(value_object) = value_type else {
        panic!("expected nested object type");
    };
    let inner = value_object
        .members
        .iter()
        .find(|member| member.has_name("inner"))
        .expect("inner member");
    assert!(inner.is_const_asserted());
}

#[test]
fn const_assertion_preserves_negative_number_literal() {
    const CODE: &str = r#"-1 as const"#;

    let syntax_tree = parse_ts(CODE);
    let expression = get_expression(&syntax_tree);
    let mut resolver = GlobalsResolver::default();
    let expression_type =
        TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expression);
    assert_eq!(expression_type.to_string(), "number: -1");
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
