use super::*;

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
