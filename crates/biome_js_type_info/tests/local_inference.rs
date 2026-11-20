mod utils;

use biome_js_semantic::ScopeId;
use biome_js_syntax::{
    JsArrowFunctionExpression, JsClassDeclaration, JsFormalParameter, TsTypeAliasDeclaration,
};
use biome_js_type_info::{
    Object, ObjectKind, RawTypeCollector, ReturnType, TypeData, TypeMember, TypeReference,
    TypeofExpression,
};
use biome_rowan::{AstNode, Text};

use utils::{
    TestTypeCollector, assert_type_data_snapshot, assert_typed_bindings_snapshot, get_expression,
    get_function_declaration, get_interface_declaration, get_variable_declaration, parse_ts,
};

#[test]
fn infer_type_of_identifier() {
    const CODE: &str = r#"foo"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_identifier");
}

#[test]
fn infer_type_of_indexed_access_preserves_operands() {
    const CODE: &str = "type Element = (typeof values)[Index];";
    let root = parse_ts(CODE);
    let declaration = root
        .syntax()
        .descendants()
        .find_map(TsTypeAliasDeclaration::cast)
        .unwrap();
    let mut collector = TestTypeCollector::default();
    let ty =
        TypeData::from_ts_type_alias_declaration(&mut collector, ScopeId::GLOBAL, &declaration)
            .unwrap();
    assert!(matches!(ty, TypeData::IndexedAccess(_)));
    assert_type_data_snapshot(
        CODE,
        &ty,
        &collector,
        "infer_type_of_indexed_access_preserves_operands",
    );
}

#[test]
fn infer_type_of_conditional_type_preserves_operands() {
    const CODE: &str = "type Unwrap<T> = T extends Promise<infer U extends string> ? [U] : [T] extends [string] ? 1 : 2;";
    let root = parse_ts(CODE);
    let declaration = root
        .syntax()
        .descendants()
        .find_map(TsTypeAliasDeclaration::cast)
        .unwrap();
    let mut collector = TestTypeCollector::default();
    let ty =
        TypeData::from_ts_type_alias_declaration(&mut collector, ScopeId::GLOBAL, &declaration)
            .unwrap();
    assert!(matches!(ty, TypeData::InstanceOf(_)));
    assert_type_data_snapshot(
        CODE,
        &ty,
        &collector,
        "infer_type_of_conditional_type_preserves_operands",
    );
}

#[test]
fn infer_type_of_object_member_expression() {
    const CODE: &str = r#"foo.bar"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = TestTypeCollector::default();
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
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_regex");
}

#[test]
fn infer_type_of_regex_with_flags() {
    const CODE: &str = r#"/ab+c/gi"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_regex_with_flags");
}

#[test]
fn infer_type_of_template_literal() {
    const CODE: &str = r#"`foo`"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_template_literal");
}

#[test]
fn infer_type_of_empty_template_literal() {
    const CODE: &str = r#"``"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_empty_template_literal");
}

#[test]
fn infer_type_of_template_literal_with_substitution() {
    const CODE: &str = r#"`foo ${bar}`"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_type_of_template_literal_with_substitution",
    );
}

#[test]
fn infer_type_of_tagged_template_literal() {
    const CODE: &str = r#"tag`foo`"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_type_of_tagged_template_literal",
    );
}

#[test]
fn infer_type_of_typeof_expression() {
    const CODE: &str = r#"typeof foo"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_typeof_expression");
}

