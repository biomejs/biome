use super::*;

#[test]
fn test_infer_module_types_selects_call_overloads_by_parameter_types_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function reader(value: string): string;
            export function reader(value: number): number;
            export function reader(left: number, right: number): boolean;
            export function reader(..._args: [string] | [number] | [number, number]) {
                return undefined as string | number | boolean;
            }

            export const textual = reader("value");
            export const numeric = reader(1);
            export const args = [1, 2];
            export const spread = reader(...args);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let textual_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "textual")
        .expect("textual binding type must be inferred");
    let textual_ty = inferred.resolve_type(&db, textual_ty);
    assert!(
        is_inferred_string(&db, textual_ty),
        "textual must be string, got {}",
        format_inferred_type(&db, textual_ty)
    );

    let numeric_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "numeric")
        .expect("numeric binding type must be inferred");
    let numeric_ty = inferred.resolve_type(&db, numeric_ty);
    assert!(
        is_inferred_number(&db, numeric_ty),
        "numeric must be number, got {}",
        format_inferred_type(&db, numeric_ty)
    );

    let spread_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "spread")
        .expect("spread binding type must be inferred");
    let spread_ty = inferred.resolve_type(&db, spread_ty);
    assert!(
        is_inferred_boolean(&db, spread_ty),
        "spread must be boolean, got {}",
        format_inferred_type(&db, spread_ty)
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_by_parameter_types_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_selects_call_overloads_by_nominal_class_parameters_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Foo {}
            export class Bar {}

            export function select(value: Foo): string;
            export function select(value: Bar): number;
            export function select(_value: Foo | Bar): string | number {
                return undefined as string | number;
            }

            export const selected = select(new Bar());
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let selected_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "selected")
        .expect("selected binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, selected_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_by_nominal_class_parameters_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_selects_call_overloads_for_subclass_arguments_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Base {}
            export class Derived extends Base {}
            export class Other {}

            export function select(value: Other): number;
            export function select(value: Base): string;
            export function select(_value: Other | Base): string | number {
                return undefined as string | number;
            }

            export const selected = select(new Derived());
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let selected_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "selected")
        .expect("selected binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, selected_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_for_subclass_arguments_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_selects_call_overloads_for_array_and_optional_tuple_spreads_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readArray(value: string): string;
            export function readArray(left: number, right: number): boolean;
            export function readArray(..._args: [string] | [number, number]) {
                return undefined as string | boolean;
            }

            export const numbers: number[] = [1, 2];
            export const arraySpread = readArray(...numbers);

            export function readTuple(value: string): string;
            export function readTuple(value: string, count: number): number;
            export function readTuple(..._args: [string, number?]) {
                return undefined as string | number;
            }

            export const optionalTuple: [string, number?] = ["value"];
            export const optionalTupleSpread = readTuple(...optionalTuple);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let array_spread_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "arraySpread")
        .expect("arraySpread binding type must be inferred");
    assert!(is_inferred_boolean(
        &db,
        inferred.resolve_type(&db, array_spread_ty)
    ));

    let optional_tuple_spread_ty =
        inferred_binding_ty_by_name(&db, index_module, inferred, "optionalTupleSpread")
            .expect("optionalTupleSpread binding type must be inferred");
    let optional_tuple_spread_ty = inferred.resolve_type(&db, optional_tuple_spread_ty);
    assert!(
        is_inferred_string(&db, optional_tuple_spread_ty),
        "optionalTupleSpread must be string, got {}",
        format_inferred_type(&db, optional_tuple_spread_ty)
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_for_array_and_optional_tuple_spreads_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_resolves_imported_default_function_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"
            export default function(): string {
                return "value";
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import readValue from "./base.ts";

            export const value = readValue();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/base.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let read_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readValue")
        .expect("readValue binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, read_value_ty),
        Vec::new(),
    );

    assert!(is_inferred_string(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_resolves_annotated_function_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const readValue: () => string = () => "value";
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let read_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readValue")
        .expect("readValue binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, read_value_ty),
        Vec::new(),
    );

    assert!(is_inferred_string(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_resolves_callable_interface_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Reader {
                (): string;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Interface(interface)
                    if interface.name(&db).text() == "Reader"
                        && interface
                            .members(&db)
                            .iter()
                            .any(|member| member.kind.is_call_signature())
            )
        })
        .expect("Reader interface type must be inferred");
    let call_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());

    assert!(is_inferred_string(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_selects_callable_interface_overload_by_arity() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Reader {
                (): string;
                (value: number): number;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Interface(interface)
                    if interface.name(&db).text() == "Reader"
                        && interface
                            .members(&db)
                            .iter()
                            .filter(|member| member.kind.is_call_signature())
                            .count()
                            == 2
            )
        })
        .expect("Reader interface type must be inferred");

    let zero_arg_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());
    assert!(is_inferred_string(&db, zero_arg_ty));

    let one_arg_ty = infer_call_expression_type(
        &db,
        index_module,
        reader_ty,
        Vec::from([InferredTypeData::Number]),
    );
    assert!(is_inferred_number(&db, one_arg_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_callable_interface_overload_by_arity",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_selects_callable_interface_overload_with_optional_parameter_by_arity()
 {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Reader {
                (): string;
                (value?: number): number;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Interface(interface)
                    if interface.name(&db).text() == "Reader"
                        && interface
                            .members(&db)
                            .iter()
                            .filter(|member| member.kind.is_call_signature())
                            .count()
                            == 2
            )
        })
        .expect("Reader interface type must be inferred");

    let zero_arg_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());
    assert!(is_inferred_string(&db, zero_arg_ty));

    let one_arg_ty = infer_call_expression_type(
        &db,
        index_module,
        reader_ty,
        Vec::from([InferredTypeData::Number]),
    );
    assert!(is_inferred_number(&db, one_arg_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_callable_interface_overload_with_optional_parameter_by_arity",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_selects_callable_interface_overload_with_rest_parameter_by_arity()
 {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Reader {
                (): string;
                (...values: number[]): number;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Interface(interface)
                    if interface.name(&db).text() == "Reader"
                        && interface
                            .members(&db)
                            .iter()
                            .filter(|member| member.kind.is_call_signature())
                            .count()
                            == 2
            )
        })
        .expect("Reader interface type must be inferred");

    let zero_arg_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());
    assert!(is_inferred_string(&db, zero_arg_ty));

    let many_arg_ty = infer_call_expression_type(
        &db,
        index_module,
        reader_ty,
        Vec::from([InferredTypeData::Number, InferredTypeData::Number]),
    );
    assert!(is_inferred_number(&db, many_arg_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_callable_interface_overload_with_rest_parameter_by_arity",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_resolves_callable_object_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export type Reader = {
                (): string;
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Object(object)
                    if object
                        .members(&db)
                        .iter()
                        .any(|member| member.kind.is_call_signature())
            )
        })
        .expect("Reader object type must be inferred");
    let call_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());

    assert!(is_inferred_string(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_selects_callable_object_overload_by_arity() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export type Reader = {
                (): string;
                (value: number): number;
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Object(object)
                    if object
                        .members(&db)
                        .iter()
                        .filter(|member| member.kind.is_call_signature())
                        .count()
                        == 2
            )
        })
        .expect("Reader object type must be inferred");

    let zero_arg_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());
    assert!(is_inferred_string(&db, zero_arg_ty));

    let one_arg_ty = infer_call_expression_type(
        &db,
        index_module,
        reader_ty,
        Vec::from([InferredTypeData::Number]),
    );
    assert!(is_inferred_number(&db, one_arg_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_callable_object_overload_by_arity",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_selects_function_declaration_overload_by_callback_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function bestEffort<T>(cb: () => Promise<T>): Promise<T | undefined>;
            export function bestEffort<T>(cb: () => T): T | undefined;
            export function bestEffort<T>(cb: (() => T) | (() => Promise<T>)) {
                return cb();
            }

            export function readPromise(): Promise<string> {
                return Promise.resolve("value");
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
    let best_effort_ty = inferred_overload_ty_by_name(&db, index_module, inferred, "bestEffort")
        .expect("bestEffort overload type must be inferred");
    let read_promise_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readPromise")
        .expect("readPromise binding type must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readString")
        .expect("readString binding type must be inferred");

    let promise_result_ty = infer_call_expression_type(
        &db,
        index_module,
        best_effort_ty,
        Vec::from([inferred.resolve_type(&db, read_promise_ty)]),
    );
    assert!(
        promise_result_ty.is_promise_instance(&db),
        "promise callback overload must return a Promise, got {promise_result_ty:?}",
    );
    let InferredTypeData::InstanceOf(instance) = promise_result_ty else {
        panic!("promise callback overload must return a Promise instance");
    };
    assert!(
        instance
            .type_parameters(&db)
            .iter()
            .any(|ty| contains_inferred_string(&db, *ty)),
        "promise callback overload must substitute the callback return type"
    );

    let sync_result_ty = infer_call_expression_type(
        &db,
        index_module,
        best_effort_ty,
        Vec::from([inferred.resolve_type(&db, read_string_ty)]),
    );
    assert_ne!(sync_result_ty, InferredTypeData::Unknown);
    assert!(
        !sync_result_ty.is_promise_instance(&db),
        "sync callback overload must not return a Promise, got {sync_result_ty:?}",
    );
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_function_declaration_overload_by_callback_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_selects_imported_function_declaration_overload_by_callback_return_type()
 {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/functions.ts".into(),
        r#"
            export function bestEffort<T>(cb: () => Promise<T>): Promise<T | undefined>;
            export function bestEffort<T>(cb: () => T): T | undefined;
            export function bestEffort<T>(cb: (() => T) | (() => Promise<T>)) {
                return cb();
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { bestEffort } from "./functions.ts";

            export function readPromise(): Promise<string> {
                return Promise.resolve("value");
            }

            export function readString(): string {
                return "value";
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/functions.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let best_effort_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "bestEffort")
        .expect("bestEffort import type must be inferred");
    let read_promise_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readPromise")
        .expect("readPromise binding type must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readString")
        .expect("readString binding type must be inferred");
    let best_effort_ty = inferred.resolve_type(&db, best_effort_ty);

    let promise_result_ty = infer_call_expression_type(
        &db,
        index_module,
        best_effort_ty,
        Vec::from([inferred.resolve_type(&db, read_promise_ty)]),
    );
    assert!(
        promise_result_ty.is_promise_instance(&db),
        "promise callback overload must return a Promise, got {promise_result_ty:?}",
    );
    let InferredTypeData::InstanceOf(instance) = promise_result_ty else {
        panic!("promise callback overload must return a Promise instance");
    };
    assert!(
        instance
            .type_parameters(&db)
            .iter()
            .any(|ty| contains_inferred_string(&db, *ty)),
        "promise callback overload must substitute the callback return type"
    );

    let sync_result_ty = infer_call_expression_type(
        &db,
        index_module,
        best_effort_ty,
        Vec::from([inferred.resolve_type(&db, read_string_ty)]),
    );
    assert_ne!(sync_result_ty, InferredTypeData::Unknown);
    assert!(
        !sync_result_ty.is_promise_instance(&db),
        "sync callback overload must not return a Promise, got {sync_result_ty:?}",
    );
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_imported_function_declaration_overload_by_callback_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_calls_generic_function_type_aliases() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Factory<T> = () => Promise<T>;
            declare const makeString: Factory<string>;
            export const result = makeString();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");

    let result_ty = inferred.resolve_type(&db, result_ty);
    assert!(
        is_inferred_promise_with_type_parameter(&db, result_ty, |ty| is_inferred_string(&db, ty)),
        "generic alias must return Promise<string>, got {}",
        format_inferred_type(&db, result_ty)
    );
}

#[test]
fn test_infer_module_types_calls_imported_generic_function_type_aliases() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/factory.ts".into(),
        r#"
            type Factory<T> = () => Promise<T>;
            export declare const makeString: Factory<string>;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { makeString } from "./factory";
            export const result = makeString();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/factory.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");
    let result_ty = inferred.resolve_type(&db, result_ty);

    assert!(is_inferred_promise_with_type_parameter(
        &db,
        result_ty,
        |ty| is_inferred_string(&db, ty)
    ));
    assert_inferred_type_snapshot(
        "test_infer_module_types_calls_imported_generic_function_type_aliases",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_calls_nested_generic_callable_aliases() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Callable<T> {
                (value: T): T;
            }

            type First<T> = Callable<T>;
            type Second<T> = First<T>;
            declare const call: Second<string>;
            export const result = call("value");
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");

    let result_ty = inferred.resolve_type(&db, result_ty);
    assert!(
        is_inferred_string(&db, result_ty),
        "nested generic alias must return string, got {}",
        format_inferred_type(&db, result_ty)
    );
}

#[test]
fn test_infer_call_expression_type_preserves_shadowed_nested_function_generic() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function makeIdentity<T>(value: T): <T>(value: T) => T {
                return value => value;
            }

            const identity = makeIdentity(1);
            export const result = identity("value");
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");

    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, result_ty),
        "value"
    ));
}

