use super::*;
use biome_module_graph::type_inference::{
    NormalizedBindingTypeRequest, TypeInferenceCaller, execute_type_inference_request,
};

fn projected_binding<'db>(
    db: &'db TestModuleDb,
    module: ModuleInfo,
    name: &str,
) -> InferredTypeData<'db> {
    let range = binding_range_by_name(db, module, name);
    execute_type_inference_request(
        db,
        TypeInferenceCaller::new("test", "typeProjections"),
        NormalizedBindingTypeRequest::new(module, range, range),
    )
    .expect("binding must be inferred")
}

#[test]
fn test_normalize_type_projections() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        const values = ["A", "B", "C"] as const;
        type Values = typeof values;
        type Index = number;
        declare const element: Values[Index];
        declare const readonlyElement: (readonly ["A", "B", "C"])[number];
        declare const optionalElement: ["A", "B"?][number];
        declare const arrayElement: ("A" | "B")[][number];
        declare const emptyElement: [][number];
        const mutable = ["A", "B", "C"];
        declare const mutableElement: (typeof mutable)[number];
        declare const unknownElement: ["A", unknown][number];
        declare const anyElement: ["A", any][number];
        declare const restElement: ["A", ...string[]][number];
        declare const unsupportedIndex: Values[string];
        const object = { A: 1, "B": 2, C: 3 } as const;
        type ObjectType = typeof object;
        declare const key: keyof ObjectType;
        declare const plainKey: keyof { A: number; B?: string };
        declare const emptyKey: keyof {};
        declare const numericKey: keyof { 1: number; A: string };
        declare const quotedNumericKey: keyof { "1": number; A: string };
        declare const indexSignatureKey: keyof { [key: string]: number; A: number };
        declare const extra: Record<string, number>;
        const incomplete = { A: 1, ...extra };
        declare const incompleteKey: keyof typeof incomplete;
        const computed = { [Symbol.iterator]: 1, A: 2 };
        declare const computedKey: keyof typeof computed;
        declare const computedName: "C";
        declare const computedSignatureKey: keyof { A: number; [computedName]: number };
        declare const setterKey: keyof { A: number; set C(value: number) };
        const escaped = { "\u0041": 1, B: 2, C: 3 } as const;
        declare const escapedKey: keyof typeof escaped;
        const escapedValues = ["\u0041", "B", "C"] as const;
        declare const escapedElement: (typeof escapedValues)[number];
        declare const escapedTupleElement: ["\u0041", "B", "C"][number];
        declare const controlElement: ["A", "\n"][number];
        declare const quotedElement: ['a"b', "C"][number];
        declare const surrogateElement: ["\uD800", "\uD801"][number];
        declare const brandedEscapedElement: ["A", "\u0042" & { brand: true }][number];
        declare const quotedKey: keyof { 'a"b': number; C: number };
        type Incomplete = { A: number; [computedName]: number };
        declare const readonlyKey: keyof Readonly<Incomplete>;
        declare const partialKey: keyof Partial<Incomplete>;
        declare const requiredKey: keyof Required<Incomplete>;
        declare const pickedKey: keyof Pick<Incomplete, "A" | "C">;
        declare const omittedKey: keyof Omit<Incomplete, "A">;
        interface Parent { C: number }
        interface Child extends Parent { A: number }
        declare const inheritedKey: keyof Readonly<Child>;
        declare const intersectedKey: keyof (Child & { B: number });
        interface Computed { A: number; [computedName]: number }
        declare const computedInterfaceKey: keyof Partial<Computed>;
        declare const completeReadonlyKey: keyof Readonly<ObjectType>;
        declare const completePartialKey: keyof Partial<ObjectType>;
        declare const completeRequiredKey: keyof Required<ObjectType>;
        type Indexed = { [key: string]: number; A: number };
        declare const pickedIndexedKey: keyof Pick<Indexed, "A" | "C">;
        declare const pickedMixedKey: keyof Pick<{ A: number; 1: number }, "A" | 1>;
        declare const escapedOmitKey: keyof Omit<{ A: number; B: number }, "\u0041">;
        declare const negativeElement: [-1, 2][number];
        const negatives = [-1, 2] as const;
        declare const negativeConstElement: (typeof negatives)[number];
        declare const negativeBigintElement: [-1n, 2n][number];
        type Recursive = Recursive[number];
        declare const recursiveElement: Recursive;
        interface Shape { A: number; B?: string; method(): void; (): void }
        declare const interfaceKey: keyof Shape;
        declare const inheritedInterfaceKey: keyof Child;
        interface Box<T> { value: T }
        interface NumberBox extends Box<number> { size: number }
        declare const genericInterfaceKey: keyof Box<number>;
        declare const inheritedGenericInterfaceKey: keyof NumberBox;
        interface Shadowing extends Parent { C: string }
        declare const shadowingInterfaceKey: keyof Shadowing;
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    db.clear_salsa_events();
    for (name, expected) in [
        ("element", "string: A | string: B | string: C"),
        ("readonlyElement", "string: A | string: B | string: C"),
        ("optionalElement", "string: A | string: B | undefined"),
        ("arrayElement", "string: A | string: B"),
        ("emptyElement", "never"),
        ("mutableElement", "string"),
        ("unknownElement", "unknown"),
        ("anyElement", "any"),
        ("restElement", "unknown"),
        ("unsupportedIndex", "unknown"),
        ("key", "string: A | string: B | string: C"),
        ("plainKey", "string: A | string: B"),
        ("emptyKey", "never"),
        ("numericKey", "unknown"),
        ("quotedNumericKey", "unknown"),
        ("indexSignatureKey", "unknown"),
        ("incompleteKey", "unknown"),
        ("computedKey", "unknown"),
        ("computedSignatureKey", "unknown"),
        ("setterKey", "unknown"),
        ("escapedKey", "unknown"),
        ("escapedElement", "unknown"),
        ("escapedTupleElement", "unknown"),
        ("controlElement", "unknown"),
        ("quotedElement", "unknown"),
        ("surrogateElement", "unknown"),
        ("brandedEscapedElement", "unknown"),
        ("quotedKey", "unknown"),
        ("readonlyKey", "unknown"),
        ("partialKey", "unknown"),
        ("requiredKey", "unknown"),
        ("pickedKey", "unknown"),
        ("omittedKey", "unknown"),
        ("inheritedKey", "unknown"),
        ("intersectedKey", "unknown"),
        ("computedInterfaceKey", "unknown"),
        ("completeReadonlyKey", "string: A | string: B | string: C"),
        ("completePartialKey", "string: A | string: B | string: C"),
        ("completeRequiredKey", "string: A | string: B | string: C"),
        ("pickedIndexedKey", "unknown"),
        ("pickedMixedKey", "unknown"),
        ("escapedOmitKey", "unknown"),
        ("negativeElement", "number: -1 | number: 2"),
        ("negativeConstElement", "number: -1 | number: 2"),
        ("negativeBigintElement", "unknown"),
        ("recursiveElement", "unknown"),
        ("interfaceKey", "string: A | string: B | string: method"),
        ("inheritedInterfaceKey", "string: A | string: C"),
        ("genericInterfaceKey", "string: value"),
        (
            "inheritedGenericInterfaceKey",
            "string: size | string: value",
        ),
        ("shadowingInterfaceKey", "string: C"),
    ] {
        let ty = projected_binding(&db, module, name);
        assert_eq!(format_inferred_type(&db, ty), expected, "{name}");
    }
    assert_function_query_was_not_run(&db, infer_module_types, module, &db.take_salsa_events());
}

