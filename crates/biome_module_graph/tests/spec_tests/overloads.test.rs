use super::*;

#[test]
fn test_infer_module_types_selects_call_overloads_by_required_object_members_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Options { initial?: string }
            interface DefinedOptions extends Options { initial: string }

            type DefinedResult = { isPending: false };
            type MaybeResult = { isPending: true } | { isPending: false };

            declare function query(options: DefinedOptions): DefinedResult;
            declare function query(options: Options): MaybeResult;

            declare function select(options: { kind: "text" }): string;
            declare function select(options: { kind: "number" }): number;

            export const { isPending } = query({});
            export const selected = select({ kind: "number" });
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let is_pending_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "isPending")
        .expect("isPending binding type must be inferred");

    assert!(is_inferred_boolean(
        &db,
        inferred.resolve_type(&db, is_pending_ty)
    ));
    let selected_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "selected")
        .expect("selected binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, selected_ty)
    ));
    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_by_required_object_members_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_keeps_overloads_viable_for_partially_modelled_arguments_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Options { initial?: string }
            interface DefinedOptions extends Options { initial: string }

            declare function query(options: DefinedOptions): number;
            declare function query(options: Options): string;

            declare const defaults: DefinedOptions;
            declare function tag<T>(value: T): T & { tagged: true };

            export const fromSpread = query({ ...defaults });
            export const fromEmpty = query({});
            export const fromIntersection = query(tag({ ...defaults }));
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    // Inference cannot expand the spread into members, so `initial` may still
    // be present and the first overload stays viable.
    let from_spread_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "fromSpread")
        .expect("fromSpread binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, from_spread_ty)
    ));

    // An object literal without a spread lists every member it has, so the
    // first overload is rejected.
    let from_empty_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "fromEmpty")
        .expect("fromEmpty binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, from_empty_ty)
    ));

    // Intersecting the spread carries the missing members into the result, so
    // the first overload stays viable there too.
    let from_intersection_ty =
        inferred_binding_ty_by_name(&db, index_module, inferred, "fromIntersection")
            .expect("fromIntersection binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, from_intersection_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_keeps_overloads_viable_for_partially_modelled_arguments_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_selects_call_overloads_by_interface_call_signatures_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface SyncHandler { (): string }
            interface AsyncHandler { (): Promise<string> }
            type VoidHandler = () => void;

            declare function run(handler: SyncHandler): number;
            declare function run(handler: AsyncHandler): string;

            declare function schedule(handler: VoidHandler): number;
            declare function schedule(handler: AsyncHandler): string;

            export const ran = run(async () => "value");
            export const scheduled = schedule(async () => "value");
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    // `SyncHandler` returns a value the async handler cannot produce, so the
    // first overload is rejected.
    let ran_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "ran")
        .expect("ran binding type must be inferred");
    assert!(is_inferred_string(&db, inferred.resolve_type(&db, ran_ty)));

    // `VoidHandler` discards the result, so an async handler satisfies it and
    // the first overload wins.
    let scheduled_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "scheduled")
        .expect("scheduled binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, scheduled_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_by_interface_call_signatures_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_substitutes_callback_returns_through_type_aliases_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Mapper<T> = () => T;

            declare function mapAliased<T>(mapper: Mapper<T>): T;
            declare function mapInline<T>(mapper: () => T): T;

            export const aliased = mapAliased(() => 42);
            export const inline = mapInline(() => 42);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    for name in ["aliased", "inline"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .unwrap_or_else(|| panic!("{name} binding type must be inferred"));
        let ty = inferred.resolve_type(&db, ty);
        assert!(
            is_inferred_number_literal(&db, ty, "42"),
            "{name} must be the literal 42, got {}",
            format_inferred_type(&db, ty)
        );
    }

    assert_inferred_type_snapshot(
        "test_infer_module_types_substitutes_callback_returns_through_type_aliases_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_selects_call_overloads_for_union_arguments_within_union_parameters_on_build()
 {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            declare function pick(value: string | number | boolean): "primitive";
            declare function pick(value: unknown): "unknown";

            declare const subset: string | number;
            declare const mixed: string | { armour: number };

            export const fromSubset = pick(subset);
            export const fromMixed = pick(mixed);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    // Each member of the argument union fits the parameter union, so the
    // first overload is selected even though no single parameter member
    // accepts the whole argument.
    let from_subset_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "fromSubset")
        .expect("fromSubset binding type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, from_subset_ty),
        "primitive"
    ));

    // `{ armour: number }` fits no parameter member, so the first overload
    // is rejected.
    let from_mixed_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "fromMixed")
        .expect("fromMixed binding type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, from_mixed_ty),
        "unknown"
    ));
}

