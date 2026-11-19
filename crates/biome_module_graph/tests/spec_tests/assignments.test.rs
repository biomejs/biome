use super::*;
use biome_module_graph::{
    ExpressionTypeInput, infer_expression_type,
    type_inference::{
        NormalizedExpressionTypeRequest, TypeInferenceCaller, execute_type_inference_request,
    },
};

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

fn narrowing_db(source: &str) -> (TestModuleDb, ModuleInfo) {
    narrowing_db_with(source, &[])
}

/// Builds `/src/index.ts` from `source`, with each `(path, source)` of
/// `modules` as another module it can import.
fn narrowing_db_with(source: &str, modules: &[(&str, &str)]) -> (TestModuleDb, ModuleInfo) {
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

/// Names each member of `ty`, sorted: an interface by its name, an object
/// type literal by its members, Biome's Unknown as `Unknown`, and any other
/// type by its formatted text on one line, such as `string`, `null`, or
/// `string: a`. A read on a path that no value takes is Unknown.
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
            if first.starts_with("Object")
                && let Some(start) = lines.iter().position(|line| line.starts_with("members: "))
            {
                // Skip `members: ` and the closing brace of the object.
                let members = lines[start..lines.len() - 1].join(" ");
                return members["members: ".len()..].to_owned();
            }
            lines.join(" ")
        })
        .collect();
    names.sort();
    names
}

/// Declarations shared by the tests, as interfaces, so that a read names each
/// of its members.
const DECLARATIONS: &str = r#"
    declare function f<T>(): T;
    declare const c: boolean;
    declare function g(): void;
    interface Circle { kind: "c"; r: number }
    interface Square { kind: "s"; w: number }
"#;

/// Asserts the type of each `(marker, expression, expected)` read in `source`,
/// which follows [`DECLARATIONS`], by the names of its members, as
/// [`variant_names`] gives them, and that complete module inference gives
/// each read the type that a lookup gives it.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    assert_reads_with(source, &[], reads);
}