#[test]
fn test_type_projections_track_imports_without_whole_module_inference() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/values.ts".into(),
        r#"
        export const values = ["A", "B", "C"] as const;
        export const object = { A: 1, B: 2, C: 3 } as const;
    "#,
    );
    fs.insert("/src/unrelated.ts".into(), "export const unrelated = 1;");
    fs.insert(
        "/src/index.ts".into(),
        r#"
        import { values, object } from "./values.ts";
        import { unrelated } from "./unrelated.ts";
        type Element = (typeof values)[number];
        type Key = keyof typeof object;
        declare const element: Element;
        declare const key: Key;
    "#,
    );
    let mut db = build_js_test_module_db(
        &fs,
        &["/src/values.ts", "/src/unrelated.ts", "/src/index.ts"],
        true,
    );
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let values = db.module_for_path(Utf8Path::new("/src/values.ts")).unwrap();
    let unrelated = db
        .module_for_path(Utf8Path::new("/src/unrelated.ts"))
        .unwrap();

    for phase in 0..4 {
        if phase == 2 {
            fs.insert(
                "/src/unrelated.ts".into(),
                "export const unrelated = false;",
            );
            let kind = resolve_js_module_kind_for_test(&fs, "/src/unrelated.ts", true);
            salsa::Setter::to(unrelated.set_kind(&mut db), kind);
        } else if phase == 3 {
            fs.insert(
                "/src/values.ts".into(),
                r#"
                export const values = ["A", "B", "D"] as const;
                export const object = { A: 1, B: 2, D: 3 } as const;
            "#,
            );
            let kind = resolve_js_module_kind_for_test(&fs, "/src/values.ts", true);
            salsa::Setter::to(values.set_kind(&mut db), kind);
        }
        db.clear_salsa_events();
        for name in ["element", "key"] {
            let ty = projected_binding(&db, module, name);
            let expected = if phase == 3 {
                "string: A | string: B | string: D"
            } else {
                "string: A | string: B | string: C"
            };
            assert_eq!(
                format_inferred_type(&db, ty),
                expected,
                "phase {phase}, {name}"
            );
        }
        let events = db.take_salsa_events();
        for module in [module, values, unrelated] {
            assert_function_query_was_not_run(&db, infer_module_types, module, &events);
        }
        let normalizations =
            function_query_will_execute_count_by_name(&db, "normalize_type", &events);
        if matches!(phase, 1 | 2) {
            assert_eq!(normalizations, 0);
        } else {
            assert!(normalizations > 0);
        }
    }
}

#[test]
fn test_type_projections_exceeding_normalization_budget_are_unknown() {
    let fs = MemoryFileSystem::default();
    let elements = "\"A\",".repeat(1100);
    let members = (0..1100)
        .map(|index| format!("key{index}: number;"))
        .collect::<String>();
    fs.insert(
        "/src/index.ts".into(),
        format!(
            "declare const element: [{elements}][number]; declare const key: keyof {{ {members} }};"
        )
        .as_str(),
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    for name in ["element", "key"] {
        assert_eq!(
            projected_binding(&db, module, name),
            InferredTypeData::Unknown
        );
    }
}

#[test]
fn test_normalize_type_preserves_recursive_array_local_edge() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Tree = number | Tree[];

            export function readTree(value: Tree): Tree {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let tree_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readTree")
        .expect("readTree return type must be inferred");
    let tree_index = local_type_id_of_instance(&db, tree_ty)
        .expect("readTree must return an instance of the local Tree type");
    let normalized_ty = normalize_type(&db, index_module, tree_ty);

    let InferredTypeData::Union(union) = normalized_ty else {
        panic!("recursive Tree type must normalize to a union, got {normalized_ty:?}");
    };
    let normalized_tree = format_inferred_type(&db, normalized_ty);
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_number(&db, *ty)),
        "recursive Tree union must keep its number branch"
    );
    assert!(
        union.types(&db).iter().any(|ty| {
            matches!(
                ty,
                InferredTypeData::InstanceOf(instance)
                    if instance.ty(&db).is_array_class(&db)
                    && instance.type_parameters(&db).iter().any(|parameter| {
                        matches!(
                            parameter,
                            InferredTypeData::Local(local)
                                if local.type_id(&db).index() == tree_index
                        )
                        || local_type_id_of_instance(&db, *parameter) == Some(tree_index)
                    })
            )
        }),
        "recursive Tree union must keep the recursive Array local edge: {normalized_tree}"
    );

    assert_inferred_type_snapshot(
        "test_normalize_type_preserves_recursive_array_local_edge",
        &db,
        &fs,
    );
}

#[test]
fn test_const_generic_tuple_projections() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        declare function keep<const T>(value: T): T;
        declare function collect<const T extends readonly unknown[]>(...values: T): T;
        const direct = keep(["a", "b"]);
        const nested = keep({ values: ["a", "b"] }).values;
        const rest = collect("a", "b");
        const empty = collect();
        declare const directElement: (typeof direct)[number];
        declare const nestedElement: (typeof nested)[number];
        declare const restElement: (typeof rest)[number];
        declare const emptyElement: (typeof empty)[number];
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    for (name, expected) in [
        ("directElement", "string: a | string: b"),
        ("nestedElement", "string: a | string: b"),
        ("restElement", "string: a | string: b"),
        ("emptyElement", "never"),
    ] {
        assert_eq!(
            format_inferred_type(&db, projected_binding(&db, module, name)),
            expected,
            "{name}"
        );
    }
}

/// Formats each member of an object as `name: type`, marking optional
/// members with `?`.
fn object_member_summaries<'db>(db: &'db TestModuleDb, ty: InferredTypeData<'db>) -> Vec<String> {
    let InferredTypeData::Object(object) = ty else {
        panic!("expected an object, got {}", format_inferred_type(db, ty));
    };
    object
        .members(db)
        .iter()
        .map(|member| {
            let name = member.kind.name().expect("mapped members must be named");
            let optional = if member.kind.is_optional() { "?" } else { "" };
            format!(
                "{}{optional}: {}",
                name.text(),
                format_inferred_type(db, member.ty)
            )
        })
        .collect()
}

#[test]
fn test_normalize_mapped_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        type Source = { A: number; B?: string };
        declare const literalKeys: { [K in "A" | "B"]: number };
        declare const singleKey: { [K in "A"]: boolean };
        declare const noKeys: { [K in never]: number };
        declare const optionalKeys: { [K in "A" | "B"]?: number };
        declare const homomorphic: { [K in keyof Source]: Source[K] };
        declare const required: { [K in keyof Source]-?: Source[K] };
        declare const optional: { [K in keyof Source]+?: Source[K] };
        declare const readonlyKeys: { readonly [K in keyof Source]: Source[K] };
        declare const wrapped: { [K in keyof Source]: Array<Source[K]> };
        declare const keyValues: { [K in keyof Source]: K };
        declare const unannotated: { [K in "A"] };
        declare const numericKeys: { [K in 1 | 2]: number };
        declare const stringKeys: { [K in string]: number };
        declare const remapped: { [K in keyof Source as Uppercase<K>]: Source[K] };
        declare const computedName: "C";
        type Incomplete = { A: number; [computedName]: number };
        declare const incompleteSource: { [K in keyof Incomplete]: number };
        interface Iface { A: number }
        declare const interfaceSource: { [K in keyof Iface]: number };
        declare const otherObject: { [K in keyof Source]: Iface[K] };
        declare const constant: { [K in keyof Source]: boolean };
        declare const constantRequired: { [K in keyof Source]-?: boolean };
        declare const parenthesized: { [K in (keyof Source)]: Source[K] };
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    db.clear_salsa_events();
    for (name, expected) in [
        ("literalKeys", vec!["A: number", "B: number"]),
        ("singleKey", vec!["A: boolean"]),
        ("noKeys", vec![]),
        (
            "optionalKeys",
            vec!["A?: number | undefined", "B?: number | undefined"],
        ),
        ("homomorphic", vec!["A: number", "B?: string | undefined"]),
        ("required", vec!["A: number", "B: string"]),
        (
            "optional",
            vec!["A?: number | undefined", "B?: string | undefined"],
        ),
        ("readonlyKeys", vec!["A: number", "B?: string | undefined"]),
        (
            "wrapped",
            vec![
                "A: instanceof Array<number>",
                "B?: instanceof Array<string | undefined> | undefined",
            ],
        ),
        (
            "keyValues",
            vec!["A: string: A", "B?: string: B | undefined"],
        ),
        ("unannotated", vec!["A: unknown"]),
        ("constant", vec!["A: boolean", "B?: boolean | undefined"]),
        ("constantRequired", vec!["A: boolean", "B: boolean"]),
        ("parenthesized", vec!["A: number", "B?: string | undefined"]),
    ] {
        let ty = projected_binding(&db, module, name);
        assert_eq!(object_member_summaries(&db, ty), expected, "{name}");
    }
    for name in [
        "numericKeys",
        "stringKeys",
        "incompleteSource",
        "interfaceSource",
    ] {
        let ty = projected_binding(&db, module, name);
        assert!(
            matches!(ty, InferredTypeData::MappedType(_)),
            "{name} must stay unevaluated, got {}",
            format_inferred_type(&db, ty)
        );
    }
    let remapped = projected_binding(&db, module, "remapped");
    assert_eq!(remapped, InferredTypeData::Unknown);

    // `Iface[K]` is not the mapped type's own source, so the property types
    // stay symbolic while the keys are still projected.
    let other_object = projected_binding(&db, module, "otherObject");
    let InferredTypeData::Object(object) = other_object else {
        panic!("expected an object");
    };
    let members = object.members(&db);
    assert_eq!(members.len(), 2);
    assert!(members[0].kind.has_name("A") && !members[0].kind.is_optional());
    assert!(members[1].kind.has_name("B") && members[1].kind.is_optional());
    assert!(
        matches!(members[0].ty, InferredTypeData::IndexedAccess(_)),
        "{}",
        format_inferred_type(&db, other_object)
    );
    assert!(
        matches!(members[1].ty, InferredTypeData::Union(_))
            && contains_inferred_undefined(&db, members[1].ty),
        "{}",
        format_inferred_type(&db, other_object)
    );
    assert_function_query_was_not_run(&db, infer_module_types, module, &db.take_salsa_events());
}