#[test]
fn test_infer_module_types_selects_call_overloads_by_permissive_callback_return_types_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            declare function run(callback: () => unknown): "unknown";
            declare function run(callback: unknown): "fallback";

            declare function loose(callback: () => any): "any";
            declare function loose(callback: unknown): "fallback";

            declare function either(callback: () => string | Promise<string>): "either";
            declare function either(callback: unknown): "fallback";

            declare function strict(callback: () => string): "string";
            declare function strict(callback: unknown): "fallback";

            export const fromUnknown = run(async () => 1);
            export const fromAny = loose(async () => 1);
            export const fromEither = either(async () => "value");
            export const fromStrict = strict(async () => "value");
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let literal = |name: &str| {
        inferred_binding_ty_by_name(&db, index_module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        )
    };

    // `unknown`, `any`, and a union containing a Promise all accept an async
    // callback, so the first overload is selected.
    assert!(is_inferred_string_literal(
        &db,
        literal("fromUnknown"),
        "unknown"
    ));
    assert!(is_inferred_string_literal(&db, literal("fromAny"), "any"));
    assert!(is_inferred_string_literal(
        &db,
        literal("fromEither"),
        "either"
    ));

    // A concrete non-Promise return type still rejects the async callback.
    assert!(is_inferred_string_literal(
        &db,
        literal("fromStrict"),
        "fallback"
    ));
}

#[test]
fn test_infer_module_types_keeps_strict_callback_return_types_for_async_callbacks_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            declare function voided(callback: () => string | void): "voided";
            declare function voided(callback: () => Promise<string>): "promise";

            declare function constrained<T extends string>(callback: () => T): "constrained";
            declare function constrained(callback: () => Promise<string>): "promise";

            declare function unconstrained<T>(callback: () => T): "unconstrained";
            declare function unconstrained(callback: () => Promise<string>): "promise";

            declare function predicate(callback: (value: unknown) => value is string): "predicate";
            declare function predicate(callback: unknown): "fallback";

            export const fromVoided = voided(async () => "");
            export const fromVoidedSync = voided(() => "");
            export const fromConstrained = constrained(async () => "");
            export const fromUnconstrained = unconstrained(async () => "");
            export const fromPredicate = predicate(async (value: unknown) => true);
            export const fromPredicateSync = predicate((value: unknown): value is string => true);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let literal = |name: &str| {
        inferred_binding_ty_by_name(&db, index_module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        )
    };

    // Only a complete `void` return type discards the result; `void` inside a
    // union is an ordinary member that a Promise does not satisfy.
    assert!(is_inferred_string_literal(
        &db,
        literal("fromVoided"),
        "promise"
    ));
    assert!(is_inferred_string_literal(
        &db,
        literal("fromVoidedSync"),
        "voided"
    ));

    // A generic return type accepts what its constraint accepts.
    assert!(is_inferred_string_literal(
        &db,
        literal("fromConstrained"),
        "promise"
    ));
    assert!(is_inferred_string_literal(
        &db,
        literal("fromUnconstrained"),
        "unconstrained"
    ));

    // A type predicate is a boolean result that an async callback cannot provide.
    assert!(is_inferred_string_literal(
        &db,
        literal("fromPredicate"),
        "fallback"
    ));
    assert!(is_inferred_string_literal(
        &db,
        literal("fromPredicateSync"),
        "predicate"
    ));
}