#[test]
fn infer_type_of_const_assertion() {
    const CODE: &str = r#""value" as const"#;

    let syntax_tree = parse_ts(CODE);
    let expression = get_expression(&syntax_tree);
    let mut resolver = TestTypeCollector::default();
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
    let mut resolver = TestTypeCollector::default();
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
        .get_by_reference(&value.ty)
        .expect("value type")
        .clone();
    assert_eq!(value_type.to_string(), "string: x");

    let nested = object
        .members
        .iter()
        .find(|member| member.has_name("nested"))
        .expect("nested member");
    assert!(!nested.is_const_asserted());
    let nested_type = resolver
        .get_by_reference(&nested.ty)
        .expect("nested type")
        .clone();
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
        .get_by_reference(&flag.ty)
        .expect("flag type")
        .clone();
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
    let mut resolver = TestTypeCollector::default();
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
        .get_by_reference(&value.ty)
        .expect("value type")
        .clone();
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
    let mut resolver = TestTypeCollector::default();
    let expression_type =
        TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expression);
    assert_eq!(expression_type.to_string(), "number: -1");
}

#[test]
fn infer_asserts_this_return_type() {
    const CODE: &str = r#"function x(): asserts this {}"#;

    let root = parse_ts(CODE);
    let declaration = get_function_declaration(&root);
    let mut resolver = TestTypeCollector::default();
    let TypeData::Function(function) =
        TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &declaration)
    else {
        panic!("expected function type");
    };
    let ReturnType::Asserts(assertion) = function.return_type else {
        panic!("expected assertion return type");
    };

    assert_eq!(assertion.parameter_name.text(), "this");
}

#[test]
fn const_assertion_depth_limit_degrades_to_unknown() {
    const LIMIT: usize = 50;

    for depth in [LIMIT - 1, LIMIT, LIMIT + 1] {
        let mut expression = String::from("\"leaf\"");
        for _ in 0..depth {
            expression = format!("{{ value: {expression} }}");
        }
        let code = format!("({expression} as const)");
        let syntax_tree = parse_ts(&code);
        let expression = get_expression(&syntax_tree);
        let mut resolver = TestTypeCollector::default();
        let mut ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expression);
        let mut complete = true;

        for _ in 0..depth {
            let TypeData::Object(object) = ty else {
                complete = false;
                break;
            };
            let value = object
                .members
                .iter()
                .find(|member| member.has_name("value"))
                .expect("nested value member");
            if value.ty == TypeReference::unknown() {
                complete = false;
                break;
            }
            ty = resolver
                .get_by_reference(&value.ty)
                .expect("nested value type")
                .clone();
        }

        assert_eq!(
            complete,
            depth < LIMIT,
            "unexpected result at depth {depth}"
        );
    }
}

#[test]
fn infer_type_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = TestTypeCollector::default();
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
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    assert_type_data_snapshot(CODE, &ty, &resolver, "infer_type_of_async_function");
}

#[test]
fn infer_return_type_with_a_reachable_end() {
    // The union holds the value of the first `return`, the `undefined` of the
    // bare `return`, and the marker of the end, whose reachability only the
    // module graph decides. The method's `return` returns from the method.
    const CODE: &str = r#"function f(c: boolean) {
	if (c) return 1;
	if (!c) return;
	const o = { m() { return "x"; } };
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_return_type_with_a_reachable_end",
    );
}

/// Returns whether the return type that collection gives the first function
/// declaration of `code` holds the marker of the end of its body.
fn marks_the_end(code: &str) -> bool {
    let root = parse_ts(code);
    let decl = get_function_declaration(&root);
    let mut collector = TestTypeCollector::default();
    TypeData::from_js_function_declaration(&mut collector, ScopeId::GLOBAL, &decl);
    collector.types.as_slice().iter().any(|ty| {
        matches!(ty, TypeData::TypeofExpression(expression)
            if matches!(expression.as_ref(), TypeofExpression::ImplicitReturn(_)))
    })
}

