use super::*;
use biome_module_graph::{
    ExpressionTypeInput, infer_expression_type,
    type_inference::{
        NormalizedExpressionTypeRequest, TypeInferenceCaller, TypeInferenceClassification,
        UnreachableCodeClassificationRequest, execute_type_inference_request,
    },
};

// Expected types are those of TypeScript 7.0.2 (`tsc --noEmit --strict
// --target es2022`) on the probes named in each test, in
// `narrowing-probes/m6/` and `narrowing-probes/m6-l2/`. TypeScript gives a
// read that only ended paths reach its declared type, where it reports
// TS7027 with `--allowUnreachableCode false`.

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
                TypeInferenceCaller::new("test", "reachability"),
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

/// The declarations of `narrowing-probes/m6/never.ts`.
const NEVER: &str = r#"
        declare function f<T>(): T;
        declare const c: boolean;
        declare function fail(): never;
        declare function failWith(m: string): never;
        function failDecl(): never { throw 0; }
        function failInferred() { throw 0; }
        const failArrow = (): never => { throw 0; };
        const failAnnotated: () => never = () => { throw 0; };
        declare const log: { fatal(): never; warn(): void };
        declare const l: L;
        declare namespace NS { function fail(): never; }
        declare function failGeneric<T>(v: T): never;
        declare function failOver(a: string): never;
        declare function failOver(a: number): void;
        function assertNever(v: never): never { throw v; }
        declare function assert(v: unknown): asserts v;
"#;

// `never.ts` L11, L18 to L20, L23, L24, L26, L27, L34, L35, L41, L46 give
// `string`; L21, L22, L25, L30, L33, L36, L42 to L45 keep `string | null`.
#[test]
fn calls_that_never_return_end_the_path() {
    let source = format!(
        "{NEVER}{}",
        r#"
        class L { fail(): never { throw 0; } m(x: string | null) { if (x === null) this.fail(); /*n11*/x; } }
        function v1(x: string | null) { if (x === null) fail(); /*n18*/x; }
        function v2(x: string | null) { if (x === null) failWith("m"); /*n19*/x; }
        function v3(x: string | null) { if (x === null) failDecl(); /*n20*/x; }
        function v4(x: string | null) { if (x === null) failInferred(); /*n21*/x; }
        function v5(x: string | null) { if (x === null) failArrow(); /*n22*/x; }
        function v6(x: string | null) { if (x === null) failAnnotated(); /*n23*/x; }
        function v7(x: string | null) { if (x === null) log.fatal(); /*n24*/x; }
        function v8(x: string | null) { if (x === null) log.warn(); /*n25*/x; }
        function v9(x: string | null) { if (x === null) l.fail(); /*n26*/x; }
        function v10(x: string | null) { if (x === null) NS.fail(); /*n27*/x; }
        function v13(x: string | null) { if (x === null) failOver(1); /*n30*/x; }
        function v16(x: string | null) { if (x === null) { (fail()); } /*n33*/x; }
        function v17(x: string | null) { if (x === null) log?.fatal(); /*n34*/x; }
        function v18(x: string | null) { if (x === null) (0, fail()); /*n35*/x; }
        function v19(x: string | null) { if (x === null) void fail(); /*n36*/x; }
        function v24(x: string | null, k: () => never) { if (x === null) k(); /*n41*/x; }
        function v25(x: string | null) { if (x === null) log["fatal"](); /*n42*/x; }
        function v26(x: string | null) { const lg = log; if (x === null) lg.fatal(); /*n43*/x; }
        function v27(x: string | null) { const { fatal } = log; if (x === null) fatal(); /*n44*/x; }
        function v28(x: string | null) { if (x === null) new Error(fail()); /*n45*/x; }
        function v29(x: string | null) { while (x === null) fail(); /*n46*/x; }
        "#
    );
    let string: &[&str] = &["string"];
    let declared: &[&str] = &["null", "string"];
    assert_reads(
        &source,
        &[
            ("n11", "x", string),
            ("n18", "x", string),
            ("n19", "x", string),
            ("n20", "x", string),
            ("n21", "x", declared),
            ("n22", "x", declared),
            ("n23", "x", string),
            ("n24", "x", string),
            ("n25", "x", declared),
            ("n26", "x", string),
            ("n27", "x", string),
            ("n30", "x", declared),
            ("n33", "x", declared),
            ("n34", "x", string),
            ("n35", "x", string),
            ("n36", "x", declared),
            ("n41", "x", string),
            ("n42", "x", declared),
            ("n43", "x", declared),
            ("n44", "x", declared),
            ("n45", "x", declared),
            ("n46", "x", string),
        ],
    );
}

