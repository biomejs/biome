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
