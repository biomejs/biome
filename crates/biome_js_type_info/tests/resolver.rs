mod utils;

use std::sync::Arc;

use biome_js_semantic::ScopeId;
use biome_js_syntax::{AnyJsModuleItem, AnyJsRoot, AnyJsStatement, JsExpressionStatement};
use biome_js_type_info::{
    GlobalsResolver, Resolvable, Type, TypeData, TypeMemberKind, TypeReference,
    TypeReferenceQualifier, TypeResolver,
};
use biome_rowan::Text;

use utils::{
    HardcodedSymbolResolver, assert_type_data_snapshot, assert_typed_bindings_snapshot,
    get_function_declaration, get_variable_declaration, parse_ts,
};

#[test]
fn infer_resolved_type_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    resolver.resolve_all();

    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_resolved_type_of_promise_returning_function",
    )
}

#[test]
fn infer_resolved_type_of_async_function() {
    const CODE: &str = r#"async function returnsPromise(): Promise<string> {
	return "value";
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    resolver.resolve_all();

    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_resolved_type_of_async_function",
    )
}

#[test]
fn infer_resolved_type_from_invocation_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}

returnsPromise()"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let function_ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    resolver.resolve_all();

    let expr = get_expression_statement(&root);
    let mut resolver = HardcodedSymbolResolver::new("returnsPromise", function_ty, resolver);
    let expr_ty = TypeData::from_any_js_expression(
        &mut resolver,
        ScopeId::GLOBAL,
        &expr.expression().unwrap(),
    );
    resolver.resolve_all();

    let expr_ty = expr_ty.resolved(&mut resolver).expect("must be resolved");

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_resolved_type_from_invocation_of_promise_returning_function",
    )
}

#[test]
fn infer_resolved_type_from_chained_invocation_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}

returnsPromise().then(() => {})"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let function_ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    resolver.resolve_all();

    let expr = get_expression_statement(&root);
    let mut resolver = HardcodedSymbolResolver::new("returnsPromise", function_ty, resolver);
    let expr_ty = TypeData::from_any_js_expression(
        &mut resolver,
        ScopeId::GLOBAL,
        &expr.expression().unwrap(),
    );
    resolver.resolve_all();

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_resolved_type_from_chained_invocation_of_promise_returning_function",
    )
}

#[test]
fn infer_resolved_type_from_double_chained_invocation_of_promise_returning_function() {
    const CODE: &str = r#"function returnsPromise(): Promise<number> {
    return Promise.resolved(true);
}

returnsPromise().then(() => {}).finally(() => {})"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let function_ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    resolver.resolve_all();

    let expr = get_expression_statement(&root);
    let mut resolver = HardcodedSymbolResolver::new("returnsPromise", function_ty, resolver);
    let expr_ty = TypeData::from_any_js_expression(
        &mut resolver,
        ScopeId::GLOBAL,
        &expr.expression().unwrap(),
    );
    resolver.resolve_all();

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_resolved_type_from_double_chained_invocation_of_promise_returning_function",
    )
}

#[test]
fn infer_resolved_type_from_direct_promise_instance() {
    const CODE: &str = r#"new Promise((resolve) => resolve("value"))"#;

    let root = parse_ts(CODE);
    let expr = get_expression_statement(&root);
    let mut resolver = GlobalsResolver::default();
    let expr_ty = TypeData::from_any_js_expression(
        &mut resolver,
        ScopeId::GLOBAL,
        &expr.expression().unwrap(),
    );
    resolver.resolve_all();

    let expr_ty = expr_ty.resolved(&mut resolver).expect("must be resolved");

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_resolved_type_from_direct_promise_instance",
    )
}

#[test]
fn infer_resolved_type_from_static_promise_function() {
    const CODE: &str = r#"Promise.resolve("value")"#;

    let root = parse_ts(CODE);
    let expr = get_expression_statement(&root);
    let mut resolver = GlobalsResolver::default();
    let expr_ty = TypeData::from_any_js_expression(
        &mut resolver,
        ScopeId::GLOBAL,
        &expr.expression().unwrap(),
    );
    resolver.resolve_all();

    assert_type_data_snapshot(
        CODE,
        &expr_ty,
        &resolver,
        "infer_resolved_type_from_static_promise_function",
    )
}

