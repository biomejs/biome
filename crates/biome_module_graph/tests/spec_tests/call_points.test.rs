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

/// Names each member of `ty`, sorted: an interface by its name, Biome's
/// unknown type as `Unknown`, and any other type by its formatted text on one
/// line, such as `string`, `null`, or `instanceof LeafNode`. A read on a path
/// that no value takes has the unknown type.
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

/// Asserts the type of each `(marker, expression, expected)` read in `source`
/// by the names of its members, from [`variant_names`]. `modules` are the
/// other modules, as [`narrowing_db_with`] takes them. Also asserts that
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

/// The module at `/src/guards.ts`. It exports type-guard functions, assertion
/// functions, and functions that never return, each declared in several
/// forms for the import tests.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A call of an imported type-guard function narrows through a named, default,
// namespace, or re-exported import, and through a method of an imported class
// or of an instance whose type the export infers, because a call in a
// condition takes its signature from the callee's ordinary type.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A statement call asserts only when every name along its callee has an
// explicit type. An imported function, a function or a variable with a type
// annotation that a namespace import names, and a method of an annotated
// import have one, while a variable that an unannotated arrow or `new`
// initializes doesn't (TS2775).
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A type-guard function narrows through a namespace, an object type, a
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. `this is T` narrows the object that
// the callee reads its method from.
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
            ("L28_45", "n", &["instanceof LeafNode"]),
            ("L29_69", "o2.n", &["instanceof LeafNode"]),
            // TypeScript gives `this & { tag: 1 }`. Normalization merges the
            // operands of Biome's `E & { tag: 1 }` into an instance of an
            // anonymous class.
            (
                "L38_93",
                "this",
                &["instanceof class { extends: none implements: [] type_args: [] }"],
            ),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. A callee of `any`, a union of
