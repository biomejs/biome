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
            run(async (optional?) => {});
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