// `never.ts` L37 to L40 and L47: the paths that a call ends add nothing
// where they join others, a `try` block's exceptions still reach `catch`,
// and a read that only ended paths reach keeps its declared type (L31, L40,
// L47; TS7027). Generic and overloaded signatures (L28, L29) end the path in
// TypeScript, which resolves the call's signature; Biome keeps the declared
// type there, which is wider. A `never` return type that comes from a
// function body ends nothing (`m6-l2/extra.ts` L19).
#[test]
fn reads_that_only_ended_paths_reach_keep_their_declared_type() {
    let source = format!(
        "{NEVER}{}",
        r#"
        function v14(x: string | null) { fail(); /*n31*/x; }
        function v20(x: "a" | "b", y: string | null) { switch (x) { case "a": if (y === null) fail(); break; case "b": return; } /*n37*/y; }
        function v21(x: string | null) { try { if (x === null) fail(); /*n38*/x; } catch { /*n38catch*/x; } }
        function v22(x: string | null) { if (x === null) fail(); const g = () => /*n39*/x; }
        function v23(x: "a" | "b") { switch (x) { case "a": return 1; case "b": return 2; default: assertNever(x); } /*n40*/x; }
        function v30(x: string | null) { do { fail(); } while (x === null); /*n47*/x; }
        function v31(x: string | null) { if (x === null) log.fatal(); else { fail(); } /*both*/x; }
        function v32(x: string | null) { if (x === null) { log.fatal(); /*after*/x; } }
        function v11(x: string | null) { if (x === null) failGeneric(x); /*n28*/x; }
        function v12(x: string | null) { if (x === null) failOver("a"); /*n29*/x; }
        function inferredMember(x: string | null) { if (x === null) typed.m(); /*m*/x; }
        declare const typed: { m: typeof failVia };
        function failVia() { return fail(); }
        "#
    );
    let declared: &[&str] = &["null", "string"];
    assert_reads(
        &source,
        &[
            ("n31", "x", declared),
            ("n37", "y", &["string"]),
            ("n38", "x", &["string"]),
            ("n38catch", "x", declared),
            ("n39", "x", &["string"]),
            ("n40", "x", &["string: a", "string: b"]),
            ("n47", "x", declared),
            ("both", "x", declared),
            ("after", "x", declared),
            ("n28", "x", declared),
            ("n29", "x", declared),
            ("m", "x", declared),
        ],
    );
}

// `m6/imports/main.ts` L24 to L31 with `m6/imports/guards.ts`: an import and
// a method end the path when the declaration that they name annotates the
// return type as `never` and every name along the callee has an explicit
// type.
#[test]
fn imported_calls_that_never_return_end_the_path() {
    let source = r#"
        import { fail, failInferred, failArrow, failAnnotated, checker, inferredChecker, assert } from "./guards";
        import * as g from "./guards";
        function n1(x: string | null) { if (x === null) fail(); /*n1*/x; }
        function n2(x: string | null) { if (x === null) failInferred(); /*n2*/x; }
        function n3(x: string | null) { if (x === null) failArrow(); /*n3*/x; }
        function n4(x: string | null) { if (x === null) failAnnotated(); /*n4*/x; }
        function n5(x: string | null) { if (x === null) g.fail(); /*n5*/x; }
        function n6(x: string | null) { if (x === null) checker.fail(); /*n6*/x; }
        function n7(x: string | null) { if (x === null) inferredChecker.fail(); /*n7*/x; }
        function n8(x: string | null) { fail(); /*n8*/x; }
        function n9(x: string | null) { if (x === null) assert(false); /*n9*/x; }
    "#;
    let string: &[&str] = &["string"];
    let declared: &[&str] = &["null", "string"];
    assert_reads_with(
        source,
        &[("/src/guards.ts", GUARDS)],
        &[
            ("n1", "x", string),
            ("n2", "x", declared),
            ("n3", "x", declared),
            ("n4", "x", string),
            ("n5", "x", string),
            ("n6", "x", string),
            ("n7", "x", declared),
            ("n8", "x", declared),
            ("n9", "x", string),
        ],
    );
}

