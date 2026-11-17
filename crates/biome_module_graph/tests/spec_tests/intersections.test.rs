use super::*;

#[test]
fn test_infer_module_types_merges_class_instance_intersections_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readPromiseObject(
                value: Promise<string> & { value: number },
            ): Promise<string> & { value: number } {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let promise_object_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readPromiseObject")
            .expect("readPromiseObject return type must be inferred");
    let promise_object_ty = normalize_type(&db, index_module, promise_object_ty);
    let InferredTypeData::Intersection(intersection) = promise_object_ty else {
        panic!("readPromiseObject must preserve the intersection, got {promise_object_ty:?}");
    };
    assert!(
        intersection
            .types(&db)
            .iter()
            .any(|ty| is_inferred_promise_instance(&db, *ty))
    );

    let value_ty = inferred
        .find_member_type(&db, promise_object_ty, "value")
        .expect("merged class instance must expose value");
    assert!(is_inferred_number(&db, value_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_merges_class_instance_intersections_on_build",
        &db,
        &fs,
    );
}

mod normalization {
    use super::*;
    use biome_module_graph::{
        ExpressionTypeInput, infer_expression_type,
        type_inference::{
            NormalizedExpressionTypeRequest, TypeInferenceCaller, execute_type_inference_request,
        },
    };

    /// How [`variant_names`] names the members of
    /// `({ k: "a"; x: 1 } | { k: "b"; y: 2 }) & { z: 3 }`.
    const MEMBER_A: &str = r#"Object { prototype: No prototype members: ["k": string: a, "x": number: 1, "z": number: 3] }"#;
    const MEMBER_B: &str = r#"Object { prototype: No prototype members: ["k": string: b, "y": number: 2, "z": number: 3] }"#;

    fn marked_range(source: &str, marker: &str, expression: &str) -> TextRange {
        let marker = format!("/*{marker}*/");
        let mut occurrences = source.match_indices(&marker);
        let (offset, _) = occurrences.next().expect("source marker must exist");
        assert!(occurrences.next().is_none(), "source marker must be unique");
        let start = offset + marker.len();
        assert!(source[start..].starts_with(expression));
        TextRange::at(
            start.try_into().unwrap(),
            expression.len().try_into().unwrap(),
        )
    }

    /// Names each member of `ty`, sorted: an interface by its name, Biome's
    /// unknown type as `Unknown`, and any other type by its formatted text on
    /// one line, such as `string` or `instanceof T & string`.
    fn variant_names(db: &TestModuleDb, ty: InferredTypeData) -> Vec<String> {
        if ty == InferredTypeData::Unknown {
            return vec!["Unknown".to_owned()];
        }
        let variants = match ty {
            InferredTypeData::Union(union) => union.types(db).to_vec(),
            _ => vec![ty],
        };
        let mut names: Vec<_> = variants
            .into_iter()
            .map(|ty| {
                let text = format_inferred_type(db, ty);
                let first = text.lines().next().unwrap_or_default();
                if let Some(name) = first
                    .strip_prefix("interface \"")
                    .and_then(|name| name.split('"').next())
                {
                    return name.to_owned();
                }
                let lines: Vec<_> = text.lines().map(str::trim).collect();
                lines.join(" ")
            })
            .collect();
        names.sort();
        names
    }

    fn build(source: &str, modules: &[(&str, &str)]) -> (TestModuleDb, ModuleInfo) {
        let fs = MemoryFileSystem::default();
        fs.insert("/src/index.ts".into(), source);
        let mut paths = vec!["/src/index.ts"];
        for (path, module) in modules {
            fs.insert((*path).into(), *module);
            paths.push(path);
        }
        let db = build_js_test_module_db(&fs, &paths, true);
        let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
        (db, module)
    }

    /// Asserts the normalized type of each `(marker, expression, expected)`
    /// read in `source`, the module at `/src/index.ts`, by the names of its
    /// members, from [`variant_names`]. Also asserts that complete module
    /// inference gives each read the type that a lookup gives it.
    fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
        let lookups: Vec<_> = {
            let (db, module) = build(source, &[]);
            reads
                .iter()
                .map(|(marker, expression, _)| {
                    let range = marked_range(source, marker, expression);
                    infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                        .map(|ty| format_inferred_type(&db, ty))
                })
                .collect()
        };
        let (db, module) = build(source, &[]);
        let complete = infer_module_types(&db, module).unwrap();
        let mismatches: Vec<_> = reads
            .iter()
            .zip(lookups)
            .flat_map(|((marker, expression, expected), lookup)| {
                let range = marked_range(source, marker, expression);
                let ty = execute_type_inference_request(
                    &db,
                    TypeInferenceCaller::new("test", "intersections"),
                    NormalizedExpressionTypeRequest::new(module, range),
                )
                .unwrap_or_else(|| panic!("expression at {marker} must be inferred"));
                let actual = variant_names(&db, ty);
                let complete = Some(format_inferred_type(&db, complete.expressions[&range]));
                [
                    (actual != *expected)
                        .then(|| format!("{marker}: expected {expected:?}, got {actual:?}")),
                    (complete != lookup).then(|| {
                        format!("{marker}: module inference gives {complete:?}, lookups {lookup:?}")
                    }),
                ]
            })
            .flatten()
            .collect();
        assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    }

    // TypeScript 7.0.2 (`tsc --strict`) gives every read below the type
    // written in its comment. Biome displays some of them otherwise, with the
    // same values: it merges the operands of `{ a: 1 } & { b: 2 }` and of
    // `Foo & Bar` into one anonymous type, and writes `(string | number) & Foo`
    // distributed, as `(string & Foo) | (number & Foo)`. `string & {}` is
    // `string`, because every string is in `{}`; TypeScript keeps writing
    // `string & {}`. `null & {}` is `never`.
    #[test]
    fn intersections_normalize_as_in_typescript() {
        assert_reads(
            r#"
            export {};
            interface Foo { a: number }
            interface Bar { c: number }
            class A { a = 1; r?: string }
            declare const v1: NonNullable<string | null>;
            declare const v2: (string | undefined) & string;
            declare const v3: { a: 1 } & { b: 2 };
            declare const v4: Foo & Bar;
            declare const v5: A & { r: string };
            declare const v6: (string | number) & Foo;
            declare const v7: null & {};
            declare const v8: string & {};
            declare const v9: Record<"a", unknown>;
            declare const v10: object & Record<"a", unknown>;
            declare const v12: NonNullable<Foo | undefined>;
            declare const v13: ({ k: "a"; x: 1 } | { k: "b"; y: 2 }) & { z: 3 };
            function g<T>(x: T & {}, y: T & string) {
                /*x*/x; // T & {}
                /*y*/y; // T & string
                /*length*/y.length; // number
            }
            function h1() {
                /*v1*/v1; // string
                /*v2*/v2; // string
                /*v3*/v3; // { a: 1 } & { b: 2 }
                /*v4*/v4; // Foo & Bar
            }
            function h2() {
                /*v5*/v5; // A & { r: string }
                /*r*/v5.r; // string
                /*v6*/v6; // (string | number) & Foo
                /*v7*/v7; // never
            }
            function h3() {
                /*v8*/v8; // string & {}
                /*v9*/v9; // Record<"a", unknown>
                /*v10*/v10; // object & Record<"a", unknown>
                /*a*/v10.a; // unknown
            }
            // `v11` is a parameter, because Biome reads an element of a tuple
            // declared outside the function as Unknown.
            function h4(v11: [string | number] & string[]) {
                /*v11*/v11; // [string | number] & string[]
                /*element*/v11[0]; // string | number
                /*foo*/v4.a; // number
                /*v12*/v12; // Foo
            }
            function h5() {
                /*v13*/v13; // ({ k: "a"; x: 1 } | { k: "b"; y: 2 }) & { z: 3 }
                /*z*/v13.z; // 3
                if (v13.k === "a") /*narrowed*/v13; // { k: "a"; x: 1 } & { z: 3 }
            }
            "#,
            &[
                (
                    "x",
                    "x",
                    &["instanceof T & Object { prototype: No prototype members: [] }"],
                ),
                ("y", "y", &["instanceof T & string"]),
                ("length", "y.length", &["number"]),
                ("v1", "v1", &["string"]),
                ("v2", "v2", &["string"]),
                (
                    "v3",
                    "v3",
                    &[
                        r#"Object { prototype: No prototype members: ["a": number: 1, "b": number: 2] }"#,
                    ],
                ),
                ("v4", "v4", &["(merged)"]),
                (
                    "v5",
                    "v5",
                    &["instanceof class { extends: none implements: [] type_args: [] }"],
                ),
                ("r", "v5.r", &["string"]),
                (
                    "v6",
                    "v6",
                    &[
                        r#"number & interface "Foo" { extends: [] type_args: [] members: ["a": number] }"#,
                        r#"string & interface "Foo" { extends: [] type_args: [] members: ["a": number] }"#,
                    ],
                ),
                ("v7", "v7", &["never"]),
                ("v8", "v8", &["string"]),
                (
                    "v9",
                    "v9",
                    &["Object { prototype: No prototype members: [[string: a]: unknown] }"],
                ),
                (
                    "v10",
                    "v10",
                    &[
                        "object & Object { prototype: No prototype members: [[string: a]: unknown] }",
                    ],
                ),
                ("a", "v10.a", &["unknown"]),
                (
                    "v11",
                    "v11",
                    &["[string | number] & instanceof Array<string>"],
                ),
                ("element", "v11[0]", &["number", "string"]),
                ("foo", "v4.a", &["number"]),
                ("v12", "v12", &["Foo"]),
                ("v13", "v13", &[MEMBER_A, MEMBER_B]),
                ("z", "v13.z", &["number: 3"]),
                ("narrowed", "v13", &[MEMBER_A]),
            ],
        );
    }

    // A property that several operands declare has the intersection of their
    // types, where an optional property's type includes `undefined`, and is
    // optional only when every operand declares it optional. TypeScript
    // 7.0.2 gives every read below the type written in its comment, except
    // where a comment says that Biome is wider.
    #[test]
    fn properties_declared_by_several_operands_intersect() {
        assert_reads(
            r#"
            export {};
            declare const m1: { value: string } & { value: number };
            declare const m2: { r?: string } & { r: string | null };
            declare const m3: { r?: string } & { r?: number | string };
            declare const m4: { r: string | undefined } & { r?: string };
            declare const m5: { f(): string } & { f(): number };
            declare const m6: { f: () => string } & { f: () => number };
            declare const m7: { o: { x: 1 } } & { o: { y: 2 } };
            declare const m8: { a: string } & Record<string, number>;
            declare const m9: string & { length: 5 };
            declare const m10: ({ p: "a" } | { q: 1 }) & { p: string };
            function h1() {
                /*m1*/m1.value; // never
                /*m2*/m2.r; // string
                /*m3*/m3.r; // string | undefined
                /*m4*/m4.r; // string | undefined
            }
            function h2() {
                // TypeScript calls the first overload and gives `string`.
                // Biome gives the union of the methods' return types.
                /*m5*/m5.f(); // string
                /*m6*/m6.f(); // string
                /*m7*/m7.o; // { x: 1 } & { y: 2 }
                /*m7x*/m7.o.x; // 1
            }
            function h3() {
                // An index signature and the apparent type of a primitive
                // don't declare a property, so only the other operand counts.
                /*m8*/m8.a; // string
                /*m9*/m9.length; // 5
                // One member of the union operand lacks `p`, so the union
                // doesn't declare it, and the intersection distributes over it.
                /*m10*/m10.p; // string
            }
            "#,
            &[
                ("m1", "m1.value", &["never"]),
                ("m2", "m2.r", &["string"]),
                ("m3", "m3.r", &["string", "undefined"]),
                ("m4", "m4.r", &["string", "undefined"]),
                ("m5", "m5.f()", &["number", "string"]),
                ("m6", "m6.f()", &["number", "string"]),
                (
                    "m7",
                    "m7.o",
                    &[
                        r#"Object { prototype: No prototype members: ["x": number: 1, "y": number: 2] }"#,
                    ],
                ),
                ("m7x", "m7.o.x", &["number: 1"]),
                ("m8", "m8.a", &["string"]),
                ("m9", "m9.length", &["number: 5"]),
                ("m10", "m10.p", &["string"]),
            ],
        );
    }

    // An element that tuple operands declare has the intersection of their
    // element types. An array operand declares no element, so its element
    // type doesn't count. TypeScript 7.0.2 gives every read below the type
    // written in its comment.
    #[test]
    fn elements_declared_by_tuple_operands_intersect() {
        assert_reads(
            r#"
            export {};
            function f(
                t: [string | number] & string[],
                u: readonly [string | number] & readonly string[],
                w: [string?] & [string | number],
            ) {
                /*t*/t[0]; // string | number
                /*u*/u[0]; // string | number
                /*w*/w[0]; // string
            }
            "#,
            &[
                ("t", "t[0]", &["number", "string"]),
                ("u", "u[0]", &["number", "string"]),
                ("w", "w[0]", &["string"]),
            ],
        );
    }

    // `null` and `undefined` give `never` next to an object type, and `{}` is
    // removed next to an operand that is never `null` or `undefined`.
    // TypeScript 7.0.2 gives every read below the type written in its comment.
    #[test]
    fn nullish_operands_and_empty_objects_normalize() {
        assert_reads(
            r#"
            export {};
            interface Foo { a: number }
            declare const n1: undefined & Foo;
            declare const n2: null & object;
            declare const n3: (Foo | null) & {};
            declare const n4: Foo & {};
            declare const n5: (string | undefined) & {};
            declare const n6: undefined & string;
            function f() {
                /*n1*/n1; // never
                /*n2*/n2; // never
                /*n3*/n3; // Foo
                /*n4*/n4; // Foo
                /*n5*/n5; // string
                /*n6*/n6; // never
            }
            "#,
            &[
                ("n1", "n1", &["never"]),
                ("n2", "n2", &["never"]),
                ("n3", "n3", &["Foo"]),
                ("n4", "n4", &["Foo"]),
                ("n5", "n5", &["string"]),
                ("n6", "n6", &["never"]),
            ],
        );
    }

    // An intersection distributes over its union operands. TypeScript 7.0.2
    // gives every read below the type written in its comment. Biome keeps the
    // union operands of `o`, whose product has more than 32 members, where
    // TypeScript gives `null`; that type has fewer values, so Biome is wider.
    #[test]
    fn intersections_distribute_over_unions() {
        assert_reads(
            r#"
            export {};
            interface Foo { a: number }
            interface Bar { c: number }
            declare const d1: ("a" | "b") & ("b" | "c");
            declare const d2: (string | number) & (number | boolean);
            declare const d3: (Foo | Bar) & { z: 1 };
            declare const o: (1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | null) & (10 | 11 | 12 | 13 | null);
            function f() {
                /*d1*/d1; // "b"
                /*d2*/d2; // number
                /*d3*/d3; // (Foo | Bar) & { z: 1 }
                /*z*/d3.z; // 1
                /*o*/o; // null
            }
            "#,
            &[
                ("d1", "d1", &["string: b"]),
                ("d2", "d2", &["number"]),
                (
                    "d3",
                    "d3",
                    &[
                        r#"Object { unknown members prototype: No prototype members: ["a": number, "z": number: 1] }"#,
                        r#"Object { unknown members prototype: No prototype members: ["c": number, "z": number: 1] }"#,
                    ],
                ),
                ("z", "d3.z", &["number: 1"]),
                (
                    "o",
                    "o",
                    &[
                        "number: 1 | number: 2 | number: 3 | number: 4 | number: 5 | number: 6 | number: 7 | number: 8 | number: 9 | null & number: 10 | number: 11 | number: 12 | number: 13 | null",
                    ],
                ),
            ],
        );
    }

    // A generic alias whose body is an intersection is instantiated with its
    // type arguments, so its intersection is normalized. An alias that refers
    // to itself is instantiated until normalization runs out of
    // instantiations. TypeScript 7.0.2 gives every read below the type written
    // in its comment, except where a comment says that Biome is wider.
    #[test]
    fn intersection_aliases_are_instantiated() {
        assert_reads(
            r#"
            export {};
            interface Foo { a: number }
            type WithId<T> = T & { id: string };
            type List<T> = T & { next: List<T> };
            declare const a1: WithId<Foo>;
            declare const a2: NonNullable<"a" | null | undefined>;
            declare const a3: List<{ v: 1 }>;
            declare const a4: NonNullable<unknown>;
            function f() {
                /*a1*/a1; // WithId<Foo>
                /*id*/a1.id; // string
                /*a2*/a2; // "a"
                /*next*/a3.next.next.v; // 1
                // Biome gives `unknown & {}` its unknown type, which is wider.
                /*a4*/a4; // {}
            }
            "#,
            &[
                (
                    "a1",
                    "a1",
                    &[
                        r#"Object { unknown members prototype: No prototype members: ["a": number, "id": string] }"#,
                    ],
                ),
                ("id", "a1.id", &["string"]),
                ("a2", "a2", &["string: a"]),
                ("next", "a3.next.next.v", &["number: 1"]),
                ("a4", "a4", &["Unknown"]),
            ],
        );
    }

    // Normalizing an intersection resolves only the types it reaches: reading
    // a distributed intersection whose operand is imported resolves the
    // imported interface through lookups, and infers no module completely,
    // neither the reading module, nor the imported one, nor an unrelated one.
    #[test]
    fn distributed_intersections_resolve_selectively() {
        const SOURCE: &str = r#"
            import type { Foo } from "./foo";
            declare const v: (Foo | null) & { z: 1 };
            function f() {
                /*z*/v.z;
                /*v*/v;
            }
        "#;
        let (db, module) = build(
            SOURCE,
            &[
                ("/src/foo.ts", "export interface Foo { a: number }"),
                ("/src/other.ts", "export interface Other { b: string }"),
            ],
        );
        let foo = db.module_for_path(Utf8Path::new("/src/foo.ts")).unwrap();
        let other = db.module_for_path(Utf8Path::new("/src/other.ts")).unwrap();
        let read = |marker, expression| {
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "intersections"),
                NormalizedExpressionTypeRequest::new(
                    module,
                    marked_range(SOURCE, marker, expression),
                ),
            )
            .unwrap();
            variant_names(&db, ty)
        };

        db.clear_salsa_events();
        assert_eq!(read("z", "v.z"), ["number: 1"]);
        assert_eq!(
            read("v", "v"),
            [
                r#"Object { unknown members prototype: No prototype members: ["a": number, "z": number: 1] }"#
            ]
        );
        let events = db.take_salsa_events();
        for module in [module, foo, other] {
            assert_function_query_was_not_run(&db, infer_module_types, module, &events);
        }
    }
}