#[test]
fn test_infer_module_types_selects_call_overloads_for_primitives_within_empty_object_parameters_on_build()
 {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            declare function accept(value: {}): "object";
            declare function accept(value: unknown): "fallback";

            declare function named(value: { name: string }): "named";
            declare function named(value: unknown): "fallback";

            export const fromNumber = accept(1);
            export const fromString = accept("");
            export const fromNull = accept(null);
            export const fromUndefined = accept(undefined);
            export const fromNamed = named(1);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let literal = |name: &str| {
        inferred_binding_ty_by_name(&db, index_module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        )
    };

    // `{}` accepts every non-nullish value.
    assert!(is_inferred_string_literal(
        &db,
        literal("fromNumber"),
        "object"
    ));
    assert!(is_inferred_string_literal(
        &db,
        literal("fromString"),
        "object"
    ));
    assert!(is_inferred_string_literal(
        &db,
        literal("fromNull"),
        "fallback"
    ));
    assert!(is_inferred_string_literal(
        &db,
        literal("fromUndefined"),
        "fallback"
    ));

    // A parameter that requires a member still rejects a primitive.
    assert!(is_inferred_string_literal(
        &db,
        literal("fromNamed"),
        "fallback"
    ));
}

#[test]
fn test_infer_module_types_selects_call_overloads_for_promise_union_callback_returns_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Pending = Promise<string> | Promise<number>;
            type Wrap<T> = T | Promise<number>;

            declare function settle(callback: () => Promise<string> | Promise<number>): "promise";
            declare function settle(callback: unknown): "fallback";

            declare function aliased(callback: () => Pending): "promise";
            declare function aliased(callback: unknown): "fallback";

            declare function wrapped(callback: () => Wrap<Promise<string>>): "promise";
            declare function wrapped(callback: unknown): "fallback";

            declare function strict(callback: () => Promise<string>): "promise";
            declare function strict(callback: unknown): "fallback";

            declare const identical: () => Promise<string> | Promise<number>;
            declare const viaAlias: () => Pending;
            declare const viaGeneric: () => Wrap<Promise<string>>;
            declare const viaGenericString: () => Wrap<string>;
            declare const mixed: () => string | Promise<string>;
            declare const loose: () => any;

            export const fromIdentical = settle(identical);
            export const fromAliasArgument = settle(viaAlias);
            export const fromAliasParameter = aliased(identical);
            export const fromAliasBoth = aliased(viaAlias);
            export const fromAsync = settle(async () => "");
            export const fromGenericArgument = settle(viaGeneric);
            export const fromGenericParameter = wrapped(identical);
            export const fromGenericString = settle(viaGenericString);
            export const fromMixed = strict(mixed);
            export const fromAny = strict(loose);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let literal = |name: &str| {
        inferred_binding_ty_by_name(&db, index_module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        )
    };

    // A callback returning only Promises fits a parameter returning only
    // Promises, whether either side spells the union out or names an alias.
    for name in [
        "fromIdentical",
        "fromAliasArgument",
        "fromAliasParameter",
        "fromAliasBoth",
        "fromAsync",
        "fromGenericArgument",
        "fromGenericParameter",
    ] {
        let ty = literal(name);
        assert!(
            is_inferred_string_literal(&db, ty, "promise"),
            "{name} must be `\"promise\"`, got {}",
            format_inferred_type(&db, ty)
        );
    }

    // A callback that may return a non-Promise does not fit a parameter
    // returning only Promises.
    for name in ["fromMixed", "fromGenericString"] {
        let ty = literal(name);
        assert!(
            is_inferred_string_literal(&db, ty, "fallback"),
            "{name} must be `\"fallback\"`, got {}",
            format_inferred_type(&db, ty)
        );
    }

    // A callback returning `any` is not taken to return a Promise. TypeScript
    // also selects the fallback here.
    assert!(is_inferred_string_literal(
        &db,
        literal("fromAny"),
        "fallback"
    ));
}