// `m6/asserts.ts` L27 to L29, L42, and `m6-l2/extra.ts` L15: `asserts value`
// never returns for an argument that is always false, as TypeScript's
// `isFalseExpression` reads it. `0` and `!true` aren't.
#[test]
fn assertions_of_false_end_the_path() {
    let source = format!(
        "{NEVER}{}",
        r#"
        declare const a: { ok(v: unknown): asserts v };
        function s13(x: string | null) { if (x === null) assert(false); /*s13*/x; }
        function s14(x: string | null) { if (x === null) assert(0); /*s14*/x; }
        function s15(x: string | null) { if (x === null) assert(!true); /*s15*/x; }
        function s28(x: string | null) { if (x === null) assert(false, "m" as any); /*s28*/x; }
        function e8(x: string | null) { if (x === null) assert(x !== null && false); /*e8*/x; }
        function m1(x: string | null) { if (x === null) a.ok((false)); /*m1*/x; }
        function m2(x: string | null) { if (x === null) a.ok(c || false); /*m2*/x; }
        "#
    );
    let string: &[&str] = &["string"];
    let declared: &[&str] = &["null", "string"];
    assert_reads(
        &source,
        &[
            ("s13", "x", string),
            ("s14", "x", declared),
            ("s15", "x", declared),
            ("s28", "x", string),
            ("e8", "x", string),
            ("m1", "x", string),
            ("m2", "x", declared),
        ],
    );
}

// `m6/reach.ts` L6 to L30 and `m6-l2/extra.ts` L8 to L14, L23, L26: the
// literal `true` or `false`, without parentheses, makes the branch that it
// never takes unreachable, in `if`, loops, conditional expressions, and the
// operands of `&&` and `||`, which TypeScript's binder decides in
// `createFlowCondition`. `!false`, `(false)`, `0`, a `const` holding
// `false`, and the operands of `??` are ordinary conditions.
#[test]
fn literal_conditions_make_branches_unreachable() {
    let source = r#"
        declare const c: boolean;
        declare function fail(): never;
        function r1(x: string | null) { if (x) { if (false) { /*r1*/x; } } }
        function r2(x: string | null) { if (x) { if (true) {} else { /*r2*/x; } } }
        function r3(x: string | null) { if (x) { while (false) { /*r3*/x; } } }
        function r4(x: string | null) { if (x) { false && /*r4*/x; } }
        function r5(x: string | null) { if (x) { true || /*r5*/x; } }
        function r6(x: string | null) { if (x) { false ? /*r6*/x : 0; } }
        function r7(x: string | null) { if (x) { if (!false) {} else { /*r7*/x; } } }
        function r8(x: string | null) { if (x) { if ((false)) { /*r8*/x; } } }
        function r9(x: string | null) { if (x) { if (0) { /*r9*/x; } } }
        function r10(x: string | null) { if (x) { const k = false; if (k) { /*r10*/x; } } }
        function r11(x: string | null) { if (x === null) { if (true) return; } /*r11*/x; }
        function r12(x: string | null) { if (x === null) { if (!false) return; } /*r12*/x; }
        function r13(x: string | null) { if (x === null) { do {} while (false); return; } /*r13*/x; }
        function r14(x: string | null) { if (x === null) { for (; false; ) {} return; } /*r14*/x; }
        function r15(x: string | null) { if (x === null) { c && false; return; } /*r15*/x; }
        function r16(x: string | null) { if (x) { if (false) { const g = () => /*r16*/x; } } }
        function r17(x: string | null) { if (x === null) { if (c && false) { return; } } /*r17*/x; }
        function r18(x: string | null) { if (x === null) { if (false || c) { return; } } /*r18*/x; }
        function r20(x: string | null) { if (x === null) { if (false as boolean) return; } /*r20*/x; }
        function r21(x: string | null) { if (x) { if (false) { /*r21a*/x; } else { /*r21b*/x; } } }
        function r22(x: string | null) { if (x === null) { while (true) { if (c) break; } } /*r22*/x; }
        function r23(x: string | null) { if (x === null) { false ?? fail(); } /*r23*/x; }
        function r25(x: string | null) { if (x) { if (false) {} /*r25*/x; } }
        function e1(x: string | null) { for (; true; ) { if (x) break; } /*e1*/x; }
        function e2(x: string | null) { if (x) { for (; false; ) { /*e2*/x; } } }
        function e3(x: string | null) { if (x) { if (c && false) { /*e3*/x; } } }
        function e4(x: string | null) { if (x) { if (!(c && false)) {} else { /*e4*/x; } } }
        function e5(x: string | null) { if (x) { if (c || true) {} else { /*e5*/x; } } }
        function e6(x: string | null) { if (x === null) { if (c || true) return; } /*e6*/x; }
        function e7(x: string | null) { if (x === null) { while (c || true) {} } /*e7*/x; }
        function e16(x: string | null) { if (x) { true ? 0 : /*e16*/x; } }
        function e19(x: string | null) { do { if (x) break; } while (c || true); /*e19*/x; }
    "#;
    let string: &[&str] = &["string"];
    let declared: &[&str] = &["null", "string"];
    assert_reads(
        source,
        &[
            ("r1", "x", declared),
            ("r2", "x", declared),
            ("r3", "x", declared),
            ("r4", "x", declared),
            ("r5", "x", declared),
            ("r6", "x", declared),
            ("r7", "x", string),
            ("r8", "x", string),
            ("r9", "x", string),
            ("r10", "x", string),
            ("r11", "x", string),
            ("r12", "x", declared),
            ("r13", "x", string),
            ("r14", "x", string),
            ("r15", "x", string),
            ("r16", "x", declared),
            ("r17", "x", declared),
            ("r18", "x", declared),
            ("r20", "x", declared),
            ("r21a", "x", declared),
            ("r21b", "x", string),
            ("r22", "x", declared),
            ("r23", "x", declared),
            ("r25", "x", string),
            ("e1", "x", string),
            ("e2", "x", declared),
            ("e3", "x", declared),
            ("e4", "x", declared),
            ("e5", "x", declared),
            ("e6", "x", string),
            ("e7", "x", string),
            ("e16", "x", declared),
            ("e19", "x", string),
        ],
    );
}

