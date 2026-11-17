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
    pub(super) fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
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

mod narrowing {
    use super::normalization::assert_reads;

    /// How the reads below name `T & {}`, TypeScript's `NonNullable<T>`.
    const T_EMPTY: &str = "instanceof T & Object { prototype: No prototype members: [] }";
    const T_FOO: &str =
        r#"instanceof T & interface "Foo" { extends: [] type_args: [] members: ["a": number] }"#;
    /// How the reads below name an intersection whose operands normalization
    /// merges into an instance of an anonymous class.
    const MERGED_CLASS: &str = "instanceof class { extends: none implements: [] type_args: [] }";

    const DECLARATIONS: &str = r#"
        export {};
        declare function p(v: unknown): void;
        declare function isFoo(v: unknown): v is Foo;
        declare function isString(v: unknown): v is string;
        declare function isNonNull<T>(v: T): v is NonNullable<T>;
        declare function assertNonNull<T>(v: T): asserts v is NonNullable<T>;
        interface Foo { a: number }
        interface Bar { c: number }
        class A { a = 1 }
        class B { b = 2 }
    "#;

    // TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the
    // comment of its line, in order. A test narrows a value of a type
    // parameter `T` to the intersection of `T` with the type of the values
    // that pass, unless its constraint decides the test. Biome writes
    // `T & ({} | undefined)` distributed, as `(T & {}) | (T & undefined)`,
    // which has the same values.
    #[test]
    fn type_parameters_narrow_to_intersections() {
        let source = format!(
            "{DECLARATIONS}{}",
            r#"
            function g1<T>(x: T) { if (x) p([/*L12_33*/x] as const); else p([/*L12_55*/x] as const); } // NonNullable<T>; T
            function g2<T>(x: T) { if (x != null) p([/*L13_41*/x] as const); else p([/*L13_63*/x] as const); } // NonNullable<T>; T
            function g3<T>(x: T) { if (x !== null) p([/*L14_42*/x] as const); else p([/*L14_64*/x] as const); } // T & ({} | undefined); T
            function g4<T>(x: T) { if (x !== undefined) p([/*L15_47*/x] as const); else p([/*L15_69*/x] as const); } // T & ({} | null); T
            function g5<T>(x: T) { if (typeof x === "string") p([/*L16_53*/x] as const); else p([/*L16_75*/x] as const); } // T & string; T
            function g6<T>(x: T) { if (typeof x === "object") p([/*L17_53*/x] as const); else p([/*L17_75*/x] as const); } // (T & null) | (T & object); T
            function g7<T>(x: T) { if (typeof x === "function") p([/*L18_55*/x] as const); else p([/*L18_77*/x] as const); } // T & Function; T
            function g8<T>(x: T) { if (typeof x === "undefined") p([/*L19_56*/x] as const); else p([/*L19_78*/x] as const); } // T & undefined; T & ({} | null)
            function g9<T>(x: T) { x && p([/*L20_31*/x] as const); } // NonNullable<T>
            function g10<T>(x: T) { if (x) { const y = x; p([/*L21_49*/y] as const); } } // NonNullable<T>
            function g11<T>(x: T | null) { if (x) p([/*L22_41*/x] as const); } // NonNullable<T>
            function g12<T>(x: T | null) { if (x !== null) p([/*L23_50*/x] as const); } // T & ({} | undefined)
            function g13<T extends string | undefined>(x: T) { if (x) p([/*L24_61*/x] as const); } // NonNullable<T>
            function g14<T extends string | undefined>(x: T) { if (x) p([/*L25_61*/x.length] as const); } // number
            function g15<T extends string | number>(x: T) { if (typeof x === "string") p([/*L26_78*/x] as const); } // T & string
            function g16<T extends string | number>(x: T) { if (typeof x === "string") { const y = x; p([/*L27_93*/y] as const); } } // T & string
            function g17<T extends { a: 1 } | null>(x: T) { if (x) p([/*L28_58*/x] as const); } // NonNullable<T>
            function g18<T extends object>(x: T) { if (x) p([/*L29_49*/x] as const); else p([/*L29_71*/x] as const); } // T; never
            function g19<T extends {}>(x: T) { if (x != null) p([/*L30_53*/x] as const); } // T
            function u3<T>(x: T) { if (x) { if (typeof x === "string") p([/*L80_62*/x] as const); } } // T & string
            function u4<T>(x: T) { if (x != null) { if (x === null) p([/*L81_59*/x] as const); } } // never
            function u5<T>(x: T) { if (typeof x === "string" || typeof x === "number") p([/*L82_78*/x] as const); } // (T & string) | (T & number)
            function u6<T>(x: T) { if (!x) { return; } p([/*L83_46*/x] as const); } // NonNullable<T>
            function u7<T>(o: { v: T }) { if (o.v) p([/*L84_42*/o.v] as const); } // NonNullable<T>
            function u8<T>(x: T) { if (x) { if (x) p([/*L85_42*/x] as const); } } // NonNullable<T>
            function u9<T>(x: T) { if (typeof x === "string") { if (x) p([/*L86_62*/x] as const); } } // T & string
            function u10<T>(x: T) { if (x === "a") p([/*L87_42*/x] as const); } // T
            function u11<T>(x: T) { if (x === null) p([/*L88_43*/x] as const); else p([/*L88_65*/x] as const); } // T; T & ({} | undefined)
            "#
        );
        assert_reads(
            &source,
            &[
                ("L12_33", "x", &[T_EMPTY]),
                ("L12_55", "x", &["instanceof T"]),
                ("L13_41", "x", &[T_EMPTY]),
                ("L13_63", "x", &["instanceof T"]),
                ("L14_42", "x", &[T_EMPTY, "instanceof T & undefined"]),
                ("L14_64", "x", &["instanceof T"]),
                ("L15_47", "x", &[T_EMPTY, "instanceof T & null"]),
                ("L15_69", "x", &["instanceof T"]),
                ("L16_53", "x", &["instanceof T & string"]),
                ("L16_75", "x", &["instanceof T"]),
                (
                    "L17_53",
                    "x",
                    &["instanceof T & null", "instanceof T & object"],
                ),
                ("L17_75", "x", &["instanceof T"]),
                ("L18_55", "x", &["instanceof T & instanceof Function"]),
                ("L18_77", "x", &["instanceof T"]),
                ("L19_56", "x", &["instanceof T & undefined"]),
                ("L19_78", "x", &[T_EMPTY, "instanceof T & null"]),
                ("L20_31", "x", &[T_EMPTY]),
                // Biome types a variable by its initializer without
                // narrowing, so `y` is `T`.
                ("L21_49", "y", &["instanceof T"]),
                ("L22_41", "x", &[T_EMPTY]),
                ("L23_50", "x", &[T_EMPTY, "instanceof T & undefined"]),
                ("L24_61", "x", &[T_EMPTY]),
                ("L25_61", "x.length", &["number"]),
                ("L26_78", "x", &["instanceof T & string"]),
                // As for `L21_49`.
                ("L27_93", "y", &["instanceof T"]),
                ("L28_58", "x", &[T_EMPTY]),
                ("L29_49", "x", &["instanceof T"]),
                // Biome keeps `T` where its constraint leaves no value.
                ("L29_71", "x", &["instanceof T"]),
                ("L30_53", "x", &["instanceof T"]),
                ("L80_62", "x", &["instanceof T & string"]),
                // `T & {}` has no `null` operand that Biome removes.
                ("L81_59", "x", &[T_EMPTY]),
                (
                    "L82_78",
                    "x",
                    &["instanceof T & number", "instanceof T & string"],
                ),
                ("L83_46", "x", &[T_EMPTY]),
                ("L84_42", "o.v", &[T_EMPTY]),
                ("L85_42", "x", &[T_EMPTY]),
                ("L86_62", "x", &["instanceof T & string"]),
                ("L87_42", "x", &["instanceof T"]),
                ("L88_43", "x", &["instanceof T"]),
                ("L88_65", "x", &[T_EMPTY, "instanceof T & undefined"]),
            ],
        );
    }

    // TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the
    // comment of its line, in order. A nullish value that the type already
    // has, as a member or in the constraint, needs no `{}` union. Biome keeps
    // `T` where the constraint leaves no value, where TypeScript gives
    // `never`, and gives `T & ({} | undefined)` where TypeScript gives
    // `T & {}` for a constraint with an `undefined` member, which has more
    // values.
    #[test]
    fn type_parameters_narrow_by_their_constraints_and_other_members() {
        let source = format!(
            "{DECLARATIONS}{}",
            r#"
            function e1<T extends string | null | undefined>(x: T) { if (x !== null) p([/*L8_76*/x] as const); } // T & {}
            function e2<T extends string | undefined>(x: T) { if (x !== null) p([/*L9_69*/x] as const); } // T
            function e3<T>(x: T | undefined) { if (x !== null) p([/*L10_54*/x] as const); } // (T & {}) | undefined
            function e4<T>(x: T | null) { if (x !== undefined) p([/*L11_54*/x] as const); } // (T & {}) | null
            function e5<T extends number>(x: T) { if (typeof x === "string") p([/*L12_68*/x] as const); else p([/*L12_90*/x] as const); } // never; T
            function e6<T extends string>(x: T) { if (typeof x === "string") p([/*L13_68*/x] as const); else p([/*L13_90*/x] as const); } // T; never
            function e7<T extends object>(x: T) { if (x != null) p([/*L14_56*/x] as const); else p([/*L14_78*/x] as const); } // T; never
            function e12<T>(x: T) { if (x == null) p([/*L19_42*/x] as const); else p([/*L19_64*/x] as const); } // T; NonNullable<T>
            function e13<T>(x: T) { if (x === undefined) p([/*L20_48*/x] as const); else p([/*L20_70*/x] as const); } // T; T & ({} | null)
            function e14<T extends string | number>(x: T) { if (typeof x === "object") p([/*L21_78*/x] as const); else p([/*L21_100*/x] as const); } // never; T
            function e15<T>(x: T | null) { if (x) p([/*L22_41*/x] as const); else p([/*L22_63*/x] as const); } // NonNullable<T>; T | null
            function e21<T>(x: T) { if (typeof x === "boolean") p([/*L28_55*/x] as const); } // T & boolean
            function e24<T>(x: T | undefined) { if (typeof x !== "undefined") p([/*L31_69*/x] as const); } // T & ({} | null)
            function e25<T>(x: T) { if (x !== null && x !== undefined) p([/*L32_62*/x] as const); } // T & {}
            function e26<T>(x: T) { if (x !== undefined) { if (x !== null) p([/*L33_66*/x] as const); } } // T & {}
            function e27<T>(x: T) { if (typeof x === "object") { if (x) p([/*L34_63*/x] as const); } } // T & object
            "#
        );
        assert_reads(
            &source,
            &[
                ("L8_76", "x", &[T_EMPTY, "instanceof T & undefined"]),
                ("L9_69", "x", &["instanceof T"]),
                ("L10_54", "x", &[T_EMPTY, "undefined"]),
                ("L11_54", "x", &[T_EMPTY, "null"]),
                ("L12_68", "x", &["instanceof T"]),
                ("L12_90", "x", &["instanceof T"]),
                ("L13_68", "x", &["instanceof T"]),
                ("L13_90", "x", &["instanceof T"]),
                ("L14_56", "x", &["instanceof T"]),
                ("L14_78", "x", &["instanceof T"]),
                ("L19_42", "x", &["instanceof T"]),
                ("L19_64", "x", &[T_EMPTY]),
                ("L20_48", "x", &["instanceof T"]),
                ("L20_70", "x", &[T_EMPTY, "instanceof T & null"]),
                ("L21_78", "x", &["instanceof T"]),
                ("L21_100", "x", &["instanceof T"]),
                ("L22_41", "x", &[T_EMPTY]),
                ("L22_63", "x", &["instanceof T", "null"]),
                ("L28_55", "x", &["instanceof T & boolean"]),
                ("L31_69", "x", &[T_EMPTY, "instanceof T & null"]),
                ("L32_62", "x", &[T_EMPTY]),
                ("L33_66", "x", &[T_EMPTY]),
                ("L34_63", "x", &["instanceof T & object"]),
            ],
        );
    }

    // TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the
    // comment of its line, in order, except where a comment says otherwise. A
    // type predicate or `instanceof` gives a type parameter the intersection
    // with its target when no other member relates to the target, and keeps it
    // in the `else` branch. A `NonNullable` target takes the call's type
    // argument.
    #[test]
    fn type_predicates_and_instanceof_intersect_type_parameters() {
        let source = format!(
            "{DECLARATIONS}{}",
            r#"
            function g20<T>(x: T) { if (isFoo(x)) p([/*L32_41*/x] as const); else p([/*L32_63*/x] as const); } // T & Foo; T
            function g21<T>(x: T) { if (x instanceof A) p([/*L33_47*/x] as const); else p([/*L33_69*/x] as const); } // T & A; T
            function g22<T>(x: T) { if (isString(x)) p([/*L34_44*/x] as const); } // T & string
            function g23<T>(x: T) { if (isNonNull(x)) p([/*L35_45*/x] as const); } // NonNullable<T>
            function g24<T>(x: T) { assertNonNull(x); p([/*L36_45*/x] as const); } // NonNullable<T>
            function g25<T extends Foo | Bar>(x: T) { if (isFoo(x)) p([/*L37_59*/x] as const); } // T & Foo
            function g26<T>(x: T | string) { if (typeof x === "string") p([/*L38_63*/x] as const); else p([/*L38_85*/x] as const); } // string | (T & string); T
            function g27<T>(x: T[] | T) { if (Array.isArray(x)) p([/*L39_55*/x] as const); } // T[]
            function n1(x: string | null) { if (isNonNull(x)) p([/*L41_53*/x] as const); else p([/*L41_75*/x] as const); } // string; null
            function n2(x: string | null | undefined) { assertNonNull(x); p([/*L42_65*/x] as const); } // string
            function n3(x: Foo | undefined) { if (isNonNull(x)) p([/*L43_55*/x] as const); } // Foo
            function e8<T>(x: T | string) { if (isFoo(x)) p([/*e8*/x] as const); else p([/*e8Else*/x] as const); } // T & Foo; string | T
            function e9<T extends Bar>(x: T) { if (isFoo(x)) p([/*e9*/x] as const); } // T & Foo
            function e10<T>(x: T | B) { if (x instanceof A) p([/*e10*/x] as const); else p([/*e10Else*/x] as const); } // T & A; T | B
            function e11<T extends Foo>(x: T) { if (isFoo(x)) p([/*e11*/x] as const); else p([/*e11Else*/x] as const); } // T; never
            function e16<T>(x: T | Foo) { if (isFoo(x)) p([/*e16*/x] as const); else p([/*e16Else*/x] as const); } // Foo; T
            "#
        );
        assert_reads(
            &source,
            &[
                ("L32_41", "x", &[T_FOO]),
                ("L32_63", "x", &["instanceof T"]),
                ("L33_47", "x", &["instanceof T & instanceof A"]),
                ("L33_69", "x", &["instanceof T"]),
                ("L34_44", "x", &["instanceof T & string"]),
                ("L35_45", "x", &[T_EMPTY]),
                ("L36_45", "x", &[T_EMPTY]),
                ("L37_59", "x", &[T_FOO]),
                ("L38_63", "x", &["instanceof T & string", "string"]),
                ("L38_85", "x", &["instanceof T"]),
                ("L39_55", "x", &["instanceof Array<instanceof T>"]),
                ("L41_53", "x", &["string"]),
                // TypeScript gives `null`. Biome keeps the declared type,
                // which is wider: `T` comes from an argument that a test
                // Biome doesn't decompose could narrow, so it may be wider
                // than TypeScript's, and the call narrows only where it is
                // truthy.
                ("L41_75", "x", &["null", "string"]),
                ("L42_65", "x", &["string"]),
                ("L43_55", "x", &["Foo"]),
                ("e8", "x", &[T_FOO]),
                ("e8Else", "x", &["instanceof T", "string"]),
                ("e9", "x", &[T_FOO]),
                ("e10", "x", &["instanceof T & instanceof A"]),
                ("e10Else", "x", &["instanceof B", "instanceof T"]),
                ("e11", "x", &["instanceof T"]),
                ("e11Else", "x", &["Unknown"]),
                ("e16", "x", &["Foo"]),
                ("e16Else", "x", &["instanceof T"]),
            ],
        );
    }

    // TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the
    // comment of its line, in order. When a type predicate or `instanceof`
    // leaves no member, the result is the target if it is a subtype of the
    // type, the type if it is assignable to the target, the target if it is
    // assignable to the type, and otherwise their intersection.
    // Normalization merges the operands of `Bar & Foo`, `Bar & A`, and
    // `B & A` into one anonymous type each, which has the same values.
    #[test]
    fn tests_that_leave_no_member_give_intersections() {
        let source = format!(
            "{DECLARATIONS}{}",
            r#"
            function i1(x: string | number) { if (isFoo(x)) p([/*L46_51*/x] as const); else p([/*L46_73*/x] as const); } // (string | number) & Foo; string | number
            function i2(x: Bar) { if (isFoo(x)) p([/*L47_39*/x] as const); else p([/*L47_61*/x] as const); } // Bar & Foo; Bar
            function i3(x: Bar) { if (x instanceof A) p([/*L48_45*/x] as const); else p([/*L48_67*/x] as const); } // Bar & A; Bar
            function i4(x: B) { if (x instanceof A) p([/*L49_43*/x] as const); else p([/*L49_65*/x] as const); } // B & A; B
            function i5(x: { a: number } | string) { if (x instanceof A) p([/*L50_64*/x] as const); } // A
            function i7(x: Bar | B) { if (isFoo(x)) p([/*L52_43*/x] as const); } // (B | Bar) & Foo
            function i8(x: object) { if (isFoo(x)) p([/*L53_42*/x] as const); } // Foo
            function i9(x: {}) { if (isFoo(x)) p([/*L54_38*/x] as const); } // Foo
            function u1(x: Bar) { if (isFoo(x)) p([/*L78_a*/x.a, /*L78_c*/x.c] as const); } // number, number
            function u2<T>(x: T) { if (typeof x === "string") p([/*L79_53*/x.length] as const); } // number
            "#
        );
        assert_reads(
            &source,
            &[
                // Biome narrows a type without an object type only by a
                // target that is not an object type, so it keeps the type of
                // `x`.
                ("L46_51", "x", &["number", "string"]),
                ("L46_73", "x", &["number", "string"]),
                ("L47_39", "x", &["(merged)"]),
                ("L47_61", "x", &["Bar"]),
                ("L48_45", "x", &[MERGED_CLASS]),
                ("L48_67", "x", &["Bar"]),
                ("L49_43", "x", &[MERGED_CLASS]),
                ("L49_65", "x", &["instanceof B"]),
                ("L50_64", "x", &["instanceof A"]),
                ("L52_43", "x", &["(merged)", MERGED_CLASS]),
                ("L53_42", "x", &["Foo"]),
                ("L54_38", "x", &["Foo"]),
                ("L78_a", "x.a", &["number"]),
                ("L78_c", "x.c", &["number"]),
                ("L79_53", "x.length", &["number"]),
            ],
        );
    }

    // TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the
    // comment of its line, in order. When no member of the type can have the
    // property, `in` gives the intersection with `Record<"a", unknown>`,
    // which Biome writes as an object type with the property `a: unknown`
    // and merges with object type operands.
    #[test]
    fn in_intersects_with_a_property_that_no_member_has() {
        let source = format!(
            "{DECLARATIONS}{}",
            r#"
            function r1(x: object) { if ("a" in x) p([/*L56_42*/x] as const); else p([/*L56_64*/x] as const); } // object & Record<"a", unknown>; object
            function r2(x: { b: 1 } | { c: 2 }) { if ("a" in x) p([/*L57_55*/x] as const); else p([/*L57_77*/x] as const); } // ({ b: 1 } | { c: 2 }) & Record<"a", unknown>; { b: 1 } | { c: 2 }
            function r3(x: { a: 1 } | { c: 2 }) { if ("a" in x) p([/*L58_55*/x] as const); } // { a: 1 }
            function r4(x: { b: 1 }) { if ("a" in x) p([/*L59_44*/x] as const); else p([/*L59_66*/x] as const); } // { b: 1 } & Record<"a", unknown>; { b: 1 }
            function r5(x: Foo) { if ("z" in x) p([/*L60_39*/x.z] as const); } // unknown
            function r6<T extends object>(x: T) { if ("a" in x) p([/*L61_55*/x] as const); } // T & Record<"a", unknown>
            function r7(x: unknown) { if (typeof x === "object" && x !== null && "a" in x) p([/*L62_82*/x] as const); } // object & Record<"a", unknown>
            function r8(x: { b: 1 } | string[]) { if ("a" in x) p([/*L63_55*/x] as const); } // (string[] | { b: 1 }) & Record<"a", unknown>
            function r9(x: A | { a: 1 }) { if ("b" in x) p([/*L64_48*/x] as const); } // (A | { a: 1 }) & Record<"b", unknown>
            function e17<T extends { a: 1 }>(x: T) { if ("a" in x) p([/*e17*/x] as const); } // T
            function e18<T extends { b: 1 }>(x: T) { if ("a" in x) p([/*e18*/x] as const); else p([/*e18Else*/x] as const); } // T & Record<"a", unknown>; T
            function e19(x: object) { if ("toString" in x) p([/*e19*/x] as const); } // object
            function e20(x: {}) { if ("a" in x) p([/*e20*/x] as const); else p([/*e20Else*/x] as const); } // Record<"a", unknown>; {}
            "#
        );
        let record =
            |members: &str| format!("Object {{ prototype: No prototype members: [{members}] }}");
        let a = record(r#""a": unknown"#);
        let object_a = format!("object & {a}");
        let b_a = record(r#""b": number: 1, "a": unknown"#);
        let c_a = record(r#""c": number: 2, "a": unknown"#);
        let t_a = format!("instanceof T & {a}");
        assert_reads(
            &source,
            &[
                ("L56_42", "x", &[&object_a]),
                ("L56_64", "x", &["object"]),
                ("L57_55", "x", &[&b_a, &c_a]),
                (
                    "L57_77",
                    "x",
                    &[
                        r#"Object { prototype: No prototype members: ["b": number: 1] }"#,
                        r#"Object { prototype: No prototype members: ["c": number: 2] }"#,
                    ],
                ),
                (
                    "L58_55",
                    "x",
                    &[r#"Object { prototype: No prototype members: ["a": number: 1] }"#],
                ),
                ("L59_44", "x", &[&b_a]),
                (
                    "L59_66",
                    "x",
                    &[r#"Object { prototype: No prototype members: ["b": number: 1] }"#],
                ),
                ("L60_39", "x.z", &["unknown"]),
                ("L61_55", "x", &[&t_a]),
                ("L62_82", "x", &[&object_a]),
                // Biome can't tell whether an array or an instance of a class
                // has the property, so it keeps the declared type.
                (
                    "L63_55",
                    "x",
                    &[
                        r#"Object { prototype: No prototype members: ["b": number: 1] }"#,
                        "instanceof Array<string>",
                    ],
                ),
                (
                    "L64_48",
                    "x",
                    &[
                        r#"Object { prototype: No prototype members: ["a": number: 1] }"#,
                        "instanceof A",
                    ],
                ),
                ("e17", "x", &["instanceof T"]),
                (
                    "e18",
                    "x",
                    &[&format!("instanceof T & {}", record(r#""a": unknown"#))],
                ),
                ("e18Else", "x", &["instanceof T"]),
                ("e19", "x", &["object"]),
                ("e20", "x", &[&a]),
                (
                    "e20Else",
                    "x",
                    &["Object { prototype: No prototype members: [] }"],
                ),
            ],
        );
    }

    // TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the
    // comment of its line, in order. Equality keeps an intersection with a
    // type parameter as it is when the intersection can equal the value, as
    // `T & string` for `=== "a"`, and removes it when an operand can't, as
    // `T & string` for `=== null`. `T & boolean` stands for
    // `(T & true) | (T & false)`, so equality keeps the member that can be
    // equal. Biome gives its unknown type to a read where TypeScript gives
    // `never`.
    #[test]
    fn equality_keeps_intersections_with_type_parameters() {
        let source = format!(
            "{DECLARATIONS}{}",
            r#"
            declare function isA(v: unknown): v is "a";
            declare function isNull(v: unknown): v is null;
            function q1<T>(x: T) { if (typeof x === "string") { if (x === "a") p([/*q1*/x] as const); else p([/*q1Else*/x] as const); } } // T & string; T & string
            function q2<T>(x: T) { if (typeof x === "number") { if (x === 1) p([/*q2*/x] as const); else p([/*q2Else*/x] as const); } } // T & number; T & number
            function q3<T>(x: T) { if (typeof x === "bigint") { if (x === 1n) p([/*q3*/x] as const); else p([/*q3Else*/x] as const); } } // T & bigint; T & bigint
            function q4<T>(x: T) { if (typeof x === "boolean") { if (x === true) p([/*q4*/x] as const); else p([/*q4Else*/x] as const); } } // T & true; T & false
            function q5<T>(x: T) { if (typeof x === "string") { if (x !== "a") p([/*q5*/x] as const); else p([/*q5Else*/x] as const); } } // T & string; T & string
            function q6<T>(x: T & string) { if (x === "a") p([/*q6*/x] as const); else p([/*q6Else*/x] as const); } // T & string; T & string
            function q7<T>(x: (T & "a") | (T & "b")) { if (x === "a") p([/*q7*/x] as const); else p([/*q7Else*/x] as const); } // T & "a"; T & "b"
            function q8<T>(x: T & "a") { if (x === "a") p([/*q8*/x] as const); else p([/*q8Else*/x] as const); } // T & "a"; never
            function q9<T>(x: T) { if (x) { if (x === "a") p([/*q9*/x] as const); else p([/*q9Else*/x] as const); } } // NonNullable<T>; NonNullable<T>
            function q10<T>(x: T) { if (x !== undefined) { if (x === "a") p([/*q10*/x] as const); else p([/*q10Else*/x] as const); } } // T & {}; T & ({} | null)
            function q11<T>(x: T) { if (typeof x === "string") { switch (x) { case "a": p([/*q11*/x] as const); break; default: p([/*q11Else*/x] as const); } } } // T & string; T & string
            function q12<T>(x: T) { if (typeof x === "string") { if (x === null) p([/*q12*/x] as const); else p([/*q12Else*/x] as const); } } // never; T & string
            function q13<T>(x: T) { if (typeof x === "string") { if (x === undefined) p([/*q13*/x] as const); else p([/*q13Else*/x] as const); } } // never; T & string
            function q14<T>(x: T) { if (typeof x === "object") { if (x === null) p([/*q14*/x] as const); else p([/*q14Else*/x] as const); } } // T & null; T & object
            function q15<T>(x: T) { if (typeof x === "object") { if (x === undefined) p([/*q15*/x] as const); else p([/*q15Else*/x] as const); } } // never; (T & null) | (T & object)
            function q16<T>(x: T) { if (x !== null) { if (x === undefined) p([/*q16*/x] as const); else p([/*q16Else*/x] as const); } } // T & undefined; T & {}
            function q17<T>(x: T) { if (x) { if (x === null) p([/*q17*/x] as const); else p([/*q17Else*/x] as const); } } // never; NonNullable<T>
            function q18<T>(x: T) { if (typeof x === "string") { if (x == null) p([/*q18*/x] as const); else p([/*q18Else*/x] as const); } } // never; T & string
            function q19<T>(x: T) { if (typeof x === "object") { if (x == null) p([/*q19*/x] as const); else p([/*q19Else*/x] as const); } } // T & null; T & object
            function q20<T>(x: T) { if (typeof x === "string") { if (isA(x)) p([/*q20*/x] as const); else p([/*q20Else*/x] as const); } } // T & "a"; T & string
            function q21<T>(x: T) { if (isA(x)) p([/*q21*/x] as const); else p([/*q21Else*/x] as const); } // T & "a"; T
            function q22<T>(x: T) { if (typeof x === "object") { if (isNull(x)) p([/*q22*/x] as const); else p([/*q22Else*/x] as const); } } // T & null; T & object
            "#
        );
        assert_reads(
            &source,
            &[
                ("q1", "x", &["instanceof T & string"]),
                ("q1Else", "x", &["instanceof T & string"]),
                ("q2", "x", &["instanceof T & number"]),
                ("q2Else", "x", &["instanceof T & number"]),
                ("q3", "x", &["instanceof T & BigInt"]),
                ("q3Else", "x", &["instanceof T & BigInt"]),
                ("q4", "x", &["instanceof T & bool: true"]),
                ("q4Else", "x", &["instanceof T & bool: false"]),
                ("q5", "x", &["instanceof T & string"]),
                ("q5Else", "x", &["instanceof T & string"]),
                ("q6", "x", &["instanceof T & string"]),
                ("q6Else", "x", &["instanceof T & string"]),
                ("q7", "x", &["instanceof T & string: a"]),
                ("q7Else", "x", &["instanceof T & string: b"]),
                ("q8", "x", &["instanceof T & string: a"]),
                ("q8Else", "x", &["Unknown"]),
                ("q9", "x", &[T_EMPTY]),
                ("q9Else", "x", &[T_EMPTY]),
                ("q10", "x", &[T_EMPTY]),
                ("q10Else", "x", &[T_EMPTY, "instanceof T & null"]),
                ("q11", "x", &["instanceof T & string"]),
                ("q11Else", "x", &["instanceof T & string"]),
                ("q12", "x", &["Unknown"]),
                ("q12Else", "x", &["instanceof T & string"]),
                ("q13", "x", &["Unknown"]),
                ("q13Else", "x", &["instanceof T & string"]),
                ("q14", "x", &["instanceof T & null"]),
                ("q14Else", "x", &["instanceof T & object"]),
                ("q15", "x", &["Unknown"]),
                (
                    "q15Else",
                    "x",
                    &["instanceof T & null", "instanceof T & object"],
                ),
                // Biome keeps `T & {}`, which can't be `undefined`.
                ("q16", "x", &[T_EMPTY, "instanceof T & undefined"]),
                ("q16Else", "x", &[T_EMPTY]),
                // Biome keeps `T & {}`, which can't be `null`.
                ("q17", "x", &[T_EMPTY]),
                ("q17Else", "x", &[T_EMPTY]),
                ("q18", "x", &["Unknown"]),
                ("q18Else", "x", &["instanceof T & string"]),
                ("q19", "x", &["instanceof T & null"]),
                ("q19Else", "x", &["instanceof T & object"]),
                // A type predicate with a literal target narrows as equality
                // does, so Biome keeps `T & string` and `T`, which are wider.
                ("q20", "x", &["instanceof T & string"]),
                ("q20Else", "x", &["instanceof T & string"]),
                ("q21", "x", &["instanceof T"]),
                ("q21Else", "x", &["instanceof T"]),
                ("q22", "x", &["instanceof T & null"]),
                ("q22Else", "x", &["instanceof T & object"]),
            ],
        );
    }

    // TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the
    // comment of its line, in order: the read in the case clause, then the
    // read in `default` or after the statement. Where no case of a `switch`
    // on a value or on `typeof` of one matches, TypeScript removes the
    // members of the type that every case value or kind covers and keeps the
    // others as they are, so a type parameter stays whole unless its
    // constraint leaves it no value that passes. An `if` with the same test
    // gives an intersection. Biome keeps `T` where TypeScript gives `never`.
    #[test]
    fn switch_defaults_keep_type_parameters() {
        let source = format!(
            "{DECLARATIONS}{}",
            r#"
            function f1<T>(x: T) { switch (x) { case null: case undefined: p([/*f1*/x] as const); break; default: p([/*f1Default*/x] as const); } } // T; T
            function f2<T>(x: T) { switch (typeof x) { case "undefined": p([/*f2*/x] as const); break; default: p([/*f2Default*/x] as const); } } // T & undefined; T
            function f3<T>(x: T | null) { switch (x) { case null: p([/*f3*/x] as const); break; default: p([/*f3Default*/x] as const); } } // T | null; T
            function f4<T>(x: T) { switch (x) { case null: break; } p([/*f4After*/x] as const); } // T
            function f5<T>(x: T) { switch (x) { case undefined: p([/*f5*/x] as const); break; default: p([/*f5Default*/x] as const); } } // T; T
            function f6<T>(x: T) { switch (x) { case null: p([/*f6*/x] as const); break; default: p([/*f6Default*/x] as const); } } // T; T
            function f7<T>(o: { v: T }) { switch (o.v) { case null: p([/*f7*/o.v] as const); break; default: p([/*f7Default*/o.v] as const); } } // T; T
            function f8<T>(x: T | undefined) { switch (typeof x) { case "undefined": p([/*f8*/x] as const); break; default: p([/*f8Default*/x] as const); } } // (T & undefined) | undefined; T
            function f9<T>(x: T) { if (x !== undefined) { switch (x) { case null: p([/*f9*/x] as const); break; default: p([/*f9Default*/x] as const); } } } // T & null; T & {}
            function f10<T extends string | undefined>(x: T) { switch (typeof x) { case "undefined": p([/*f10*/x] as const); break; default: p([/*f10Default*/x] as const); } } // T & undefined; T
            function f11<T>(x: T | string) { switch (typeof x) { case "string": p([/*f11*/x] as const); break; default: p([/*f11Default*/x] as const); } } // string | (T & string); T
            function f12<T>(x: T) { switch (typeof x) { case "undefined": return; } p([/*f12After*/x] as const); } // T
            function f13<T>(x: T | null) { switch (x) { case null: return; } p([/*f13After*/x] as const); } // T
            function f14<T extends null>(x: T) { switch (x) { case null: p([/*f14*/x] as const); break; default: p([/*f14Default*/x] as const); } } // T; never
            function f15<T>(x: T | null | undefined) { switch (x) { case null: case undefined: p([/*f15*/x] as const); break; default: p([/*f15Default*/x] as const); } } // T | null | undefined; T
            "#
        );
        assert_reads(
            &source,
            &[
                ("f1", "x", &["instanceof T"]),
                ("f1Default", "x", &["instanceof T"]),
                ("f2", "x", &["instanceof T & undefined"]),
                ("f2Default", "x", &["instanceof T"]),
                ("f3", "x", &["instanceof T", "null"]),
                ("f3Default", "x", &["instanceof T"]),
                ("f4After", "x", &["instanceof T"]),
                ("f5", "x", &["instanceof T"]),
                ("f5Default", "x", &["instanceof T"]),
                ("f6", "x", &["instanceof T"]),
                ("f6Default", "x", &["instanceof T"]),
                ("f7", "o.v", &["instanceof T"]),
                ("f7Default", "o.v", &["instanceof T"]),
                ("f8", "x", &["instanceof T & undefined", "undefined"]),
                ("f8Default", "x", &["instanceof T"]),
                // `T & null`: as at `L81_59`, Biome removes no `T & {}`
                // where the value equals `null`.
                ("f9", "x", &[T_EMPTY, "instanceof T & null"]),
                ("f9Default", "x", &[T_EMPTY]),
                ("f10", "x", &["instanceof T & undefined"]),
                ("f10Default", "x", &["instanceof T"]),
                ("f11", "x", &["instanceof T & string", "string"]),
                ("f11Default", "x", &["instanceof T"]),
                ("f12After", "x", &["instanceof T"]),
                ("f13After", "x", &["instanceof T"]),
                ("f14", "x", &["instanceof T"]),
                ("f14Default", "x", &["instanceof T"]),
                ("f15", "x", &["instanceof T", "null", "undefined"]),
                ("f15Default", "x", &["instanceof T"]),
            ],
        );
    }

    // TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the
    // comment of its line, in order, except where a comment says otherwise.
    // Loose equality and equality with a value of a declared type keep
    // `T & string` and `T & boolean` whole or remove them, as strict equality
    // with a literal does, except that the members of `T & boolean`,
    // `(T & true) | (T & false)`, narrow one by one. Biome gives its unknown
    // type to a read where TypeScript gives `never`.
    #[test]
    fn loose_and_typed_equality_keep_intersections_with_type_parameters() {
        let source = format!(
            "{DECLARATIONS}{}",
            r#"
            function r1<T>(x: T) { if (typeof x === "boolean") { if (x == true) p([/*r1*/x] as const); else p([/*r1Else*/x] as const); } } // T & true; T & false
            function r2<T>(x: T) { if (typeof x === "boolean") { if (x == 1) p([/*r2*/x] as const); else p([/*r2Else*/x] as const); } } // never; T & boolean
            function r3<T>(x: T) { if (typeof x === "boolean") { if (x == "a") p([/*r3*/x] as const); else p([/*r3Else*/x] as const); } } // never; T & boolean
            function r4<T>(x: T) { if (typeof x === "boolean") { if (x == null) p([/*r4*/x] as const); else p([/*r4Else*/x] as const); } } // never; T & boolean
            function r5<T>(x: T) { if (typeof x === "string") { if (x == true) p([/*r5*/x] as const); else p([/*r5Else*/x] as const); } } // never; T & string
            function r6<T>(x: T) { if (typeof x === "string") { if (x == 1) p([/*r6*/x] as const); else p([/*r6Else*/x] as const); } } // never; T & string
            function r7<T>(x: T) { if (typeof x === "string") { if (x == "a") p([/*r7*/x] as const); else p([/*r7Else*/x] as const); } } // T & string; T & string
            function r8<T>(x: T) { if (typeof x === "string") { if (x == null) p([/*r8*/x] as const); else p([/*r8Else*/x] as const); } } // never; T & string
            function r9<T>(x: T, y: true) { if (typeof x === "boolean") { if (x === y) p([/*r9*/x] as const); else p([/*r9Else*/x] as const); } } // T & true; T & false
            function r10<T>(x: T, y: boolean) { if (typeof x === "boolean") { if (x === y) p([/*r10*/x] as const); else p([/*r10Else*/x] as const); } } // T & boolean; T & boolean
            function r11<T>(x: T, y: "a" | "b") { if (typeof x === "boolean") { if (x === y) p([/*r11*/x] as const); else p([/*r11Else*/x] as const); } } // never; T & boolean
            function r12<T>(x: T, y: true) { if (typeof x === "string") { if (x === y) p([/*r12*/x] as const); else p([/*r12Else*/x] as const); } } // never; T & string
            function r13<T>(x: T, y: boolean) { if (typeof x === "string") { if (x === y) p([/*r13*/x] as const); else p([/*r13Else*/x] as const); } } // never; T & string
            function r14<T>(x: T, y: "a" | "b") { if (typeof x === "string") { if (x === y) p([/*r14*/x] as const); else p([/*r14Else*/x] as const); } } // T & string; T & string
            function r15<T>(x: T) { if (typeof x === "boolean") { if (x == 0) p([/*r15*/x] as const); else p([/*r15Else*/x] as const); } } // never; T & boolean
            function r16<T>(x: T) { if (typeof x === "boolean") { if (x != true) p([/*r16*/x] as const); else p([/*r16Else*/x] as const); } } // T & false; T & true
            function r17<T>(x: T, y: true) { if (typeof x === "boolean") { if (x == y) p([/*r17*/x] as const); else p([/*r17Else*/x] as const); } } // T & true; T & false
            function r18<T>(x: T, y: "a" | "b") { if (typeof x === "boolean") { if (x == y) p([/*r18*/x] as const); else p([/*r18Else*/x] as const); } } // never; T & boolean
            function r19<T>(x: T, y: true) { if (typeof x === "string") { if (x == y) p([/*r19*/x] as const); else p([/*r19Else*/x] as const); } } // never; T & string
            function r20<T>(x: T, y: "a" | "b") { if (typeof x === "string") { if (x == y) p([/*r20*/x] as const); else p([/*r20Else*/x] as const); } } // T & string; T & string
            function r21<T>(x: T, y: { a: 1 }) { if (typeof x === "string") { if (x === y) p([/*r21*/x] as const); else p([/*r21Else*/x] as const); } } // never; T & string
            function r22<T>(x: T, y: { a: 1 }) { if (typeof x === "boolean") { if (x === y) p([/*r22*/x] as const); else p([/*r22Else*/x] as const); } } // never; T & boolean
            function r23<T>(x: T) { if (typeof x === "string") { if (x === 1) p([/*r23*/x] as const); else p([/*r23Else*/x] as const); } } // never; T & string
            function r24<T>(x: T) { if (typeof x === "boolean") { if (x === "a") p([/*r24*/x] as const); else p([/*r24Else*/x] as const); } } // never; T & boolean
            "#
        );
        assert_reads(
            &source,
            &[
                ("r1", "x", &["instanceof T & bool: true"]),
                ("r1Else", "x", &["instanceof T & bool: false"]),
                // Deliberate choice: loose equality keeps primitive types of
                // other kinds, so Biome keeps the intersection, which is wider.
                ("r2", "x", &["instanceof T & boolean"]),
                ("r2Else", "x", &["instanceof T & boolean"]),
                // Deliberate choice: loose equality keeps primitive types of
                // other kinds, so Biome keeps the intersection, which is wider.
                ("r3", "x", &["instanceof T & boolean"]),
                ("r3Else", "x", &["instanceof T & boolean"]),
                ("r4", "x", &["Unknown"]),
                ("r4Else", "x", &["instanceof T & boolean"]),
                // Deliberate choice: loose equality keeps primitive types of
                // other kinds, so Biome keeps the intersection, which is wider.
                ("r5", "x", &["instanceof T & string"]),
                ("r5Else", "x", &["instanceof T & string"]),
                // Deliberate choice: loose equality keeps primitive types of
                // other kinds, so Biome keeps the intersection, which is wider.
                ("r6", "x", &["instanceof T & string"]),
                ("r6Else", "x", &["instanceof T & string"]),
                ("r7", "x", &["instanceof T & string"]),
                ("r7Else", "x", &["instanceof T & string"]),
                ("r8", "x", &["Unknown"]),
                ("r8Else", "x", &["instanceof T & string"]),
                ("r9", "x", &["instanceof T & bool: true"]),
                ("r9Else", "x", &["instanceof T & bool: false"]),
                ("r10", "x", &["instanceof T & boolean"]),
                ("r10Else", "x", &["instanceof T & boolean"]),
                ("r11", "x", &["Unknown"]),
                ("r11Else", "x", &["instanceof T & boolean"]),
                ("r12", "x", &["Unknown"]),
                ("r12Else", "x", &["instanceof T & string"]),
                ("r13", "x", &["Unknown"]),
                ("r13Else", "x", &["instanceof T & string"]),
                ("r14", "x", &["instanceof T & string"]),
                ("r14Else", "x", &["instanceof T & string"]),
                // Deliberate choice: loose equality keeps primitive types of
                // other kinds, so Biome keeps the intersection, which is wider.
                ("r15", "x", &["instanceof T & boolean"]),
                ("r15Else", "x", &["instanceof T & boolean"]),
                ("r16", "x", &["instanceof T & bool: false"]),
                ("r16Else", "x", &["instanceof T & bool: true"]),
                ("r17", "x", &["instanceof T & bool: true"]),
                ("r17Else", "x", &["instanceof T & bool: false"]),
                ("r18", "x", &["Unknown"]),
                ("r18Else", "x", &["instanceof T & boolean"]),
                // Deliberate choice: loose equality keeps primitive types of
                // other kinds, so Biome keeps the intersection, which is wider.
                ("r19", "x", &["instanceof T & string"]),
                ("r19Else", "x", &["instanceof T & string"]),
                ("r20", "x", &["instanceof T & string"]),
                ("r20Else", "x", &["instanceof T & string"]),
                // Biome keeps a primitive compared with a value of an object
                // type literal, such as `{ a: 1 }`, which it takes every value
                // except `null` and `undefined` to possibly have, so it keeps
                // the intersection, which is wider.
                ("r21", "x", &["instanceof T & string"]),
                ("r21Else", "x", &["instanceof T & string"]),
                // Biome keeps a primitive compared with a value of an object
                // type literal, such as `{ a: 1 }`, which it takes every value
                // except `null` and `undefined` to possibly have, so it keeps
                // the intersection, which is wider.
                ("r22", "x", &["instanceof T & boolean"]),
                ("r22Else", "x", &["instanceof T & boolean"]),
                ("r23", "x", &["Unknown"]),
                ("r23Else", "x", &["instanceof T & string"]),
                ("r24", "x", &["Unknown"]),
                ("r24Else", "x", &["instanceof T & boolean"]),
            ],
        );
    }

    // TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the
    // comment of its line, in order. A `this is T` or `asserts this is T`
    // predicate whose target the class doesn't relate to gives the
    // intersection, which normalization merges into an instance of an
    // anonymous class, and `every` gives a tuple its intersection with the
    // array of the target. Biome types `this` as an instance of the class,
    // where TypeScript writes `this`.
    #[test]
    fn this_predicates_and_every_on_tuples_give_intersections() {
        let source = format!(
            "{DECLARATIONS}{}",
            r#"
            class R1 { ready?: string; check(): this is { ready: string } { return true; } m() { if (this.check()) p([/*L66_106*/this] as const); } } // this & { ready: string }
            class R2 { ready?: string; assertReady(): asserts this is { ready: string } {} m() { this.assertReady(); p([/*L67_108*/this] as const); } } // this & { ready: string }
            class R3 { ready?: string; assertReady(): asserts this is { ready: string } {} m() { this.assertReady(); p([/*L68_108*/this.ready] as const); } } // string
            class T1 { isTagged(): this is { tag: 1 } { return false; } }
            function t1(n: T1) { if (n.isTagged()) p([/*L70_42*/n] as const); else p([/*L70_64*/n] as const); } // T1 & { tag: 1 }; T1
            function t2(n: T1) { if (n.isTagged()) p([/*L71_42*/n.tag] as const); } // 1
            function e1(t: [string | number, string | number]) { if (t.every(isString)) p([/*L73_79*/t] as const); } // [string | number, string | number] & string[]
            function e2(t: readonly [string | number]) { if (t.every(isString)) p([/*L74_71*/t] as const); } // readonly [string | number] & readonly string[]
            function e3(t: [string, number]) { if (t.every(isString)) p([/*L75_61*/t] as const); } // [string, number] & string[]
            function e4(t: [string | number, string | number]) { if (t.every(isString)) p([/*L76_79*/t[0]] as const); } // string | number
            "#
        );
        assert_reads(
            &source,
            &[
                ("L66_106", "this", &[MERGED_CLASS]),
                ("L67_108", "this", &[MERGED_CLASS]),
                ("L68_108", "this.ready", &["string"]),
                ("L70_42", "n", &[MERGED_CLASS]),
                ("L70_64", "n", &["instanceof T1"]),
                ("L71_42", "n.tag", &["number: 1"]),
                (
                    "L73_79",
                    "t",
                    &["[string | number, string | number] & instanceof Array<string>"],
                ),
                (
                    "L74_71",
                    "t",
                    &["Readonly [string | number] & Readonly instanceof Array<string>"],
                ),
                (
                    "L75_61",
                    "t",
                    &["[string, number] & instanceof Array<string>"],
                ),
                ("L76_79", "t[0]", &["number", "string"]),
            ],
        );
    }
}