/// Asserts the reads in `source` as [`assert_reads`] does, with `modules`
/// beside it, as [`narrowing_db_with`] takes them.
fn assert_reads_with(source: &str, modules: &[(&str, &str)], reads: &[(&str, &str, &[&str])]) {
    let source = format!("{DECLARATIONS}{source}");
    let lookups: Vec<_> = {
        let (db, module) = narrowing_db_with(&source, modules);
        reads
            .iter()
            .map(|(marker, expression, _)| {
                let range = marked_range(&source, marker, expression);
                infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                    .map(|ty| format_inferred_type(&db, ty))
            })
            .collect()
    };
    let (db, module) = narrowing_db_with(&source, modules);
    let complete = infer_module_types(&db, module).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .zip(lookups)
        .flat_map(|((marker, expression, expected), lookup)| {
            let range = marked_range(&source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "assignments"),
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// with `never` where Biome gives Unknown. An assignment keeps the members of a
// declared union that the assigned value may have, and keeps the declared
// type when it isn't a union, when every member stays, and when the value is
// `any`.
#[test]
fn assignments_narrow_to_the_members_of_the_declared_union() {
    assert_reads(
        r#"
        function a1() { let x: string | number = "a"; /*initializer*/x; }
        function a2() { let x: string | number; x = 1; /*assigned*/x; }
        function a3() { let x: "a" | "b" | "c" = "a"; /*literal*/x; }
        function a4() { const x: string | number = 1; /*constInitializer*/x; }
        function a5() { let x: string | null = f(); if (x) { x = null; /*afterTest*/x; } }
        function a6() { let x: string | null = f(); if (x === null) x = "d"; /*joined*/x; }
        function a7() { let x: unknown = 1; /*notUnion*/x; }
        function a9() { let x: string | number = f<string | number>(); /*wholeUnion*/x; }
        function a10() { let x: string | number = f<any>(); /*any*/x; }
        function a14() { let s: Circle | Square = { kind: "c", r: 1 }; /*object*/s; }
        function a15() { let x: string | number | boolean = c ? "a" : 1; /*conditional*/x; }
        function a16() { let x: Date | string = new Date(); /*instance*/x; }
        function a18(x: string | null) { x = "a"; /*parameter*/x; }
        function a19() { let x: string | number; const y = (x = 1); /*nestedAssignment*/x; }
        function a21() { let x: string | number = f<string | number>(); x = 1; if (typeof x === "string") /*impossible*/x; }
        function a22() { let x: string | number = "a"; if (c) x = 1; /*branches*/x; }
        function a24() { let x: { a: 1 } | { b: 2 } | null = { a: 1 }; /*objectLiteral*/x; }
        function a25() { let x: string | number = "a"; /*first*/x; x = 1; /*second*/x; }
        function a26() { let x: number | undefined = 1; /*undefinedRemoved*/x; }
        function a27() { let x: string | number = 1 as 1 | "s"; /*asserted*/x; }
        function a28() { let x: number | bigint = 1n; /*bigint*/x; }
        function a33() { let x: Circle | Square = f<Circle>(); /*member*/x; }
        function a35() { let x: string | number = f<never>(); /*never*/x; }
        function a36() { let x: string | number = f<unknown>() as string; /*cast*/x; }
        function a38() { let x: { k: "a" } | { k: "b" } = { k: "a" }; /*discriminant*/x; }
        function a41() { let s = c ? "a" : 1; s = "b"; /*unannotated*/s; }
        function a42() { let i = 0; i++; /*widened*/i; }
        "#,
        &[
            ("initializer", "x", &["string"]),
            ("assigned", "x", &["number"]),
            ("literal", "x", &["string: a"]),
            ("constInitializer", "x", &["number"]),
            ("afterTest", "x", &["null"]),
            ("joined", "x", &["string"]),
            ("notUnion", "x", &["unknown"]),
            ("wholeUnion", "x", &["number", "string"]),
            ("any", "x", &["number", "string"]),
            ("object", "s", &["Circle"]),
            ("conditional", "x", &["number", "string"]),
            ("instance", "x", &["instanceof Date"]),
            ("parameter", "x", &["string"]),
            ("nestedAssignment", "x", &["number"]),
            ("impossible", "x", &["Unknown"]),
            ("branches", "x", &["number", "string"]),
            ("objectLiteral", "x", &["[\"a\": number: 1]"]),
            ("first", "x", &["string"]),
            ("second", "x", &["number"]),
            ("undefinedRemoved", "x", &["number"]),
            ("asserted", "x", &["number", "string"]),
            ("bigint", "x", &["BigInt"]),
            ("member", "x", &["Circle"]),
            ("never", "x", &["Unknown"]),
            ("cast", "x", &["string"]),
            ("discriminant", "x", &["[\"k\": string: a]"]),
            // TypeScript widens the literal types of a `let` without an
            // annotation, as Biome does.
            ("unannotated", "s", &["string"]),
            ("widened", "i", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A `true` or `false` literal keeps only its own value of a `boolean` member
// of the declared union. A value that is `true` or `false` by another
// expression, as a comparison, keeps `boolean` whole, because Biome types
// some comparisons, as `1 === 1`, as `true` where TypeScript gives `boolean`.
#[test]
fn boolean_literals_reduce_boolean_members_of_the_declared_union() {
    assert_reads(
        r#"
        function a17() { let b: boolean | undefined = true; /*literal*/b; }
        function a17b() { let b: boolean | undefined; b = false; /*assigned*/b; }
        function a17d() { let b: boolean | string = (true); /*parenthesized*/b; }
        function a17e() { let b: boolean | undefined = 1 === 1; /*comparison*/b; }
        function a17i() { let b: boolean | null = null; b = true; if (c) b = false; /*both*/b; }
        function a17k(o: { b: boolean | undefined }) { o.b = true; /*property*/o.b; }
        "#,
        &[
            ("literal", "b", &["bool: true"]),
            ("assigned", "b", &["bool: false"]),
            ("parenthesized", "b", &["bool: true"]),
            ("comparison", "b", &["boolean"]),
            ("both", "b", &["boolean"]),
            ("property", "o.b", &["bool: true"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives the narrower type in the comment for
// each read below. Biome is wider: it keeps a declared `boolean` that isn't
// in a union, and `boolean` for a value other than a `true` or `false`
// literal, types the value of an assignment expression and an empty array as
// Unknown, types a right operand without narrowing, gives a target behind
// `as` its declared type, where TypeScript ignores the write, and doesn't
// reduce by a call to a method without a return type annotation, even one
// that always returns, or to an overloaded method whose implementation has
// none, which TypeScript never calls through.
#[test]
fn assignments_that_biome_reduces_less_than_typescript() {
    assert_reads(
        r#"
        function a17c() { let b: boolean = true; /*boolean*/b; }
        function a17l() { const t = true; let b: boolean | undefined = t; /*constant*/b; }
        function a20() { let x: string | number, y: string | number; x = y = 1; /*chained*/x; }
        function a30() { let x: string[] | string = []; /*emptyArray*/x; }
        function a40() { let x: string | number = "a"; x = x; /*self*/x; }
        function o8(x: string | null) { let y: string | null = f(); if (x) { y = x; /*narrowedValue*/y; } }
        function o9() { let x: string | number = "a"; (x as any) = 1; /*assertedTarget*/x; }
        class Overloaded { o(): number; o(x: string): number; o(x?: string) { return x ? 1 : 2; } }
        function m4(o: Overloaded) { let x: number | undefined = o.o(); /*overloadedMethod*/x; }
        class K { m() { return 1; } }
        function m5(k: K) { let x: number | undefined = k.m(); /*inferredMethod*/x; }
        "#,
        &[
            // TypeScript: `true`.
            ("boolean", "b", &["boolean"]),
            // TypeScript: `true`.
            ("constant", "b", &["boolean"]),
            // TypeScript: `number`.
            ("chained", "x", &["number", "string"]),
            // TypeScript: `string[]`.
            ("emptyArray", "x", &["instanceof Array<string>", "string"]),
            // TypeScript: `string`.
            ("self", "x", &["number", "string"]),
            // TypeScript: `string`.
            ("narrowedValue", "y", &["null", "string"]),
            // TypeScript: `string`.
            ("assertedTarget", "x", &["number", "string"]),
            // TypeScript: `number`.
            ("overloadedMethod", "x", &["number", "undefined"]),
            // TypeScript: `number`.
            ("inferredMethod", "x", &["number", "undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. An assignment compares the
// value with the declared members by assignability, which can't tell whether
// a value relates to a type parameter, because a call may still replace the
// type parameter, so Biome keeps a member that it can't compare.
#[test]
fn assignments_keep_type_parameters_that_biome_cant_compare() {
    assert_reads(
        r#"
        function a1<T>(t: T, s: string) { let x: T | string = s; x = t; /*assignedT*/x; }
        function a2<T>(t: T) { let x: T | null = null; x = t; /*assignedNull*/x; }
        function a3<T>(t: T, n: number) { let x: T | string | number = t; x = n; /*assignedNumber*/x; }
        function a4<T>(t: T) { let x: string | number = 1; x = "a"; /*concrete*/x; }
        "#,
        &[
            // TypeScript: `T`.
            ("assignedT", "x", &["instanceof T", "string"]),
            // TypeScript: `T`.
            ("assignedNull", "x", &["instanceof T", "null"]),
            // TypeScript: `number`.
            ("assignedNumber", "x", &["instanceof T", "number"]),
            ("concrete", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A call to a local function without a return annotation reduces by the
// return type that Biome infers from the function's body, which has the
// `undefined` of an end that the function can reach, also through a `const`
// that holds the call's result. A function expression called where it is
// defined, and `typeof`, which Biome types as one kind, give the declared
// type.
#[test]
fn calls_to_local_functions_keep_the_undefined_of_a_reachable_end() {
    assert_reads(
        r#"
        function g1() { if (c) return 1; }
        const h1 = () => { if (c) return 1; };
        function v1() { let x: number | undefined = g1(); /*declaration*/x; }
        function v2() { let x: number | undefined = h1(); /*arrow*/x; }
        function v3() { let x: number | undefined = (() => { if (c) return 1; })(); /*invoked*/x; }
        function v4(y: string) { let k: "string" | "number" = "number"; k = typeof y; /*typeof*/k; }
        function v5() { let x: number | undefined = c ? g1() : 2; /*branch*/x; }
        async function v7() { let x: number | undefined; x = await k1(); /*awaited*/x; async function k1() { if (c) return 1; } }
        function v8() { let x: number | undefined = 0; const y = g1(); x = y; /*held*/x; }
        "#,
        &[
            ("declaration", "x", &["number", "undefined"]),
            ("arrow", "x", &["number", "undefined"]),
            ("invoked", "x", &["number", "undefined"]),
            ("typeof", "k", &["string: number", "string: string"]),
            ("branch", "x", &["number", "undefined"]),
            ("awaited", "x", &["number", "undefined"]),
            ("held", "x", &["number", "undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A call to a method or an imported function keeps the declared type unless
// its callee's return type comes from an annotation: one that syntax shows,
// at the callee or at the declaration that an import names, or a method of a
// lib global or of a type that a declaration file declares. Checking the
// values that such a callee returns would need the syntax of another module
// or the types of an object, so a return type that Biome infers for it keeps
// the declared type, and so does the type of an element, of a member that a
// module other than a declaration file merges into a global or that a
// declaration file's class inherits from one, and of a `?.` whose object is
// such a call.
#[test]
fn calls_reduce_only_by_annotated_return_types() {
    assert_reads_with(
        r#"
        import { maybe, annotated, arrow } from "./source";
        import { Box, declared } from "./declared";
        import { reannotated, remaybe } from "./reexport";
        import { bridged } from "./bridge";
        import "./augment";
        import { Derived } from "./derived";
        class Local { m(c: boolean) { if (c) return 1; } }
        declare const local: Local;
        declare const box: Box;
        const object = { m() { if (c) return 1; } };
        function m1() { let x: number | undefined = local.m(c); /*method*/x; }
        function m2() { let x: number | undefined; x = local.m(c); /*assignedMethod*/x; }
        function m3() { let x: number | undefined = object.m(); /*objectMethod*/x; }
        function i1() { let x: number | undefined = maybe(c); /*imported*/x; }
        function i2() { let x: number | undefined = arrow(c); /*importedArrow*/x; }
        function i3() { let x: number | undefined = annotated(c); /*importedAnnotated*/x; }
        function d1() { let x: number | undefined = box.get(); /*declaredMethod*/x; }
        function d2() { let x: number | undefined = declared(); /*declaredFunction*/x; }
        function g1() { let x: string | undefined = new Date().toISOString(); /*libMethod*/x; }
        function g2() { let x: number | undefined = parseInt("1"); /*libFunction*/x; }
        function g3(s: string) { let x: string | undefined = s.trim(); /*primitiveMethod*/x; }
        function g4(h: () => number) { let x: number | undefined = h(); /*annotatedParameter*/x; }
        function g5() { let x: number | undefined; [x = local.m(c)] = [] as number[]; /*methodDefault*/x; }
        function g6() { let x: number | undefined; [x = box.get()] = [] as number[]; /*declaredDefault*/x; }
        function r1() { let x: number | undefined = reannotated(c); /*reexported*/x; }
        function r2() { let x: number | undefined = remaybe(c); /*reexportedInferred*/x; }
        function e1() { const t = [maybe] as const; let x: number | undefined = t[0](c); /*element*/x; }
        function b1() { let x: number | undefined = bridged(c); /*declarationReexport*/x; }
        function trimmed(c: boolean) { if (c) return " a"; }
        function o1() { let x: string | undefined = trimmed(c)?.trim(); /*chainedCall*/x; }
        function o2() { let x: number | undefined = trimmed(c)?.length; /*chainedProperty*/x; }
        function o3() { let x: string | undefined = local.m(c)?.toFixed(); /*chainedMethod*/x; }
        function a1() { let x: number | undefined = [1].helper(c); /*augmentedGlobal*/x; }
        function h1(d: Derived) { let x: number | undefined = d.m(c); /*inheritedMethod*/x; }
        "#,
        &[
            (
                "/src/source.ts",
                r#"
                export function maybe(c: boolean) { if (c) return 1; }
                export function annotated(c: boolean): number { return c ? 1 : 2; }
                export const arrow = (c: boolean) => { if (c) return 1; };
                "#,
            ),
            (
                "/src/declared.d.ts",
                r#"
                export declare class Box { get(): number; }
                export declare function declared(): number;
                "#,
            ),
            (
                "/src/reexport.ts",
                r#"
                import { annotated, maybe } from "./source";
                export { annotated as reannotated, maybe as remaybe };
                "#,
            ),
            (
                "/src/bridge.d.ts",
                r#"
                import { maybe } from "./source";
                export { maybe as bridged };
                "#,
            ),
            (
                "/src/augment.ts",
                r#"
                function helperImpl(c: boolean) { if (c) return 1; }
                declare global { interface Array<T> { helper: typeof helperImpl; } }
                export {};
                "#,
            ),
            (
                "/src/base.ts",
                "export class Base { m(c: boolean) { if (c) return 1; } }",
            ),
            (
                "/src/derived.d.ts",
                r#"
                import { Base } from "./base";
                export declare class Derived extends Base {}
                "#,
            ),
        ],
        &[
            ("method", "x", &["number", "undefined"]),
            ("assignedMethod", "x", &["number", "undefined"]),
            ("objectMethod", "x", &["number", "undefined"]),
            ("imported", "x", &["number", "undefined"]),
            ("importedArrow", "x", &["number", "undefined"]),
            ("importedAnnotated", "x", &["number"]),
            ("declaredMethod", "x", &["number"]),
            ("declaredFunction", "x", &["number"]),
            ("libMethod", "x", &["string"]),
            ("libFunction", "x", &["number"]),
            ("primitiveMethod", "x", &["string"]),
            ("annotatedParameter", "x", &["number"]),
            ("methodDefault", "x", &["number", "undefined"]),
            ("declaredDefault", "x", &["number"]),
            ("reexported", "x", &["number"]),
            ("reexportedInferred", "x", &["number", "undefined"]),
            ("element", "x", &["number", "undefined"]),
            ("declarationReexport", "x", &["number", "undefined"]),
            ("chainedCall", "x", &["string", "undefined"]),
            ("chainedProperty", "x", &["number", "undefined"]),
            ("chainedMethod", "x", &["string", "undefined"]),
            ("augmentedGlobal", "x", &["number", "undefined"]),
            ("inheritedMethod", "x", &["number", "undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A method that a class or an interface declares with a return type
// annotation reduces, whether the object's type declares it or inherits it,
// and an override decides over the method it overrides. A method that some
// type of the object declares or inherits without one keeps the declared
// type.
#[test]
fn calls_of_annotated_methods_reduce() {
    assert_reads_with(
        r#"
        import { AnnotatedBase, InferredBase } from "./base";
        class Local { n(): number { return 1; } m(c: boolean) { if (c) return 1; } }
        interface Shape { area(): number; maybe?(): number; }
        class Child extends Local {}
        class FromAnnotated extends AnnotatedBase {}
        class FromInferred extends InferredBase {}
        class Shadow extends InferredBase { m(c: boolean): 1 { return 1; } }
        function k1(l: Local) { let x: number | undefined = l.n(); /*method*/x; }
        function k2(s: Shape) { let x: number | undefined = s.area(); /*interfaceMethod*/x; }
        function k3(d: Child) { let x: number | undefined = d.n(); /*inherited*/x; }
        function k4(d: Child) { let x: number | undefined = d.m(c); /*inheritedInferred*/x; }
        function k5(d: FromAnnotated) { let x: number | undefined = d.m(); /*importedBase*/x; }
        function k6(d: FromInferred) { let x: number | undefined = d.m(c); /*importedInferredBase*/x; }
        function k8(s: Shadow) { let x: number | undefined = s.m(c); /*override*/x; }
        class Self { n(): number { return 1; } run() { let x: number | undefined = this.n(); /*thisMethod*/x; } }
        function k10(u: Local | AnnotatedBase) { let x: number | undefined = u.m(c); /*unionMember*/x; }
        "#,
        &[(
            "/src/base.ts",
            r#"
            export class AnnotatedBase { m(): number { return 1; } }
            export class InferredBase { m(c: boolean) { if (c) return 1; } }
            "#,
        )],
        &[
            ("method", "x", &["number"]),
            ("interfaceMethod", "x", &["number"]),
            ("inherited", "x", &["number"]),
            ("inheritedInferred", "x", &["number", "undefined"]),
            ("importedBase", "x", &["number"]),
            ("importedInferredBase", "x", &["number", "undefined"]),
            ("override", "x", &["number"]),
            ("thisMethod", "x", &["number"]),
            ("unionMember", "x", &["number", "undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A compound assignment, `++`, and `--` give the primitive type of the literal
// types that the variable has before the write, even when that is wider than
// the declared type, and `x = x * 2` reduces the primitive type of the
// literal types of the declared type.
#[test]
fn compound_assignments_widen_literal_types() {
    assert_reads(
        r#"
        function c1() { let x: number | string = 0; x += 1; /*addNumber*/x; }
        function c2() { let x: number | string = "a"; x += 1; /*addString*/x; }
        function c3() { let x: 1 | 2 = 1; x++; /*increment*/x; }
        function c4() { let x: number | undefined = 0; x++; /*optional*/x; }
        function c5() { let s: string | undefined = ""; s += "a"; /*string*/s; }
        function c8() { let x: number | string = f(); x -= 1; /*subtract*/x; }
        function c9() { let x: 1 | 2 = 1; x += 1; /*literalAdd*/x; }
        function c10() { let x: 1 | 2 = 1; ++x; /*prefix*/x; }
        function c11() { let x: bigint | string = 1n; x *= 2n; /*bigint*/x; }
        function c12() { let x: number | string = 1; x = (x) * 2; /*compoundLike*/x; }
        function c13() { let x: 1 | 2 | 3 = 1; x = c ? 2 : 3; /*branches*/x; }
        function c14() { let x: number | string = 1; x = -x; /*negation*/x; }
        function c15(o: { v: 1 | 2 }) { o.v++; /*path*/o.v; }
        function c16() { let x: 1 | 2 = 1; const y = x++; /*postfix*/x; }
        function c17() { let x: number | null = f(); if (x !== null) { x |= 1; /*bitwise*/x; } }
        "#,
        &[
            ("addNumber", "x", &["number"]),
            ("addString", "x", &["string"]),
            ("increment", "x", &["number"]),
            ("optional", "x", &["number"]),
            ("string", "s", &["string"]),
            ("subtract", "x", &["number", "string"]),
            ("literalAdd", "x", &["number"]),
            ("prefix", "x", &["number"]),
            ("bigint", "x", &["BigInt"]),
            ("compoundLike", "x", &["number"]),
            ("branches", "x", &["number: 2", "number: 3"]),
            ("negation", "x", &["number"]),
            ("path", "o.v", &["number"]),
            ("postfix", "x", &["number"]),
            ("bitwise", "x", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `number` at both reads. Biome types
// the right operand `x + 1` without narrowing, so it adds `string` from the
// declared type of `x`, which the reduction keeps.
#[test]
fn compound_like_assignments_type_their_right_operand_without_narrowing() {
    assert_reads(
        r#"
        function c6() { let x: number | string = 1; x = x + 1; /*plus*/x; }
        function c7() { let x: 1 | 2 | "a" = 1; x = x + 1; /*literals*/x; }
        "#,
        &[
            // TypeScript: `number`.
            ("plus", "x", &["number", "string"]),
            ("literals", "x", &["number", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A logical assignment writes its target where its right operand runs, and
// the code after it joins that path with the one that skipped the right
// operand. The value of the assignment, as `v` in `const v = (x ??= "d")`,
// is Unknown in Biome.
#[test]
fn logical_assignments_narrow_their_target() {
    assert_reads(
        r#"
        function l1(x: string | undefined) { x ??= "d"; /*coalescing*/x; }
        function l2(x: string | undefined) { x ||= "d"; /*or*/x; }
        function l3(x: string | undefined) { x &&= "a"; /*and*/x; }
        function l7(x: string | undefined) { if (x ??= "d") /*condition*/x; }
        function l8(x: string | null, y: string | null) { x ??= (y = null); /*conditionalWrite*/y; }
        function l9(o: { v?: number }) { o.v ??= 1; /*path*/o.v; }
        function l10(x: string | number | undefined) { x ??= 1; /*reduced*/x; }
        function l11(x: "a" | "b" | undefined) { x ??= f<"a" | "b">(); /*literals*/x; }
        function l12(x: string | undefined, y: boolean) { if (y) x ??= "d"; /*branch*/x; }
        function l13(x: string | null | undefined) { x ??= null; /*null*/x; }
        function l14(x: number | undefined) { x ||= 0; /*falsy*/x; }
        function l15(x: string | undefined) { const v = (x ??= "d"); /*value*/v; }
        "#,
        &[
            ("coalescing", "x", &["string"]),
            ("or", "x", &["string"]),
            ("and", "x", &["string", "undefined"]),
            ("condition", "x", &["string"]),
            ("conditionalWrite", "y", &["null", "string"]),
            ("path", "o.v", &["number"]),
            ("reduced", "x", &["number", "string"]),
            ("literals", "x", &["string: a", "string: b"]),
            ("branch", "x", &["string", "undefined"]),
            ("null", "x", &["null", "string"]),
            ("falsy", "x", &["number"]),
            // TypeScript: `string`.
            ("value", "v", &["Unknown"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A target of a destructuring assignment, of the head of a `for...in` or
// `for...of` loop, and a variable of a declarator with a type annotation are
// assigned the field that destructuring reads, with the default that supplies
// it.
#[test]
fn destructuring_targets_narrow_to_their_fields() {
    assert_reads(
        r#"
        function d1() { let x: string | number; [x] = ["a"]; /*element*/x; }
        function d2() { let a: string | number, b: string | number; [a, b] = [1, "s"]; /*first*/a; /*second*/b; }
        function d3() { let x: string | number; ({ a: x } = { a: 1 }); /*property*/x; }
        function d4() { let x: string | number; ({ x } = { x: "s" }); /*shorthand*/x; }
        function d6() { let x: string | number; [x = 1] = f<(string | undefined)[]>(); /*default*/x; }
        function d7() { let r: number[] | string; [...r] = [1]; /*rest*/r; }
        function d9() { let x: string | number; for (x of ["a"]) /*forOf*/x; }
        function d10() { let x: string | number; for (x in {}) /*forIn*/x; }
        function d11() { const [a]: (string | number)[] = ["a"]; /*declaredElement*/a; }
        function d12() { const { a }: { a: string | number } = { a: 1 }; /*declaredProperty*/a; }
        function d13() { const { a = "d" }: { a?: string | number } = {}; /*declaredDefault*/a; }
        function d14(o: { a: string | number }) { let x: string | number; ({ a: x } = o); /*declaredValue*/x; }
        function d15() { let x: string | null = f(); if (x) { [x] = [null]; /*afterTest*/x; } }
        function d16() { let x: string | number; const t = ([x] = ["a"]); /*nested*/x; }
        function d17(o: { v: string | number }) { [o.v] = ["a"]; /*path*/o.v; }
        function d18() { let x: string | number; [, x] = [1, "s"]; /*hole*/x; }
        function d19() { let x: string | number; ({ ["a"]: x } = { a: 1 }); /*computed*/x; }
        function d21() { let x: string | number = 1; for (const v of [x]) { x = "a"; } /*loop*/x; }
        function d23() { let x: string | number; let y: number | null; [x, y = 1] = ["a"]; /*beforeDefault*/x; }
        "#,
        &[
            ("element", "x", &["string"]),
            ("first", "a", &["number"]),
            ("second", "b", &["string"]),
            ("property", "x", &["number"]),
            ("shorthand", "x", &["string"]),
            ("default", "x", &["number", "string"]),
            ("rest", "r", &["instanceof Array<number>"]),
            ("forOf", "x", &["string"]),
            ("forIn", "x", &["string"]),
            ("declaredElement", "a", &["string"]),
            ("declaredProperty", "a", &["number"]),
            ("declaredDefault", "a", &["number", "string"]),
            ("declaredValue", "x", &["number", "string"]),
            ("afterTest", "x", &["null"]),
            ("nested", "x", &["string"]),
            ("path", "o.v", &["string"]),
            ("hole", "x", &["string"]),
            ("computed", "x", &["number"]),
            ("loop", "x", &["number", "string"]),
            ("beforeDefault", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives the narrower type in the comment for
// each read below. Biome is wider: it types a missing element of an array
// literal as unknown, and an element of an array as possibly `undefined`,
// which the declared type lacks.
#[test]
fn destructuring_targets_that_biome_narrows_less() {
    assert_reads(
        r#"
        function d5() { let x: string | number; [x = 1] = []; /*missing*/x; }
        function d8() { let x: string | number; [[x]] = [["a"]]; /*nestedArray*/x; }
        function d23() { let x: string | number; let y: number | null; [x, y = 1] = ["a"]; /*outOfRange*/y; }
        "#,
        &[
            // TypeScript: `number`.
            ("missing", "x", &["number", "string"]),
            // TypeScript: `string`.
            ("nestedArray", "x", &["number", "string"]),
            // TypeScript: `number`.
            ("outOfRange", "y", &["null", "number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// TypeScript's binder evaluates a destructuring pattern, with its
// defaults, before the value, and a declaration's pattern before its
// initializer, although JavaScript runs them the other way around. A default
// of a destructuring assignment runs unconditionally, and a default of a
// declaration only for `undefined`. Biome follows TypeScript, as
// `NARROWING.md` records among the deliberate choices.
#[test]
fn destructuring_follows_the_order_of_typescripts_binder() {
    assert_reads(
        r#"
        declare function id<T>(v: T): T;
        function a1(x: string | number) { if (typeof x === "string") { let a: unknown; [a = (x = 1)] = [/*arrayValue*/x]; } }
        function a2(x: string | number) { if (typeof x === "string") { let a: unknown; ({ a = (x = 1) } = { a: /*objectValue*/x }); } }
        function o1(x: string | number) { if (typeof x === "string") { const [a = id(x = 1)] = [/*initializer*/x]; } }
        function o2(x: string | number) { if (typeof x === "string") { const [a = id(x = 1)] = f<string[]>(); /*afterDeclaration*/x; } }
        function o3(x: string | number) { if (typeof x === "string") { let a: unknown; [a = (x = 1)] = f<string[]>(); /*afterAssignment*/x; } }
        "#,
        &[
            ("arrayValue", "x", &["number"]),
            ("objectValue", "x", &["number"]),
            ("initializer", "x", &["number", "string"]),
            ("afterDeclaration", "x", &["number", "string"]),
            ("afterAssignment", "x", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A write to a property narrows the path, a write to a prefix gives it its
// declared type, and `delete o.p` assigns `undefined`.
#[test]
fn writes_narrow_property_paths() {
    assert_reads(
        r#"
        function e1(o: { p?: string }) { if (o.p) { delete o.p; /*deleted*/o.p; } }
        function e3(o: { p: string | undefined; q?: string }) { if (o.q) { delete o.p; /*other*/o.q; } }
        function e4(o: { p?: string | null }) { if (o.p) { delete o.p; /*deletedNullable*/o.p; } }
        function q1(o: { v: string | number }) { o.v = "a"; /*assigned*/o.v; }
        function q2(o: { v: string | number }, w: { v: string | number }) { if (typeof o.v === "string") { o = w; /*object*/o.v; } }
        function q3(o: { v: { w: string | null } }) { if (o.v.w) { o.v = f(); /*prefix*/o.v.w; } }
        function q8() { const o: { v: string | number } = { v: 1 }; /*initializer*/o.v; }
        function q9(o: { v: string | null }) { o.v = "a"; if (c) o.v = null; /*branches*/o.v; }
        function q10(t: [string | number]) { t[0] = "a"; /*element*/t[0]; }
        function q11(o: { v: string | number }) { o["v"] = 1; /*literalKey*/o.v; }
        function q12(o: { v?: string | number }) { o.v = 1; /*optional*/o.v; }
        function q13(o: { v: string | null }) { if (o.v) { o.v += "x"; /*compound*/o.v; } }
        function q14(o: { a: { v: string | null } }) { if (o.a.v) { o.a = { v: null }; /*objectWrite*/o.a.v; } }
        function q19(o: { readonly v: string | number }) { const ok = typeof o.v === "string"; o.v; if (ok) /*readonly*/o.v; }
        function q20(o: { v: string | number }) { const ok = typeof o.v === "string"; o.v = 1; if (ok) /*afterAlias*/o.v; }
        class Q17 { v: string | number = 0; m() { this.v = "a"; /*this*/this.v; } }
        "#,
        &[
            ("deleted", "o.p", &["undefined"]),
            ("other", "o.q", &["string"]),
            ("deletedNullable", "o.p", &["undefined"]),
            ("assigned", "o.v", &["string"]),
            ("object", "o.v", &["number", "string"]),
            ("prefix", "o.v.w", &["null", "string"]),
            ("initializer", "o.v", &["number", "string"]),
            ("branches", "o.v", &["null", "string"]),
            ("element", "t[0]", &["string"]),
            ("literalKey", "o.v", &["number"]),
            ("optional", "o.v", &["number"]),
            ("compound", "o.v", &["string"]),
            ("objectWrite", "o.a.v", &["null", "string"]),
            ("readonly", "o.v", &["string"]),
            ("afterAlias", "o.v", &["number"]),
            ("this", "this.v", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives the narrower type in the comment for
// each read below. Biome is wider: `delete` through an element access gives
// the path its declared type, where TypeScript ignores it; a write through a
// key that is not a literal gives every path of the object its declared type;
// and a subject that an `async` function called where it is defined or a
// field initializer writes doesn't narrow anywhere in the function. Calls
// and callbacks never cancel narrowing, as in TypeScript, and a function
// called where it is defined and a class static block write in the flow.
#[test]
fn writes_that_biome_follows_less_than_typescript() {
    assert_reads(
        r#"
        function e2(o: { p?: string }) { if (o["p"]) { delete o["p"]; /*deletedElement*/o.p; } }
        function q4(o: { v: string | null }, k: "v") { if (o.v) { o[k] = null; /*anyKey*/o.v; } }
        function q5(o: { v: string | null }) { if (o.v) { [1].forEach(() => { o.v = null; }); /*callback*/o.v; } }
        function q7(o: { v: string | null }) { if (o.v) { (() => { o.v = null; })(); /*invoked*/o.v; } }
        function q8(o: { v: string | null }) { if (o.v) { (async () => { o.v = null; })(); /*async*/o.v; } }
        function o1(x: string | null) { if (x) { (() => { x = null; })(); /*invokedVariable*/x; } }
        function o2(x: string | null) { if (x) { class C { static { x = null; } } /*staticBlock*/x; } }
        function o3(x: string | null) { if (x) { [1].forEach(() => { x = null; }); /*callbackVariable*/x; } }
        function o4(x: string | null) { if (x) { class C { y = (x = null); } /*fieldInitializer*/x; } }
        "#,
        &[
            // TypeScript: `string`.
            ("deletedElement", "o.p", &["string", "undefined"]),
            // TypeScript: `string`.
            ("anyKey", "o.v", &["null", "string"]),
            ("callback", "o.v", &["string"]),
            ("invoked", "o.v", &["null"]),
            // TypeScript: `string`.
            ("async", "o.v", &["null", "string"]),
            ("invokedVariable", "x", &["null"]),
            ("staticBlock", "x", &["null"]),
            ("callbackVariable", "x", &["string"]),
            // TypeScript: `string`.
            ("fieldInitializer", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A write in a try block reaches its catch clause and finally block, writes
// join where branches and loops meet, and expressions write in the order in
// which they run.
#[test]
fn writes_flow_through_statements_and_expressions() {
    assert_reads(
        r#"
        function t1() { let x: string | number = "a"; try { x = 1; g(); } catch { /*catch*/x; } }
        function t2() { let x: string | number = "a"; try { x = 1; g(); } finally { /*finally*/x; } /*afterFinally*/x; }
        function t3() { let x: string | number = "a"; try { g(); x = 1; } catch { /*catchBefore*/x; } /*afterTry*/x; }
        function t4() { let x: string | number = "a"; try { g(); } catch { x = 1; } /*afterCatch*/x; }
        function t6() { let x: string | null = f(); try { if (!x) return; x = null; g(); } catch { /*catchAfterTest*/x; } }
        function t7() { let x: string | number = "a"; try { try { x = 1; g(); } finally { /*innerFinally*/x; } } catch { /*outerCatch*/x; } }
        function w1() { let x: string | null = null; while (c) { if (x) /*loopTest*/x; x = "a"; } }
        function w2() { let x: string | null = null; while (c) { /*loopBody*/x; x = "a"; } /*afterLoop*/x; }
        function w5(xs: (string | null)[]) { for (const v of xs) { if (v === null) continue; /*continue*/v; } }
        function w6() { let x: string | number = 1; for (let i = 0; i < 3; i++) { /*forBody*/x; x = "a"; } }
        function w7() { let x: string | number = 1; do { x = "a"; } while (c); /*doWhile*/x; }
        function w8() { let x: string | number = 1; switch (f<number>()) { case 1: x = "a"; break; default: } /*switch*/x; }
        function w9() { let x: string | number = 1; label: { x = "a"; if (c) break label; x = 2; } /*label*/x; }
        function w10(x: string | number) { const r = c ? (x = "a") : (x = 1); /*conditional*/x; }
        function w11(x: string | number) { c && (x = "a"); /*logical*/x; }
        function w12(x: string | number) { const o = { a: (x = "a"), b: /*objectLiteral*/x }; }
        function w13(x: string | number) { f<any>((x = 1), /*argument*/x); }
        function w15(x: string | number, o: { f(v: unknown): void } | undefined) { o?.f(x = 1); /*optionalChain*/x; }
        function w18(x: string | number) { (x = "a") && /*rightOperand*/x; }
        function w19(x: string | number) { x = "a"; x = x.length; /*length*/x; }
        function w20(x: string | null) { if (x === null) return; const s = `${(x = null)}`; /*template*/x; }
        "#,
        &[
            ("catch", "x", &["number", "string"]),
            ("finally", "x", &["number", "string"]),
            ("afterFinally", "x", &["number"]),
            ("catchBefore", "x", &["number", "string"]),
            ("afterTry", "x", &["number", "string"]),
            ("afterCatch", "x", &["number", "string"]),
            ("catchAfterTest", "x", &["null", "string"]),
            ("innerFinally", "x", &["number", "string"]),
            ("outerCatch", "x", &["number", "string"]),
            ("loopTest", "x", &["string"]),
            ("loopBody", "x", &["null", "string"]),
            ("afterLoop", "x", &["null", "string"]),
            ("continue", "v", &["string"]),
            ("forBody", "x", &["number", "string"]),
            ("doWhile", "x", &["string"]),
            ("switch", "x", &["number", "string"]),
            ("label", "x", &["number", "string"]),
            ("conditional", "x", &["number", "string"]),
            ("logical", "x", &["number", "string"]),
            ("objectLiteral", "x", &["string"]),
            ("argument", "x", &["number"]),
            ("optionalChain", "x", &["number", "string"]),
            ("rightOperand", "x", &["string"]),
            ("length", "x", &["number"]),
            ("template", "x", &["null"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A `let` without an initializer, and in a constructor a field without an
// initializer, start with `undefined`. A read where `undefined` remains,
// although the declared type lacks it, has the declared type, as TypeScript
// gives it after TS2454 or TS2565.
#[test]
fn variables_and_fields_read_before_they_are_assigned_keep_their_declared_type() {
    assert_reads(
        r#"
        function u1() { let x: string | null; if (c) x = "a"; if (x !== null) /*maybeAssigned*/x; }
        function u2() { let x: string | null; if (x !== null) /*unassigned*/x; }
        function u3() { let x: string | undefined; if (x !== null) /*declaredUndefined*/x; }
        function u4() { let x: string | null; x = "a"; if (x !== null) /*assigned*/x; }
        function u5() { let x!: string | null; if (x !== null) /*definite*/x; }
        function u8() { let x: string | null; if (c) x = "a"; else x = null; if (x !== null) /*bothBranches*/x; }
        function u9() { let x: string | null; for (;;) { if (x !== null) /*loop*/x; x = "a"; } }
        function u11() { let x: string | number; if (typeof x === "string") /*typeofRemovesUndefined*/x; }
        class K1 { x: string | null = null; constructor() { if (this.x) /*initialized*/this.x; } }
        class K2 { x: string | null; constructor() { if (this.x !== null) /*field*/this.x; this.x = null; } }
        class K3 { x: string | undefined; constructor() { /*fieldUndefined*/this.x; if (this.x) /*truthy*/this.x; } }
        class K4 { x: string | number = 0; constructor() { this.x = "a"; /*assignedField*/this.x; } }
        class K5 { x: string | null; constructor() { this.x = "a"; /*assignedFirst*/this.x; } }
        class K6 { x: string | null = null; constructor() { const g = () => { if (this.x) /*arrow*/this.x; }; } }
        class K7 { x: string | null; constructor() { (() => { if (this.x !== null) /*invoked*/this.x; })(); this.x = null; } }
        class K8 { x!: string | null; constructor() { if (this.x !== null) /*definiteField*/this.x; } }
        class K9 { x: string | null; constructor() { this.x = f(); if (this.x !== null) /*afterCall*/this.x; } }
        class B0 { x: string | null = null; } class K10 extends B0 { constructor() { super(); if (this.x !== null) /*inherited*/this.x; } }
        class K12 { x?: string | null; constructor() { if (this.x !== null) /*optional*/this.x; } }
        class K14 { x: string | null; constructor(x: string | null) { this.x = x; if (this.x) /*parameter*/this.x; } }
        class K15 { #x: string | null; constructor() { if (this.#x !== null) /*private*/this.#x; this.#x = null; } }
        "#,
        &[
            ("maybeAssigned", "x", &["null", "string"]),
            ("unassigned", "x", &["null", "string"]),
            ("declaredUndefined", "x", &["string", "undefined"]),
            ("assigned", "x", &["string"]),
            ("definite", "x", &["string"]),
            ("bothBranches", "x", &["string"]),
            ("loop", "x", &["null", "string"]),
            ("typeofRemovesUndefined", "x", &["string"]),
            ("initialized", "this.x", &["string"]),
            ("field", "this.x", &["null", "string"]),
            ("fieldUndefined", "this.x", &["string", "undefined"]),
            ("truthy", "this.x", &["string"]),
            ("assignedField", "this.x", &["string"]),
            ("assignedFirst", "this.x", &["string"]),
            ("arrow", "this.x", &["string"]),
            ("invoked", "this.x", &["null", "string"]),
            ("definiteField", "this.x", &["string"]),
            ("afterCall", "this.x", &["string"]),
            ("inherited", "this.x", &["string"]),
            ("optional", "this.x", &["string", "undefined"]),
            ("parameter", "this.x", &["string"]),
            ("private", "this.#x", &["null", "string"]),
        ],
    );
}

// Lookups resolve the types that the writes of a subject assign once per
// root and subject, through `infer_flow_assigned_types`, which the reads of
// the subject share. A read of another subject, or of a subject without a
// write that assigns a value, doesn't run it.
#[test]
fn lookups_resolve_assigned_types_once_per_subject() {
    const SOURCE: &str = r#"
        declare function make<T>(): T;
        function f(x: string | number | null, y: string | null, z: string | null) {
            x = make<string | null>();
            if (x !== null) { /*first*/x; }
            /*second*/x;
            if (y) { /*tested*/y; }
            z = "a";
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    let read = |marker, expression| {
        let ty = infer_expression_type(
            &db,
            ExpressionTypeInput::new(&db, module, marked_range(SOURCE, marker, expression)),
        )
        .unwrap();
        variant_names(&db, ty)
    };
    let resolutions = |events: &[salsa::Event]| {
        function_query_will_execute_count_by_name(&db, "infer_flow_assigned_types", events)
    };
    db.clear_salsa_events();
    assert_eq!(read("tested", "y"), ["string"]);
    let events = db.take_salsa_events();
    assert_eq!(resolutions(&events), 0, "`y` has no write");
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
    assert_eq!(read("first", "x"), ["string"]);
    assert_eq!(read("second", "x"), ["null", "string"]);
    let events = db.take_salsa_events();
    assert_eq!(
        resolutions(&events),
        1,
        "the reads of `x` share its assigned types"
    );
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

// Complete module inference resolves the assigned types in its own context,
// which has resolved every raw type of the module already, and lookups
// starting from the same declared and assigned types reuse its solve.
#[test]
fn complete_module_inference_shares_assignment_solves_with_lookups() {
    const SOURCE: &str = r#"
        function f(x: string | number) {
            x = 1;
            /*read*/x;
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    let range = marked_range(SOURCE, "read", "x");
    db.clear_salsa_events();
    let complete = infer_module_types(&db, module).unwrap();
    assert_eq!(complete.expressions[&range], InferredTypeData::Number);
    let events = db.take_salsa_events();
    for query in ["infer_flow_assigned_types", "infer_flow_binding_baseline"] {
        assert_eq!(
            function_query_will_execute_count_by_name(&db, query, &events),
            0,
            "complete module inference must resolve {query} itself"
        );
    }
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_subject_states", &events),
        1
    );
    db.clear_salsa_events();
    assert_eq!(
        infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range)),
        Some(InferredTypeData::Number)
    );
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_assigned_types", &events),
        1,
        "lookups must resolve the assigned types themselves"
    );
    for query in ["infer_flow_subject_states", "infer_flow_subject_type"] {
        assert_eq!(
            function_query_will_execute_count_by_name(&db, query, &events),
            0,
            "lookups starting from the same types must reuse the solve: {query}"
        );
    }
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the type in its
// comment and reports TS2322 at `enumLiteral`, because `"a"` isn't assignable
// to the string enum `S`. A destructuring target that reads an element of an
// array keeps its declared type, because Biome types the element as possibly
// `undefined`, as in `[x] = xs` and `[[x]] = [["a"]]`. So does a target whose
// default supplies the value because the array has no such element, as in
// `[x = 1] = []`. Biome takes `"a"` as assignable to `S`, whose member `A`
// has that value, so `x` reads `"a"`.
#[test]
fn assignments_from_array_elements_and_to_string_enums() {
    assert_reads(
        r#"
        enum S { A = "a", B = "b" }
        function a1(xs: string[]) { let x: string | number = 1; [x] = xs; /*element*/x; } // string
        function a2() { let x: string | number = 1; [[x]] = [["a"]]; /*nested*/x; } // string
        function a3() { let x: string | number = "a"; [x = 1] = []; /*missing*/x; } // number
        function a4() { let x: S | number = "a"; /*enumLiteral*/x; } // number | S
        "#,
        &[
            // TypeScript: `string`.
            ("element", "x", &["number", "string"]),
            // TypeScript: `string`.
            ("nested", "x", &["number", "string"]),
            // TypeScript: `number`.
            ("missing", "x", &["number", "string"]),
            // TypeScript: `number | S`.
            ("enumLiteral", "x", &["string: a"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the type in its
// comment. A value that a declaration file types as `typeof` a function
// returns what the function returns, so a call of it has a return type from
// an annotation only when the function has one, or is declared in a
// declaration file. This holds for an export, a member of an interface or a
// class, and a member added to a global interface, also through a chain of
// `typeof`. `helperImpl` and `kindImpl` have no return type annotation, so a
// call through them keeps the declared type, as a direct call of an imported
// function without one does. TypeScript gives `kindImpl` every result of
// `typeof`, where Biome types `typeof v` as `"string"`. Biome doesn't see a
// variable that `declare global` declares, so a call of it keeps the
// declared type too.
#[test]
fn calls_through_typeof_a_function_in_a_declaration_file() {
    assert_reads_with(
        r#"
        import { exported, typed, holder, kind, annotatedKind, viaLocal, viaChain, klass } from "./globals";
        import "./augment";
        declare const s: string;
        type Kind = "bigint" | "boolean" | "function" | "number" | "object" | "string" | "symbol" | "undefined";
        function a1() { let x: number | string | undefined = helper(c); /*globalVariable*/x; } // number | undefined
        function a2() { let x: number | string | undefined = [1].helperMember(c); /*globalMember*/x; } // number | undefined
        function a3() { let x: number | string | undefined = exported(c); /*exported*/x; } // number | undefined
        function a4() { let x: number | string | undefined = typed.m(c); /*typeLiteral*/x; } // number | undefined
        function a5() { let x: number | string | undefined = moduleHelper(c); /*moduleGlobal*/x; } // number | undefined
        function a6() { let x: number | string | undefined = holder.m(c); /*interfaceMember*/x; } // number | undefined
        function k1() { let x: Kind; x = kind(s); /*kindExport*/x; } // Kind
        function k2() { let x: Kind; x = annotatedKind(s); /*annotatedExport*/x; } // "string"
        function k3() { let x: Kind; x = holder.kind(s); /*kindMember*/x; } // Kind
        function k4() { let x: Kind; x = holder.annotatedKind(s); /*annotatedMember*/x; } // "string"
        function k5() { let x: Kind; x = [1].kindMember(s); /*kindGlobalMember*/x; } // Kind
        function k6() { let x: Kind; x = [1].annotatedKindMember(s); /*annotatedGlobalMember*/x; } // "string"
        function k7() { let x: Kind; x = viaLocal(s); /*viaLocal*/x; } // "string"
        function k8() { let x: Kind; x = viaChain(s); /*viaChain*/x; } // Kind
        function k9() { let x: Kind; x = klass.m(s); /*classProperty*/x; } // Kind
        "#,
        &[
            (
                "/src/impl.ts",
                r#"
                export function helperImpl(c: boolean) { if (c) return 1; }
                export function kindImpl(v: string) { return typeof v; }
                export function annotatedKindImpl(v: string): "string" { return "string"; }
                "#,
            ),
            (
                "/src/globals.d.ts",
                r#"
                import { helperImpl, kindImpl, annotatedKindImpl } from "./impl";
                declare global {
                  var helper: typeof helperImpl;
                  interface Array<T> {
                    helperMember: typeof helperImpl;
                    kindMember: typeof kindImpl;
                    annotatedKindMember: typeof annotatedKindImpl;
                  }
                }
                export declare const exported: typeof helperImpl;
                export declare const typed: { m: typeof helperImpl };
                export interface Holder {
                  m: typeof helperImpl;
                  kind: typeof kindImpl;
                  annotatedKind: typeof annotatedKindImpl;
                }
                export declare const holder: Holder;
                export declare const kind: typeof kindImpl;
                export declare const annotatedKind: typeof annotatedKindImpl;
                declare function localKind(v: string): "string";
                export declare const viaLocal: typeof localKind;
                export declare const viaChain: typeof kind;
                export declare class Klass { m: typeof kindImpl; }
                export declare const klass: Klass;
                "#,
            ),
            (
                "/src/augment.ts",
                r#"
                import { helperImpl } from "./impl";
                declare global { var moduleHelper: typeof helperImpl; }
                export {};
                "#,
            ),
        ],
        &[
            ("globalVariable", "x", &["number", "string", "undefined"]),
            ("globalMember", "x", &["number", "string", "undefined"]),
            ("exported", "x", &["number", "string", "undefined"]),
            ("typeLiteral", "x", &["number", "string", "undefined"]),
            ("moduleGlobal", "x", &["number", "string", "undefined"]),
            ("interfaceMember", "x", &["number", "string", "undefined"]),
            ("kindExport", "x", KIND),
            ("annotatedExport", "x", &["string: string"]),
            ("kindMember", "x", KIND),
            ("annotatedMember", "x", &["string: string"]),
            ("kindGlobalMember", "x", KIND),
            ("annotatedGlobalMember", "x", &["string: string"]),
            ("viaLocal", "x", &["string: string"]),
            ("viaChain", "x", KIND),
            ("classProperty", "x", KIND),
        ],
    );
}

/// The members of `Kind` in [`calls_through_typeof_a_function_in_a_declaration_file`],
/// every result of `typeof`.
const KIND: &[&str] = &[
    "string: bigint",
    "string: boolean",
    "string: function",
    "string: number",
    "string: object",
    "string: string",
    "string: symbol",
    "string: undefined",
];