// `m6/literals.ts` L7 to L13 and L16 to L18: only the literal `true` makes a
// loop run until `break`. `while (1)`, `while ("a")`, `while ((true))`,
// `do...while (1)`, `for (; 1;)`, and `while (!0)` can also end by their
// test.
#[test]
fn loops_exit_unless_their_test_is_the_literal_true() {
    let source = r#"
        function t1(x: string | null) { while (1) { if (x) break; } /*t1*/x; }
        function t2(x: string | null) { while ("a") { if (x) break; } /*t2*/x; }
        function t3(x: string | null) { while ((true)) { if (x) break; } /*t3*/x; }
        function t4(x: string | null) { while (true) { if (x) break; } /*t4*/x; }
        function t5(x: string | null) { do { if (x) break; } while (1); /*t5*/x; }
        function t6(x: string | null) { for (; 1; ) { if (x) break; } /*t6*/x; }
        function t7(x: string | null) { for (;;) { if (x) break; } /*t7*/x; }
        function t10(x: string | null) { do { if (x) break; } while (true); /*t10*/x; }
        function t11(x: string | null) { while (!0) { if (x) break; } /*t11*/x; }
        function t12(x: string | null) { if (x === null) { while (1) {} } /*t12*/x; }
    "#;
    let string: &[&str] = &["string"];
    let declared: &[&str] = &["null", "string"];
    assert_reads(
        source,
        &[
            ("t1", "x", declared),
            ("t2", "x", declared),
            ("t3", "x", declared),
            ("t4", "x", string),
            ("t5", "x", declared),
            ("t6", "x", declared),
            ("t7", "x", string),
            ("t10", "x", string),
            ("t11", "x", declared),
            ("t12", "x", declared),
        ],
    );
}

