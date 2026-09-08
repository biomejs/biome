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