#[test]
fn test_mapped_type_alias_members_are_found_after_instantiation() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        type Source = { A: number; B?: string };
        type Mapped<T> = { [K in keyof T]: T[K] };
        type Boxed<T> = { [K in keyof T]: { value: T[K] } };
        type Nullable<T> = { [K in keyof T]: T[K] | null };
        declare const mapped: Mapped<Source>;
        declare const boxed: Boxed<Source>;
        declare const nullable: Nullable<Source>;
        declare const generic: Mapped<Iface>;
        interface Iface { A: number }
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    db.clear_salsa_events();

    let mapped = projected_binding(&db, module, "mapped");
    let member = find_value_member_type(&db, mapped, "B").expect("B must be found");
    // `B` stays optional through the instantiation, so reading it may yield
    // `undefined`.
    assert!(
        contains_inferred_string(&db, member),
        "{}",
        format_inferred_type(&db, member)
    );
    assert!(
        contains_inferred_undefined(&db, member),
        "{}",
        format_inferred_type(&db, member)
    );
    let member = find_value_member_type(&db, mapped, "A").expect("A must be found");
    assert!(
        is_inferred_number(&db, member),
        "{}",
        format_inferred_type(&db, member)
    );
    assert_eq!(find_value_member_type(&db, mapped, "C"), None);

    let boxed = projected_binding(&db, module, "boxed");
    let member = find_value_member_type(&db, boxed, "A").expect("A must be found");
    let value = find_value_member_type(&db, member, "value").expect("value must be found");
    assert!(
        is_inferred_number(&db, value),
        "{}",
        format_inferred_type(&db, value)
    );

    let nullable = projected_binding(&db, module, "nullable");
    let member = find_value_member_type(&db, nullable, "A").expect("A must be found");
    assert!(
        contains_inferred_number(&db, member),
        "{}",
        format_inferred_type(&db, member)
    );
    assert!(
        contains_inferred_null(&db, member),
        "{}",
        format_inferred_type(&db, member)
    );

    // Interface members may be merged elsewhere, so the mapped type stays
    // unevaluated and reports no members instead of an incomplete list.
    let generic = projected_binding(&db, module, "generic");
    assert_eq!(find_value_member_type(&db, generic, "A"), None);

    assert_function_query_was_not_run(&db, infer_module_types, module, &db.take_salsa_events());
}

#[test]
fn test_mapped_types_track_imports_without_whole_module_inference() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
        export type Mapped<T> = { [K in keyof T]: T[K] };
        export type Source = { A: number; B?: string };
    "#,
    );
    fs.insert("/src/unrelated.ts".into(), "export const unrelated = 1;");
    fs.insert(
        "/src/index.ts".into(),
        r#"
        import type { Mapped, Source } from "./types.ts";
        import { unrelated } from "./unrelated.ts";
        declare const local: { [K in keyof Source]: Source[K] };
        declare const instantiated: Mapped<Source>;
    "#,
    );
    let mut db = build_js_test_module_db(
        &fs,
        &["/src/types.ts", "/src/unrelated.ts", "/src/index.ts"],
        true,
    );
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let types = db.module_for_path(Utf8Path::new("/src/types.ts")).unwrap();
    let unrelated = db
        .module_for_path(Utf8Path::new("/src/unrelated.ts"))
        .unwrap();

    for phase in 0..4 {
        if phase == 2 {
            fs.insert(
                "/src/unrelated.ts".into(),
                "export const unrelated = false;",
            );
            let kind = resolve_js_module_kind_for_test(&fs, "/src/unrelated.ts", true);
            salsa::Setter::to(unrelated.set_kind(&mut db), kind);
        } else if phase == 3 {
            fs.insert(
                "/src/types.ts".into(),
                r#"
                export type Mapped<T> = { [K in keyof T]: T[K] };
                export type Source = { A: number; B?: boolean };
            "#,
            );
            let kind = resolve_js_module_kind_for_test(&fs, "/src/types.ts", true);
            salsa::Setter::to(types.set_kind(&mut db), kind);
        }
        db.clear_salsa_events();

        let expected_b = if phase == 3 {
            "B?: boolean | undefined"
        } else {
            "B?: string | undefined"
        };
        let local = projected_binding(&db, module, "local");
        assert_eq!(
            object_member_summaries(&db, local),
            ["A: number", expected_b],
            "phase {phase}"
        );

        let instantiated = projected_binding(&db, module, "instantiated");
        let member = find_value_member_type(&db, instantiated, "B").expect("B must be found");
        assert!(
            if phase == 3 {
                contains_inferred_boolean(&db, member)
            } else {
                contains_inferred_string(&db, member)
            },
            "phase {phase}: {}",
            format_inferred_type(&db, member)
        );

        let events = db.take_salsa_events();
        for module in [module, types, unrelated] {
            assert_function_query_was_not_run(&db, infer_module_types, module, &events);
        }
        let normalizations =
            function_query_will_execute_count_by_name(&db, "normalize_type", &events);
        if matches!(phase, 1 | 2) {
            assert_eq!(normalizations, 0, "phase {phase}");
        } else {
            assert!(normalizations > 0, "phase {phase}");
        }
    }
}