/// Classifies the statement that holds the expression after `/*marker*/`.
fn classify(
    db: &TestModuleDb,
    module: ModuleInfo,
    source: &str,
    marker: &str,
) -> TypeInferenceClassification {
    execute_type_inference_request(
        db,
        TypeInferenceCaller::new("test", "reachability"),
        UnreachableCodeClassificationRequest::new(module, marked_range(source, marker, "x")),
    )
}

// TypeScript 7.0.2 reports TS7027 with `--allowUnreachableCode false` at
// every statement classified `Match` below (`m6/never.ts` L31, `m6/reach.ts`
// L6, `m4/exhaustive.ts` L17), and at none of the others. A read that
// narrowing makes impossible is reachable code.
#[test]
fn unreachable_code_requests_classify_statements() {
    const SOURCE: &str = r#"
        declare function fail(): never;
        declare const log: { fatal(): never };
        declare const c: boolean;
        function a(x: string | null) { return; /*afterReturn*/x; }
        function b(x: string | null) { log.fatal(); /*afterFatal*/x; }
        function d(x: string | null) { if (false) { /*inFalse*/x; } }
        function e(x: "a" | "b") { switch (x) { case "a": return 1; case "b": return 2; } /*afterSwitch*/x; }
        function g(x: string | null) { fail(); if (c) { /*nested*/x; } }
        function h(x: string | null) { if (x === null) fail(); /*afterTest*/x; }
        function i(x: string) { if (typeof x === "number") { /*impossible*/x; } }
        function j(x: "a" | "b") { switch (x) { case "a": return 1; } /*partial*/x; }
        function k(x: string | null) { while (1) { if (c) break; } /*afterLoop*/x; }
        /*top*/x;
        declare const x: string;
    "#;
    let (db, module) = narrowing_db_with(SOURCE, &[]);
    for (marker, expected) in [
        ("afterReturn", TypeInferenceClassification::Match),
        ("afterFatal", TypeInferenceClassification::Match),
        ("inFalse", TypeInferenceClassification::Match),
        ("afterSwitch", TypeInferenceClassification::Match),
        ("nested", TypeInferenceClassification::Match),
        ("afterTest", TypeInferenceClassification::NoMatch),
        ("impossible", TypeInferenceClassification::NoMatch),
        ("partial", TypeInferenceClassification::NoMatch),
        ("afterLoop", TypeInferenceClassification::NoMatch),
        ("top", TypeInferenceClassification::NoMatch),
    ] {
        assert_eq!(classify(&db, module, SOURCE, marker), expected, "{marker}");
    }
    // A range that no statement holds, such as the whole module.
    let whole = TextRange::new(0.into(), u32::try_from(SOURCE.len()).unwrap().into());
    assert_eq!(
        execute_type_inference_request(
            &db,
            TypeInferenceCaller::new("test", "reachability"),
            UnreachableCodeClassificationRequest::new(module, whole),
        ),
        TypeInferenceClassification::Indeterminate
    );
}

// The request builds the graph of the asked root only, once for every
// statement of the root, and resolves only the callees of that root's
// statement calls. Neither complete module inference nor a lookup of a read
// builds one.
#[test]
fn unreachable_code_requests_build_only_the_asked_root() {
    const SOURCE: &str = r#"
        declare const log: { fatal(): never };
        declare const other: { fatal(): never };
        function asked(x: string | null) { if (x === null) log.fatal(); /*first*/x; log.fatal(); /*second*/x; }
        function unrelated(x: string | null) { if (x === null) other.fatal(); /*read*/x; }
    "#;
    let (db, module) = narrowing_db_with(SOURCE, &[]);
    db.clear_salsa_events();
    assert_eq!(
        classify(&db, module, SOURCE, "first"),
        TypeInferenceClassification::NoMatch
    );
    assert_eq!(
        classify(&db, module, SOURCE, "second"),
        TypeInferenceClassification::Match
    );
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_reachability", &events),
        1
    );
    // One effect per statement call of `asked`, none for `other.fatal()`.
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_call_effect", &events),
        2
    );
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_module_types", &events),
        0
    );

    let (db, module) = narrowing_db_with(SOURCE, &[]);
    db.clear_salsa_events();
    infer_module_types(&db, module).unwrap();
    let range = marked_range(SOURCE, "read", "x");
    infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range));
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_reachability", &events),
        0
    );
}
