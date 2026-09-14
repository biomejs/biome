//! Binding queries for contextually typed callback parameters.

use super::*;
use biome_module_graph::{BindingTypeInput, infer_binding_type};

const SERVICE_DECLARATIONS: &str = r#"
    interface Service {
        go(): Promise<void>;
    }
    interface Context {
        service: Service;
    }
"#;

#[test]
fn test_collection_overloads_keep_opaque_objects_viable() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        "declare function read(value: Iterable<string>): number; declare function read(value: object): string; declare const value: object; const result = read(value);",
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    assert!(is_inferred_number(
        &db,
        normalized_binding_ty(&db, module, "result")
    ));
}

#[test]
fn test_concrete_parameters_do_not_exhaust_generic_inference() {
    let variants = (0..65)
        .map(|index| format!("'value{index}'"))
        .collect::<Vec<_>>()
        .join(" | ");
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        format!(
            "declare function read(value: string): number; declare const value: {variants}; const result = read(value);"
        ),
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    assert!(is_inferred_number(
        &db,
        normalized_binding_ty(&db, module, "result")
    ));
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        format!(
            "declare function read<T>(tag: string, value: T): T; declare const tag: {variants}; declare const value: number; const result = read(tag, value);"
        ),
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    assert!(is_inferred_number(
        &db,
        normalized_binding_ty(&db, module, "result")
    ));
}

#[test]
fn test_array_from_preserves_promise_returning_mappers() {
    for call in [
        "Array.from('abc', async value => value)",
        "Array.from('abc', mapper)",
        "Array.from<string, Promise<string>>('abc', async value => value)",
    ] {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/src/index.ts".into(),
            format!(
                "declare const mapper: (value: string) => Promise<string>; const result = {call};"
            ),
        );
        let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
        let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
        let result = normalized_binding_ty(&db, module, "result");
        let InferredTypeData::InstanceOf(array) = result else {
            panic!("{call}: expected array");
        };
        assert!(array.ty(&db).is_array_class(&db), "{call}");
        let element = *array.type_parameters(&db).first().expect("array element");
        assert!(
            is_inferred_promise_instance(&db, element),
            "{call}: {}",
            biome_js_type_info::format_inferred_type(&db, element)
        );
        let InferredTypeData::InstanceOf(promise) = element else {
            unreachable!()
        };
        assert!(
            is_inferred_string(
                &db,
                *promise.type_parameters(&db).first().expect("Promise value")
            ),
            "{call}"
        );
    }
}

#[test]
fn test_array_from_infers_source_elements_and_mapper_parameters() {
    for declaration in [
        "declare const contexts: Context[];",
        "declare const contexts: readonly Context[];",
        "declare const contexts: ReadonlyArray<Context>;",
        "declare const contexts: [Context, Context];",
        "declare const contexts: { [Symbol.iterator](): { next(): { value: Context; done: false } | { value: undefined; done: true } } };",
        "interface Contexts extends Iterable<Context> {} declare const contexts: Contexts;",
        "interface Contexts extends ArrayLike<Context> {} declare const contexts: Contexts;",
        "declare const contexts: Set<Context>;",
        "declare const contexts: ReadonlySet<Context>;",
        "declare const contexts: IterableIterator<Context>;",
        "declare const contexts: Generator<Context, void, unknown>;",
        "declare const contexts: { [Symbol.iterator](): Iterator<Context> };",
        "declare const contexts: { [Symbol.iterator]: () => { next(): IteratorResult<Context> } };",
        "declare const contexts: Iterable<Context>;",
        "declare const contexts: ArrayLike<Context>;",
        "declare const contexts: { length: number; [key: number]: Context };",
        "declare const first: Context; const contexts = { 0: first, length: 1 };",
    ] {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/src/index.ts".into(),
            source_with_declarations(&format!(
                r#"
                {declaration}
                const copied = Array.from(contexts);
                const copiedElement = copied[0];
                const mapped = Array.from(contexts, (context, index) => context.service);
                const mappedElement = mapped[0];
            "#
            )),
        );
        let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
        let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();

        for name in ["copiedElement", "context"] {
            assert_has_service_returning_promise(
                &db,
                module,
                normalized_binding_ty(&db, module, name),
            );
        }
        assert!(is_inferred_number(
            &db,
            normalized_binding_ty(&db, module, "index")
        ));
        assert_service_returns_promise(
            &db,
            module,
            normalized_binding_ty(&db, module, "mappedElement"),
        );
    }
}

