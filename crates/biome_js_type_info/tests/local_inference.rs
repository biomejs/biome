mod utils;

use biome_js_semantic::ScopeId;
use biome_js_type_info::{
    GlobalsResolver, TypeData, TypeMemberKind, TypeResolver, interned_types::well_known_symbol_name,
};

use utils::{
    assert_type_data_snapshot, assert_typed_bindings_snapshot, get_class_declaration,
    get_expression, get_function_declaration, get_type_alias_declaration, get_variable_declaration,
    parse_ts,
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
fn infer_readonly_type_members() {
    const CODE: &str = r#"type Value = { readonly name: string; readonly [key: string]: number };"#;

    let root = parse_ts(CODE);
    let decl = get_type_alias_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let data = TypeData::from_ts_type_alias_declaration(&mut resolver, ScopeId::GLOBAL, &decl)
        .expect("type alias should resolve");
    let TypeData::Object(object) = data else {
        panic!("expected object type");
    };
    let name = object
        .members
        .iter()
        .find(|member| member.has_name("name"))
        .expect("name member");
    assert!(name.is_readonly());
    let index = object
        .members
        .iter()
        .find(|member| matches!(member.kind, TypeMemberKind::ReadonlyIndexSignature(_)))
        .expect("index signature member");
    assert!(index.is_readonly());
}

#[test]
fn infer_readonly_class_property() {
    const CODE: &str = r#"class Example { readonly value: string; }"#;

    let root = parse_ts(CODE);
    let decl = get_class_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let class_data = TypeData::from_js_class_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    let TypeData::Class(class) = class_data else {
        panic!("expected class type");
    };
    let value = class
        .members
        .iter()
        .find(|member| member.has_name("value"))
        .expect("value member");
    assert!(value.is_readonly());
}

#[test]
fn infer_readonly_constructor_parameter_property() {
    const CODE: &str = r#"class Example { constructor(readonly value: string) {} }"#;

    let root = parse_ts(CODE);
    let decl = get_class_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let class_data = TypeData::from_js_class_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    let TypeData::Class(class) = class_data else {
        panic!("expected class type");
    };
    let value = class
        .members
        .iter()
        .find(|member| member.has_name("value"))
        .expect("value member");
    assert!(value.is_readonly());
}

#[test]
fn infer_readonly_static_optional_class_property() {
    const CODE: &str = r#"class Example { static readonly value?: string; }"#;

    let root = parse_ts(CODE);
    let decl = get_class_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let class_data = TypeData::from_js_class_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    let TypeData::Class(class) = class_data else {
        panic!("expected class type");
    };
    let value = class
        .members
        .iter()
        .find(|member| member.has_name("value"))
        .expect("value member");
    assert!(value.is_static());
    assert!(value.is_readonly());
    assert!(value.is_optional());
}

#[test]
fn infer_readonly_static_computed_class_property() {
    const CODE: &str = r#"class Example { static readonly [Symbol.iterator]: string; }"#;

    let root = parse_ts(CODE);
    let decl = get_class_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let class_data = TypeData::from_js_class_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    let TypeData::Class(class) = class_data else {
        panic!("expected class type");
    };
    let member = class.members.first().expect("computed member");
    assert!(member.is_static());
    assert!(member.is_readonly());
    assert!(member.is_keyed_member_with_ty(|_| true));
}

#[test]
fn infer_readonly_computed_type_member() {
    const CODE: &str = r#"type Value = { readonly [Symbol.iterator]: string; }"#;

    let root = parse_ts(CODE);
    let decl = get_type_alias_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let data = TypeData::from_ts_type_alias_declaration(&mut resolver, ScopeId::GLOBAL, &decl)
        .expect("type alias should resolve");
    let TypeData::Object(object) = data else {
        panic!("expected object type");
    };
    let member = object.members.first().expect("computed member");
    assert!(member.is_readonly());
    assert!(member.is_keyed_member_with_ty(|_| true));
}

#[test]
fn infer_optional_computed_type_members() {
    const CODE: &str =
        r#"type Value = { readonly [Symbol.asyncDispose]?: string; [Symbol.dispose]?(): void; };"#;

    let root = parse_ts(CODE);
    let decl = get_type_alias_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let data = TypeData::from_ts_type_alias_declaration(&mut resolver, ScopeId::GLOBAL, &decl)
        .expect("type alias should resolve");
    let TypeData::Object(object) = data else {
        panic!("expected object type");
    };
    let [property, method] = object.members.as_ref() else {
        panic!("expected two computed members");
    };

    assert!(property.is_optional());
    assert!(property.is_readonly());
    assert!(method.is_optional());
    assert!(property.is_keyed_member_with_ty(|key_type| {
        well_known_symbol_name(key_type).is_some_and(|name| name.text() == "Symbol.asyncDispose")
    }));
    assert!(method.is_keyed_member_with_ty(|key_type| {
        well_known_symbol_name(key_type).is_some_and(|name| name.text() == "Symbol.dispose")
    }));

    let method_type = resolver
        .resolve_and_get(&method.ty)
        .expect("optional method type should resolve");
    let TypeData::Union(method_type) = method_type.as_raw_data() else {
        panic!("optional method type should include undefined");
    };
    assert!(method_type.types().iter().any(|variant| {
        resolver
            .resolve_and_get(variant)
            .is_some_and(|variant| matches!(variant.as_raw_data(), TypeData::Undefined))
    }));
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