#[test]
fn test_normalize_mapped_type_shapes() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        const object = { A: 1, B: "b" } as const;
        declare const fromTypeof: { [K in keyof typeof object]: (typeof object)[K] };
        type Keys = "A" | "B";
        declare const aliasedKeys: { [K in Keys]: K };
        declare const duplicateKeys: { [K in "A" | "A"]: number };
        declare const shadowed: { [K in "A"]: <K>(value: K) => K };
        type Source = { A: number; B?: string };
        declare const nested: { [K in keyof Source]: { [P in keyof Source]: P } };
        declare const nestedDependent: { [K in keyof Source]: { [P in K]: Source[K] } };
        declare const getters: { [K in keyof { get A(): number }]: boolean };
        declare const intersected: { [K in keyof (Source & { C: boolean })]: number };
        declare const spread: { [K in keyof Source]: Source[K] } & { C: boolean };
        declare const picker: {
            [K in keyof Source]: <R extends keyof Source>() => { [K in R]: Source[K] };
        };
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();

    for (name, expected) in [
        ("fromTypeof", vec!["A: number: 1", "B: string: b"]),
        ("aliasedKeys", vec!["A: string: A", "B: string: B"]),
        ("duplicateKeys", vec!["A: number"]),
        ("getters", vec!["A: boolean"]),
    ] {
        let ty = projected_binding(&db, module, name);
        assert_eq!(object_member_summaries(&db, ty), expected, "{name}");
    }

    // A nested declaration of `K` is not replaced by the key.
    let shadowed = projected_binding(&db, module, "shadowed");
    let (_, member) = object_member_ty_by_name(&db, shadowed, "A").expect("A must be a member");
    let InferredTypeData::Function(function) = member else {
        panic!(
            "expected a function, got {}",
            format_inferred_type(&db, member)
        );
    };
    assert_eq!(function.type_parameters(&db).len(), 1);
    assert!(
        function
            .parameters(&db)
            .iter()
            .all(|parameter| parameter.ty().is_generic_reference(&db)),
        "{}",
        format_inferred_type(&db, member)
    );

    // Inner mapped types with resolved keys are evaluated as well. The outer
    // `B` is optional, so its object is also joined with `undefined`.
    let nested = projected_binding(&db, module, "nested");
    let (_, member) = object_member_ty_by_name(&db, nested, "A").expect("A must be a member");
    assert_eq!(
        object_member_summaries(&db, member),
        ["A: string: A", "B?: string: B | undefined"]
    );
    let (_, member) = object_member_ty_by_name(&db, nested, "B").expect("B must be a member");
    let InferredTypeData::Union(union) = member else {
        panic!(
            "expected a union, got {}",
            format_inferred_type(&db, member)
        );
    };
    assert!(contains_inferred_undefined(&db, member));
    let inner = union
        .types(&db)
        .iter()
        .copied()
        .find(|ty| matches!(ty, InferredTypeData::Object(_)))
        .expect("union must contain the inner object");
    assert_eq!(
        object_member_summaries(&db, inner),
        ["A: string: A", "B?: string: B | undefined"]
    );

    // Inner keys that depend on the outer key are only known after the outer
    // key is substituted, so the inner mapped type stays unevaluated.
    let nested_dependent = projected_binding(&db, module, "nestedDependent");
    let (_, member) =
        object_member_ty_by_name(&db, nested_dependent, "A").expect("A must be a member");
    assert!(
        matches!(member, InferredTypeData::MappedType(_)),
        "{}",
        format_inferred_type(&db, member)
    );

    // An intersection of objects normalizes into one object, so its keys and
    // optionality are projected.
    let intersected = projected_binding(&db, module, "intersected");
    assert_eq!(
        object_member_summaries(&db, intersected),
        ["A: number", "B?: number | undefined", "C: number"]
    );

    // Replacing `Source[K]` stops at the nested mapped type that declares its
    // own `K`, so its property type is not resolved with the outer key.
    let picker = projected_binding(&db, module, "picker");
    let (_, member) = object_member_ty_by_name(&db, picker, "A").expect("A must be a member");
    let InferredTypeData::Function(function) = member else {
        panic!(
            "expected a function, got {}",
            format_inferred_type(&db, member)
        );
    };
    let InferredReturnType::Type(InferredTypeData::MappedType(inner)) = function.return_type(&db)
    else {
        panic!(
            "expected a mapped return type, got {}",
            format_inferred_type(&db, member)
        );
    };
    assert!(
        matches!(inner.ty(&db), InferredTypeData::IndexedAccess(_)),
        "{}",
        format_inferred_type(&db, member)
    );

    // The evaluated object takes part in intersection merging like any other.
    let spread = projected_binding(&db, module, "spread");
    assert_eq!(
        object_member_summaries(&db, spread),
        ["A: number", "B?: string | undefined", "C: boolean"]
    );
    assert_eq!(
        find_value_member_type(&db, spread, "C"),
        Some(InferredTypeData::Boolean)
    );
}

#[test]
fn test_mapped_type_instances_compose_with_utility_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        type Source = { A: number; B?: string };
        type Mapped<T> = { [K in keyof T]: T[K] };
        type Recursive<T> = { [K in keyof T]: Recursive<T[K]> };
        declare const mappedPartial: Mapped<Partial<Source>>;
        declare const mappedPick: Mapped<Pick<Source, "A">>;
        declare const partialMapped: Partial<Mapped<Source>>;
        declare const keys: keyof Mapped<Source>;
        declare const recursive: Recursive<Source>;
        declare const chained: Mapped<Mapped<Source>>;
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();

    let mapped_partial = projected_binding(&db, module, "mappedPartial");
    let member = find_value_member_type(&db, mapped_partial, "A").expect("A must be found");
    assert!(
        contains_inferred_number(&db, member),
        "{}",
        format_inferred_type(&db, member)
    );
    assert!(
        contains_inferred_undefined(&db, member),
        "{}",
        format_inferred_type(&db, member)
    );

    let mapped_pick = projected_binding(&db, module, "mappedPick");
    let member = find_value_member_type(&db, mapped_pick, "A").expect("A must be found");
    assert!(
        is_inferred_number(&db, member),
        "{}",
        format_inferred_type(&db, member)
    );
    assert_eq!(find_value_member_type(&db, mapped_pick, "B"), None);

    // Utility types read members from an already resolved object, which an
    // instantiated mapped type is not; the result stays unknown rather than
    // claiming an empty member list.
    let partial_mapped = projected_binding(&db, module, "partialMapped");
    assert_eq!(partial_mapped, InferredTypeData::Unknown);
    let keys = projected_binding(&db, module, "keys");
    assert_eq!(keys, InferredTypeData::Unknown);

    // Recursion terminates: `A` maps to `Recursive<number>`, whose keys are
    // not those of an object, so it has no members.
    let recursive = projected_binding(&db, module, "recursive");
    let member = find_value_member_type(&db, recursive, "A").expect("A must be found");
    assert!(
        matches!(member, InferredTypeData::InstanceOf(_)),
        "{}",
        format_inferred_type(&db, member)
    );
    assert_eq!(find_value_member_type(&db, member, "toFixed"), None);

    // The outer mapped type sees the inner instance, not an object, so the
    // chain stays unevaluated instead of guessing members.
    let chained = projected_binding(&db, module, "chained");
    assert_eq!(find_value_member_type(&db, chained, "A"), None);
}