#[test]
fn test_infer_call_expression_type_resolves_union_function_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readString(): string {
                return "value";
            }

            export function readNumber(): number {
                return 1;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readString")
        .expect("readString binding type must be inferred");
    let read_number_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readNumber")
        .expect("readNumber binding type must be inferred");
    let callee_ty = InferredTypeData::Union(InferredUnion::new(
        &db,
        Vec::from([
            inferred.resolve_type(&db, read_string_ty),
            inferred.resolve_type(&db, read_number_ty),
        ])
        .into_boxed_slice(),
    ));
    let call_ty = infer_call_expression_type(&db, index_module, callee_ty, Vec::new());
    let InferredTypeData::Union(union) = call_ty else {
        panic!("union function call must return a union, got {call_ty:?} from {callee_ty:?}");
    };

    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_string(&db, *ty))
    );
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_number(&db, *ty))
    );
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_resolves_union_function_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_call_and_new_expressions_apply_explicit_type_arguments() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export declare function make<T>(): T;
            export declare class Box<T> {
                value: T;
            }
            export const made = make<string>();
            export const boxed = new Box<number>();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let made = inferred_binding_ty_by_name(&db, module, inferred, "made")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("made binding type must be inferred");
    assert!(
        is_inferred_string(&db, made),
        "explicit type argument must instantiate the return type, got {made:?}"
    );

    let boxed = inferred_binding_ty_by_name(&db, module, inferred, "boxed")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("boxed binding type must be inferred");
    let value = find_value_member_type(&db, boxed, "value")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("value member must be inferred");
    assert!(
        is_inferred_number(&db, value),
        "explicit type argument must instantiate the class, got {value:?}"
    );
}