#[test]
fn test_array_from_collection_and_overload_results() {
    let mut failures = Vec::new();
    for (source, expected) in [
        (
            "declare const source: object; const result = Array.from(source);",
            "unknown",
        ),
        (
            "declare const source: { length: number; [key: string]: string | number; [key: number]: string }; const result = Array.from(source);",
            "string",
        ),
        (
            "interface Indexed { [key: number]: string } interface Source extends Indexed { length: number; [key: string]: string | number } declare const source: Source; const result = Array.from(source);",
            "string",
        ),
        (
            "const source = { length: 1, get 0() { return 'a'; } }; const result = Array.from(source);",
            "string: a",
        ),
        (
            "const source = { length: 1, get 0() { return 'a'; } }; const result = Array.from(source, value => value);",
            "string: a",
        ),
        (
            "const result = Array.from({ length: 1, '01': 'bad' });",
            "unknown",
        ),
        (
            "type Result<T> = IteratorResult<T>; declare const source: { [Symbol.iterator](): { next(): Result<string> } }; const result = Array.from(source);",
            "string",
        ),
        (
            "type Cursor<T> = Iterator<T>; declare const source: { [Symbol.iterator](): Cursor<string> }; const result = Array.from(source);",
            "string",
        ),
        (
            "interface Base { length: 2; 0: 'a' } interface Source extends Base { 1: 'b' } declare const source: Source; const result = Array.from(source, value => value);",
            "string: b | string: a",
        ),
        (
            "declare const source: { length: 2; 0: 'a' } & { 1: 'b' }; const result = Array.from(source);",
            "string: a | string: b",
        ),
        (
            "interface Base { length: 1; 0: string } interface Source extends Base { 0: 'a' } declare const source: Source; const result = Array.from(source);",
            "string: a",
        ),
        (
            "interface Indexed { [n: number]: string } interface Source extends Indexed { length: number } declare const source: Source; const result = Array.from(source);",
            "string",
        ),
        (
            "interface Indexed<T> { [n: number]: T } interface Source extends Indexed<string> { length: number } declare const source: Source; const result = Array.from(source);",
            "string",
        ),
        (
            "interface Source extends ArrayLike<string | number> { [n: number]: string } declare const source: Source; const result = Array.from(source);",
            "string",
        ),
        (
            "interface Base { [Symbol.iterator](): Iterator<string> } interface Source extends Base { [Symbol.iterator](): Iterator<'specific'> } declare const source: Source; const result = Array.from(source);",
            "string: specific",
        ),
        (
            "declare const source: {length: number} & {[key: number]: string}; const result = Array.from(source);",
            "string",
        ),
        ("const result = Array.from([]);", "never"),
        (
            "declare const source: ReadonlyMap<string, number>; const result = Array.from(source, entry => entry[0]);",
            "string",
        ),
        (
            "declare const source: [number?]; const result = Array.from(source);",
            "number | undefined",
        ),
        (
            "declare const source: [number, ...string[]]; const result = Array.from(source);",
            "number | string",
        ),
        (
            "declare const source: object; const result = Array.from({ length: 1, 0: 'known', ...source });",
            "unknown",
        ),
        (
            "interface Recursive extends Recursive { length: number } declare const source: Recursive; const result = Array.from(source);",
            "unknown",
        ),
        (
            "declare const source: { [Symbol.iterator](): {next(): {done: true; value: string}} }; const result = Array.from(source);",
            "never",
        ),
        ("const result = Array.from('abc');", "string"),
        (
            "const result = Array.from('abc', value => value);",
            "string",
        ),
        (
            "const result = Array.from({ length: 3 }, (_, index) => index);",
            "number",
        ),
        ("const result = Array.from({ length: 3 });", "unknown"),
        (
            "declare const source: unknown; const result = Array.from(source);",
            "unknown",
        ),
        (
            "const result = Array.from(['a', 'b'] as const);",
            "string: a | string: b",
        ),
        (
            "declare const source: Map<string, number>; const result = Array.from(source, entry => entry[1]);",
            "number",
        ),
        (
            "declare const source: Set<string>; const result = Array.from(source);",
            "string",
        ),
        (
            "declare const source: Iterable<string> | ArrayLike<number>; const result = Array.from(source);",
            "number | string",
        ),
        (
            "const from = Array.from; const result = from('abc', value => value);",
            "string",
        ),
        (
            "const result = Array.from<string>({ length: 3 });",
            "string",
        ),
        (
            "const result = Array.from<string, number>('abc', (_, index) => index);",
            "number",
        ),
        (
            "const result = Array.from('abc', (_, index) => index, {});",
            "number",
        ),
    ] {
        let fs = MemoryFileSystem::default();
        fs.insert("/src/index.ts".into(), source.to_string());
        let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
        let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
        let result = normalized_binding_ty(&db, module, "result");
        let InferredTypeData::InstanceOf(array) = result else {
            panic!(
                "{source}: expected array, got {}",
                biome_js_type_info::format_inferred_type(&db, result)
            );
        };
        assert!(array.ty(&db).is_array_class(&db), "{source}");
        let element = *array.type_parameters(&db).first().expect("array element");
        let actual = biome_js_type_info::format_inferred_type(&db, element);
        if actual != expected {
            failures.push(format!("{source}\nexpected: {expected}\nactual: {actual}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

fn source_with_declarations(source: &str) -> String {
    format!("{SERVICE_DECLARATIONS}\n{source}")
}

fn normalized_binding_ty<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    name: &str,
) -> InferredTypeData<'db> {
    let input = BindingTypeInput::new(db, module, binding_range_by_name(db, module, name));
    let ty = infer_binding_type(db, input).expect("binding type must be inferred");
    normalize_type(db, module, ty)
}

fn assert_has_service_returning_promise<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    ty: InferredTypeData<'db>,
) {
    let service = find_value_member_type(db, ty, "service").expect("service member must exist");
    assert_service_returns_promise(db, module, normalize_type(db, module, service));
}

fn assert_service_returns_promise<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    ty: InferredTypeData<'db>,
) {
    let go = find_value_member_type(db, ty, "go").expect("go member must exist");
    let go = normalize_type(db, module, go);
    let function = go.callable_function(db).expect("go must be callable");
    let InferredReturnType::Type(return_ty) = function.return_type(db) else {
        panic!("go must declare a return type");
    };
    let return_ty = normalize_type(db, module, *return_ty);
    assert!(
        is_inferred_promise_instance(db, return_ty),
        "go must return a Promise, got {return_ty:?}"
    );
}

#[test]
fn test_callback_parameter_uses_explicit_call_type_arguments() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare function test<T>(callback: (input: T) => Promise<void>): void;
            test<Context>(async (args) => {
                const service = args.service;
            });
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    let args = normalized_binding_ty(&db, module, "args");
    assert_has_service_returning_promise(&db, module, args);

    let service = normalized_binding_ty(&db, module, "service");
    assert_service_returns_promise(&db, module, service);
}

