use super::*;
use biome_module_graph::{
    ExpressionTypeInput, infer_expression_function_returns_promise,
    type_inference::TypeInferenceClassification,
};

fn expression_function_returns_promise(
    db: &TestModuleDb,
    module: ModuleInfo,
    source: &str,
    expression: &str,
) -> TypeInferenceClassification {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        panic!("module must contain JavaScript information");
    };
    let range = js_info
        .raw_expressions
        .keys()
        .find(|range| source_snippet(source, **range) == expression)
        .copied()
        .unwrap_or_else(|| panic!("{expression} expression must exist"));
    infer_expression_function_returns_promise(db, ExpressionTypeInput::new(db, module, range))
}

#[test]
fn test_expression_function_return_classifies_selected_object_members() {
    const SOURCE: &str = r#"
        declare function consume(value: unknown): void;
        const callbacks = {
            selected: async () => {},
            cacheDir: "/tmp",
        };
        consume(callbacks.selected);
        consume(callbacks.cacheDir);
        consume({ selected: async () => {}, cacheDir: "/tmp" });
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    assert_eq!(
        expression_function_returns_promise(&db, module, SOURCE, "callbacks.selected"),
        TypeInferenceClassification::Match
    );
    assert_eq!(
        expression_function_returns_promise(&db, module, SOURCE, "callbacks.cacheDir"),
        TypeInferenceClassification::NoMatch
    );
    assert_eq!(
        expression_function_returns_promise(
            &db,
            module,
            SOURCE,
            r#"{ selected: async () => {}, cacheDir: "/tmp" }"#,
        ),
        TypeInferenceClassification::NoMatch
    );
}

#[test]
fn test_expression_function_return_classifies_local_aliases_and_static_members() {
    const SOURCE: &str = r#"
        declare function consume(value: unknown): void;
        const callback = async () => {};
        const alias = callback;
        class Callbacks {
            static selected = alias;
        }
        consume(alias);
        consume(Callbacks.selected);
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    assert_eq!(
        expression_function_returns_promise(&db, module, SOURCE, "alias"),
        TypeInferenceClassification::Match
    );
    assert_eq!(
        expression_function_returns_promise(&db, module, SOURCE, "Callbacks.selected"),
        TypeInferenceClassification::Match
    );
}

#[test]
fn test_expression_function_return_classifies_promise_like_returns() {
    const SOURCE: &str = r#"
        interface PromiseLike<T> {
            then(resolve: (value: T) => void): void;
        }
        declare function callback(): PromiseLike<void>;
        declare function consume(value: unknown): void;
        consume(callback);
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    assert_eq!(
        expression_function_returns_promise(&db, module, SOURCE, "callback"),
        TypeInferenceClassification::Match
    );
}

#[test]
fn test_expression_function_return_classifies_returned_calls() {
    const SOURCE: &str = r#"
        declare function invokeAsync(callback: () => void): Promise<void>;
        declare function invokeSync(callback: () => void): void;
        declare function identity<T>(value: T): T;
        declare function consume(value: unknown): void;
        const asyncCallback = () => invokeAsync(() => {});
        const syncCallback = () => invokeSync(() => {});
        const genericCallback = () => identity(Promise.resolve());
        class Runner {
            async otherMethod() {}
            callback() { return this.otherMethod(); }
            run() { consume(this.callback); }
        }
        consume(asyncCallback);
        consume(syncCallback);
        consume(genericCallback);
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for (expression, expected) in [
        ("asyncCallback", TypeInferenceClassification::Match),
        ("syncCallback", TypeInferenceClassification::NoMatch),
        (
            "genericCallback",
            TypeInferenceClassification::Indeterminate,
        ),
        ("this.callback", TypeInferenceClassification::Match),
    ] {
        assert_eq!(
            expression_function_returns_promise(&db, module, SOURCE, expression),
            expected,
            "{expression}"
        );
    }
}