#[test]
fn test_mapped_types_exceeding_projection_budget_stay_unevaluated() {
    let fs = MemoryFileSystem::default();
    let keys = (0..1100)
        .map(|index| format!("\"key{index}\""))
        .collect::<Vec<_>>()
        .join(" | ");
    let members = (0..1100)
        .map(|index| format!("key{index}: number;"))
        .collect::<String>();
    fs.insert(
        "/src/index.ts".into(),
        format!(
            "declare const literal: {{ [K in {keys}]: number }}; declare const object: {{ [K in keyof {{ {members} }}]: number }};"
        )
        .as_str(),
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    for name in ["literal", "object"] {
        let ty = projected_binding(&db, module, name);
        assert!(
            !matches!(ty, InferredTypeData::Object(_)),
            "{name}: {}",
            format_inferred_type(&db, ty)
        );
    }
}

#[test]
fn test_normalize_conditional_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        type IsString<T> = T extends string ? "yes" : "no";
        type Unwrap<T> = T extends Promise<infer U> ? U : T;
        type MyExclude<T, U> = T extends U ? never : T;
        type MyReturnType<F> = F extends (...args: any) => infer R ? R : never;
        type MyParameters<F> = F extends (...args: infer P) => any ? P : never;
        type First<T> = T extends [infer H, ...unknown[]] ? H : never;
        type Tail<T> = T extends [unknown, ...infer R] ? R : never;
        type NonDistributive<T> = [T] extends [string] ? "yes" : "no";
        type Deep<T> = T extends Promise<infer U> ? Deep<U> : T;
        type HasThen<T> = T extends { then: unknown } ? true : false;
        type ElementOf<T> = T extends (infer E)[] ? E : never;
        type Literal = IsString<"x">;
        declare function callback(input: string, count?: number): boolean;
        declare function pending<T>(value: IsString<T>): void;
        declare const literal: IsString<"x">;
        declare const aliased: Literal;
        declare const unwrapped: Unwrap<Promise<number>>;
        declare const inline: string extends string ? 1 : 2;
        declare const distributed: Unwrap<Promise<number> | string>;
        declare const excluded: MyExclude<"a" | "b" | "c", "a">;
        declare const returned: MyReturnType<typeof callback>;
        declare const parameters: MyParameters<typeof callback>;
        declare const first: First<[1, 2, 3]>;
        declare const tail: Tail<[1, 2, 3]>;
        declare const nonDistributive: NonDistributive<"a" | "b">;
        declare const nonDistributiveMixed: NonDistributive<"a" | 1>;
        declare const deep: Deep<Promise<Promise<string>>>;
        declare const hasThen: HasThen<{ then: number }>;
        declare const hasNoThen: HasThen<{ other: number }>;
        declare const element: ElementOf<string[]>;
        declare const union: IsString<number | "x">;
        declare const never: IsString<never>;
        declare const any: IsString<any>;
        declare const boolean: IsString<boolean>;
        declare const arrayIsPromise: number[] extends Promise<any> ? 1 : 2;
        declare const arrayIsThenable: number[] extends { then: any } ? 1 : 2;
        declare const promiseIsThenable: Promise<string> extends { then: any } ? 1 : 2;
        declare const nested: IsString<"x"> | 5;
        declare const libExclude: Exclude<"a" | "b" | "c", "a" | "b">;
        declare const libExtract: Extract<"a" | 1 | true, string | number>;
        declare const libReturnType: ReturnType<typeof callback>;
        declare const libParameters: Parameters<typeof callback>;
        declare const libAwaited: Awaited<Promise<Promise<number>>>;
        declare const libAwaitedPlain: Awaited<string>;
        declare const libAwaitedUnion: Awaited<Promise<number> | string>;
        interface Tables { a: { id: string }; b: { id: string } }
        type HasTable<K> = K extends keyof Tables ? "yes" : "no";
        declare const keyOfInterface: HasTable<"b">;
        declare const notKeyOfInterface: HasTable<"c">;
        declare const keysOfInterface: HasTable<"a" | "c">;
        interface Named { value: keyof { 1: number } }
        interface Other { other: number }
        declare const sameDeclaration: Named extends Named ? "yes" : "no";
        declare const excludedDeclaration: Exclude<Named | Other, Named>;
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    db.clear_salsa_events();
    for (name, expected) in [
        ("literal", "string: yes"),
        ("aliased", "string: yes"),
        ("unwrapped", "number"),
        ("inline", "number: 1"),
        ("distributed", "number | string"),
        ("excluded", "string: b | string: c"),
        ("returned", "boolean"),
        ("parameters", "[input: string,\ncount: number | undefined?]"),
        ("first", "number: 1"),
        ("tail", "[number: 2,\nnumber: 3]"),
        ("nonDistributive", "string: yes"),
        ("nonDistributiveMixed", "string: no"),
        ("deep", "string"),
        ("hasThen", "bool: true"),
        ("hasNoThen", "bool: false"),
        ("element", "string"),
        ("union", "string: no | string: yes"),
        ("never", "never"),
        ("any", "string: yes | string: no"),
        ("boolean", "string: no"),
        ("arrayIsPromise", "number: 2"),
        ("arrayIsThenable", "number: 2"),
        ("promiseIsThenable", "number: 1"),
        ("nested", "string: yes | number: 5"),
        ("libExclude", "string: c"),
        ("libExtract", "string: a | number: 1"),
        ("libReturnType", "boolean"),
        (
            "libParameters",
            "[input: string,\ncount: number | undefined?]",
        ),
        ("libAwaited", "number"),
        ("libAwaitedPlain", "string"),
        ("libAwaitedUnion", "number | string"),
        ("keyOfInterface", "string: yes"),
        ("notKeyOfInterface", "string: no"),
        ("keysOfInterface", "string: yes | string: no"),
        ("sameDeclaration", "string: yes"),
        (
            "excludedDeclaration",
            "interface \"Other\" {\n  extends: []\n  type_args: []\n  members: [\"other\": number]\n}",
        ),
    ] {
        let ty = projected_binding(&db, module, name);
        assert_eq!(format_inferred_type(&db, ty), expected, "{name}");
    }

    // The check type is generic, so neither branch can be selected yet.
    let ty = projected_binding(&db, module, "value");
    assert!(
        format_inferred_type(&db, ty).contains("extends"),
        "{}",
        format_inferred_type(&db, ty)
    );
    assert_function_query_was_not_run(&db, infer_module_types, module, &db.take_salsa_events());
}

/// Asserts the normalized type of each binding in `source`, and that each
/// binding in `unevaluated` still contains a conditional type. The bindings
/// must be computed without inferring the types of the whole module.
///
/// Every mismatch is reported at once, so one run shows all failing cases.
fn assert_conditional_results(source: &str, expected: &[(&str, &str)], unevaluated: &[&str]) {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    db.clear_salsa_events();

    let format_binding = |name| format_inferred_type(&db, projected_binding(&db, module, name));
    let mut mismatches = Vec::new();
    for (name, expected) in expected {
        let actual = format_binding(name);
        if actual != *expected {
            mismatches.push(format!("{name}: expected `{expected}`, got `{actual}`"));
        }
    }
    for name in unevaluated {
        let actual = format_binding(name);
        if !actual.contains("extends") {
            mismatches.push(format!("{name}: expected unevaluated, got `{actual}`"));
        }
    }
    assert!(mismatches.is_empty(), "\n{}", mismatches.join("\n"));
    assert_function_query_was_not_run(&db, infer_module_types, module, &db.take_salsa_events());
}

#[test]
fn test_conditional_infer_candidates_follow_variance() {
    assert_conditional_results(
        r#"
        type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends (k: infer I) => void ? I : never;
        type Contra<T> = T extends { a: (x: infer U) => void; b: (x: infer U) => void } ? U : never;
        type Co<T> = T extends { a: infer U; b: infer U } ? U : never;
        type Mixed<T> = T extends { a: infer U; b: (x: infer U) => void } ? U : never;
        type Callback<T> = T extends (cb: (x: infer U) => void) => void ? U : never;
        type MethodParams<T> = T extends { a(x: infer U): void; b(x: infer U): void } ? U : never;
        declare const intersected: UnionToIntersection<{ a: 1 } | { b: 2 }> extends { a: 1 } ? "yes" : "no";
        declare const intersectedBoth: UnionToIntersection<{ a: 1 } | { b: 2 }> extends { b: 2 } ? "yes" : "no";
        declare const contraPrimitives: Contra<{ a: (x: string) => void; b: (x: number) => void }>;
        declare const contraSame: Contra<{ a: (x: string) => void; b: (x: string) => void }>;
        declare const co: Co<{ a: string; b: number }>;
        declare const mixed: Mixed<{ a: string; b: (x: number) => void }>;
        declare const callback: Callback<(cb: (x: string) => void) => void>;
        declare const method: MethodParams<{ a(x: string): void; b(x: number): void }>;
        declare const inferUnion: (string | undefined) extends (infer U) | undefined ? U : never;
        declare const inferUnionReordered: (string | undefined) extends undefined | (infer U) ? U : never;
        "#,
        &[
            ("intersected", "string: yes"),
            ("intersectedBoth", "string: yes"),
            ("contraPrimitives", "never"),
            ("contraSame", "string"),
            ("co", "string | number"),
            ("mixed", "never"),
            ("callback", "string"),
            ("method", "string | number"),
            ("inferUnion", "string"),
            ("inferUnionReordered", "string"),
        ],
        &[],
    );
}

