mod utils;

use biome_js_semantic::ScopeId;
use biome_js_syntax::{
    JsArrowFunctionExpression, JsClassDeclaration, JsFormalParameter, TsEnumDeclaration,
    TsTypeAliasDeclaration,
};
use biome_js_type_info::{
    AnyPredicateFunction, EnumTypes, Object, ObjectKind, RawTypeCollector, RawTypeId, ReturnType,
    TypeData, TypeMember, TypeReference, TypeofExpression, TypeofImplicitReturnExpression,
    inferred_predicate_parameter,
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

/// Returns the marker of the end of the body that collection adds to the
/// return type of the first function declaration of `code`, if any.
fn end_marker(code: &str) -> Option<TypeofImplicitReturnExpression> {
    let root = parse_ts(code);
    let decl = get_function_declaration(&root);
    let mut collector = TestTypeCollector::default();
    TypeData::from_js_function_declaration(&mut collector, ScopeId::GLOBAL, &decl);
    collector.types.as_slice().iter().find_map(|ty| match ty {
        TypeData::TypeofExpression(expression) => match expression.as_ref() {
            TypeofExpression::ImplicitReturn(marker) => Some(marker.clone()),
            _ => None,
        },
        _ => None,
    })
}

fn marks_the_end(code: &str) -> bool {
    end_marker(code).is_some()
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
fn end_markers_record_returns_without_values() {
    let marker = end_marker("function f() { if (c) return; if (d) return 1; }").unwrap();
    assert!(marker.has_bare_return);
    let marker = end_marker("function f() { if (d) return 1; }").unwrap();
    assert!(!marker.has_bare_return);
    // The `return` of a nested function returns from that function.
    let marker =
        end_marker("function f() { const g = () => { return; }; if (d) return 1; }").unwrap();
    assert!(!marker.has_bare_return);
}

#[test]
fn infer_predicate_marker_of_a_boolean_test_of_a_parameter() {
    // The function returns a test of `v`, so its return type holds the
    // marker that the module graph resolves to the target of the type
    // predicate, or to nothing, with `boolean` as the plain return type.
    const CODE: &str = r#"function f(a: number, v: unknown) {
	return typeof v === "string";
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = TestTypeCollector::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_predicate_marker_of_a_boolean_test_of_a_parameter",
    );
}

/// Returns the name of the parameter that the first function in `code`
/// would test with a type predicate inferred from its body.
fn inferred_predicate_parameter_name(code: &str) -> Option<String> {
    let root = parse_ts(code);
    let function = root
        .syntax()
        .descendants()
        .find_map(AnyPredicateFunction::cast)
        .unwrap();
    inferred_predicate_parameter(&function)
        .map(|(_, binding)| binding.syntax().text_trimmed().to_string())
}

#[test]
fn infer_predicate_parameters_only_for_functions_of_the_right_shape() {
    for (code, expected) in [
        (
            "function f(v: unknown) { return typeof v === 'string'; }",
            Some("v"),
        ),
        ("const f = (v: unknown) => v instanceof Date;", Some("v")),
        ("const f = (v: unknown) => { return !v; };", Some("v")),
        (
            "const o = { m(v: unknown) { return 'a' in Object(v) || v === 1; } };",
            Some("v"),
        ),
        ("class C { m(v: unknown) { return isFoo(v); } }", Some("v")),
        ("xs.filter((x) => x !== undefined);", Some("x")),
        ("xs.filter(x => x !== undefined);", Some("x")),
        (
            "xs.filter(function (x) { return x !== undefined; });",
            Some("x"),
        ),
        // The first parameter that the value mentions decides.
        (
            "const f = (a: unknown, b: unknown) => typeof b === 'string';",
            Some("b"),
        ),
        ("const f = (a: unknown, b: unknown) => a === b;", Some("a")),
        // `this`, rest, destructured, and `boolean` parameters are skipped.
        (
            "function f(this: unknown, v: unknown) { return v === 1; }",
            Some("v"),
        ),
        (
            "const f = (...a: unknown[]) => typeof a[0] === 'string';",
            None,
        ),
        (
            "const f = ({ a }: { a: unknown }, b: unknown) => a === b;",
            Some("b"),
        ),
        (
            "const f = (a: boolean, b: unknown) => a === true && b === 1;",
            Some("b"),
        ),
        // An operand of `&&` that may not be a boolean.
        ("const f = (a: boolean, b: unknown) => a && b === 1;", None),
        ("const f = (a?: boolean) => a === true;", Some("a")),
        // A return type annotation, `async`, and generators.
        (
            "const f = (v: unknown): boolean => typeof v === 'string';",
            None,
        ),
        (
            "const f = async (v: unknown) => typeof v === 'string';",
            None,
        ),
        (
            "function* f(v: unknown) { return typeof v === 'string'; }",
            None,
        ),
        // Values that may not be booleans.
        ("const f = (v: unknown) => v;", None),
        ("const f = (v: string) => v && v.length > 0;", None),
        ("const f = (v: unknown) => v ?? false;", None),
        (
            "const f = (o: { is?: (v: unknown) => boolean }, v: unknown) => o.is?.(v);",
            None,
        ),
        // Bodies other than a single `return` with a value.
        (
            "function f(v: unknown) { if (v) return true; return false; }",
            None,
        ),
        (
            "function f(v: unknown) { const t = typeof v; return t === 'string'; }",
            None,
        ),
        ("function f(v: unknown) { return; }", None),
        // Parameters that Biome can't infer a type predicate about.
        ("const f = (v: string | null = null) => v !== null;", None),
        ("function f(v) { return typeof v === 'string'; }", None),
        ("const f = (v) => typeof v === 'string';", None),
        (
            "function f(v: string | null) { v = null; return v !== null; }",
            None,
        ),
        (
            "const f = (v: unknown) => [1].some(() => { v = 1; return true; }) && v === 1;",
            None,
        ),
        ("const f = (v: unknown) => true;", None),
        // Values that mention the parameter without testing it directly.
        ("xs.filter((eb) => eb('a', '=', 1));", None),
        ("const f = (v: { done(): boolean }) => v.done();", None),
        ("const f = (v: { a: unknown }) => isFoo(v.a);", None),
        ("const f = (v: number) => v > 0;", None),
        (
            "const f = (s: { k: 'a' } | { k: 'b' }) => s.k === 'a';",
            Some("s"),
        ),
        (
            "const f = (v: unknown) => !(typeof v === 'string');",
            Some("v"),
        ),
    ] {
        assert_eq!(
            inferred_predicate_parameter_name(code).as_deref(),
            expected,
            "{code}"
        );
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

/// Returns the type of each member of the first enum declaration in `code`,
/// as `name: type`, with the types that collection gives them.
fn enum_member_types(code: &str) -> Vec<String> {
    let root = parse_ts(code);
    let declaration = root
        .syntax()
        .descendants()
        .find_map(TsEnumDeclaration::cast)
        .expect("the code must declare an enum");
    let mut collector = TestTypeCollector::default();
    let types = EnumTypes::from_ts_enum_declaration(&mut collector, ScopeId::GLOBAL, &declaration);
    let source = root.syntax().to_string();
    types
        .members
        .iter()
        .map(|(range, ty)| {
            let ty = match ty {
                TypeReference::Resolved(RawTypeId::Local(id)) => collector.get_by_id(*id).clone(),
                ty => TypeData::reference(ty.clone()),
            };
            format!("{}: {ty}", &source[*range])
        })
        .collect()
}

// TypeScript 7.0.2 (`tsc --strict`) gives each member the literal type of
// the value below, unless a comment says otherwise.
#[test]
fn enum_members_have_the_types_of_their_values() {
    for (code, expected) in [
        // A member without an initializer counts up from the member before
        // it, or from `0`.
        (
            "enum A { X, Y, Z }",
            vec!["X: number: 0", "Y: number: 1", "Z: number: 2"],
        ),
        // TypeScript gives `W` the value `-1`. Biome has no spelling for a
        // computed negative number, so it gives `number`, which is wider.
        (
            "enum B { X = 5, Y, Z = -2, W }",
            vec!["X: number: 5", "Y: number: 6", "Z: number: -2", "W: number"],
        ),
        // Operators between known values give their results. TypeScript gives
        // `Z` the value `-1` and `U` the value `2.5`, which Biome has no
        // spelling for, so they are `number`.
        (
            "enum C { X = 1 << 3, Y = X | 1, Z = ~0, W = 7 % 4, V = 2 ** 3, U = 10 / 4, T = 0x10, S = 1e3 }",
            vec![
                "X: number: 8",
                "Y: number: 9",
                "Z: number",
                "W: number: 3",
                "V: number: 8",
                "U: number",
                "T: number: 0x10",
                "S: number: 1e3",
            ],
        ),
        // TypeScript gives `Y` the value `"ab"` and `W` the value `"d1"`.
        // Biome doesn't build the text of a concatenation, so they are
        // `string`.
        (
            r#"enum D { X = "a", Y = X + "b", Z = `c`, W = "d" + 1 }"#,
            vec!["X: string: a", "Y: string", "Z: string: c", "W: string"],
        ),
        // Earlier members, read through the enum.
        (
            r#"enum E { X = 1, Y = E.X + 1, Z = E["Y"] * 2, W = (Z) }"#,
            vec![
                "X: number: 1",
                "Y: number: 2",
                "Z: number: 4",
                "W: number: 4",
            ],
        ),
        // TypeScript gives `X` the value `"k"`, `Y` the value `3`, `Z` the
        // value of `A.Y`, and `W` the value `"kv"`: they read constants
        // outside the enum, which Biome doesn't follow, because they can be
        // numbers or strings. A call, a member of a value that isn't a name,
        // and an assertion are numbers, as TypeScript requires for them.
        (
            "enum F { X = K, Y = N, Z = A.Y, W = `${K}v`, V = f(), U = \"c\".length, T = 1 as number, S = Math.PI }",
            vec![
                "X: unknown",
                "Y: unknown",
                "Z: unknown",
                "W: string",
                "V: number",
                "U: number",
                "T: number",
                "S: unknown",
            ],
        ),
        // A member after a number that Biome doesn't know is a number too.
        // After a string, TypeScript requires an initializer.
        (
            r#"enum G { X = f(), Y, Z = "z", W }"#,
            vec!["X: number", "Y: number", "Z: string: z", "W: unknown"],
        ),
        // `NaN` and infinite values have no literal type.
        (
            "enum L { X = 0 / 0, Y = 1 / 0, Z = 2 ** 64 }",
            vec!["X: number", "Y: number", "Z: number"],
        ),
        (
            "const enum P { X = 2, Y }",
            vec!["X: number: 2", "Y: number: 3"],
        ),
        // In an ambient enum that isn't `const`, the code that defines the
        // enum chooses the value of a member without an initializer.
        (
            r#"declare enum Q { X, Y = "y", Z }"#,
            vec!["X: number", "Y: string: y", "Z: number"],
        ),
        ("declare namespace N { enum Q { X } }", vec!["X: number"]),
        (
            "declare const enum R { X, Y }",
            vec!["X: number: 0", "Y: number: 1"],
        ),
    ] {
        assert_eq!(enum_member_types(code), expected, "{code}");
    }
}

/// Returns the type of the first enum declaration in `code`, with the types
/// that it refers to, resolved within the collector.
fn enum_declaration_type(code: &str) -> String {
    let root = parse_ts(code);
    let declaration = root
        .syntax()
        .descendants()
        .find_map(TsEnumDeclaration::cast)
        .expect("the code must declare an enum");
    let mut collector = TestTypeCollector::default();
    let types = EnumTypes::from_ts_enum_declaration(&mut collector, ScopeId::GLOBAL, &declaration);
    let TypeReference::Resolved(RawTypeId::Local(id)) = types.declaration else {
        panic!("an enum declaration must have a local type");
    };
    let TypeData::MergedReference(reference) = collector.get_by_id(id) else {
        panic!("an enum declaration must be a merged reference");
    };
    let show = |ty: &Option<TypeReference>| match ty {
        Some(TypeReference::Resolved(RawTypeId::Local(id))) => collector.get_by_id(*id).to_string(),
        Some(ty) => TypeData::reference(ty.clone()).to_string(),
        None => "none".to_owned(),
    };
    format!(
        "type: {}\nvalue: {}\nnamespace: {}",
        show(&reference.ty),
        show(&reference.value_ty),
        show(&reference.namespace_ty)
    )
}

// An enum is the union of its members as a type, as TypeScript's `A` is
// `A.X | A.Y`, and an object with a readonly property per member as a value.
// The object maps numbers back to names when a member is a number, as
// TypeScript's `A[0]` is `string`, and not for an enum of strings.
#[test]
fn enum_declarations_are_their_members_as_types_and_objects_as_values() {
    assert_eq!(
        enum_declaration_type("enum A { X, Y }"),
        "type: Local TypeId(0) | Local TypeId(1)\n\
         value: Object {\n  prototype: No prototype\n  members: [\n    \
         readonly \"X\": Local TypeId(0),\n    readonly \"Y\": Local TypeId(1),\n    \
         [number]: string\n  ]\n}\n\
         namespace: none"
    );
    assert_eq!(
        enum_declaration_type(r#"enum S { X = "x" }"#),
        "type: string: x\n\
         value: Object {\n  prototype: No prototype\n  members: [readonly \"X\": Local TypeId(0)]\n}\n\
         namespace: none"
    );
}