#[test]
fn test_expression_function_return_classifies_this_member_chains() {
    const SOURCE: &str = r#"
        declare function consume(value: unknown): void;
        class Runner {
            #settings = {
                config: {
                    callback: async () => {},
                    cacheDir: "/tmp",
                },
            };

            run() {
                consume(this.#settings.config.callback);
                consume(this.#settings.config.cacheDir);
            }
        }
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    assert_eq!(
        expression_function_returns_promise(&db, module, SOURCE, "this.#settings.config.callback",),
        TypeInferenceClassification::Match
    );
    assert_eq!(
        expression_function_returns_promise(&db, module, SOURCE, "this.#settings.config.cacheDir",),
        TypeInferenceClassification::NoMatch
    );
}

#[test]
fn test_expression_function_return_classifies_imported_named_aliases() {
    const SOURCE: &str = r#"
        export const callback = async () => {};
        export const callbacks = { selected: callback };
    "#;
    const INDEX: &str = r#"
        import { callback as importedCallback, callbacks as importedCallbacks } from "./source.ts";
        declare function consume(value: unknown): void;
        consume(importedCallback);
        consume(importedCallbacks.selected);
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/source.ts".into(), SOURCE);
    fs.insert("/src/index.ts".into(), INDEX);
    let db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    assert_eq!(
        expression_function_returns_promise(&db, module, INDEX, "importedCallback"),
        TypeInferenceClassification::Match
    );
    assert_eq!(
        expression_function_returns_promise(&db, module, INDEX, "importedCallbacks.selected"),
        TypeInferenceClassification::Match
    );
}

#[test]
fn test_expression_function_return_keeps_member_path_in_cycle_identity() {
    const SOURCE: &str = r#"
        type Recursive = {
            next: Recursive;
            callback: () => Promise<void>;
        };
        declare const root: Recursive;
        declare function consume(value: unknown): void;
        consume(root.next.callback);
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    assert_eq!(
        expression_function_returns_promise(&db, module, SOURCE, "root.next.callback"),
        TypeInferenceClassification::Match
    );
}

#[test]
fn test_expression_function_return_classifies_callable_interfaces_conservatively() {
    const SOURCE: &str = r#"
        interface AsyncCallable {
            (): Promise<void>;
        }
        interface SyncCallable {
            (): void;
        }
        interface InheritedAsyncCallable extends AsyncCallable {}
        interface OverloadedCallable {
            (): Promise<void>;
            (value: string): Promise<void>;
        }
        interface MixedCallable {
            (): void;
            (value: string): Promise<void>;
        }
        interface CyclicCallable extends CyclicCallable {
            (): Promise<void>;
        }
        type AsyncObject = {
            (): Promise<void>;
        };
        type SyncObject = {
            (): void;
        };
        type OverloadedObject = {
            (): Promise<void>;
            (value: string): Promise<void>;
        };
        type MixedObject = {
            (): void;
            (value: string): Promise<void>;
        };
        declare const asyncCallback: AsyncCallable;
        declare const syncCallback: SyncCallable;
        declare const inheritedAsyncCallback: InheritedAsyncCallable;
        declare const overloadedCallback: OverloadedCallable;
        declare const mixedCallback: MixedCallable;
        declare const cyclicCallback: CyclicCallable;
        declare const asyncObject: AsyncObject;
        declare const syncObject: SyncObject;
        declare const overloadedObject: OverloadedObject;
        declare const mixedObject: MixedObject;
        declare const unionCallback: (() => Promise<void>) | (() => Promise<number>);
        declare function consume(value: unknown): void;
        consume(asyncCallback);
        consume(syncCallback);
        consume(inheritedAsyncCallback);
        consume(overloadedCallback);
        consume(mixedCallback);
        consume(cyclicCallback);
        consume(asyncObject);
        consume(syncObject);
        consume(overloadedObject);
        consume(mixedObject);
        consume(unionCallback);
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for (expression, expected) in [
        ("asyncCallback", TypeInferenceClassification::Match),
        ("syncCallback", TypeInferenceClassification::NoMatch),
        ("inheritedAsyncCallback", TypeInferenceClassification::Match),
        (
            "overloadedCallback",
            TypeInferenceClassification::Indeterminate,
        ),
        ("mixedCallback", TypeInferenceClassification::Indeterminate),
        ("cyclicCallback", TypeInferenceClassification::Match),
        ("asyncObject", TypeInferenceClassification::Match),
        ("syncObject", TypeInferenceClassification::NoMatch),
        (
            "overloadedObject",
            TypeInferenceClassification::Indeterminate,
        ),
        ("mixedObject", TypeInferenceClassification::Indeterminate),
        ("unionCallback", TypeInferenceClassification::Indeterminate),
    ] {
        assert_eq!(
            expression_function_returns_promise(&db, module, SOURCE, expression),
            expected,
            "{expression}"
        );
    }
}

fn inferred_promise_value_type<'db>(
    db: &'db dyn ModuleDb,
    mut ty: InferredTypeData<'db>,
) -> Option<InferredTypeData<'db>> {
    while let InferredTypeData::InstanceOf(instance) = ty {
        ty = instance.ty(db);
        if ty.is_promise_class(db) {
            return instance.type_parameters(db).first().copied();
        }
    }

    None
}