#[test]
fn ends_that_syntax_closes_get_no_marker() {
    for code in [
        "function f() { switch (x) { case 1: return 1; default: return 2; } }",
        "function f() { switch (x) { default: return 0; case 1: return 1; } }",
        // The first clause falls through to `default`.
        "function f() { switch (x) { case 1: if (c) return 1; default: return 2; } }",
        // Each `break` targets a nested loop or `switch`.
        "function f() { switch (x) { default: for (const v of a) { if (v) break; } return 1; } }",
        "function f() { switch (x) { default: switch (y) { case 1: break; default: return 1; } return 2; } }",
        "function f() { switch (x) { default: while (c) { if (d) continue; break; } return 1; } }",
        // A jump in a nested function stays in that function.
        "function f() { switch (x) { default: g(() => { for (;;) break; }); return 1; } }",
        "function f() { try { return 1; } catch { return 2; } }",
        "function f() { try { if (c) return 1; return 2; } catch (e) { throw e; } finally { log(); } }",
        "function f() { if (c) return 1; return 2; function g() {} }",
        "function f() { if (c) return 1; throw 0; type T = 1; interface I {} declare function h(): void; }",
        "function f() { switch (x) { case 1: return 1; default: return 2; function g() {} } }",
    ] {
        assert!(!marks_the_end(code), "{code}");
    }
}

#[test]
fn ends_that_syntax_leaves_open_keep_the_marker() {
    for code in [
        // Without `default`, only the module's flow can tell whether the
        // cases are exhaustive.
        "function f() { switch (x) { case 1: return 1; case 2: return 2; } }",
        // The last clause falls out of the end of the statement.
        "function f() { switch (x) { case 1: return 1; default: return 2; case 2: } }",
        "function f() { switch (x) { case 1: return 1; default: return 2; case 2: log(); } }",
        // A `break` leaves the statement.
        "function f() { switch (x) { case 1: break; default: return 2; } }",
        "function f() { switch (x) { default: if (c) break; return 1; } }",
        "function f() { switch (x) { default: try { if (c) break; } finally { log(); } return 1; } }",
        // A jump with a label always counts as leaving.
        "function f() { switch (x) { default: l: { if (c) break l; } return 1; } }",
        "function f() { l: switch (x) { case 1: return 1; default: return 2; } }",
        // A `try` without `catch` is left to the module's flow.
        "function f() { try { return 1; } finally { log(); } }",
        // The `catch` clause or the `try` block can complete.
        "function f() { try { return 1; } catch { log(); } }",
        "function f() { try { log(); } catch { return 1; } }",
        // A declaration that runs no code follows a statement that completes.
        "function f() { if (c) return 1; log(); function g() {} }",
    ] {
        assert!(marks_the_end(code), "{code}");
    }
}

#[test]
fn infer_void_return_type_of_returns_without_values() {
    const CODE: &str = r#"function f(c: boolean) {
	if (c) return;
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_void_return_type_of_returns_without_values",
    );
}

#[test]
fn infer_type_of_array() {
    const CODE: &str = r#"const array: Array<string> = [];"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = TestTypeCollector::default();
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
    let mut resolver = TestTypeCollector::default();
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
    let mut resolver = TestTypeCollector::default();
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
    let mut resolver = TestTypeCollector::default();
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
    let mut resolver = TestTypeCollector::default();
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
    let mut resolver = TestTypeCollector::default();
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
    let mut resolver = TestTypeCollector::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    assert_typed_bindings_snapshot(CODE, &bindings, &resolver, "infer_type_of_dynamic_import");
}

#[test]
fn infer_type_of_call_expression_with_type_arguments() {
    const CODE: &str = r#"first<string>([])"#;

    let root = parse_ts(CODE);
    let expr = get_expression(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_any_js_expression(&mut resolver, ScopeId::GLOBAL, &expr);
    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_type_of_call_expression_with_type_arguments",
    );
}

#[test]
fn infer_type_of_contextual_callback_parameter() {
    const CODE: &str = r#"run<Context>("kind", (first, second) => second)"#;

    let root = parse_ts(CODE);
    let param = root
        .syntax()
        .descendants()
        .filter_map(JsFormalParameter::cast)
        .nth(1)
        .expect("second parameter must exist");
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_contextual_js_formal_parameter(&mut resolver, ScopeId::GLOBAL, &param)
        .expect("callback parameter must be contextually typed");
    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_type_of_contextual_callback_parameter",
    );
}