#[test]
fn test_conditional_infers_from_every_assignable_union_member() {
    assert_conditional_results(
        r#"
        type Payload<T> = T extends { data: infer P } | { error: infer P } ? P : never;
        type Checked<T> = T extends { c: infer U; b: 2 } | { a: infer U } ? U : "none";
        type OptionalB<T> = T extends { c: infer U; b?: 2 } | { a: infer U } ? U : "none";
        declare const both: Payload<{ data: string; error: Promise<void> }>;
        declare const dataOnly: Payload<{ data: string }>;
        declare const missingMember: Checked<{ a: 1; c: "x" }>;
        declare const mismatchedMember: Checked<{ a: 1; c: "x"; b: 3 }>;
        declare const optionalMember: OptionalB<{ a: 1; c: "x" }>;
        "#,
        &[
            ("both", "string | instanceof Promise<void>"),
            ("dataOnly", "string"),
            ("missingMember", "number: 1"),
            ("mismatchedMember", "number: 1"),
            ("optionalMember", "string: x | number: 1"),
        ],
        &[],
    );
}

#[test]
fn test_conditional_infers_bare_union_member_from_unmatched_sources() {
    assert_conditional_results(
        r#"
        type Strip<T> = T extends (infer U) | undefined ? U : never;
        declare const identicalSource: string extends string | (infer U) ? U : never;
        declare const literalOfPrimitive: "a" extends string | (infer U) ? U : never;
        declare const unionWithIdentical: (string | number) extends string | (infer U) ? U : never;
        declare const unionWithLiteral: ("a" | number) extends string | (infer U) ? U : never;
        declare const unionOfLiterals: ("a" | "b") extends "a" | (infer U) ? U : never;
        declare const booleanSource: boolean extends true | (infer U) ? U : never;
        declare const allIdentical: (string | undefined) extends string | undefined | (infer U) ? U : never;
        declare const bigintLiteral: (1n | string) extends bigint | (infer U) ? U : never;
        declare const lowPriority: [string, number] extends [string | (infer U), infer U] ? U : never;
        declare const normalPriority: [number, string] extends [string | (infer U), infer U] ? U : never;
        declare const inferredElsewhere: Promise<number> extends Promise<infer U> | (infer U) ? U : never;
        declare const inferredMember: { a: 1 } extends { a: infer V } | (infer U) ? V : never;
        declare const constraintRejects: string extends undefined | (infer U extends number) ? U : "no";
        declare const constraintFallback: string extends string | (infer U extends number) ? U : "no";
        declare const distributive: Strip<string | undefined>;
        declare const objectSubtype: ({ a: 1; b: 2 } | number) extends { a: 1 } | (infer U) ? U : never;
        declare const severalBare: string extends (infer A) | (infer B) ? A : never;
        "#,
        &[
            ("identicalSource", "string"),
            ("literalOfPrimitive", "string: a"),
            ("unionWithIdentical", "number"),
            ("unionWithLiteral", "number"),
            ("unionOfLiterals", "string: b"),
            ("booleanSource", "bool: false"),
            ("allIdentical", "string | undefined"),
            ("bigintLiteral", "bigint: 1n | string"),
            ("lowPriority", "number"),
            ("normalPriority", "number | string"),
            ("inferredElsewhere", "number"),
            ("inferredMember", "number: 1"),
            ("constraintRejects", "string: no"),
            ("constraintFallback", "number"),
            ("distributive", "string | undefined"),
            ("severalBare", "string"),
        ],
        // Whether TypeScript treats two object types as identical isn't
        // modeled, and the identical ones don't take part in inference.
        &["objectSubtype"],
    );
}

#[test]
fn test_intersection_of_primitives_keeps_the_narrower_type() {
    assert_conditional_results(
        r#"
        declare const stringLiteral: string & "a";
        declare const literalString: "a" & string;
        declare const booleanLiteral: boolean & true;
        declare const sameValue: 1 & 1.0;
        declare const twoLiterals: "a" & "b";
        declare const crossKind: "a" & number;
        declare const twoPrimitives: string & number;
        declare const wrapped: [string & "a"] extends [never] ? 1 : 2;
        declare const wrappedNever: ["a" & "b"] extends [never] ? 1 : 2;
        declare const templateLiteral: [`a${string}` & "ab"] extends [never] ? 1 : 2;
        "#,
        &[
            ("stringLiteral", "string: a"),
            ("literalString", "string: a"),
            ("booleanLiteral", "bool: true"),
            ("sameValue", "number: 1"),
            ("twoLiterals", "never"),
            ("crossKind", "never"),
            ("twoPrimitives", "never"),
            ("wrapped", "number: 2"),
            ("wrappedNever", "number: 1"),
        ],
        // Whether a string matches a template literal type isn't evaluated.
        &["templateLiteral"],
    );
}

#[test]
fn test_conditional_compares_literal_values() {
    assert_conditional_results(
        r#"
        declare const numberSpelling: 1 extends 1.0 ? "yes" : "no";
        declare const hexNumber: 0x10 extends 16 ? "yes" : "no";
        declare const exponent: 1e3 extends 1000 ? "yes" : "no";
        declare const separators: 1_000 extends 1000 ? "yes" : "no";
        declare const negativeZero: -0 extends 0 ? "yes" : "no";
        declare const differentNumbers: 1 extends 2 ? "yes" : "no";
        declare const hexBigInt: 0x10n extends 16n ? "yes" : "no";
        declare const differentBigInts: 1n extends 2n ? "yes" : "no";
        declare const differentStrings: "a" extends "b" ? "yes" : "no";
        declare const escapedString: "\u0061" extends "a" ? "yes" : "no";
        "#,
        &[
            ("numberSpelling", "string: yes"),
            ("hexNumber", "string: yes"),
            ("exponent", "string: yes"),
            ("separators", "string: yes"),
            ("negativeZero", "string: yes"),
            ("differentNumbers", "string: no"),
            ("hexBigInt", "string: yes"),
            ("differentBigInts", "string: no"),
            ("differentStrings", "string: no"),
        ],
        // Escape sequences in string literal types aren't decoded.
        &["escapedString"],
    );
}

#[test]
fn test_conditional_optional_targets_accept_undefined() {
    assert_conditional_results(
        r#"
        declare const optionalElement: [undefined] extends [string?] ? "yes" : "no";
        declare const optionalNamedElement: [undefined] extends [x?: string] ? "yes" : "no";
        declare const optionalElementNull: [null] extends [string?] ? "yes" : "no";
        declare const optionalMember: { a: undefined } extends { a?: string } ? "yes" : "no";
        declare const optionalBuiltinMember: { value: undefined } extends TypedPropertyDescriptor<string> ? "yes" : "no";
        declare const inferOptionalMember: { a?: string } extends { a?: infer U } ? U : never;
        declare const inferOptionalElement: [string?] extends [(infer U)?] ? U : never;
        "#,
        &[
            ("optionalElement", "string: yes"),
            ("optionalNamedElement", "string: yes"),
            ("optionalElementNull", "string: no"),
            ("optionalMember", "string: yes"),
            ("optionalBuiltinMember", "string: yes"),
            ("inferOptionalMember", "string"),
            ("inferOptionalElement", "string"),
        ],
        &[],
    );
}

#[test]
fn test_conditional_with_generic_source_function() {
    assert_conditional_results(
        r#"
        declare function id<T extends string>(x: T): T;
        declare function wrap<T extends string>(cb: (x: T) => void): void;
        declare const erasedMatch: typeof id extends (x: string) => string ? "yes" : "no";
        declare const erasedReturnType: ReturnType<typeof id>;
        declare const erasedReturn: typeof id extends (x: "a") => "a" ? "yes" : "no";
        declare const erasedCallback: typeof wrap extends (cb: (x: "a") => void) => void ? "yes" : "no";
        "#,
        &[
            ("erasedMatch", "string: yes"),
            ("erasedReturnType", "string"),
        ],
        // TypeScript instantiates the source's type parameters from the
        // target, which replacing them with their constraints can't match.
        &["erasedReturn", "erasedCallback"],
    );
}

