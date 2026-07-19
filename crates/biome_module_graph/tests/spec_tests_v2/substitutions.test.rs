use super::*;

#[test]
fn test_infer_module_types_substitutes_generic_members_through_lookup_traversal() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface InterfaceBase<T> {
                value: T;
            }

            interface InterfaceBox<T> extends InterfaceBase<T> {}

            class ClassBase<T> {
                value: T;
            }

            class ClassBox<T> extends ClassBase<T> {}

            interface UnionBox<T> {
                value: T;
            }

            export function readInterfaceBox(
                value: InterfaceBox<number>,
            ): InterfaceBox<number> {
                return value;
            }

            export function readClassBox(value: ClassBox<string>): ClassBox<string> {
                return value;
            }

            export function readUnionBox(
                value: UnionBox<string> | UnionBox<number>,
            ): UnionBox<string> | UnionBox<number> {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let interface_box_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readInterfaceBox")
            .expect("readInterfaceBox return type must be inferred");
    let interface_value_ty = inferred
        .find_member_type(&db, interface_box_ty, "value")
        .expect("InterfaceBox<number>.value must be inferred through extends");
    assert!(is_inferred_number(&db, interface_value_ty));

    let class_box_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readClassBox")
            .expect("readClassBox return type must be inferred");
    let class_value_ty = inferred
        .find_member_type(&db, class_box_ty, "value")
        .expect("ClassBox<string>.value must be inferred through extends");
    assert!(is_inferred_string(&db, class_value_ty));

    let union_box_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readUnionBox")
            .expect("readUnionBox return type must be inferred");
    let union_value_ty = inferred
        .find_member_type(&db, union_box_ty, "value")
        .expect("UnionBox<string> | UnionBox<number> value must be inferred");
    assert!(contains_inferred_string(&db, union_value_ty));
    assert!(contains_inferred_number(&db, union_value_ty));
}

#[test]
fn test_infer_module_types_collects_members_from_an_imported_generic_union() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
            export namespace Types {
                export type Result<T> =
                    | { success: true; data: T }
                    | { success: false; error: string };
            }

            export default Types;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type Types from "./types";

            declare function parse(): Types.Result<string>;
            export const success = parse().success;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let success_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "success")
        .expect("success binding type must be inferred");
    let success_ty = normalize_type(&db, index_module, success_ty);

    assert!(
        is_inferred_boolean(&db, success_ty),
        "success must include both boolean variants, got {}",
        format_inferred_type(&db, success_ty)
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_direct_generic_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function identity<T>(value: T): T {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let identity_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "identity")
        .expect("identity binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, identity_ty),
        Vec::from([InferredTypeData::Number]),
    );

    assert!(is_inferred_number(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_substitutes_chained_generic_defaults() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export declare function fallback<T = string, U = T>(): U;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let fallback_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "fallback")
        .expect("fallback binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, fallback_ty),
        Vec::new(),
    );

    assert!(
        is_inferred_string(&db, call_ty),
        "expected the chained defaults to resolve to string, got {}",
        format_inferred_type(&db, call_ty)
    );
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_chained_generic_defaults",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_chained_generic_default_from_argument() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export declare function pick<T = string, U = T>(value: T): U;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let pick_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "pick")
        .expect("pick binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, pick_ty),
        Vec::from([InferredTypeData::Number]),
    );

    assert!(
        is_inferred_number(&db, call_ty),
        "expected the default to follow the bound argument, got {}",
        format_inferred_type(&db, call_ty)
    );
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_chained_generic_default_from_argument",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_deduplicates_substituted_union_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function maybeString<T>(value: T): T | string {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let maybe_string_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "maybeString")
        .expect("maybeString binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, maybe_string_ty),
        Vec::from([InferredTypeData::String]),
    );

    assert!(is_inferred_string(&db, call_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_deduplicates_substituted_union_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_callback_generic_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function run<T>(cb: () => T): T {
                return cb();
            }

            export function readString(): string {
                return "value";
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let run_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "run")
        .expect("run binding type must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readString")
        .expect("readString binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, run_ty),
        Vec::from([inferred.resolve_type(&db, read_string_ty)]),
    );

    assert!(is_inferred_string(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_substitutes_nested_generic_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function wrap<T>(value: T): Promise<T> {
                return Promise.resolve(value);
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let wrap_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "wrap")
        .expect("wrap binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, wrap_ty),
        Vec::from([InferredTypeData::Number]),
    );
    let InferredTypeData::InstanceOf(instance) = call_ty else {
        panic!("wrap must return a Promise instance, got {call_ty:?}");
    };

    assert!(call_ty.is_promise_instance(&db));
    assert!(
        instance
            .type_parameters(&db)
            .iter()
            .any(|ty| is_inferred_number(&db, *ty))
    );
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_nested_generic_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_in_function_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function wrap<T>(value: T): () => T {
                return () => value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let wrap_ty = inferred_binding_ty_by_name(&db, module, inferred, "wrap")
        .expect("wrap binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        module,
        inferred.resolve_type(&db, wrap_ty),
        Vec::from([InferredTypeData::Number]),
    );
    let InferredTypeData::Function(function) = call_ty else {
        panic!("wrap must return a function, got {call_ty:?}");
    };
    let InferredReturnType::Type(return_ty) = function.return_type(&db) else {
        panic!("nested function return type must be inferred");
    };

    assert!(is_inferred_number(&db, *return_ty));
}

#[test]
fn test_infer_call_expression_type_substitutes_multiple_generics_inside_union_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function pair<T, U>(left: T, right: U): Promise<T | U> {
                return Promise.resolve(left as T | U);
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let pair_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "pair")
        .expect("pair binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, pair_ty),
        Vec::from([InferredTypeData::String, InferredTypeData::Number]),
    );

    assert!(is_inferred_promise_with_type_parameter(
        &db,
        call_ty,
        |ty| { contains_inferred_string(&db, ty) && contains_inferred_number(&db, ty) }
    ));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_multiple_generics_inside_union_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_inside_intersection_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type WithName = {
                name: string;
            };

            export function withName<T>(value: T): T & WithName {
                return value as T & WithName;
            }

            export function readValue(): { value: number } {
                return { value: 1 };
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let with_name_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "withName")
        .expect("withName binding type must be inferred");
    let read_value_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readValue")
            .expect("readValue return type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, with_name_ty),
        Vec::from([read_value_ty]),
    );
    let InferredTypeData::Object(object) = call_ty else {
        panic!("withName call must return a normalized object, got {call_ty:?}");
    };
    assert_eq!(object.members(&db).len(), 2);

    let name_ty = inferred
        .find_member_type(&db, call_ty, "name")
        .expect("normalized call return must expose WithName.name");
    assert!(is_inferred_string(&db, name_ty));

    let value_ty = inferred
        .find_member_type(&db, call_ty, "value")
        .expect("normalized call return must expose readValue.value");
    assert!(is_inferred_number(&db, value_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_generic_inside_intersection_return_type",
        &db,
        &fs,
    );
}