#[test]
fn infer_type_of_contextual_arrow_function_binding() {
    const CODE: &str = r#"new Job(value => value)"#;

    let root = parse_ts(CODE);
    let arrow = root
        .syntax()
        .descendants()
        .find_map(JsArrowFunctionExpression::cast)
        .expect("arrow function must exist");
    let mut resolver = TestTypeCollector::default();
    let ty =
        TypeData::from_contextual_js_arrow_function_binding(&mut resolver, ScopeId::GLOBAL, &arrow)
            .expect("arrow function binding must be contextually typed");
    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_type_of_contextual_arrow_function_binding",
    );
}

#[test]
fn contextual_callback_parameter_requires_direct_call_argument() {
    const CODE: &str = r#"
        function declared(value) {}
        const assigned = (value: string, annotated) => annotated;
        run([(inArray) => inArray]);
        run(...[(spread) => spread]);
    "#;

    let root = parse_ts(CODE);
    let mut resolver = TestTypeCollector::default();
    for param in root
        .syntax()
        .descendants()
        .filter_map(JsFormalParameter::cast)
    {
        assert!(
            TypeData::from_contextual_js_formal_parameter(&mut resolver, ScopeId::GLOBAL, &param)
                .is_none(),
            "{param:?} must not be contextually typed"
        );
    }
}

#[test]
fn infer_type_of_mapped_type_preserves_parts() {
    const CODE: &str = "type Optional<T> = { [K in keyof T]?: T[K] };";
    let root = parse_ts(CODE);
    let declaration = root
        .syntax()
        .descendants()
        .find_map(TsTypeAliasDeclaration::cast)
        .unwrap();
    let mut collector = TestTypeCollector::default();
    let ty =
        TypeData::from_ts_type_alias_declaration(&mut collector, ScopeId::GLOBAL, &declaration)
            .unwrap();
    assert!(matches!(ty, TypeData::InstanceOf(_)));
    assert_type_data_snapshot(
        CODE,
        &ty,
        &collector,
        "infer_type_of_mapped_type_preserves_parts",
    );
}

#[test]
fn infer_type_of_mapped_type_with_literal_keys() {
    const CODE: &str = "type Flags = { readonly [K in \"A\" | \"B\"]-?: boolean };";
    let root = parse_ts(CODE);
    let declaration = root
        .syntax()
        .descendants()
        .find_map(TsTypeAliasDeclaration::cast)
        .unwrap();
    let mut collector = TestTypeCollector::default();
    let ty =
        TypeData::from_ts_type_alias_declaration(&mut collector, ScopeId::GLOBAL, &declaration)
            .unwrap();
    assert!(matches!(ty, TypeData::MappedType(_)));
    assert_type_data_snapshot(
        CODE,
        &ty,
        &collector,
        "infer_type_of_mapped_type_with_literal_keys",
    );
}

#[test]
fn infer_type_of_mapped_type_with_as_clause_is_unknown() {
    const CODE: &str = "type Remapped<T> = { [K in keyof T as Uppercase<K>]: T[K] };";
    let root = parse_ts(CODE);
    let declaration = root
        .syntax()
        .descendants()
        .find_map(TsTypeAliasDeclaration::cast)
        .unwrap();
    let mut collector = TestTypeCollector::default();
    let ty =
        TypeData::from_ts_type_alias_declaration(&mut collector, ScopeId::GLOBAL, &declaration)
            .unwrap();
    assert_type_data_snapshot(
        CODE,
        &ty,
        &collector,
        "infer_type_of_mapped_type_with_as_clause_is_unknown",
    );
}

#[test]
fn infer_type_of_mapped_type_binding() {
    const CODE: &str = "let flags: { [K in keyof Source]?: boolean };";
    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut collector = TestTypeCollector::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut collector,
        ScopeId::GLOBAL,
        &decl,
    );
    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        &collector,
        "infer_type_of_mapped_type_binding",
    );
}