#[test]
fn test_callback_parameter_remains_generic_without_type_arguments() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare function test<T>(callback: (input: T) => Promise<void>): void;
            test(async (args) => {});
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    let args = normalized_binding_ty(&db, module, "args");
    assert!(
        find_value_member_type(&db, args, "service").is_none(),
        "unbound generic parameter must not expose members, got {args:?}"
    );
}

#[test]
fn test_callback_parameter_from_non_generic_signature() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare function run(callback: (input: Context) => Promise<void>): void;
            run(async (args) => {});
            run(async function (fnArgs) {});
            run((async (parenthesised) => {}));
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for name in ["args", "fnArgs", "parenthesised"] {
        let ty = normalized_binding_ty(&db, module, name);
        assert_has_service_returning_promise(&db, module, ty);
    }
}

#[test]
fn test_callback_parameter_of_unparenthesised_arrow_and_destructuring() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare function run(callback: (input: Context) => Promise<void>): void;
            run(async single => {});
            run(async ({ service }) => {});
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    let single = normalized_binding_ty(&db, module, "single");
    assert_has_service_returning_promise(&db, module, single);

    let service = normalized_binding_ty(&db, module, "service");
    assert_service_returns_promise(&db, module, service);
}

#[test]
fn test_callback_parameters_are_typed_by_position() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare function each(
                callback: (item: Context, index: number, done: boolean) => Promise<void>,
            ): void;
            each(async (item, index, done) => {});

            declare function reversed(
                callback: (position: number, value: Context) => Promise<void>,
            ): void;
            reversed(async (position, value) => {});

            declare function bound(
                callback: (this: Window, target: Context) => Promise<void>,
            ): void;
            bound(async function (target) {});
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for name in ["item", "value", "target"] {
        let ty = normalized_binding_ty(&db, module, name);
        assert_has_service_returning_promise(&db, module, ty);
    }
    for name in ["index", "position"] {
        let ty = normalized_binding_ty(&db, module, name);
        assert!(
            is_inferred_number(&db, ty),
            "{name} must be number, got {ty:?}"
        );
    }
    let done = normalized_binding_ty(&db, module, "done");
    assert!(
        is_inferred_boolean(&db, done),
        "done must be boolean, got {done:?}"
    );
}

