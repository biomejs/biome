mod utils;

use std::sync::Arc;

use biome_js_semantic::ScopeId;
use biome_js_syntax::{AnyJsModuleItem, AnyJsRoot, AnyJsStatement, JsExpressionStatement};
use biome_js_type_info::{
    GlobalsResolver, ModuleId, Resolvable, ResolvedTypeId, Type, TypeData, TypeId, TypeMember,
    TypeMemberKind, TypeReference, TypeReferenceQualifier, TypeResolver, TypeResolverLevel,
};
use biome_rowan::Text;

use utils::{
    HardcodedSymbolResolver, assert_type_data_snapshot, assert_typed_bindings_snapshot,
    get_function_declaration, get_interface_declaration, get_variable_declaration, parse_ts,
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
fn readonly_utility_marks_members_readonly() {
    let data = resolved_variable_data(
        r#"const value: Readonly<{ name: string; [key: string]: number }> = {};"#,
    );
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

    let data = resolved_variable_data(r#"const value: Readonly<Record<string, string>> = {};"#);
    let TypeData::Object(object) = data else {
        panic!("expected object type");
    };
    let index = object
        .members
        .iter()
        .find(|member| member.is_index_signature_with_ty(|_| true))
        .expect("index signature member");
    assert!(index.is_readonly());
}

#[test]
fn readonly_utility_marks_named_type_members_readonly() {
    const CODE: &str = r#"interface Foo { name: string; }"#;

    let root = parse_ts(CODE);
    let mut globals = GlobalsResolver::default();
    let interface = TypeData::from_ts_interface_declaration(
        &mut globals,
        ScopeId::GLOBAL,
        &get_interface_declaration(&root),
    )
    .expect("interface should resolve");
    let mut resolver = HardcodedSymbolResolver::new("Foo", interface, globals);
    let reference = TypeReference::from(
        TypeReferenceQualifier::from_path(ScopeId::GLOBAL, Text::new_static("Readonly"))
            .with_type_parameters([TypeReference::from(TypeReferenceQualifier::from_path(
                ScopeId::GLOBAL,
                Text::new_static("Foo"),
            ))]),
    );
    let reference = reference
        .resolved(&mut resolver)
        .expect("Readonly<Foo> should resolve");
    let data = resolver
        .resolve_and_get(&reference)
        .expect("resolved Readonly<Foo> should resolve")
        .to_data();
    let TypeData::Object(object) = data else {
        panic!("expected object type");
    };
    let name = object
        .members
        .iter()
        .find(|member| member.has_name("name"))
        .expect("name member");
    assert!(name.is_readonly());
}

#[test]
fn readonly_utility_preserves_foreign_member_context() {
    let module_id = ModuleId::new(7);
    let local_reference =
        TypeReference::Resolved(ResolvedTypeId::new(TypeResolverLevel::Thin, TypeId::new(1)));
    let object = TypeData::object_with_members(
        [TypeMember {
            kind: TypeMemberKind::ComputedValue(local_reference.clone()),
            ty: local_reference,
        }]
        .into(),
    );
    let globals = GlobalsResolver::default();
    let mut resolver =
        HardcodedSymbolResolver::new("Foo", object, globals).with_foreign_symbol_module(module_id);
    let reference = TypeReference::from(
        TypeReferenceQualifier::from_path(ScopeId::GLOBAL, Text::new_static("Readonly"))
            .with_type_parameters([TypeReference::from(TypeReferenceQualifier::from_path(
                ScopeId::GLOBAL,
                Text::new_static("Foo"),
            ))]),
    );
    let reference = reference
        .resolved(&mut resolver)
        .expect("Readonly<Foo> should resolve");
    let data = resolver
        .resolve_and_get(&reference)
        .expect("resolved Readonly<Foo> should resolve")
        .to_data();
    let TypeData::Object(object) = data else {
        panic!("expected object type");
    };
    let [member] = object.members.as_ref() else {
        panic!("expected one member");
    };

    let TypeMemberKind::ReadonlyComputedValue(TypeReference::Resolved(key_type_id)) = &member.kind
    else {
        panic!("expected readonly computed member");
    };
    let TypeReference::Resolved(value_type_id) = &member.ty else {
        panic!("expected resolved member type");
    };

    assert_eq!(key_type_id.module_id(), module_id);
    assert_eq!(value_type_id.module_id(), module_id);
}

#[test]
fn required_utility_preserves_readonly_foreign_union_member_context() {
    let module_id = ModuleId::new(7);
    let optional_type_id = TypeId::new(1);
    let local_reference =
        TypeReference::Resolved(ResolvedTypeId::new(TypeResolverLevel::Thin, TypeId::new(2)));
    let object = TypeData::object_with_members(
        [TypeMember {
            kind: TypeMemberKind::ReadonlyComputedValueOptional(local_reference.clone()),
            ty: TypeReference::Resolved(ResolvedTypeId::new(
                TypeResolverLevel::Thin,
                optional_type_id,
            )),
        }]
        .into(),
    );
    let globals = GlobalsResolver::default();
    let mut resolver = HardcodedSymbolResolver::new("Foo", object, globals);
    assert_eq!(resolver.optional(local_reference), optional_type_id);
    let mut resolver = resolver.with_foreign_symbol_module(module_id);
    let reference = TypeReference::from(
        TypeReferenceQualifier::from_path(ScopeId::GLOBAL, Text::new_static("Required"))
            .with_type_parameters([TypeReference::from(TypeReferenceQualifier::from_path(
                ScopeId::GLOBAL,
                Text::new_static("Foo"),
            ))]),
    );
    let reference = reference
        .resolved(&mut resolver)
        .expect("Required<Foo> should resolve");
    let data = resolver
        .resolve_and_get(&reference)
        .expect("resolved Required<Foo> should resolve")
        .to_data();
    let TypeData::Object(object) = data else {
        panic!("expected object type");
    };
    let [member] = object.members.as_ref() else {
        panic!("expected one member");
    };

    let TypeMemberKind::ReadonlyComputedValue(TypeReference::Resolved(key_type_id)) = &member.kind
    else {
        panic!("expected required computed member");
    };
    let TypeReference::Resolved(value_type_id) = &member.ty else {
        panic!("expected resolved member type");
    };

    assert_eq!(key_type_id.module_id(), module_id);
    assert_eq!(value_type_id.module_id(), module_id);
}

#[test]
fn mapped_utilities_preserve_readonly_members() {
    let data = resolved_variable_data(
        r#"const value: Partial<{ readonly name: string; readonly [key: string]: number; readonly [Symbol.iterator]: () => string }> = {};"#,
    );
    let TypeData::Object(object) = data else {
        panic!("expected partial object type");
    };
    let name = object
        .members
        .iter()
        .find(|member| member.has_name("name"))
        .expect("name member");
    assert!(name.is_readonly());
    assert!(name.is_optional());
    let index = object
        .members
        .iter()
        .find(|member| matches!(member.kind, TypeMemberKind::ReadonlyIndexSignature(_)))
        .expect("index signature member");
    assert!(index.is_readonly());
    let computed = object
        .members
        .iter()
        .find(|member| {
            matches!(
                member.kind,
                TypeMemberKind::ReadonlyComputedValueOptional(_)
            )
        })
        .expect("computed member");
    assert!(computed.is_readonly());
    assert!(computed.is_optional());

    let data = resolved_variable_data(
        r#"const value: Required<{ readonly name?: string; readonly [Symbol.iterator]?: () => string }> = {};"#,
    );
    let TypeData::Object(object) = data else {
        panic!("expected required object type");
    };
    let name = object
        .members
        .iter()
        .find(|member| member.has_name("name"))
        .expect("name member");
    assert!(name.is_readonly());
    assert!(!name.is_optional());
    let computed = object
        .members
        .iter()
        .find(|member| matches!(member.kind, TypeMemberKind::ReadonlyComputedValue(_)))
        .expect("computed member");
    assert!(computed.is_readonly());
    assert!(!computed.is_optional());
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

#[test]
fn generated_error_prototype_is_readonly() {
    let resolver = GlobalsResolver::default();
    let reference = TypeReference::from(
        TypeReferenceQualifier::from_path(ScopeId::GLOBAL, Text::new_static("Error"))
            .with_type_only(),
    );
    let resolved = resolver
        .resolve_and_get(&reference)
        .expect("Error should resolve");
    let TypeData::Class(class) = resolved.as_raw_data() else {
        panic!("Error should resolve to generated class data");
    };
    let prototype = class
        .members
        .iter()
        .find(|member| member.has_name("prototype"))
        .expect("prototype member");
    assert!(prototype.is_static());
    assert!(prototype.is_readonly());
}

fn resolved_variable_data(code: &str) -> TypeData {
    let root = parse_ts(code);
    let decl = get_variable_declaration(&root);
    let mut resolver = GlobalsResolver::default();
    let bindings = TypeData::typed_bindings_from_js_variable_declaration(
        &mut resolver,
        ScopeId::GLOBAL,
        &decl,
    );

    let [(name, reference)] = bindings.as_ref() else {
        panic!("expected exactly one binding");
    };
    assert_eq!(name.text(), "value");

    resolver
        .resolve_and_get(reference)
        .expect("resolved binding should resolve")
        .to_data()
        .inferred(&mut resolver)
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