#[test]
fn infer_type_of_function_with_mapped_types() {
    const CODE: &str = r#"function nullable<T>(value: { readonly [K in keyof T]: T[K] }): { [K in keyof T]: T[K] | null } {
    return value;
}"#;
    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut collector = TestTypeCollector::default();
    let ty = TypeData::from_js_function_declaration(&mut collector, ScopeId::GLOBAL, &decl);
    assert_type_data_snapshot(
        CODE,
        &ty,
        &collector,
        "infer_type_of_function_with_mapped_types",
    );
}

#[test]
fn infer_type_of_interface_with_mapped_member() {
    const CODE: &str = r#"interface Config {
    flags: { [K in "A" | "B"]: boolean };
}"#;
    let root = parse_ts(CODE);
    let decl = get_interface_declaration(&root);
    let mut collector = TestTypeCollector::default();
    let ty =
        TypeData::from_ts_interface_declaration(&mut collector, ScopeId::GLOBAL, &decl).unwrap();
    assert_type_data_snapshot(
        CODE,
        &ty,
        &collector,
        "infer_type_of_interface_with_mapped_member",
    );
}

fn mapped_type_alias_snapshot(code: &str, test_name: &str) {
    let root = parse_ts(code);
    let declaration = root
        .syntax()
        .descendants()
        .find_map(TsTypeAliasDeclaration::cast)
        .unwrap();
    let mut collector = TestTypeCollector::default();
    let ty =
        TypeData::from_ts_type_alias_declaration(&mut collector, ScopeId::GLOBAL, &declaration)
            .unwrap();
    assert_type_data_snapshot(code, &ty, &collector, test_name);
}

#[test]
fn infer_type_of_mapped_type_with_typeof_keys() {
    mapped_type_alias_snapshot(
        "type Keys = { [K in keyof typeof object]: K };",
        "infer_type_of_mapped_type_with_typeof_keys",
    );
}

#[test]
fn infer_type_of_mapped_type_with_aliased_keys() {
    mapped_type_alias_snapshot(
        "type Flags = { [K in Keys]: boolean };",
        "infer_type_of_mapped_type_with_aliased_keys",
    );
}

#[test]
fn infer_type_of_mapped_type_with_primitive_keys() {
    mapped_type_alias_snapshot(
        "type Dictionary = { [K in string]: number };",
        "infer_type_of_mapped_type_with_primitive_keys",
    );
}

#[test]
fn infer_type_of_mapped_type_with_never_keys() {
    mapped_type_alias_snapshot(
        "type Empty = { [K in never]: number };",
        "infer_type_of_mapped_type_with_never_keys",
    );
}

#[test]
fn infer_type_of_mapped_type_with_removed_modifiers() {
    mapped_type_alias_snapshot(
        "type Mutable<T> = { -readonly [K in keyof T]-?: T[K] };",
        "infer_type_of_mapped_type_with_removed_modifiers",
    );
}

#[test]
fn infer_type_of_mapped_type_with_added_modifiers() {
    mapped_type_alias_snapshot(
        "type Frozen<T> = { +readonly [K in keyof T]+?: T[K] };",
        "infer_type_of_mapped_type_with_added_modifiers",
    );
}

#[test]
fn infer_type_of_mapped_type_without_annotation() {
    mapped_type_alias_snapshot(
        "type Implicit = { [K in \"A\"] };",
        "infer_type_of_mapped_type_without_annotation",
    );
}

#[test]
fn infer_type_of_mapped_type_with_shadowed_type_parameter() {
    mapped_type_alias_snapshot(
        "type Identities<T> = { [K in keyof T]: <K>(value: K) => K };",
        "infer_type_of_mapped_type_with_shadowed_type_parameter",
    );
}

#[test]
fn infer_type_of_nested_mapped_types() {
    mapped_type_alias_snapshot(
        "type Nested<T> = { [K in keyof T]: { [P in keyof T[K]]: T[K][P] } };",
        "infer_type_of_nested_mapped_types",
    );
}