#[test]
fn test_optional_callback_parameter_includes_undefined() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare function run(callback: (input?: Context) => Promise<void>): void;
            run(async (optional) => {});
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    let optional = normalized_binding_ty(&db, module, "optional");
    assert!(
        contains_inferred_undefined(&db, optional),
        "optional parameter must include undefined, got {optional:?}"
    );
    let InferredTypeData::Union(union) = optional else {
        panic!("optional parameter must be a union, got {optional:?}");
    };
    let context = union
        .types(&db)
        .iter()
        .copied()
        .find(|ty| find_value_member_type(&db, *ty, "service").is_some())
        .expect("optional parameter must include the Context type");
    assert_has_service_returning_promise(&db, module, context);
}

#[test]
fn test_default_initialized_callback_parameter_excludes_undefined() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare const fallback: Context;
            declare function run(callback: (input?: Context) => Promise<void>): void;
            run(async (defaulted = fallback) => {});
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    let defaulted = normalized_binding_ty(&db, module, "defaulted");
    assert!(
        !contains_inferred_undefined(&db, defaulted),
        "default-initialized parameter must exclude undefined, got {defaulted:?}"
    );
    assert_has_service_returning_promise(&db, module, defaulted);
}

#[test]
fn test_callback_parameter_selects_overload_by_sibling_argument() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare function schedule(kind: "sync", callback: (input: number) => void): void;
            declare function schedule(kind: "async", callback: (input: Context) => Promise<void>): void;
            schedule("sync", (syncArgs) => {});
            schedule("async", async (asyncArgs) => {});
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    let sync_args = normalized_binding_ty(&db, module, "syncArgs");
    assert!(
        is_inferred_number(&db, sync_args),
        "sync callback parameter must be number, got {sync_args:?}"
    );

    let async_args = normalized_binding_ty(&db, module, "asyncArgs");
    assert_has_service_returning_promise(&db, module, async_args);
}

