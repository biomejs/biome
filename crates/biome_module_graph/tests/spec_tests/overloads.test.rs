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

    // The callback's return type is widened, because `T` gives it no literal
    // type, so `T` is `number`, as in TypeScript 7.0.2.
    for name in ["aliased", "inline"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .unwrap_or_else(|| panic!("{name} binding type must be inferred"));
        let ty = inferred.resolve_type(&db, ty);
        assert!(
            is_inferred_number(&db, ty),
            "{name} must be number, got {}",
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
fn test_infer_module_types_selects_method_overloads_of_interfaces_and_object_types_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Server {
                listen(port: number, callback: () => void): number;
                listen(port?: number): string;
            }

            type Client = {
                connect(url: string, callback: () => void): number;
                connect(url: string): string;
            };

            interface Store<T> {
                read(key: string, callback: (value: T) => void): number;
                read(key: string): T;
            }

            interface Plugin {
                start?(callback: () => void): number;
                start?(): string;
            }

            declare const server: Server;
            declare const client: Client;
            declare const store: Store<string>;
            declare const plugin: Plugin;

            export const listening = server.listen(0);
            export const listeningWithCallback = server.listen(0, () => {});
            export const connected = client.connect("url");
            export const read = store.read("key");
            export const started = plugin.start?.();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let binding_ty = |name: &str| {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .unwrap_or_else(|| panic!("{name} binding type must be inferred"));
        inferred.resolve_type(&db, ty)
    };

    // The first overload requires a callback, so a call without one selects
    // the second overload.
    assert!(is_inferred_string(&db, binding_ty("listening")));
    assert!(is_inferred_number(&db, binding_ty("listeningWithCallback")));
    assert!(is_inferred_string(&db, binding_ty("connected")));
    assert!(is_inferred_string(&db, binding_ty("read")));

    // An optional method is optional as a whole, so its overloads stay
    // selectable and the optional call adds `undefined` to the result.
    let InferredTypeData::Union(started) = binding_ty("started") else {
        panic!("started binding type must be inferred as a union");
    };
    assert!(
        started
            .types(&db)
            .iter()
            .any(|ty| is_inferred_string(&db, *ty))
    );
    assert!(started.types(&db).contains(&InferredTypeData::Undefined));

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_method_overloads_of_interfaces_and_object_types_on_build",
        &db,
        &fs,
    );
}

// TypeScript 7.0.2 (`tsc --strict`) tries the overloads of a later declaration
// of a merged interface or class first, and signatures with a parameter of a
// literal type before all others. It gives every read below the expected
// type, unless a comment gives TypeScript's type.
#[test]
fn merged_declarations_try_the_overloads_of_later_declarations_first() {
    let source = r#"
interface O1 { f(x: unknown): 1 } interface O1 { f(x: unknown): 2 }
declare const o1: O1;
/*laterFirst*/o1.f(0);
interface O2 { f(x: unknown): 1; f(x: unknown): 11 } interface O2 { f(x: unknown): 2; f(x: unknown): 22 } interface O2 { f(x: unknown): 3 }
declare const o2: O2;
/*lastFirst*/o2.f(0);
interface O3 { f(x: "a"): 1 } interface O3 { f(x: string): 2 }
declare const o3: O3;
/*specialized*/o3.f("a");
class K1 { m(x: unknown): 1 { return 1; } } interface K1 { m(x: unknown): 2 }
declare const k1: K1;
/*classFirst*/k1.m(0);
interface K2 { m(x: unknown): 2 } class K2 { m(x: unknown): 1 { return 1; } }
declare const k2: K2;
/*interfaceFirst*/k2.m(0);
interface M1 { m(): string } interface M1 { m(x: number): number }
declare const m1: M1;
/*byArity*/m1.m(1);
interface Q1 { f?(x: unknown): 1 } interface Q1 { f?(x: unknown): 2 }
declare const q1: Q1;
/*optional*/q1.f?.(0);
interface S1 { f(x: unknown): 1 } interface S1 { f: (x: unknown) => 2 }
declare const s1: S1;
/*property*/s1.f(0);
interface O4 { f(x: "a" | "b"): 1 } interface O4 { f(x: string): 2 }
declare const o4: O4;
/*literalUnion*/o4.f("a");
interface O5 { f(x: 1): 1 } interface O5 { f(x: number): 2 }
declare const o5: O5;
/*numberLiteral*/o5.f(1);
interface O6 { f(x: string): 2; f(x: "a"): 1 }
declare const o6: O6;
/*oneDeclaration*/o6.f("a");
interface O8 { f(x: string): 3 } interface O8 { f(x: "a"): 1; f(x: string): 2 }
declare const o8: O8;
/*specializedOfLater*/o8.f("b");
"#;
    assert_file_reads(
        &[("/src/index.ts", source)],
        "/src/index.ts",
        &[
            ("laterFirst", "o1.f(0)", &["number: 2"]),
            ("lastFirst", "o2.f(0)", &["number: 3"]),
            ("specialized", "o3.f(\"a\")", &["number: 1"]),
            ("classFirst", "k1.m(0)", &["number: 2"]),
            ("interfaceFirst", "k2.m(0)", &["number: 1"]),
            ("byArity", "m1.m(1)", &["number"]),
            ("optional", "q1.f?.(0)", &["number: 2", "undefined"]),
            // TypeScript reports TS2300 and keeps the first declaration.
            ("property", "s1.f(0)", &["number: 1"]),
            ("literalUnion", "o4.f(\"a\")", &["number: 2"]),
            ("numberLiteral", "o5.f(1)", &["number: 1"]),
            // TypeScript: `1`. Biome keeps the order of the overloads of one
            // declaration.
            ("oneDeclaration", "o6.f(\"a\")", &["number: 2"]),
            ("specializedOfLater", "o8.f(\"b\")", &["number: 2"]),
        ],
    );
}
