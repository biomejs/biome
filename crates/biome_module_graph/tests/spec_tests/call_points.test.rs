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

/// Names each member of `ty`, sorted: an interface or a class instance by its
/// name, Biome's Unknown, which a read on a path that no value takes has, as
/// `Unknown`, and any other type by its formatted text on one line, such as
/// `string`, `null`, or `string: a`.
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

/// Asserts the type of each `(marker, expression, expected)` read in `source`,
/// by the names of its members, as [`variant_names`] gives them, with
/// `modules` beside it, as [`narrowing_db_with`] takes them, and that
/// complete module inference gives each read the type that a lookup gives it.
fn assert_reads_with(source: &str, modules: &[(&str, &str)], reads: &[(&str, &str, &[&str])]) {
    let lookups: Vec<_> = {
        let (db, module) = narrowing_db_with(source, modules);
        reads
            .iter()
            .map(|(marker, expression, _)| {
                let range = marked_range(source, marker, expression);
                infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                    .map(|ty| format_inferred_type(&db, ty))
            })
            .collect()
    };
    let (db, module) = narrowing_db_with(source, modules);
    let complete = infer_module_types(&db, module).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .zip(lookups)
        .flat_map(|((marker, expression, expected), lookup)| {
            let range = marked_range(source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "callPoints"),
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

/// Asserts the reads in `source` as [`assert_reads_with`] does, without other
/// modules.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    assert_reads_with(source, &[], reads);
}

/// The module that `narrowing-probes/m6/imports/main.ts` imports.
const GUARDS: &str = r#"
        export function isString(v: unknown): v is string { return typeof v === "string"; }
        export declare function isNumber(v: unknown): v is number;
        export function assertString(v: unknown): asserts v is string { if (typeof v !== "string") throw 0; }
        export function assert(v: unknown): asserts v { if (!v) throw 0; }
        export function fail(): never { throw 0; }
        export function failInferred() { throw 0; }
        export const isStr = (v: unknown): v is string => typeof v === "string";
        export const assertArrow = (v: unknown): asserts v => { if (!v) throw 0; };
        export const assertAnnotated: (v: unknown) => asserts v = (v) => { if (!v) throw 0; };
        export const failArrow = (): never => { throw 0; };
        export const failAnnotated: () => never = () => { throw 0; };
        export class Checker {
          isString(v: unknown): v is string { return typeof v === "string"; }
          assert(v: unknown): asserts v { if (!v) throw 0; }
          fail(): never { throw 0; }
          static isNumber(v: unknown): v is number { return typeof v === "number"; }
        }
        export const checker: Checker = new Checker();
        export const inferredChecker = new Checker();
        export default function isDefault(v: unknown): v is string { return typeof v === "string"; }
        export { isString as isStringAlias };
"#;

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m6/imports/main.ts` L6 to L15). A call of an imported
// type-guard function narrows through a named, default, namespace, or
// re-exported import, and through a method of an imported class or of an
// instance whose type the export infers, because a call in a condition takes
// its signature from the callee's ordinary type.
#[test]
fn imported_type_guard_functions_narrow() {
    assert_reads_with(
        r#"
        import { isString, isNumber, assertString, assert, fail, failInferred, isStr, assertArrow, assertAnnotated, failArrow, failAnnotated, Checker, checker, inferredChecker, isStringAlias } from "./guards";
        import isDefault from "./guards";
        import * as g from "./guards";
        declare function f<T>(): T;
        function i1(x: string | number) { if (isString(x)) /*L6_52*/x; else /*L6_74*/x; } // L6
        function i2(x: string | number) { if (isNumber(x)) /*L7_52*/x; } // L7
        function i3(x: string | number) { if (g.isString(x)) /*L8_54*/x; } // L8
        function i4(x: string | number) { if (isStr(x)) /*L9_49*/x; } // L9
        function i5(x: string | number) { if (isDefault(x)) /*L10_53*/x; } // L10
        function i6(x: string | number) { if (isStringAlias(x)) /*L11_57*/x; } // L11
        function i7(x: string | number) { if (checker.isString(x)) /*L12_60*/x; } // L12
        function i8(x: string | number) { if (inferredChecker.isString(x)) /*L13_68*/x; } // L13
        function i9(x: string | number) { if (Checker.isNumber(x)) /*L14_60*/x; } // L14
        function i10(x: string | number) { if (g.checker.isString(x)) /*L15_63*/x; } // L15
        "#,
        &[("/src/guards.ts", GUARDS)],
        &[
            ("L6_52", "x", &["string"]),
            ("L6_74", "x", &["number"]),
            ("L7_52", "x", &["number"]),
            ("L8_54", "x", &["string"]),
            ("L9_49", "x", &["string"]),
            ("L10_53", "x", &["string"]),
            ("L11_57", "x", &["string"]),
            ("L12_60", "x", &["string"]),
            ("L13_68", "x", &["string"]),
            ("L14_60", "x", &["number"]),
            ("L15_63", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m6/imports/main.ts` L16 to L23). A statement call
// asserts only when every name along its callee has an explicit type: an
// imported function, a function or a variable with a type annotation that a
// namespace import names, and a method of an annotated import do, while a
// variable that an unannotated arrow or `new` initializes doesn't (TS2775).
// Each function here has no condition of its own, so only its statement call
// selects it for flow.
#[test]
fn imported_assertions_follow_the_explicit_type_rule() {
    assert_reads_with(
        r#"
        import { isString, isNumber, assertString, assert, fail, failInferred, isStr, assertArrow, assertAnnotated, failArrow, failAnnotated, Checker, checker, inferredChecker, isStringAlias } from "./guards";
        import isDefault from "./guards";
        import * as g from "./guards";
        declare function f<T>(): T;
        function a1(x: string | number) { assertString(x); /*L16_52*/x; } // L16
        function a2(x: string | null) { assert(x); /*L17_44*/x; } // L17
        function a3(x: string | null) { g.assert(x); /*L18_46*/x; } // L18
        function a4(x: string | null) { assertArrow(x); /*L19_49*/x; } // L19
        function a5(x: string | null) { assertAnnotated(x); /*L20_53*/x; } // L20
        function a6(x: string | null) { checker.assert(x); /*L21_52*/x; } // L21
        function a7(x: string | null) { inferredChecker.assert(x); /*L22_60*/x; } // L22
        function a8(x: string | null) { g.checker.assert(x); /*L23_54*/x; } // L23
        "#,
        &[("/src/guards.ts", GUARDS)],
        &[
            ("L16_52", "x", &["string"]),
            ("L17_44", "x", &["string"]),
            ("L18_46", "x", &["string"]),
            ("L19_49", "x", &["null", "string"]),
            ("L20_53", "x", &["string"]),
            ("L21_52", "x", &["string"]),
            ("L22_60", "x", &["null", "string"]),
            ("L23_54", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m6/preds.ts` L16 to L24, L31, L32, L37, L39, L40, L44,
// L45): a type-guard function narrows through a namespace, an object type, a
// class instance or the class itself, an object literal, an alias, a
// variable or a parameter with a function type, an optional chain, a callee
// in parentheses, `this`, and a destructured method.
#[test]
fn dotted_and_aliased_type_guard_functions_narrow() {
    assert_reads(
        r#"
        declare function f<T>(): T;
        declare function isString(v: unknown): v is string;
        interface Leaf { kind: "leaf"; value: number }
        interface Branch { kind: "branch"; children: Tree[] }
        type Tree = Leaf | Branch;
        declare namespace N { function isS(v: unknown): v is string; }
        declare const u: { isS(v: unknown): v is string; nested: { isS(v: unknown): v is string } };
        class C { isS(v: unknown): v is string { return typeof v === "string"; } static isN(v: unknown): v is number { return typeof v === "number"; } }
        declare const c: C;
        const o = { isS(v: unknown): v is string { return typeof v === "string"; } };
        class TreeNode { isLeaf(): this is LeafNode { return false; } }
        class LeafNode extends TreeNode { value = 1; }
        class BranchNode extends TreeNode { kids: TreeNode[] = []; }
        function q1(x: string | number) { if (N.isS(x)) /*L16_49*/x; } // L16
        function q2(x: string | number) { if (u.isS(x)) /*L17_49*/x; } // L17
        function q3(x: string | number) { if (u.nested.isS(x)) /*L18_56*/x; } // L18
        function q4(x: string | number) { if (c.isS(x)) /*L19_49*/x; } // L19
        function q5(x: string | number) { if (C.isN(x)) /*L20_49*/x; } // L20
        function q6(x: string | number) { if (o.isS(x)) /*L21_49*/x; } // L21
        function q7(x: string | number) { const alias = isString; if (alias(x)) /*L22_73*/x; } // L22
        function q8(x: string | number) { const g: (v: unknown) => v is string = isString; if (g(x)) /*L23_94*/x; } // L23
        function q9(g: (v: unknown) => v is string, x: string | number) { if (g(x)) /*L24_77*/x; else /*L24_99*/x; } // L24
        function q16(x: string | number) { if (u?.isS(x)) /*L31_51*/x; } // L31
        function q17(x: string | number) { if ((u.isS)(x)) /*L32_52*/x; } // L32
        class D { isS(v: unknown): v is string { return typeof v === "string"; } m(x: string | number) { if (this.isS(x)) /*L37_115*/x; } } // L37
        function q22(x: string | number, g: (this: void, v: unknown) => v is string) { if (g(x)) /*L39_90*/x; } // L39
        function q23(x: string | number) { const { isS } = u; if (isS(x)) /*L40_67*/x; } // L40
        function q26(x: string | number) { if (!N.isS(x)) /*L44_51*/x; } // L44
        function q27(x: unknown) { if (u.isS(x)) /*L45_42*/x; } // L45
        "#,
        &[
            ("L16_49", "x", &["string"]),
            ("L17_49", "x", &["string"]),
            ("L18_56", "x", &["string"]),
            ("L19_49", "x", &["string"]),
            ("L20_49", "x", &["number"]),
            ("L21_49", "x", &["string"]),
            ("L22_73", "x", &["string"]),
            ("L23_94", "x", &["string"]),
            ("L24_77", "x", &["string"]),
            ("L24_99", "x", &["number"]),
            ("L31_51", "x", &["string"]),
            ("L32_52", "x", &["string"]),
            ("L37_115", "x", &["string"]),
            ("L39_90", "x", &["string"]),
            ("L40_67", "x", &["string"]),
            ("L44_51", "x", &["number"]),
            ("L45_42", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m6/preds.ts` L27 to L29, L38), except where a comment
// says otherwise. `this is T` narrows the object that the callee reads its
// method from.
#[test]
fn this_type_predicates_narrow_their_receiver() {
    assert_reads(
        r#"
        declare function f<T>(): T;
        declare function isString(v: unknown): v is string;
        interface Leaf { kind: "leaf"; value: number }
        interface Branch { kind: "branch"; children: Tree[] }
        type Tree = Leaf | Branch;
        declare namespace N { function isS(v: unknown): v is string; }
        declare const u: { isS(v: unknown): v is string; nested: { isS(v: unknown): v is string } };
        class C { isS(v: unknown): v is string { return typeof v === "string"; } static isN(v: unknown): v is number { return typeof v === "number"; } }
        declare const c: C;
        const o = { isS(v: unknown): v is string { return typeof v === "string"; } };
        class TreeNode { isLeaf(): this is LeafNode { return false; } }
        class LeafNode extends TreeNode { value = 1; }
        class BranchNode extends TreeNode { kids: TreeNode[] = []; }
        function q12(n: LeafNode | BranchNode) { if (n.isLeaf()) /*L27_58*/n; else /*L27_80*/n; } // L27
        function q13(n: TreeNode) { if (n.isLeaf()) /*L28_45*/n; } // L28
        function q14(o2: { n: LeafNode | BranchNode }) { if (o2.n.isLeaf()) /*L29_69*/o2.n; } // L29
        class E { kind: "e" = "e"; isE(): this is { tag: 1 } { return true; } m() { if (this.isE()) /*L38_93*/this; } } // L38
        "#,
        &[
            ("L27_58", "n", &["instanceof LeafNode"]),
            ("L27_80", "n", &["instanceof BranchNode"]),
            // TypeScript gives `LeafNode`, which Biome can't compare with the
            // declared `TreeNode`, as for other instances of classes.
            ("L28_45", "n", &["instanceof TreeNode"]),
            ("L29_69", "o2.n", &["instanceof LeafNode"]),
            // TypeScript gives `this & { tag: 1 }`: Biome keeps the declared type
            // where TypeScript gives an intersection.
            ("L38_93", "this", &["instanceof E"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m6/preds.ts` L25, L26, L30, L33 to L36, L41), except
// where a comment says otherwise. A callee of `any`, a union of callees, a
// spread argument, and `.call` narrow nothing, and an optional call narrows
// only where it is truthy.
#[test]
fn calls_without_a_single_signature_or_a_dotted_callee_keep_their_types() {
    assert_reads(
        r#"
        declare function f<T>(): T;
        declare function isString(v: unknown): v is string;
        interface Leaf { kind: "leaf"; value: number }
        interface Branch { kind: "branch"; children: Tree[] }
        type Tree = Leaf | Branch;
        declare namespace N { function isS(v: unknown): v is string; }
        declare const u: { isS(v: unknown): v is string; nested: { isS(v: unknown): v is string } };
        class C { isS(v: unknown): v is string { return typeof v === "string"; } static isN(v: unknown): v is number { return typeof v === "number"; } }
        declare const c: C;
        const o = { isS(v: unknown): v is string { return typeof v === "string"; } };
        class TreeNode { isLeaf(): this is LeafNode { return false; } }
        class LeafNode extends TreeNode { value = 1; }
        class BranchNode extends TreeNode { kids: TreeNode[] = []; }
        function q10(x: string | number) { if (u["isS"](x)) /*L25_53*/x; } // L25
        function q11(x: string | number, get: () => (v: unknown) => v is string) { if (get()(x)) /*L26_90*/x; } // L26
        function q15(x: string | number, g?: (v: unknown) => v is string) { if (g?.(x)) /*L30_81*/x; else /*L30_103*/x; } // L30
        function q18(x: string | number, a: any) { if (a.isS(x)) /*L33_58*/x; } // L33
        function q19(x: string | number, g: ((v: unknown) => v is string) | ((v: unknown) => v is number)) { if (g(x)) /*L34_112*/x; } // L34
        function q20(x: string | number, g: { (v: unknown): v is string; (v: unknown, w: unknown): boolean }) { if (g(x)) /*L35_115*/x; } // L35
        function q21(x: string | number) { if (isString(...[x])) /*L36_58*/x; } // L36
        function q24(x: string | number) { if (c.isS.call(c, x)) /*L41_58*/x; } // L41
        "#,
        &[
            // TypeScript gives `string`: an element access is no dotted name.
            ("L25_53", "x", &["number", "string"]),
            // TypeScript gives `string`: a call result is no dotted name.
            ("L26_90", "x", &["number", "string"]),
            ("L30_81", "x", &["string"]),
            ("L30_103", "x", &["number", "string"]),
            ("L33_58", "x", &["number", "string"]),
            ("L34_112", "x", &["number", "string"]),
            // TypeScript gives `string` by overload resolution, which Biome
            // doesn't do for call points.
            ("L35_115", "x", &["number", "string"]),
            ("L36_58", "x", &["number", "string"]),
            ("L41_58", "x", &["number", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m6/asserts.ts` L12 to L26, L30 to L36, L38 to L41),
// except where a comment says otherwise. A statement call asserts through a
// namespace, an object type, a class instance or the class itself, an
// annotated variable or parameter, an optional chain, and a callee in
// parentheses, as the whole statement or as an operand of the comma
// operator. A variable that `new` or an object literal initializes, a
// destructured method (TS2775), an element access (TS2776), a call that is no
// statement, and a callee whose type may be `undefined` assert nothing.
#[test]
fn statement_calls_assert_through_names_with_explicit_types() {
    assert_reads(
        r#"
        declare function f<T>(): T;
        declare function assert(v: unknown): asserts v;
        declare function assertString(v: unknown): asserts v is string;
        declare namespace A { function ok(v: unknown): asserts v; }
        declare const a: { ok(v: unknown): asserts v; is(v: unknown): asserts v is string };
        class Asserter { ok(v: unknown): asserts v { if (!v) throw 0; } static ok(v: unknown): asserts v { if (!v) throw 0; } }
        declare const as: Asserter;
        const inferred = new Asserter();
        const lit = { ok(v: unknown): asserts v { if (!v) throw 0; } };
        class R { ready?: string; check(): asserts this is { ready: string } { } m() { this.check(); /*L12_94*/this.ready; } } // L12
        class S { v: string | null = null; ensure(): asserts this is { v: string } { } m() { this.ensure(); /*L13_101*/this.v; } } // L13
        function s1(x: string | null) { A.ok(x); /*L15_42*/x; } // L15
        function s2(x: string | null) { a.ok(x); /*L16_42*/x; } // L16
        function s3(x: string | number) { a.is(x); /*L17_44*/x; } // L17
        function s4(x: string | null) { as.ok(x); /*L18_43*/x; } // L18
        function s5(x: string | null) { Asserter.ok(x); /*L19_49*/x; } // L19
        function s6(x: string | null) { inferred.ok(x); /*L20_49*/x; } // L20
        function s7(x: string | null) { lit.ok(x); /*L21_44*/x; } // L21
        function s8(x: string | null) { const k: (v: unknown) => asserts v = assert; k(x); /*L22_84*/x; } // L22
        function s9(x: string | null, k: (v: unknown) => asserts v) { k(x); /*L23_69*/x; } // L23
        function s10(x: string | null) { a["ok"](x); /*L24_46*/x; } // L24
        function s11(x: string | null) { (assert(x), 0); /*L25_50*/x; } // L25
        function s12(x: string | null) { (0, assert(x)); /*L26_50*/x; } // L26
        declare const r: R;
        function s16() { r.check(); /*L30_29*/r.ready; } // L30
        function s17(x: string | null) { a?.ok(x); /*L31_44*/x; } // L31
        function s18(x: string | null) { (a.ok)(x); /*L32_45*/x; } // L32
        function s19(o: { v: string | null }) { assert(o.v); /*L33_54*/o.v; } // L33
        function s20(x: string | null) { void assert(x); /*L34_50*/x; } // L34
        function s21(x: string | null) { const y = assert(x); /*L35_55*/x; } // L35
        function s22(x: string | null) { assert(x), assert(x); /*L36_56*/x; } // L36
        function s24(x: string | null, k: ((v: unknown) => asserts v) | undefined) { k?.(x); /*L38_86*/x; } // L38
        function s25(x: string | null) { const { ok } = a; ok(x); /*L39_59*/x; } // L39
        function s26(x: unknown) { assertString(x); /*L40_45*/x; } // L40
        function s27(x: string | null) { assert(x !== null && x.length > 0); /*L41_70*/x; } // L41
        "#,
        &[
            // TypeScript gives `string` through `this & { ready: string }`:
            // Biome keeps the declared type where TypeScript gives an
            // intersection.
            ("L12_94", "this.ready", &["string", "undefined"]),
            // TypeScript gives `string`, as above.
            ("L13_101", "this.v", &["null", "string"]),
            ("L15_42", "x", &["string"]),
            ("L16_42", "x", &["string"]),
            ("L17_44", "x", &["string"]),
            ("L18_43", "x", &["string"]),
            ("L19_49", "x", &["string"]),
            ("L20_49", "x", &["null", "string"]),
            ("L21_44", "x", &["null", "string"]),
            ("L22_84", "x", &["string"]),
            ("L23_69", "x", &["string"]),
            ("L24_46", "x", &["null", "string"]),
            ("L25_50", "x", &["string"]),
            ("L26_50", "x", &["string"]),
            // TypeScript gives `string`, as above.
            ("L30_29", "r.ready", &["string", "undefined"]),
            ("L31_44", "x", &["string"]),
            ("L32_45", "x", &["string"]),
            ("L33_54", "o.v", &["string"]),
            ("L34_50", "x", &["null", "string"]),
            ("L35_55", "x", &["null", "string"]),
            ("L36_56", "x", &["string"]),
            ("L38_86", "x", &["null", "string"]),
            ("L39_59", "x", &["null", "string"]),
            ("L40_45", "x", &["string"]),
            ("L41_70", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) keeps the declared type after a statement
// call in parentheses, which its binder doesn't add to the flow
// (`narrowing-probes/m6/literals.ts` L14, L15).
#[test]
fn parenthesized_statement_calls_assert_nothing() {
    assert_reads(
        r#"
        declare function f<T>(): T;
        declare function assert(v: unknown): asserts v;
        declare function assertString(v: unknown): asserts v is string;
        declare const c: boolean;
        function t8(x: string | null) { (assert(x)); /*L14_46*/x; } // L14
        function t9(x: string | number) { ((assertString(x))); /*L15_56*/x; } // L15
        "#,
        &[
            ("L14_46", "x", &["null", "string"]),
            ("L15_56", "x", &["number", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m6/lib.ts` L4 to L14), except where a comment says
// otherwise. `Array.isArray` and `ArrayBuffer.isView` narrow as type-guard
// functions of the lib.
#[test]
fn library_type_guard_functions_narrow() {
    assert_reads(
        r#"
        declare function f<T>(): T;
        function l1(x: string | string[]) { if (Array.isArray(x)) /*L4_59*/x; else /*L4_81*/x; } // L4
        function l2(x: unknown) { if (Array.isArray(x)) /*L5_49*/x; } // L5
        function l3(x: string | readonly string[]) { if (Array.isArray(x)) /*L6_68*/x; else /*L6_90*/x; } // L6
        function l4(x: number[] | { length: number }) { if (Array.isArray(x)) /*L7_71*/x; else /*L7_93*/x; } // L7
        function l5(x: string | number[] | null) { if (!Array.isArray(x)) /*L8_67*/x; } // L8
        function l6(x: string | string[]) { const isArray = Array.isArray; if (isArray(x)) /*L9_84*/x; } // L9
        function l7(o: { v: string | string[] }) { if (Array.isArray(o.v)) /*L10_68*/o.v; } // L10
        function l8(x: ArrayBuffer | Uint8Array) { if (ArrayBuffer.isView(x)) /*L11_71*/x; else /*L11_93*/x; } // L11
        function l9(x: string | number) { if (Number.isInteger(x)) /*L12_60*/x; } // L12
        function l10(x: any) { if (Array.isArray(x)) /*L13_46*/x; } // L13
        function l11(x: string | [number, number]) { if (Array.isArray(x)) /*L14_68*/x; } // L14
        "#,
        &[
            ("L4_59", "x", &["instanceof Array<string>"]),
            ("L4_81", "x", &["string"]),
            ("L5_49", "x", &["instanceof Array<any>"]),
            // TypeScript gives `any[]`, which every member that Biome keeps
            // accepts.
            (
                "L6_68",
                "x",
                &["Readonly instanceof Array<string>", "string"],
            ),
            (
                "L6_90",
                "x",
                &["Readonly instanceof Array<string>", "string"],
            ),
            (
                "L7_71",
                "x",
                &["instanceof Array<any>", "instanceof Array<number>"],
            ),
            (
                "L7_93",
                "x",
                &[r#"Object { prototype: No prototype members: ["length": number] }"#],
            ),
            ("L8_67", "x", &["null", "string"]),
            ("L9_84", "x", &["instanceof Array<string>"]),
            ("L10_68", "o.v", &["instanceof Array<string>"]),
            ("L11_71", "x", &["instanceof Uint8Array"]),
            ("L11_93", "x", &["instanceof ArrayBuffer"]),
            ("L12_60", "x", &["number", "string"]),
            // TypeScript gives `any[]`: Biome narrows no `any`.
            ("L13_46", "x", &["any"]),
            ("L14_68", "x", &["[number, number]"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) narrows the operand after a call that is
// an operand of the comma operator, also inside the same expression
// statement, and also in a `return` statement
// (`narrowing-probes/m6-l1/existing.ts` L7, `m6-l1/comma.ts` L7). Biome adds
// only the operands of an expression statement to the flow, so the read in
// `return` keeps its declared type, which is wider.
#[test]
fn comma_operands_of_expression_statements_assert() {
    assert_reads(
        r#"
        declare function assertString(input: unknown): asserts input is string;
        function statement(value: string | number) { (assertString(value), /*statement*/value); }
        function returned(value: string | number) { return (assertString(value), /*returned*/value); }
        "#,
        &[
            ("statement", "value", &["string"]),
            ("returned", "value", &["number", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string` in the branch and
// `string | number` after it (`narrowing-probes/m6/preds.ts` L17). Lookups
// resolve the effect of a call point once, through a tracked query that the
// reads of every subject it may narrow share, and infer no module
// completely.
#[test]
fn lookups_resolve_each_call_point_once() {
    const SOURCE: &str = r#"
        declare const u: { isS(v: unknown): v is string };
        function f(x: string | number) {
            if (u.isS(x)) { /*then*/x; }
            /*after*/x;
        }
    "#;
    let (db, module) = narrowing_db_with(SOURCE, &[]);
    db.clear_salsa_events();
    for (marker, expected) in [("then", &["string"][..]), ("after", &["number", "string"])] {
        let range = marked_range(SOURCE, marker, "x");
        let ty = execute_type_inference_request(
            &db,
            TypeInferenceCaller::new("test", "callPoints"),
            NormalizedExpressionTypeRequest::new(module, range),
        )
        .unwrap();
        assert_eq!(variant_names(&db, ty), expected, "{marker}");
    }
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_call_effect", &events),
        1
    );
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_module_types", &events),
        0
    );
    // Complete module inference resolves the same effect in its own context.
    assert_reads(
        SOURCE,
        &[
            ("then", "x", &["string"]),
            ("after", "x", &["number", "string"]),
        ],
    );
}