#[test]
fn infer_type_of_mapped_type_over_intersection() {
    mapped_type_alias_snapshot(
        "type Merged<A, B> = { [K in keyof (A & B)]: (A & B)[K] };",
        "infer_type_of_mapped_type_over_intersection",
    );
}

#[test]
fn infer_type_of_mapped_type_with_parenthesized_keys() {
    mapped_type_alias_snapshot(
        "type Copy<T> = { [K in (keyof T)]: T[K] };",
        "infer_type_of_mapped_type_with_parenthesized_keys",
    );
}

/// Returns the name, `is_readonly`, and `is_optional` of each named member.
fn readonly_summary(members: &[TypeMember]) -> Vec<(String, bool, bool)> {
    members
        .iter()
        .filter_map(|member| {
            let name = member.name()?;
            Some((
                name.text().to_owned(),
                member.is_readonly(),
                member.is_optional(),
            ))
        })
        .collect()
}

fn summary(entries: &[(&str, bool, bool)]) -> Vec<(String, bool, bool)> {
    entries
        .iter()
        .map(|(name, is_readonly, is_optional)| ((*name).to_owned(), *is_readonly, *is_optional))
        .collect()
}

#[test]
fn infer_type_of_readonly_property_signatures() {
    const CODE: &str = r#"interface Frozen {
    readonly value: string | null;
    readonly label?: string;
    count: number;
    readonly [key: string]: unknown;
    get size(): number;
    find(): number;
}"#;
    let root = parse_ts(CODE);
    let decl = get_interface_declaration(&root);
    let mut collector = TestTypeCollector::default();
    let ty =
        TypeData::from_ts_interface_declaration(&mut collector, ScopeId::GLOBAL, &decl).unwrap();
    let TypeData::Interface(interface) = &ty else {
        panic!("expected an interface");
    };
    // Getters aren't readonly, because Biome doesn't collect setters.
    assert_eq!(
        readonly_summary(&interface.members),
        summary(&[
            ("value", true, false),
            ("label", true, true),
            ("count", false, false),
            ("size", false, false),
            ("find", false, false),
        ])
    );
    assert_type_data_snapshot(
        CODE,
        &ty,
        &collector,
        "infer_type_of_readonly_property_signatures",
    );
}

#[test]
fn infer_type_of_readonly_type_literal_properties() {
    mapped_type_alias_snapshot(
        "type Point = { readonly x: number; y?: number; readonly z?: number };",
        "infer_type_of_readonly_type_literal_properties",
    );
}

/// Snapshots the members of the first class in `code` and returns them.
///
/// A class is formatted without its members, so the snapshot shows them as
/// the members of an object.
fn class_members_snapshot(code: &str, test_name: &str) -> Box<[TypeMember]> {
    let root = parse_ts(code);
    let declaration = root
        .syntax()
        .descendants()
        .find_map(JsClassDeclaration::cast)
        .unwrap();
    let mut collector = TestTypeCollector::default();
    let TypeData::Class(class) =
        TypeData::from_js_class_declaration(&mut collector, ScopeId::GLOBAL, &declaration)
    else {
        panic!("expected a class");
    };
    let members = TypeData::Object(Box::new(Object {
        prototype: None,
        members: class.members.clone(),
        has_unknown_members: false,
        kind: ObjectKind::Shape,
    }));
    assert_type_data_snapshot(code, &members, &collector, test_name);
    class.members
}

#[test]
fn infer_type_of_readonly_class_fields() {
    const CODE: &str = r#"class Fields {
    readonly value: string | null = null;
    readonly label?: string;
    count = 0;
    static readonly shared = 1;
    declare readonly declared: string;
    get size() { return 1; }
    constructor(readonly first: string, private readonly second?: number, public third: boolean, fourth: number) {}
}"#;
    let members = class_members_snapshot(CODE, "infer_type_of_readonly_class_fields");
    // `static readonly` fields stay plain static members, because property
    // lookups don't read static members. `fourth` is a plain parameter.
    assert_eq!(
        readonly_summary(&members),
        summary(&[
            ("value", true, false),
            ("label", true, true),
            ("count", false, false),
            ("shared", false, false),
            ("declared", true, false),
            ("size", false, false),
            ("constructor", false, false),
            ("first", true, false),
            ("second", true, true),
            ("third", false, false),
        ])
    );
}