#[test]
fn test_conditional_compares_builtin_members() {
    assert_conditional_results(
        r#"
        declare const builtinInvariant: TypedPropertyDescriptor<"a"> extends TypedPropertyDescriptor<string> ? "yes" : "no";
        declare const builtinInvariantInfer: TypedPropertyDescriptor<"a"> extends TypedPropertyDescriptor<infer U> ? U : never;
        declare const mapCovariant: Map<"a", 1> extends Map<string, number> ? "yes" : "no";
        declare const mapCovariantReverse: Map<string, number> extends Map<"a", number> ? "yes" : "no";
        declare const setCovariant: Set<"a"> extends Set<string> ? "yes" : "no";
        declare const mapInfer: Map<string, number> extends Map<string, infer V> ? V : never;
        declare const promiseThenUnknown: Promise<number> extends { then: unknown } ? "yes" : "no";
        declare const defaultedArguments: IteratorObject<1> extends IteratorObject<1, string> ? "yes" : "no";
        declare const promiseThenMismatch: Promise<number> extends { then(cb: (v: string) => void): any } ? "yes" : "no";
        declare const promiseThenParameters: Parameters<Promise<number>["then"]>;
        "#,
        &[
            ("builtinInvariant", "string: no"),
            ("builtinInvariantInfer", "string: a"),
            ("mapCovariant", "string: yes"),
            ("mapCovariantReverse", "string: no"),
            ("setCovariant", "string: yes"),
            ("mapInfer", "number"),
            ("promiseThenUnknown", "string: yes"),
        ],
        &[
            // Iterator members use symbol keys, which can't be compared.
            "defaultedArguments",
            // The generated signatures of `Promise` methods are simplified.
            "promiseThenMismatch",
            "promiseThenParameters",
        ],
    );
}

#[test]
fn test_conditional_member_lookup_through_diamond_inheritance() {
    // Every level extends two interfaces that both extend the next level.
    let mut source = String::from("interface L6 { shared: 1 }\n");
    for level in (0..6).rev() {
        let next = level + 1;
        source.push_str(&format!(
            "interface L{level}A extends L{next} {{}}\n\
             interface L{level}B extends L{next} {{}}\n\
             interface L{level} extends L{level}A, L{level}B {{}}\n"
        ));
    }
    source.push_str(
        r#"
        declare const inherited: L0 extends { shared: 1 } ? "yes" : "no";
        declare const missing: L0 extends { missing: 1 } ? "yes" : "no";
        "#,
    );
    assert_conditional_results(
        &source,
        &[("inherited", "string: yes"), ("missing", "string: no")],
        &[],
    );
}

#[test]
fn test_conditional_with_generic_extends_type() {
    assert_conditional_results(
        r#"
        type IsAny<T> = 0 extends 1 & T ? true : false;
        type IsUnknown<T> = unknown extends T ? true : false;
        declare function pending<T>(value: IsAny<T>): void;
        declare const isAnyAny: IsAny<any>;
        declare const isAnyNumber: IsAny<number>;
        declare const isAnyString: IsAny<string>;
        declare const isAnyUnknown: IsAny<unknown>;
        declare const isUnknownUnknown: IsUnknown<unknown>;
        declare const isUnknownString: IsUnknown<string>;
        declare const isUnknownAny: IsUnknown<any>;
        "#,
        &[
            ("isAnyAny", "bool: true"),
            ("isAnyNumber", "bool: false"),
            ("isAnyString", "bool: false"),
            ("isAnyUnknown", "bool: false"),
            ("isUnknownUnknown", "bool: true"),
            ("isUnknownString", "bool: false"),
            ("isUnknownAny", "bool: true"),
        ],
        &["value"],
    );
}

#[test]
fn test_conditional_distribution_over_boolean_and_parentheses() {
    assert_conditional_results(
        r#"
        type IsTrue<T> = T extends true ? "yes" : "no";
        type Paren<T> = (T) extends string ? "y" : "n";
        type DoubleParen<T> = ((T)) extends string ? "y" : "n";
        declare const isTrueBoolean: IsTrue<boolean>;
        declare const isTrueMixed: IsTrue<boolean | string>;
        declare const isTrueTrue: IsTrue<true>;
        declare const isTrueFalse: IsTrue<false>;
        declare const extractBoolean: Extract<boolean, true>;
        declare const excludeBoolean: Exclude<boolean, true>;
        declare const paren: Paren<string | number>;
        declare const doubleParen: DoubleParen<string | number>;
        declare const nonDistributiveBoolean: [boolean] extends [true] ? "yes" : "no";
        "#,
        &[
            ("isTrueBoolean", "string: yes | string: no"),
            ("isTrueMixed", "string: yes | string: no"),
            ("isTrueTrue", "string: yes"),
            ("isTrueFalse", "string: no"),
            ("extractBoolean", "bool: true"),
            ("excludeBoolean", "bool: false"),
            ("paren", "string: y | string: n"),
            ("doubleParen", "string: y | string: n"),
            ("nonDistributiveBoolean", "string: no"),
        ],
        &[],
    );
}

#[test]
fn test_conditional_compares_generics_and_methods_by_variance() {
    assert_conditional_results(
        r#"
        interface Sink<T> { accept: (x: T) => void }
        interface Producer<T> { read: () => T }
        interface Invariant<T> { get: () => T; set: (x: T) => void }
        interface MethodSink<T> { accept(x: T): void }
        interface Phantom<T> { name: string }
        interface MSource { handle(x: "a"): void }
        interface MTarget { handle(x: string): void }
        interface PSource { handle: (x: "a") => void }
        interface PTarget { handle: (x: string) => void }
        interface Pair<A, B> { first: () => A; second: () => B }
        type ProducedBy<T> = T extends Producer<infer U> ? U : never;
        type SecondOf<T> = T extends Pair<"a", infer U> ? U : never;
        declare const contravariant: Sink<"a"> extends Sink<string> ? "yes" : "no";
        declare const contravariantReverse: Sink<string> extends Sink<"a"> ? "yes" : "no";
        declare const covariant: Producer<"a"> extends Producer<string> ? "yes" : "no";
        declare const covariantReverse: Producer<string> extends Producer<"a"> ? "yes" : "no";
        declare const invariant: Invariant<"a"> extends Invariant<string> ? "yes" : "no";
        declare const methodSink: MethodSink<"a"> extends MethodSink<string> ? "yes" : "no";
        declare const methodSinkReverse: MethodSink<string> extends MethodSink<"a"> ? "yes" : "no";
        declare const methodSinkUnrelated: MethodSink<number> extends MethodSink<string> ? "yes" : "no";
        declare const phantom: Phantom<1> extends Phantom<string> ? "yes" : "no";
        declare const methodBivariance: MSource extends MTarget ? "yes" : "no";
        declare const methodBivarianceReverse: MTarget extends MSource ? "yes" : "no";
        declare const propertyContravariant: PSource extends PTarget ? "yes" : "no";
        declare const produced: ProducedBy<Producer<number>>;
        declare const secondOf: SecondOf<Pair<"a", number>>;
        declare const secondOfMismatch: SecondOf<Pair<"b", number>>;
        "#,
        &[
            ("contravariant", "string: no"),
            ("contravariantReverse", "string: yes"),
            ("covariant", "string: yes"),
            ("covariantReverse", "string: no"),
            ("invariant", "string: no"),
            ("methodSink", "string: yes"),
            ("methodSinkReverse", "string: yes"),
            ("methodSinkUnrelated", "string: no"),
            ("phantom", "string: yes"),
            ("methodBivariance", "string: yes"),
            ("methodBivarianceReverse", "string: yes"),
            ("propertyContravariant", "string: no"),
            ("produced", "number"),
            ("secondOf", "number"),
            ("secondOfMismatch", "never"),
        ],
        &[],
    );
}