#[test]
fn test_callback_parameter_of_constructor_argument() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare class Job {
                constructor(callback: (input: Context) => Promise<void>);
            }
            new Job(async (args) => {});
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    let args = normalized_binding_ty(&db, module, "args");
    assert_has_service_returning_promise(&db, module, args);
}

#[test]
fn test_callback_parameters_of_sibling_callbacks_do_not_depend_on_each_other() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare function both(
                first: (left: Context) => unknown,
                second: (right: Context) => unknown,
            ): void;
            both((left) => left.service, (right) => right.service);
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for name in ["left", "right"] {
        let ty = normalized_binding_ty(&db, module, name);
        assert_has_service_returning_promise(&db, module, ty);
    }
}

#[test]
fn test_callback_parameter_is_unknown_when_not_a_call_argument() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare function run(callback: (input: Context) => Promise<void>): void;
            const callback = async (detached) => {};
            run(callback);
            run(async (args, extra) => {});
            declare const callbacks: Array<(input: Context) => Promise<void>>;
            run(...callbacks);
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for name in ["detached", "extra"] {
        let ty = normalized_binding_ty(&db, module, name);
        assert_eq!(ty, InferredTypeData::Unknown, "{name} must remain unknown");
    }
}

#[test]
fn test_callback_parameter_query_does_not_infer_module_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
            declare function run(callback: (input: Context) => Promise<void>): void;
            run(async (args) => {});
            export const unrelated = 1;
        "#,
        ),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "args"));

    db.clear_salsa_events();
    let ty = infer_binding_type(&db, input).expect("binding type must be inferred");
    assert_has_service_returning_promise(&db, module, normalize_type(&db, module, ty));
    let events = db.take_salsa_events();

    assert_function_query_was_run(&db, infer_binding_type, input, &events);
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn test_callback_parameter_from_imported_generic_method() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/fixtures.ts".into(),
        r#"
        export interface Test<Args> {
            (body: (args: Args) => void): void;
            extend<T>(): Test<Args & T>;
        }
        export declare const base: Test<{}>;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
        import { base } from "./fixtures";
        const test = base.extend<{ service: { go(): Promise<void> } }>();
        test(({ service }) => {});
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts", "/src/fixtures.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let service = normalized_binding_ty(&db, module, "service");
    assert_service_returns_promise(&db, module, service);
}

#[test]
fn test_method_type_parameter_shadows_enclosing_type_parameter() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        interface Factory<T> {
            create<T = string>(): T;
            outer(): T;
        }
        declare const factory: Factory<number>;
        const explicit = factory.create<boolean>();
        const defaulted = factory.create();
        const outer = factory.outer();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    assert!(is_inferred_boolean(
        &db,
        normalized_binding_ty(&db, module, "explicit")
    ));
    assert!(is_inferred_string(
        &db,
        normalized_binding_ty(&db, module, "defaulted")
    ));
    assert!(is_inferred_number(
        &db,
        normalized_binding_ty(&db, module, "outer")
    ));
}

#[test]
fn test_non_generic_method_signatures_share_return_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        interface Factory<T> {
            first(): T | null;
            second(): T | null;
        }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let ModuleInfoKind::Js(info) = module.kind(&db) else {
        panic!("module must contain JavaScript information");
    };
    let return_type = |name: &str| {
        info.raw_types
            .iter()
            .find_map(|ty| match ty {
                biome_js_type_info::RawTypeData::Function(function)
                    if function
                        .name
                        .as_ref()
                        .is_some_and(|value| value.text() == name) =>
                {
                    Some(&function.return_type)
                }
                _ => None,
            })
            .expect("method signature must be collected")
    };

    assert_eq!(return_type("first"), return_type("second"));
}