// callees, a spread argument, and `.call` narrow nothing, and an optional call
// narrows only where it is truthy.
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
        function q21(x: string | number) { if (isString(...[x])) /*L36_58*/x; } // L36
        function q24(x: string | number) { if (c.isS.call(c, x)) /*L41_58*/x; } // L41
        "#,
        &[
            // TypeScript gives `string`. Biome narrows nothing, because an
            // element access is no dotted name.
            ("L25_53", "x", &["number", "string"]),
            // TypeScript gives `string`. Biome narrows nothing, because a
            // call result is no dotted name.
            ("L26_90", "x", &["number", "string"]),
            ("L30_81", "x", &["string"]),
            ("L30_103", "x", &["number", "string"]),
            ("L33_58", "x", &["number", "string"]),
            ("L34_112", "x", &["number", "string"]),
            ("L36_58", "x", &["number", "string"]),
            ("L41_58", "x", &["number", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
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
            // The property of `this & { ready: string }`.
            ("L12_94", "this.ready", &["string"]),
            ("L13_101", "this.v", &["string"]),
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
            ("L30_29", "r.ready", &["string"]),
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
// call in parentheses, because it doesn't add such a call to the flow.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. `Array.isArray` and
// `ArrayBuffer.isView` narrow as type-guard functions of the lib.
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
            // No member is related to `any[]`, the target, which is
            // assignable to the type.
            ("L6_68", "x", &["instanceof Array<any>"]),
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
            ("L13_46", "x", &["instanceof Array<any>"]),
            ("L14_68", "x", &["[number, number]"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) narrows the operand after a call that is
// an operand of the comma operator, also inside the same expression
// statement, and also in a `return` statement. Biome adds only the operands
// of an expression statement to the flow, so the read in `return` keeps its
// declared type, which is wider.
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
// `string | number` after it. Lookups resolve the effect of a call point once,
// through a tracked query that the reads of every subject it may narrow
// share, and infer no module completely.
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

/// Declarations that the tests of generic and overloaded type predicates
/// share.
const GENERIC: &str = r#"
        declare function f<T>(): T;
        declare function isNonNull<T>(v: T | null): v is T;
        declare function isDefined<T>(v: T | null | undefined): v is T;
        declare function isOf<T>(v: unknown, sample: T): v is T;
        declare function isNotNullish<T>(v: T): v is NonNullable<T>;
        declare function assertDefined<T>(v: T): asserts v is NonNullable<T>;
        declare function isS(v: string | number): v is string;
        declare function isS(v: boolean): boolean;
        declare function isB(v: boolean): boolean;
        declare function isB(v: string | number): v is string;
        declare function isString(v: unknown): v is string;
        declare function isNumber(v: unknown): v is number;
"#;

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. A generic type-guard function
// narrows by the type arguments that the call infers, and an overloaded one by
// the first signature that accepts the arguments. `every` narrows its object
// through `this is S[]`, with `S` inferred from the type predicate of its
// callback.
#[test]
fn generic_and_overloaded_type_predicates_narrow() {
    let source = format!(
        "{GENERIC}{}",
        r#"
        function g1(x: string | null) { if (isNonNull(x)) /*L15_53*/x; else /*L15_75*/x; } // L15
        function g2(x: string | null | undefined) { if (isDefined(x)) /*L16_65*/x; } // L16
        function g3(x: string | number) { if (isOf(x, "a")) /*L17_55*/x; } // L17
        function g4(x: string | null) { if (isNotNullish(x)) /*L18_56*/x; } // L18
        function g5(x: string | null) { assertDefined(x); /*L19_53*/x; } // L19
        function g6(x: string | null) { if (isNonNull<string>(x)) /*L20_61*/x; } // L20
        function g7(x: string | number) { if (isS(x)) /*L21_49*/x; } // L21
        function g8(x: string | number) { if (isB(x)) /*L22_49*/x; } // L22
        function g9(xs: (string | number)[]) { if (xs.every(isString)) /*L23_66*/xs; else /*L23_89*/xs; } // L23
        function g10(xs: (string | number)[]) { if (xs.every((v): v is string => typeof v === "string")) /*L24_100*/xs; } // L24
        function g11(xs: (string | number)[]) { if (xs.every((v) => typeof v === "string")) /*L25_87*/xs; } // L25
        function g12(xs: readonly (string | number)[]) { if (xs.every(isString)) /*L26_76*/xs; } // L26
        function g15(xs: (string | number)[]) { if (xs.some(isString)) /*L29_66*/xs; } // L29
        function g16(t: [string | number, string | number]) { if (t.every(isString)) /*L30_80*/t; } // L30
        function g17(xs: (string | number)[] | null) { if (xs?.every(isString)) /*L31_75*/xs; } // L31
        function g18(x: string | number | boolean) { if (isS(x as string | number)) /*L32_79*/x; } // L32
        function g19<T>(x: T | null) { if (isNonNull(x)) /*L33_52*/x; } // L33
        function g20(xs: (string | number)[]) { if (xs.every(isString) && xs.every(isNumber)) /*L34_89*/xs; } // L34
        function g21(xs: (string | number)[]) { if (xs.filter(isString).length) /*L35_75*/xs; } // L35
        "#
    );
    assert_reads(
        &source,
        &[
            ("L15_53", "x", &["string"]),
            ("L15_75", "x", &["null"]),
            ("L16_65", "x", &["string"]),
            ("L17_55", "x", &["string: a"]),
            ("L18_56", "x", &["string"]),
            ("L19_53", "x", &["string"]),
            ("L20_61", "x", &["string"]),
            ("L21_49", "x", &["string"]),
            ("L22_49", "x", &["string"]),
            ("L23_66", "xs", &["instanceof Array<string>"]),
            ("L23_89", "xs", &["instanceof Array<string | number>"]),
            // TypeScript gives `string[]`. Biome types the callback's
            // parameter `v` from no annotation, so it can't tell whether the
            // first signature of `every` accepts the callback.
            ("L24_100", "xs", &["instanceof Array<string | number>"]),
            // TypeScript gives `string[]` through the type predicate that it
            // infers for the callback, which Biome doesn't infer.
            ("L25_87", "xs", &["instanceof Array<string | number>"]),
            ("L26_76", "xs", &["Readonly instanceof Array<string>"]),
            ("L29_66", "xs", &["instanceof Array<string | number>"]),
            (
                "L30_80",
                "t",
                &["[string | number, string | number] & instanceof Array<string>"],
            ),
            ("L31_75", "xs", &["instanceof Array<string>"]),
            ("L32_79", "x", &["boolean", "number", "string"]),
            ("L33_52", "x", &["instanceof T"]),
            ("L34_89", "xs", &["instanceof Array<string>"]),
            ("L35_75", "xs", &["instanceof Array<string | number>"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. Overload selection compares an
// argument whose type is a type parameter of the calling function as
// TypeScript does there: under the subtype relation, the type parameter
// relates only through its constraint, so with `T extends string`, `t`
// selects a first signature that takes a `string`.
#[test]
fn overload_selection_relates_type_parameters_through_their_constraints() {
    assert_reads(
        r#"
        declare function g(v: unknown, w: string): v is "a";
        declare function g(v: unknown, w: unknown): v is string;
        function o1<T>(x: string | number, t: T) { if (g(x, t)) /*unconstrained*/x; }
        declare function h(v: unknown, w: string): v is number;
        declare function h(v: unknown, w: unknown): v is string;
        function o2<T extends string>(x: string | number, t: T) { if (h(x, t)) /*constrained*/x; }
        declare function k<U>(v: unknown, w: U): v is string;
        function o3<T>(x: string | number, t: T) { if (k(x, t)) /*inferred*/x; }
        "#,
        &[
            // TypeScript gives `string`, because `T` doesn't relate to
            // `string`. Biome keeps the declared type, which is wider: it
            // skips a signature only when no narrowing of the arguments could
            // make it accept them, and `typeof t === "string"` would give `t`
            // the type `T & string`, which the first signature accepts.
            ("unconstrained", "x", &["number", "string"]),
            ("constrained", "x", &["number"]),
            ("inferred", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. The call's arguments select the
// signature with their types at the call, which a test before the call can
// narrow. A signature before the one that accepts the arguments, which
// Biome can't skip whatever narrowing gives them, leaves the call without a
// signature.
#[test]
fn overload_selection_uses_argument_types_at_the_call() {
    assert_reads(
        r#"
        declare function isNonNull<T>(v: T | null): v is T;
        function a1(x: string | null | undefined) { if (isNonNull(x)) /*a1then*/x; else /*a1else*/x; }
        class P1 { a = 1 } class P2 { a = 1 }
        declare function k(v: unknown, c: P1): v is string;
        declare function k(v: unknown, c: unknown): v is number;
        function a2(x: string | number, c: P2) { if (k(x, c)) /*a2*/x; }
        function a3(x: "a" | "b" | null, c: boolean) { while (c) { if (isNonNull(x)) { /*a3*/x; } } }
        declare const m: { is(v: string): v is "a"; is(v: number): v is 1 };
        function a8(x: "a" | "b") { if (m.is(x)) /*a8*/x; }
        function a8b(x: 1 | 2) { if (m.is(x)) /*a8b*/x; }
        declare function isS(v: string): v is "a";
        declare function isS(v: string | number): v is string;
        function a10(x: "a" | "b" | number) { if (typeof x === "string") { if (isS(x)) /*a10*/x; } }
        declare function isArrOf<T>(v: unknown, sample: T): v is T[];
        function a11(x: string[] | number[]) { if (isArrOf(x, "a")) /*a11*/x; }
        function a15(x: string | null) { if (!isNonNull(x)) return; /*a15*/x; }
        declare function isNonNullO<T>(v: T | null): v is T;
        declare function isNonNullO(v: unknown, w: unknown): boolean;
        function a16(x: string | null) { if (isNonNullO(x)) /*a16*/x; }
        declare function guard(input: string): input is string;
        declare function guard(input: unknown): input is number;
        function a17(x: string | number) { if (guard(x)) /*a17then*/x; else /*a17else*/x; }
        declare function unused<T>(input: unknown): input is string;
        function a18(x: string | number) { if (unused(x)) /*a18*/x; }
        declare function plain(input: unknown): input is string;
        function a19(x: string | number) { if (plain<unknown>(x)) /*a19*/x; }
        declare function f(x: string, y: string): x is "a";
        declare function f(x: string, y: unknown): x is string;
        function a20(x: "a" | "b" | null, y: any) { if (typeof x === "string" && f(x, y)) /*a20*/x; }
        declare function g(x: unknown, o: { a?: number }): x is "a";
        declare function g(x: unknown, o: {}): x is string;
        function a21(x: "a" | "b" | 1, o: {}) { if (g(x, o)) /*a21*/x; }
        "#,
        &[
            ("a1then", "x", &["string", "undefined"]),
            ("a1else", "x", &["null"]),
            // TypeScript gives `string`, because `P2` has the members of
            // `P1`. Biome can't tell that an instance of one class is
            // assignable to another, which can declare private members.
            ("a2", "x", &["number", "string"]),
            ("a3", "x", &["string: a", "string: b"]),
            ("a8", "x", &["string: a"]),
            ("a8b", "x", &["number: 1"]),
            ("a10", "x", &["string: a"]),
            ("a11", "x", &["instanceof Array<string>"]),
            ("a15", "x", &["string"]),
            ("a16", "x", &["string"]),
            // TypeScript gives `number` and `string`. A test that Biome
            // doesn't decompose could narrow `x` to `string`, which the first
            // signature accepts.
            ("a17then", "x", &["number", "string"]),
            ("a17else", "x", &["number", "string"]),
            ("a18", "x", &["string"]),
            // TypeScript gives `string`, and reports TS2558 for a type
            // argument of a function without type parameters. Biome keeps the
            // declared type when a call passes type arguments to a local
            // type-guard function without type parameters.
            ("a19", "x", &["number", "string"]),
            // TypeScript takes the second signature, because `any` is a
            // subtype only of `any` and `unknown`. Biome takes none, because
            // a test could narrow `y` to `string`, so `x` keeps its type,
            // which is the same here.
            ("a20", "x", &["string: a", "string: b"]),
            // TypeScript gives `"a" | "b"` through the second signature,
            // because `{}` is not a subtype of `{ a?: number }`. A test that
            // Biome doesn't decompose could narrow `o` to a type that the
            // first signature accepts.
            ("a21", "x", &["number: 1", "string: a", "string: b"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. TypeScript selects the signature by
// the argument types at the call, which a test that Biome doesn't decompose
// can make narrower than Biome's, as `isStr`, whose type predicate TypeScript
// infers, does for `x`. Biome skips a signature only when no narrowing of the
// arguments can make it accept them, so a call that depends on such a
// narrowing narrows nothing, and a call with a type argument inferred from an
// argument other than the one that it narrows narrows only where it is
// truthy.
#[test]
fn overload_selection_skips_only_signatures_that_reject_every_narrowing() {
    assert_reads(
        r#"
        const isStr = (v: unknown) => typeof v === "string";
        declare function g(v: string): v is "a";
        declare function g(v: string | number): v is string;
        function s1(x: string | number) { if (isStr(x) && !g(x)) /*s1*/x; }
        function s2(x: string | number) { if (isStr(x) && g(x)) /*s2*/x; }
        declare function h(v: string): v is "a";
        declare function h(v: string | number): v is string;
        function s3(x: "a" | "b") { if (h(x)) /*s3then*/x; else /*s3else*/x; }
        declare function m(v: number): v is 1;
        declare function m(v: string): v is "a";
        function s4(x: "a" | "b") { if (m(x)) /*s4then*/x; else /*s4else*/x; }
        class A { a = 1; }
        const isA = (v: unknown) => v instanceof A;
        declare function k(x: unknown, o: { a?: number }): x is "a";
        declare function k(x: unknown, o: {}): x is string;
        function s5(x: "a" | "b" | 1, o: {}) { if (isA(o) && !k(x, o)) /*s5*/x; }
        function s6(x: "a" | "b" | 1, o: {}) { if (isA(o) && k(x, o)) /*s6*/x; }
        declare function q(v: "a"): v is "a";
        declare function q(v: string): v is "b";
        declare const K: "a";
        function s7(x: string) { if (x === K && q(x)) /*s7*/x; }
        function s8(x: string) { if (q(x)) /*s8then*/x; else /*s8else*/x; }
        declare function isOf<T>(v: unknown, sample: T): v is T;
        function s9(x: string | number | boolean, y: string | number) { if (isStr(y) && !isOf(x, y)) /*s9*/x; }
        function s10(x: string | number | boolean, y: string | number) { if (isStr(y) && isOf(x, y)) /*s10*/x; }
        declare function same(input: string): input is string;
        declare function same(input: unknown): input is string;
        function s11(x: string | null) { if (same(x)) /*s11then*/x; else /*s11else*/x; }
        "#,
        &[
            // TypeScript gives `string`. Biome doesn't infer the type
            // predicate of `isStr`, so `x` may be `number` at the call, and
            // the first signature of `g` accepts the `string` that
            // TypeScript sees.
            ("s1", "x", &["number", "string"]),
            // TypeScript gives `"a"`.
            ("s2", "x", &["number", "string"]),
            // The first signature accepts every member of `x`.
            ("s3then", "x", &["string: a"]),
            ("s3else", "x", &["string: b"]),
            // No narrowing of `x` gives a `number`, so `m` takes its second
            // signature.
            ("s4then", "x", &["string: a"]),
            ("s4else", "x", &["string: b"]),
            // TypeScript gives `"b" | 1`, because `isA`, whose type
            // predicate TypeScript infers, narrows `o` to `A`, which the
            // first signature accepts.
            ("s5", "x", &["number: 1", "string: a", "string: b"]),
            // TypeScript gives `"a"`.
            ("s6", "x", &["number: 1", "string: a", "string: b"]),
            // TypeScript gives `"a"`, because the comparison with `K`
            // narrows `x` to `"a"`, which the first signature of `q`
            // accepts.
            ("s7", "x", &["string"]),
            // TypeScript gives `"b"`. A test that Biome doesn't decompose
            // could narrow `x` to `"a"` before the call, as in `s7`.
            ("s8then", "x", &["string"]),
            ("s8else", "x", &["string"]),
            // TypeScript gives `number | boolean`. `T` is `string` in
            // TypeScript, while Biome infers `string | number` from `y`.
            ("s9", "x", &["boolean", "number", "string"]),
            // TypeScript gives `string`.
            ("s10", "x", &["number", "string"]),
            // Both signatures narrow alike, so it doesn't matter which one
            // TypeScript selects.
            ("s11then", "x", &["string"]),
            ("s11else", "x", &["null"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. TypeScript keeps the literal type
// that a type parameter infers from an argument when the type parameter has
// a primitive or literal constraint, when the argument's literal type isn't
// fresh, as that of a declared or annotated variable, and when the type
// parameter is at the top level of the type predicate, also through an
// intersection. Otherwise it widens the literal type, as `"a"` to `string`.
#[test]
fn literal_type_arguments_widen_as_typescript_widens_them() {
    assert_reads(
        r#"
        declare function hasKind<K extends string>(v: unknown, kind: K): v is { kind: K };
        function l1(s: { kind: "a"; x: 1 } | { kind: "b"; y: 2 } | number) { if (hasKind(s, "a")) /*l1then*/s; else /*l1else*/s; }
        declare function hasTag<K>(v: unknown, tag: K): v is { tag: K };
        declare function hasTagIn<K extends "a" | "b">(v: unknown, tag: K): v is { tag: K };
        function l2(s: { tag: "a" } | { tag: "b" } | number) { if (hasTagIn(s, "a")) /*l2then*/s; else /*l2else*/s; }
        function l3(s: { tag: "a" } | { tag: "b" } | number) { if (hasTag(s, "a")) /*l3then*/s; else /*l3else*/s; }
        declare const lit: "a";
        function l4(s: { tag: "a" } | { tag: "b" } | number) { if (hasTag(s, lit)) /*l4then*/s; else /*l4else*/s; }
        function l5(s: { tag: "a" } | { tag: "b" } | number, y: "a") { if (hasTag(s, y)) /*l5then*/s; else /*l5else*/s; }
        declare function isK<K>(v: unknown, k: K): v is K & string;
        function l6(s: "a" | "b" | number) { if (isK(s, "a")) /*l6then*/s; else /*l6else*/s; }
        function l7(s: { tag: "a" } | { tag: "b" } | number, y: string) { if (y === "a" && hasTag(s, y)) /*l7*/s; }
        function l8(s: { tag: "a" } | { tag: "b" } | number, y: string) { if (y === "a" && !hasTag(s, y)) /*l8*/s; }
        const c = "a";
        function l9(s: { tag: "a" } | { tag: "b" } | number) { if (hasTag(s, c)) /*l9then*/s; else /*l9else*/s; }
        "#,
        &[
            (
                "l1then",
                "s",
                &[
                    "Object { prototype: No prototype members: [\"kind\": string: a, \"x\": number: 1] }",
                ],
            ),
            (
                "l1else",
                "s",
                &[
                    "Object { prototype: No prototype members: [\"kind\": string: b, \"y\": number: 2] }",
                    "number",
                ],
            ),
            (
                "l2then",
                "s",
                &["Object { prototype: No prototype members: [\"tag\": string: a] }"],
            ),
            (
                "l2else",
                "s",
                &[
                    "Object { prototype: No prototype members: [\"tag\": string: b] }",
                    "number",
                ],
            ),
            (
                "l3then",
                "s",
                &[
                    "Object { prototype: No prototype members: [\"tag\": string: a] }",
                    "Object { prototype: No prototype members: [\"tag\": string: b] }",
                ],
            ),
            ("l3else", "s", &["number"]),
            (
                "l4then",
                "s",
                &["Object { prototype: No prototype members: [\"tag\": string: a] }"],
            ),
            (
                "l4else",
                "s",
                &[
                    "Object { prototype: No prototype members: [\"tag\": string: b] }",
                    "number",
                ],
            ),
            (
                "l5then",
                "s",
                &["Object { prototype: No prototype members: [\"tag\": string: a] }"],
            ),
            (
                "l5else",
                "s",
                &[
                    "Object { prototype: No prototype members: [\"tag\": string: b] }",
                    "number",
                ],
            ),
            ("l6then", "s", &["string: a"]),
            ("l6else", "s", &["number", "string: b"]),
            (
                "l7",
                "s",
                &[
                    "Object { prototype: No prototype members: [\"tag\": string: a] }",
                    "Object { prototype: No prototype members: [\"tag\": string: b] }",
                ],
            ),
            // TypeScript gives `number`, because the comparison gives `y`
            // a fresh literal type. Biome can't tell whether a narrowed
            // literal type is fresh, so a false call removes nothing.
            (
                "l8",
                "s",
                &[
                    "Object { prototype: No prototype members: [\"tag\": string: a] }",
                    "Object { prototype: No prototype members: [\"tag\": string: b] }",
                    "number",
                ],
            ),
            (
                "l9then",
                "s",
                &[
                    "Object { prototype: No prototype members: [\"tag\": string: a] }",
                    "Object { prototype: No prototype members: [\"tag\": string: b] }",
                ],
            ),
            ("l9else", "s", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. A statement call of a generic or an
// overloaded assertion function asserts through the signature that the
// arguments select.
#[test]
fn generic_and_overloaded_assertions_narrow() {
    assert_reads(
        r#"
        declare function assertIsDefined<T>(v: T | undefined): asserts v is T;
        function a6(x: string | undefined) { assertIsDefined(x); /*a6*/x; }
        declare function truthy<T>(input: unknown): asserts input;
        function a7(x: string | null) { truthy(x); /*a7*/x; }
        declare function check(input: string): asserts input;
        declare function check(input: unknown): asserts input is string;
        function a9(x: string | null) { check(x); /*a9*/x; }
        "#,
        &[
            ("a6", "x", &["string"]),
            ("a7", "x", &["string"]),
            // TypeScript gives `string` through the second signature. A test
            // that Biome doesn't decompose could narrow `x` to `string`,
            // which the first signature accepts.
            ("a9", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string` in each read. An imported
// generic type-guard function narrows as a local one does.
#[test]
fn imported_generic_type_guard_functions_narrow() {
    assert_reads_with(
        r#"
        import { isNonNull, assertIsDefined } from "./generic";
        function a(x: string | null) { if (isNonNull(x)) /*condition*/x; }
        function b(x: string | undefined) { assertIsDefined(x); /*statement*/x; }
        "#,
        &[(
            "/src/generic.ts",
            r#"
            export declare function isNonNull<T>(v: T | null): v is T;
            export declare function assertIsDefined<T>(v: T | undefined): asserts v is T;
            "#,
        )],
        &[
            ("condition", "x", &["string"]),
            ("statement", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every call below the expected type,
// except where a comment says otherwise. `filter` and `find` infer `S` from
// the type predicate of their callback, when `S` satisfies its constraint,
// `S extends T`, with `T` the element type of the array.
#[test]
fn callback_type_predicates_infer_call_results() {
    assert_reads(
        r#"
        declare function isString(v: unknown): v is string;
        function g13(xs: (string | number)[]) { /*L27_43*/xs.filter(isString); } // L27
        function g14(xs: (string | number)[]) { /*L28_43*/xs.find(isString); } // L28
        function a12(xs: (string | number)[]) { const s = xs.filter(isString); /*a12*/s; }
        function a13(xs: (string | number)[]) { /*a13*/xs.filter((v): v is string => typeof v === "string"); }
        function a4(nums: number[]) { /*a4*/nums.filter(isString); }
        "#,
        &[
            (
                "L27_43",
                "xs.filter(isString)",
                &["instanceof Array<string>"],
            ),
            ("L28_43", "xs.find(isString)", &["string", "undefined"]),
            ("a12", "s", &["instanceof Array<string>"]),
            (
                "a13",
                "xs.filter((v): v is string => typeof v === \"string\")",
                &["instanceof Array<string>"],
            ),
            // TypeScript gives `number[]` through the second signature of
            // `filter`, because `string` doesn't satisfy `S extends number`.
            // Biome selects the first signature for the call's result and
            // infers nothing for `S` there.
            (
                "a4",
                "nums.filter(isString)",
                &["instanceof Array<S extends T>"],
            ),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `number[]` in every read below. A
// constraint that names a type parameter of the array, as `S extends T` in
// `filter` and `every`, reads it from the array's type at the call, which
// `xs.every(isNumber)` narrows to `number[]`, so `S = string` violates it.
#[test]
fn constraints_read_the_receiver_at_the_call() {
    assert_reads(
        r#"
        declare function isNumber(v: unknown): v is number;
        declare function isString(v: unknown): v is string;
        function r1(xs: (string | number)[]) { if (xs.every(isNumber)) /*r1*/xs.filter(isString); }
        function r2(xs: (string | number)[]) { if (xs.every(isNumber) && xs.every(isString)) /*r2*/xs; }
        function r3(xs: (string | number)[]) { if (xs.every(isNumber) && !xs.every(isString)) /*r3*/xs; }
        function r4(xs: (string | number)[]) { if (xs.every(isNumber)) { const r = xs.filter(isString); /*r4*/r; } }
        "#,
        &[
            // Biome selects the first signature of `filter` for the call's
            // result and infers nothing for `S` there.
            (
                "r1",
                "xs.filter(isString)",
                &["instanceof Array<S extends T>"],
            ),
            ("r2", "xs", &["instanceof Array<number>"]),
            ("r3", "xs", &["instanceof Array<number>"]),
            // Narrower than TypeScript: the type of `r` comes from its
            // initializer without narrowing, where `xs` is
            // `(string | number)[]`, so `S = string` satisfies `S extends T`.
            ("r4", "r", &["instanceof Array<string>"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string` in the branch and
// `string | null` after it. Lookups resolve the effect of a generic call point
// once, through the tracked query that the reads of the subject share, and
// infer no module completely to type its arguments.
#[test]
fn lookups_resolve_each_generic_call_point_once() {
    const SOURCE: &str = r#"
        declare function isNonNull<T>(v: T | null): v is T;
        function f(x: string | null) {
            if (isNonNull(x)) { /*then*/x; }
            /*after*/x;
        }
    "#;
    let (db, module) = narrowing_db_with(SOURCE, &[]);
    db.clear_salsa_events();
    for (marker, expected) in [("then", &["string"][..]), ("after", &["null", "string"])] {
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
    assert_reads(
        SOURCE,
        &[
            ("then", "x", &["string"]),
            ("after", "x", &["null", "string"]),
        ],
    );
}