#[test]
fn infer_resolved_type_of_destructured_array_element() {
    const CODE: &str = r#"const [a]: Array<string> = [];"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    resolver.resolve_all();

    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        &resolver,
        "infer_resolved_type_of_destructured_array_element",
    );
}

#[test]
fn infer_resolved_type_of_disposable_object() {
    const CODE: &str = r#"const a = {
        [Symbol.dispose](): void {
            // do something
        }
    };"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    resolver.resolve_all();

    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        &resolver,
        "infer_resolved_type_of_disposable_object",
    );
}

#[test]
fn infer_resolved_type_of_async_disposable_object() {
    const CODE: &str = r#"const a = {
        [Symbol.asyncDispose](): void {
            // do something
        }
    };"#;

    let root = parse_ts(CODE);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    resolver.resolve_all();

    assert_typed_bindings_snapshot(
        CODE,
        &bindings,
        &resolver,
        "infer_resolved_type_of_async_disposable_object",
    );
}

#[test]
fn disposable_detection_relies_on_the_symbol_key() {
    // A value-returning `[Symbol.dispose]` is still disposable: TypeScript
    // accepts any return type through void assignability.
    let ty = inferred_variable_type(
        r#"const a = {
            [Symbol.dispose]() { return 1; }
        };"#,
    );
    assert!(ty.is_disposable());

    // `[Symbol.asyncDispose]` marks an async disposable regardless of how the
    // return type is spelled, including the `PromiseLike<void>` lib signature.
    let ty = inferred_variable_type(
        r#"const a = {
            [Symbol.asyncDispose](): PromiseLike<void> { throw 0; }
        };"#,
    );
    assert!(ty.is_async_disposable());

    let ty = inferred_variable_type(
        r#"const a = {
            async [Symbol.asyncDispose]() {}
        };"#,
    );
    assert!(ty.is_async_disposable());
}

#[test]
fn infer_resolved_type_of_disposable_returning_function() {
    const CODE: &str = r#"function returnsDisposable(): Disposable {
    return {};
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    resolver.resolve_all();

    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_resolved_type_of_disposable_returning_function",
    )
}

#[test]
fn infer_resolved_type_of_async_disposable_returning_function() {
    const CODE: &str = r#"function returnsAsyncDisposable(): AsyncDisposable {
    return {};
}"#;

    let root = parse_ts(CODE);
    let decl = get_function_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let ty = TypeData::from_js_function_declaration(&mut resolver, ScopeId::GLOBAL, &decl);
    resolver.resolve_all();

    assert_type_data_snapshot(
        CODE,
        &ty,
        &resolver,
        "infer_resolved_type_of_async_disposable_returning_function",
    )
}

#[test]
fn generated_disposable_globals_use_computed_symbol_members() {
    let resolver = GlobalsResolver::default();

    assert_global_has_computed_member(&resolver, "Disposable");
    assert_global_has_computed_member(&resolver, "AsyncDisposable");
}

fn assert_global_has_computed_member(resolver: &GlobalsResolver, name: &'static str) {
    let reference = TypeReference::from(
        TypeReferenceQualifier::from_path(ScopeId::GLOBAL, Text::new_static(name)).with_type_only(),
    );
    let resolved = resolver
        .resolve_and_get(&reference)
        .expect("global should resolve");
    let TypeData::Interface(interface) = resolved.as_raw_data() else {
        panic!("{name} should resolve to generated interface data");
    };
    let [member] = interface.members.as_ref() else {
        panic!("{name} should have exactly one generated member");
    };
    assert!(matches!(member.kind, TypeMemberKind::ComputedValue(_)));
}

fn inferred_variable_type(code: &str) -> Type {
    let root = parse_ts(code);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );
    resolver.run_inference();

    let [(name, reference)] = bindings.as_ref() else {
        panic!("expected exactly one binding");
    };
    assert_eq!(name.text(), "a");

    let reference = reference.clone();
    let resolver = Arc::new(resolver);
    let id = resolver
        .resolve_reference(&reference)
        .expect("binding should resolve");
    Type::from_id(resolver, id)
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