#[test]
fn test_infer_module_types_resolves_promise_member_chain() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Parent {
                async returnsPromise(): Promise<string> {
                    return "value";
                }
            }

            export class Child extends Parent {}

            export const direct = new Child().returnsPromise();
            export const then = direct.then(() => 42);
            export const flattenedResolved = direct.then(() => Promise.resolve(42));
            export const flattenedAsync = direct.then(async () => 42);
            export const passthrough = direct.then();
            export const recovered = direct.catch(() => false);
            export const finalResult = direct.finally(() => {});
            export const awaitedThen = await then;
            export const awaitedResolved = await flattenedResolved;
            export const awaitedAsync = await flattenedAsync;
            export const awaitedFinal = await finalResult;
            export const awaitedRecovered = await recovered;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    for name in [
        "direct",
        "then",
        "flattenedResolved",
        "flattenedAsync",
        "passthrough",
        "recovered",
        "finalResult",
    ] {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .expect("binding type must be inferred");
        let ty = inferred.resolve_type(&db, ty);
        assert!(
            is_inferred_promise_instance(&db, ty),
            "{name} must be a Promise, got {}",
            format_inferred_type(&db, ty)
        );
    }

    let promise_value = |name| {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .expect("Promise binding type must be inferred");
        inferred_promise_value_type(&db, inferred.resolve_type(&db, ty))
            .expect("Promise must retain its value type")
    };

    assert!(contains_inferred_string(&db, promise_value("direct")));
    assert!(contains_inferred_number(&db, promise_value("then")));
    for name in ["flattenedResolved", "flattenedAsync"] {
        assert!(contains_inferred_number(&db, promise_value(name)));
    }
    assert!(contains_inferred_string(&db, promise_value("passthrough")));
    assert!(contains_inferred_string(&db, promise_value("finalResult")));
    let recovered = promise_value("recovered");
    assert!(contains_inferred_string(&db, recovered));
    assert!(contains_inferred_boolean(&db, recovered));

    for name in ["awaitedThen", "awaitedResolved", "awaitedAsync"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .expect("awaited binding type must be inferred");
        assert!(contains_inferred_number(
            &db,
            inferred.resolve_type(&db, ty)
        ));
    }
    let awaited_final = inferred_binding_ty_by_name(&db, index_module, inferred, "awaitedFinal")
        .expect("awaited finally value must be inferred");
    assert!(contains_inferred_string(
        &db,
        inferred.resolve_type(&db, awaited_final)
    ));
    let awaited_recovered =
        inferred_binding_ty_by_name(&db, index_module, inferred, "awaitedRecovered")
            .expect("awaited catch value must be inferred");
    let awaited_recovered = inferred.resolve_type(&db, awaited_recovered);
    assert!(contains_inferred_string(&db, awaited_recovered));
    assert!(contains_inferred_boolean(&db, awaited_recovered));

    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_promise_member_chain",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_await_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const promised: Promise<string> = Promise.resolve("value");

            export async function consume() {
                const awaited = await promised;
                const primitive = await 1;
                return awaited;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let promised_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "promised")
        .expect("promised binding type must be inferred");
    let promised_ty = inferred.resolve_type(&db, promised_ty);
    assert!(
        is_inferred_promise_with_type_parameter(&db, promised_ty, |ty| is_inferred_string(&db, ty)),
        "expected promised to be Promise<string>, got {}",
        format_inferred_type(&db, promised_ty)
    );

    let awaited_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "awaited")
        .expect("awaited binding type must be inferred");
    let awaited_ty = inferred.resolve_type(&db, awaited_ty);
    assert!(
        is_inferred_string(&db, awaited_ty),
        "expected awaited to be string, got {}",
        format_inferred_type(&db, awaited_ty)
    );

    let primitive_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "primitive")
        .expect("primitive binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, primitive_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_await_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_preserves_floating_promise_shapes() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Cheating<T extends 1> = T extends 1 ? Promise<string> : Promise<string>;

            async function promiseLike(): Cheating<1> {
                return "value";
            }

            const sneakyObject = {
                get something() {
                    return new Promise((_, reject) => reject("value"));
                },
            };

            function wrapper<F extends (...args: any) => any>(fn: F): F {
                return fn;
            }

            async function doWork(): Promise<void> {}

            export const mappedAsync = [1, 2, 3].map(async (value) => value + 1);
            export const mappedPromise = [1, 2, 3].map((value) => Promise.resolve(value + 1));
            export const conditional = promiseLike();
            export const getter = sneakyObject.something;
            export const wrapped = wrapper(doWork)();
            export const maybeDoWork: typeof doWork | undefined = doWork;
            export const optional = maybeDoWork?.();
            export const globalChain = globalThis.Promise.reject("value").finally();

            await new Promise((resolve) => resolve("value"));
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    for name in ["mappedAsync", "mappedPromise"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .expect("array binding type must be inferred");
        let ty = normalize_type(&db, index_module, ty);
        assert!(
            is_inferred_array_of_promises(&db, ty),
            "{name} must be an array of Promises, got {}",
            format_inferred_type(&db, ty)
        );
    }

    for name in ["conditional", "getter", "wrapped", "globalChain"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .expect("Promise binding type must be inferred");
        let ty = normalize_type(&db, index_module, ty);
        assert!(
            is_inferred_promise_instance(&db, ty),
            "{name} must be a Promise, got {}",
            format_inferred_type(&db, ty)
        );
    }

    let optional = inferred_binding_ty_by_name(&db, index_module, inferred, "optional")
        .expect("optional binding type must be inferred");
    let optional = normalize_type(&db, index_module, optional);
    let InferredTypeData::Union(optional) = optional else {
        panic!("optional call must preserve a Promise | undefined union, got {optional:?}");
    };
    assert!(
        optional
            .types(&db)
            .iter()
            .any(|ty| is_inferred_promise_instance(&db, *ty))
    );
    assert!(optional.types(&db).contains(&InferredTypeData::Undefined));

    let maybe_do_work = inferred_binding_ty_by_name(&db, index_module, inferred, "maybeDoWork")
        .expect("maybeDoWork binding type must be inferred");
    let optional_call = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, maybe_do_work),
        Vec::new(),
    );
    let InferredTypeData::Union(optional_call) = optional_call else {
        panic!(
            "optional call query must preserve Promise | undefined, got {}",
            format_inferred_type(&db, optional_call)
        );
    };
    assert!(
        optional_call
            .types(&db)
            .iter()
            .any(|ty| is_inferred_promise_instance(&db, *ty))
    );
    assert!(
        optional_call
            .types(&db)
            .contains(&InferredTypeData::Undefined)
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_preserves_floating_promise_shapes",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_await_promise_like_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class StringPromise extends Promise<string> {}

            export interface PromiseLike<T> {
                then(resolve: (value: T) => void): void;
            }

            export async function consume(
                subclass: StringPromise,
                like: PromiseLike<number>,
            ) {
                const awaitedSubclass = await subclass;
                const awaitedLike = await like;
                return awaitedSubclass;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let awaited_subclass_ty =
        inferred_binding_ty_by_name(&db, index_module, inferred, "awaitedSubclass")
            .expect("awaitedSubclass binding type must be inferred");
    let awaited_subclass_ty = inferred.resolve_type(&db, awaited_subclass_ty);
    assert!(
        is_inferred_string(&db, awaited_subclass_ty),
        "awaitedSubclass must be string, got {}",
        format_inferred_type(&db, awaited_subclass_ty)
    );

    let awaited_like_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "awaitedLike")
        .expect("awaitedLike binding type must be inferred");
    let awaited_like_ty = inferred.resolve_type(&db, awaited_like_ty);
    assert!(
        is_inferred_number(&db, awaited_like_ty),
        "awaitedLike must be number, got {}",
        format_inferred_type(&db, awaited_like_ty)
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_await_promise_like_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_await_union_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export async function consume(value: Promise<Promise<string>> | number | undefined) {
                const awaited = await value;
                return awaited;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let awaited_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "awaited")
        .expect("awaited binding type must be inferred");
    let awaited_ty = inferred.resolve_type(&db, awaited_ty);
    assert!(contains_inferred_string(&db, awaited_ty));
    assert!(contains_inferred_number(&db, awaited_ty));
    assert!(contains_inferred_undefined(&db, awaited_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_await_union_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_await_local_union_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            let promise: Promise<void> | undefined;

            async function sleep(): Promise<void> {
                return;
            }

            export async function consume() {
                if (!promise) {
                    promise = sleep();
                }

                const awaited = await promise;
                return awaited;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let awaited_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "awaited")
        .expect("awaited binding type must be inferred");
    let awaited_ty = inferred.resolve_type(&db, awaited_ty);
    assert!(contains_inferred_undefined(&db, awaited_ty));
    assert!(!contains_inferred_instance(&db, awaited_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_await_local_union_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_preserves_optional_chain_short_circuit_types_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/data.ts".into(),
        r#"
            export interface Usage { range: { startDate: string } }
            export interface LogEntry {
                id: string;
                createdAt: { toISOString(): string };
            }

            export declare function getUsage(): Promise<Usage | null>;
            export declare function getLogs(): Promise<LogEntry[]>;
            export declare function getRows(): Promise<string[] | null>;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { getLogs, getRows, getUsage } from "./data.ts";

            export async function read() {
                const usage = await getUsage();
                const startDate = usage?.range.startDate;

                const logs = await getLogs();
                const byIndex = logs[0]?.id;
                const [first] = logs;
                const byDestructuring = first?.createdAt.toISOString();

                const rows = await getRows();
                const byNullableIndex = rows?.[0];

                return { startDate, byIndex, byDestructuring, byNullableIndex };
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/data.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    for name in ["startDate", "byIndex", "byDestructuring", "byNullableIndex"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .unwrap_or_else(|| panic!("{name} binding type must be inferred"));
        let ty = inferred.resolve_type(&db, ty);
        assert!(
            contains_inferred_string(&db, ty),
            "{name} must contain string, got {}",
            format_inferred_type(&db, ty)
        );
        assert!(
            contains_inferred_undefined(&db, ty),
            "{name} must contain undefined, got {}",
            format_inferred_type(&db, ty)
        );
    }

    assert_inferred_type_snapshot(
        "test_infer_module_types_preserves_optional_chain_short_circuit_types_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_inside_promise_union_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function boxed<T>(value: T): Promise<T | number> {
                return Promise.resolve(value as T | number);
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let boxed_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "boxed")
        .expect("boxed binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, boxed_ty),
        Vec::from([InferredTypeData::String]),
    );

    assert!(is_inferred_promise_with_type_parameter(
        &db,
        call_ty,
        |ty| { contains_inferred_string(&db, ty) && contains_inferred_number(&db, ty) }
    ));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_generic_inside_promise_union_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_inside_union_with_promise_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function maybePromise<T>(value: T): T | Promise<T> {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let maybe_promise_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "maybePromise")
        .expect("maybePromise binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, maybe_promise_ty),
        Vec::from([InferredTypeData::String]),
    );
    let InferredTypeData::Union(union) = call_ty else {
        panic!("maybePromise must return a union, got {call_ty:?}");
    };

    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| contains_inferred_string(&db, *ty))
    );
    assert!(union.types(&db).iter().any(|ty| {
        is_inferred_promise_with_type_parameter(&db, *ty, |ty| contains_inferred_string(&db, ty))
    }));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_generic_inside_union_with_promise_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_from_callback_promise_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function unwrap<T>(cb: () => Promise<T>): T {
                return undefined as T;
            }

            export function readNumber(): Promise<number> {
                return Promise.resolve(1);
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let unwrap_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "unwrap")
        .expect("unwrap binding type must be inferred");
    let read_number_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readNumber")
        .expect("readNumber binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, unwrap_ty),
        Vec::from([inferred.resolve_type(&db, read_number_ty)]),
    );

    assert!(is_inferred_number(&db, call_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_generic_from_callback_promise_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_from_callback_promise_union_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function unwrap<T>(cb: () => T | Promise<T>): T {
                return undefined as T;
            }

            export function readNumber(): Promise<number> {
                return Promise.resolve(1);
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let unwrap_ty = inferred_binding_ty_by_name(&db, module, inferred, "unwrap")
        .expect("unwrap binding type must be inferred");
    let read_number_ty = inferred_binding_ty_by_name(&db, module, inferred, "readNumber")
        .expect("readNumber binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        module,
        inferred.resolve_type(&db, unwrap_ty),
        Vec::from([inferred.resolve_type(&db, read_number_ty)]),
    );

    assert!(is_inferred_number(&db, call_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_generic_from_callback_promise_union_return_type",
        &db,
        &fs,
    );
}