#[test]
fn test_conditional_object_assignability_rules() {
    assert_conditional_results(
        r#"
        interface Named { value: number }
        class Private1 { private x = 1 }
        class Private2 { private x = 1 }
        class Base { v = 1 }
        class Child extends Base {}
        declare const weak: { a: number } extends { b?: string } ? "yes" : "no";
        declare const weakCommon: { a: number; b: string } extends { b?: string } ? "yes" : "no";
        declare const weakEmptySource: {} extends { b?: string } ? "yes" : "no";
        declare const weakIntersection: { a: number } extends { a: number } & { b?: string } ? "yes" : "no";
        declare const weakIntersectionInterface: { value: number } extends Named & { b?: string } ? "yes" : "no";
        declare const notWeakRequired: { a: number } extends { a: number; b?: string } ? "yes" : "no";
        declare const objectMember: { a: 1 } extends { toString(): string } ? "yes" : "no";
        declare const interfaceObjectMember: Named extends { valueOf(): Object } ? "yes" : "no";
        declare const objectMemberMismatch: { a: 1 } extends { toString(): number } ? "yes" : "no";
        declare const unknownEmpty: unknown extends {} ? "yes" : "no";
        declare const unknownString: unknown extends string ? "yes" : "no";
        declare const unknownObject: unknown extends object ? "yes" : "no";
        declare const unknownUnknown: unknown extends unknown ? "yes" : "no";
        declare const unknownUnion: unknown extends string | unknown ? "yes" : "no";
        declare const unknownNullable: unknown extends {} | null | undefined ? "yes" : "no";
        declare const unknownNullableWider: unknown extends string | {} | null | undefined ? "yes" : "no";
        declare const unknownNullableVoid: unknown extends {} | null | void ? "yes" : "no";
        declare const unknownObjectNullable: unknown extends Object | null | undefined ? "yes" : "no";
        declare const stringPromise: string extends Promise<any> ? "yes" : "no";
        declare const numberPromise: number extends Promise<any> ? "yes" : "no";
        declare const stringArray: string extends string[] ? "yes" : "no";
        declare const stringIterable: string extends Iterable<string> ? "yes" : "no";
        declare const stringArrayLike: string extends ArrayLike<string> ? "yes" : "no";
        declare const stringObject: string extends Object ? "yes" : "no";
        declare const stringWrapper: string extends String ? "yes" : "no";
        declare const privateNominal: Private2 extends Private1 ? "yes" : "no";
        declare const childOfBase: Child extends Base ? "yes" : "no";
        declare const sameClass: Base extends Base ? "yes" : "no";
        declare const builtinClass: Promise<"a"> extends Promise<string> ? "yes" : "no";
        "#,
        &[
            ("weak", "string: no"),
            ("weakCommon", "string: yes"),
            ("weakEmptySource", "string: yes"),
            ("weakIntersection", "string: yes"),
            ("notWeakRequired", "string: yes"),
            ("objectMember", "string: yes"),
            ("interfaceObjectMember", "string: yes"),
            ("objectMemberMismatch", "string: no"),
            ("unknownEmpty", "string: no"),
            ("unknownString", "string: no"),
            ("unknownObject", "string: no"),
            ("unknownUnknown", "string: yes"),
            ("unknownUnion", "string: yes"),
            ("unknownNullable", "string: yes"),
            ("unknownNullableWider", "string: yes"),
            ("unknownNullableVoid", "string: no"),
            ("unknownObjectNullable", "string: no"),
            ("stringPromise", "string: no"),
            ("numberPromise", "string: no"),
            ("stringArray", "string: no"),
            ("stringObject", "string: yes"),
            ("stringWrapper", "string: yes"),
            ("childOfBase", "string: yes"),
            ("sameClass", "string: yes"),
            ("builtinClass", "string: yes"),
        ],
        &[
            // A weak member is only checked against the whole intersection.
            "weakIntersectionInterface",
            // Symbol keys and index signatures can't be compared.
            "stringIterable",
            "stringArrayLike",
            // Member accessibility isn't modeled.
            "privateNominal",
        ],
    );
}

#[test]
fn test_conditional_compares_constructor_parameters() {
    assert_conditional_results(
        r#"
        class Foo { constructor(public x: string) {} }
        class NoConstructor {}
        class Derived extends Foo {}
        declare const ctorMismatch: (new (x: string) => Foo) extends (new (x: number) => Foo) ? "yes" : "no";
        declare const ctorMatch: (new (x: string) => Foo) extends (new (x: string) => Foo) ? "yes" : "no";
        declare const ctorFewer: (new () => Foo) extends (new (x: string) => Foo) ? "yes" : "no";
        declare const ctorMore: (new (x: string) => Foo) extends (new () => Foo) ? "yes" : "no";
        declare const classMismatch: typeof Foo extends (new (x: number) => any) ? "yes" : "no";
        declare const classMatch: typeof Foo extends (new (x: string) => any) ? "yes" : "no";
        declare const classMore: typeof Foo extends (new () => any) ? "yes" : "no";
        declare const classNoConstructor: typeof NoConstructor extends (new () => any) ? "yes" : "no";
        declare const parameters: ConstructorParameters<typeof Foo>;
        declare const parametersNone: ConstructorParameters<typeof NoConstructor>;
        declare const parametersDerived: ConstructorParameters<typeof Derived>;
        declare const instance: InstanceType<typeof Foo> extends Foo ? "yes" : "no";
        "#,
        &[
            ("ctorMismatch", "string: no"),
            ("ctorMatch", "string: yes"),
            ("ctorFewer", "string: yes"),
            ("ctorMore", "string: no"),
            ("classMismatch", "string: no"),
            ("classMatch", "string: yes"),
            ("classMore", "string: no"),
            ("classNoConstructor", "string: yes"),
            ("parameters", "[x: string]"),
            ("parametersNone", "[]"),
            ("parametersDerived", "[x: string]"),
            ("instance", "string: yes"),
        ],
        &[],
    );
}

#[test]
fn test_conditional_nested_in_generic_alias_body() {
    assert_conditional_results(
        r#"
        type Inner<T> = T extends string ? "s" : "n";
        type Outer<T> = Inner<T>;
        type Outer2<T> = Outer<T>;
        type InUnion<T> = (T extends string ? 1 : 2) | undefined;
        type InTuple<T> = [T extends string ? 1 : 2];
        type InPromise<T> = Promise<T extends string ? 1 : 2>;
        type TwoArgs<A, B> = Inner<B> | A;
        declare function pending<T>(value: Outer<T>): void;
        declare const outer: Outer<string>;
        declare const outer2: Outer2<number>;
        declare const outerUnion: Outer<string | number>;
        declare const inUnion: InUnion<string>;
        declare const inTuple: InTuple<string>;
        declare const inPromise: Awaited<InPromise<string>>;
        declare const twoArgs: TwoArgs<1, string>;
        "#,
        &[
            ("outer", "string: s"),
            ("outer2", "string: n"),
            ("outerUnion", "string: s | string: n"),
            ("inUnion", "number: 1 | undefined"),
            ("inTuple", "[number: 1]"),
            ("inPromise", "number: 1"),
            ("twoArgs", "string: s | number: 1"),
        ],
        &["value"],
    );
}

#[test]
fn test_expressions_select_the_branch_of_a_conditional_operand() {
    assert_conditional_results(
        r#"
        declare const nullable: { p: number } | (true extends true ? null : null);
        export const optional = nullable?.p;
        declare const tuples: [string] | (true extends true ? [number] : [boolean]);
        export const indexed = tuples[0];
        declare const shape: true extends true ? { a: number; b: string } : { a: boolean };
        export const { a, ...rest } = shape;
        declare const big: true extends true ? bigint : number;
        export const negated = -big;
        export const inverted = ~big;
        declare const plainBig: bigint;
        export const negatedPlain = -plainBig;
        declare const partial: Partial<string extends string ? { a: number } : { b: number }>;
        declare const picked: Pick<{ a: number; b: string }, true extends true ? "a" : "b">;
        "#,
        &[
            ("optional", "number | undefined"),
            ("indexed", "string | number"),
            (
                "rest",
                "Object {\n  prototype: No prototype\n  members: [\"b\": string]\n}",
            ),
            ("negated", "BigInt"),
            ("inverted", "BigInt"),
            ("negatedPlain", "BigInt"),
            (
                "partial",
                "Object {\n  prototype: No prototype\n  members: [\"a\"?: number | undefined]\n}",
            ),
            (
                "picked",
                "Object {\n  prototype: No prototype\n  members: [\"a\": number]\n}",
            ),
        ],
        &[],
    );
}