#[test]
fn test_infer_module_types_applies_explicit_type_arguments_to_overload_sets_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Trooper = { armour: number };

            declare function garrison<T>(unit: T): { current: T };
            declare function garrison<T>(unit: T | null): { current: T | null };

            export const post = garrison<Trooper>(null);
            export const current = post.current;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let current = inferred_binding_ty_by_name(&db, module, inferred, "current")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("current binding type must be inferred");
    // The explicit `Trooper` must instantiate every overload before
    // selection, so `null` rejects `(unit: Trooper)` and the nullable
    // overload wins.
    let InferredTypeData::Union(union) = current else {
        panic!(
            "current must be `Trooper | null`, got {}",
            format_inferred_type(&db, current)
        );
    };
    let members = union.types(&db);
    assert_eq!(members.len(), 2, "{}", format_inferred_type(&db, current));
    assert!(
        members.contains(&InferredTypeData::Null),
        "{}",
        format_inferred_type(&db, current)
    );
    assert!(
        members.iter().any(|member| {
            let member = match member {
                InferredTypeData::InstanceOf(instance) => instance.ty(&db),
                member => *member,
            };
            matches!(
                inferred.resolve_type(&db, member),
                InferredTypeData::Object(_)
            )
        }),
        "{}",
        format_inferred_type(&db, current)
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_applies_explicit_type_arguments_to_overload_sets_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_call_expression_keeps_explicit_type_argument_over_inferable_argument() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export declare function clone<T>(unit: T): T;
            export const cloned = clone<string | null>(null);
            export const counted = clone<number | null>(0);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    // The argument would infer `T` as `null` / `0`; the written argument wins.
    let cloned = inferred_binding_ty_by_name(&db, module, inferred, "cloned")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("cloned binding type must be inferred");
    let InferredTypeData::Union(union) = cloned else {
        panic!(
            "cloned must keep `string | null`, got {}",
            format_inferred_type(&db, cloned)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(
        union
            .types(&db)
            .iter()
            .any(|member| is_inferred_string(&db, *member))
    );

    let counted = inferred_binding_ty_by_name(&db, module, inferred, "counted")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("counted binding type must be inferred");
    let InferredTypeData::Union(union) = counted else {
        panic!(
            "counted must keep `number | null`, got {}",
            format_inferred_type(&db, counted)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(
        union
            .types(&db)
            .iter()
            .any(|member| is_inferred_number(&db, *member))
    );
}

#[test]
fn test_infer_module_types_checks_overload_candidacy_for_explicit_type_arguments_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            declare function pick<T extends string>(x: T): { current: T };
            declare function pick<T>(x: T): { current: T | null };

            declare function f<T>(x: T): T | null;
            declare function f(x: null): object;
            declare function f<T>(x: T | null): T | null;

            declare function pair<T, U = T>(x: T): [T, U];
            declare function pair<T>(x: T | null): T | null;

            export const picked = pick<number>(0).current;
            export const named = pick<"name">("name").current;
            export const result = f<string>(null);
            export const paired = pair<string>(null);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let resolved = |name: &str| {
        inferred_binding_ty_by_name(&db, module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        )
    };

    // `number` violates `T extends string`, so the first overload is not a
    // candidate even though its instantiated parameter would accept `0`.
    let picked = resolved("picked");
    let InferredTypeData::Union(union) = picked else {
        panic!(
            "picked must be `number | null`, got {}",
            format_inferred_type(&db, picked)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_number(&db, *ty))
    );

    // A literal satisfying the constraint keeps the first overload.
    let named = resolved("named");
    assert!(
        is_inferred_string_literal(&db, named, "name"),
        "named must be `\"name\"`, got {}",
        format_inferred_type(&db, named)
    );

    // A signature without type parameters is not a candidate for `f<string>`.
    let result = resolved("result");
    let InferredTypeData::Union(union) = result else {
        panic!(
            "result must be `string | null`, got {}",
            format_inferred_type(&db, result)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_string(&db, *ty))
    );

    // One argument satisfies `<T, U = T>`, so the tuple overload stays a
    // candidate and its instantiated parameter rejects `null`.
    let paired = resolved("paired");
    let InferredTypeData::Union(union) = paired else {
        panic!(
            "paired must be `string | null`, got {}",
            format_inferred_type(&db, paired)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_string(&db, *ty))
    );
}

#[test]
fn test_infer_module_types_accepts_union_type_arguments_within_generic_constraints_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            declare function pick<T extends string | number | boolean>(x: T): T | null;
            declare function pick<T>(x: T): T;

            export const result = pick<string | number>(0);
            export const rejected = pick<string | { armour: number }>("");
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let resolved = |name: &str| {
        inferred_binding_ty_by_name(&db, module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        )
    };

    // Every member of `string | number` satisfies the constraint, so the
    // first overload remains a candidate and contributes `null`.
    let result = resolved("result");
    let InferredTypeData::Union(union) = result else {
        panic!(
            "result must be `string | number | null`, got {}",
            format_inferred_type(&db, result)
        );
    };
    let members = union.types(&db);
    assert!(
        members.contains(&InferredTypeData::Null),
        "{}",
        format_inferred_type(&db, result)
    );
    assert!(members.iter().any(|ty| is_inferred_string(&db, *ty)));
    assert!(members.iter().any(|ty| is_inferred_number(&db, *ty)));

    // `{ armour: number }` satisfies no constraint member, so the constrained
    // overload is rejected and the result keeps the written union without
    // `null`.
    let rejected = resolved("rejected");
    let InferredTypeData::Union(union) = rejected else {
        panic!(
            "rejected must be `string | {{ armour: number }}`, got {}",
            format_inferred_type(&db, rejected)
        );
    };
    assert!(
        !union.types(&db).contains(&InferredTypeData::Null),
        "{}",
        format_inferred_type(&db, rejected)
    );
}

#[test]
fn test_infer_module_types_accepts_callable_type_arguments_within_generic_constraints_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            declare function pick<T extends () => unknown>(): T | null;
            declare function pick<T>(): T;

            declare function sync<T extends () => string>(): T | null;
            declare function sync<T>(): T;

            export const result = pick<() => Promise<number>>();
            export const rejected = sync<() => Promise<string>>();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let resolved = |name: &str| {
        inferred_binding_ty_by_name(&db, module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        )
    };

    // `unknown` accepts a Promise, so the constrained overload stays a
    // candidate and contributes `null`.
    let result = resolved("result");
    let InferredTypeData::Union(union) = result else {
        panic!(
            "result must be `(() => Promise<number>) | null`, got {}",
            format_inferred_type(&db, result)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(union.types(&db).iter().any(|ty| {
        ty.callable_function(&db)
            .is_some_and(|function| function.returns_promise(&db))
    }));

    // A concrete `string` return still rejects a Promise-returning argument.
    let rejected = resolved("rejected");
    assert!(
        rejected.callable_function(&db).is_some(),
        "rejected must be the bare callable, got {}",
        format_inferred_type(&db, rejected)
    );
}

#[test]
fn test_infer_module_types_rejects_callable_type_arguments_outside_generic_constraints_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            declare function pick<T extends () => string | void>(): T | null;
            declare function pick<T>(): T;

            export const voided = pick<() => Promise<string>>();
            export const accepted = pick<() => void>();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let resolved = |name: &str| {
        inferred_binding_ty_by_name(&db, module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        )
    };

    // `void` inside `string | void` does not discard a Promise result, so the
    // constrained overload is rejected and no `null` is added.
    let voided = resolved("voided");
    assert!(
        voided.callable_function(&db).is_some(),
        "voided must be the bare callable, got {}",
        format_inferred_type(&db, voided)
    );

    // A `void`-returning callable satisfies the constraint.
    let accepted = resolved("accepted");
    let InferredTypeData::Union(union) = accepted else {
        panic!(
            "accepted must be `(() => void) | null`, got {}",
            format_inferred_type(&db, accepted)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
}

#[test]
fn test_infer_module_types_calls_instantiated_generic_alias_call_signatures_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Fns<T> = { (value: T): T };
            type Overloaded<T> = {
                (value: T): T;
                (value: T | null): T | null;
            };

            declare const format: Fns<string>;
            declare const lookup: Overloaded<string>;

            export const formatted = format("value");
            export const found = lookup(null);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let resolved = |name: &str| {
        inferred_binding_ty_by_name(&db, module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        )
    };

    // The alias's own arguments bind `T`; they are not call-site arguments
    // for the non-generic signature, which must remain selectable.
    let formatted = resolved("formatted");
    assert!(
        is_inferred_string(&db, formatted),
        "formatted must be `string`, got {}",
        format_inferred_type(&db, formatted)
    );

    let found = resolved("found");
    let InferredTypeData::Union(union) = found else {
        panic!(
            "found must be `string | null`, got {}",
            format_inferred_type(&db, found)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_string(&db, *ty))
    );
}