#[test]
fn infer_type_of_readonly_ambient_class_fields() {
    const CODE: &str = r#"declare class Ambient {
    readonly value: string;
    readonly initialized = 1;
    readonly label?: string;
    count: number;
}"#;
    let members = class_members_snapshot(CODE, "infer_type_of_readonly_ambient_class_fields");
    assert_eq!(
        readonly_summary(&members),
        summary(&[
            ("value", true, false),
            ("initialized", true, false),
            ("label", true, true),
            ("count", false, false),
        ])
    );
}

#[test]
fn infer_type_of_const_asserted_object_and_readonly_tuple() {
    const CODE: &str =
        r#"({ value: "a", nested: { flag: true }, get size() { return 1; } } as const)"#;
    let root = parse_ts(CODE);
    let expression = get_expression(&root);
    let mut collector = TestTypeCollector::default();
    let ty = TypeData::from_any_js_expression(&mut collector, ScopeId::GLOBAL, &expression);
    let TypeData::Object(object) = &ty else {
        panic!("expected an object");
    };
    // Members under `as const` are readonly, except getters.
    assert_eq!(
        readonly_summary(&object.members),
        summary(&[
            ("value", true, false),
            ("nested", true, false),
            ("size", false, false),
        ])
    );
    assert_type_data_snapshot(CODE, &ty, &collector, "infer_type_of_const_asserted_object");
    mapped_type_alias_snapshot(
        "type Pair = readonly [string | null, number?];",
        "infer_type_of_readonly_tuple",
    );
}

/// Returns the bindings of the first variable declaration of `code`, with a
/// snapshot of them and of the collected types.
fn variable_bindings_snapshot(code: &str, test_name: &str) -> Box<[(Text, TypeReference)]> {
    let root = parse_ts(code);
    let declaration = get_variable_declaration(&root);
    let mut collector = TestTypeCollector::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut collector,
        ScopeId::GLOBAL,
        &declaration,
    );
    assert_typed_bindings_snapshot(code, &bindings, &collector, test_name);
    bindings
}

#[test]
fn infer_widened_literal_types_of_let_declarations() {
    // `n`, `s`, and `l` widen. An assertion's literal type isn't fresh, and
    // an object literal is widened where it is resolved, so `a`, `k`, and
    // `o` keep the types of their initializers.
    variable_bindings_snapshot(
        r#"let n = 1, s = flag ? "a" : 1, l = name || "anonymous", a = 1 as 1, k = 1 as const, o = { a: 1 };"#,
        "infer_widened_literal_types_of_let_declarations",
    );
}

#[test]
fn infer_literal_types_of_const_declarations() {
    // A `const` keeps the literal types of its initializer.
    variable_bindings_snapshot(
        r#"const n = 1, s = flag ? "a" : 1;"#,
        "infer_literal_types_of_const_declarations",
    );
}

#[test]
fn infer_widened_literal_types_of_class_fields_parameters_and_return_types() {
    // `count`, `label`, the parameter `step`, and the return types of
    // `next` and `size` widen. `readonly` and annotated fields don't.
    class_members_snapshot(
        r#"class Counter {
    count = 0;
    static label = "counter";
    readonly start = 0;
    limit: 10 = 10;
    next(step = 1) { return 1; }
    get size() { return "s"; }
}"#,
        "infer_widened_literal_types_of_class_fields_parameters_and_return_types",
    );
}

#[test]
fn infer_widened_literal_types_of_destructured_array_elements() {
    // A destructuring pattern gives the elements of its array no literal
    // type, so they widen.
    variable_bindings_snapshot(
        "const [first, second] = [1n, 2];",
        "infer_widened_literal_types_of_destructured_array_elements",
    );
}
