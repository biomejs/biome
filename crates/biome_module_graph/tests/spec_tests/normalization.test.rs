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
        ("mutableElement", "unknown"),
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