#[test]
fn test_array_from_callback_tracks_only_consumed_modules() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/source.ts".into(),
        "export declare const source: Iterable<string>;",
    );
    fs.insert("/src/unrelated.ts".into(), "export const unrelated = 1;");
    fs.insert(
        "/src/index.ts".into(),
        r#"
        import { source } from "./source.ts";
        export const result = Array.from(source, (value, index) => value);
    "#,
    );
    let mut db = build_js_test_module_db(
        &fs,
        &["/src/source.ts", "/src/index.ts", "/src/unrelated.ts"],
        true,
    );
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let source_module = db.module_for_path(Utf8Path::new("/src/source.ts")).unwrap();
    let unrelated_module = db
        .module_for_path(Utf8Path::new("/src/unrelated.ts"))
        .unwrap();
    let range = binding_range_by_name(&db, module, "value");
    assert!(is_inferred_string(
        &db,
        normalized_binding_ty(&db, module, "value")
    ));

    for (path, changed_module, source, recomputes) in [
        (
            "/src/unrelated.ts",
            unrelated_module,
            "export const unrelated = 'changed';",
            false,
        ),
        (
            "/src/source.ts",
            source_module,
            "export declare const source: Iterable<number>;",
            true,
        ),
    ] {
        fs.insert(path.into(), source);
        let kind = resolve_js_module_kind_for_test(&fs, path, true);
        salsa::Setter::to(changed_module.set_kind(&mut db), kind);
        db.clear_salsa_events();
        let input = BindingTypeInput::new(&db, module, range);
        let ty = infer_binding_type(&db, input).unwrap();
        let ty = normalize_type(&db, module, ty);
        if recomputes {
            assert!(is_inferred_number(&db, ty));
        } else {
            assert!(is_inferred_string(&db, ty));
        }
        let events = db.take_salsa_events();
        if recomputes {
            assert_function_query_was_run(&db, infer_binding_type, input, &events);
        } else {
            assert_function_query_was_not_run(&db, infer_binding_type, input, &events);
        }
        for module in [module, source_module, unrelated_module] {
            assert_function_query_was_not_run(&db, infer_module_types, module, &events);
        }
    }
}

#[test]
fn test_generic_collection_arguments_contextually_type_callbacks() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        source_with_declarations(
            r#"
        declare function transform<T, U>(source: Iterable<T>, mapper: (value: T) => U): U[];
        declare const source: Context[];
        const result = transform(source, context => context.service);
        const service = result[0];
        const destructured = Array.from(source, ({ service: selected }) => selected);
    "#,
        ),
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    assert_has_service_returning_promise(
        &db,
        module,
        normalized_binding_ty(&db, module, "context"),
    );
    for name in ["service", "selected"] {
        assert_service_returns_promise(&db, module, normalized_binding_ty(&db, module, name));
    }
}

#[test]
fn test_iterator_signatures_keep_scoped_symbol_references_during_collection() {
    use biome_js_type_info::{RawTypeData, TypeMemberKind, TypeReference};

    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        interface MethodSource { [Symbol.iterator](): Iterator<string> }
        interface PropertySource { [Symbol.iterator]: () => Iterator<string> }
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let ModuleInfoKind::Js(info) = module.kind(&db) else {
        panic!("JavaScript module expected")
    };
    for name in ["MethodSource", "PropertySource"] {
        let interface = info
            .raw_types
            .iter()
            .find_map(|ty| match ty {
                RawTypeData::Interface(interface) if interface.name.text() == name => {
                    Some(interface)
                }
                _ => None,
            })
            .unwrap();
        let member = interface.members.first().expect("iterator member");
        let TypeMemberKind::ComputedValue(TypeReference::Qualifier(key)) = &member.kind else {
            panic!("iterator key must remain a scoped qualifier");
        };
        assert_eq!(
            key.path.iter().map(|part| part.text()).collect::<Vec<_>>(),
            ["Symbol", "iterator"]
        );
    }
}
