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
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    (db, module)
}

/// Names each member of `ty`, sorted. An interface is named by its name, an
/// object type literal by its members, and Biome's Unknown as `Unknown`. Any
/// other type is named by its formatted text on one line, such as `unknown`
/// for the `unknown` keyword. A read on a path that no value takes has
/// Biome's Unknown type.
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

/// Types shared by the tests, as interfaces, so that a read names each of its
/// members.
const TYPES: &str = r#"
    type ABC = "a" | "b" | "c";
    interface Circle { kind: "c"; r: number }
    interface Square { kind: "s"; w: number }
    interface Tri { kind: "t"; h: number }
    type Shape = Circle | Square | Tri;
    interface Ok { ok: true; v: number }
    interface Err { ok: false; e: string }
    interface Foo { a: number }
    interface Len { length: number }
    declare function more(): boolean;
"#;

/// Asserts the type of each `(marker, expression, expected)` read in `source`,
/// which follows [`TYPES`]. `expected` lists the sorted names of the type's
/// members in the form that [`variant_names`] returns.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    assert_reads_importing(source, "", reads);
}

/// Asserts the reads of `source` as [`assert_reads`] does, in a module that
/// can import `constants` from `./constants.ts`.
fn assert_reads_importing(source: &str, constants: &str, reads: &[(&str, &str, &[&str])]) {
    let source = format!("{TYPES}{source}");
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source.as_str());
    fs.insert("/src/constants.ts".into(), constants);
    let db = build_js_test_module_db(&fs, &["/src/index.ts", "/src/constants.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .filter_map(|(marker, expression, expected)| {
            let range = marked_range(&source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "switch"),
                NormalizedExpressionTypeRequest::new(module, range),
            )
            .unwrap_or_else(|| panic!("expression at {marker} must be inferred"));
            let actual = variant_names(&db, ty);
            (actual != *expected)
                .then(|| format!("{marker}: expected {expected:?}, got {actual:?}"))
        })
        .collect();
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
#[test]
fn literal_cases_narrow_the_discriminant() {
    assert_reads(
        r#"
        function f1(x: ABC) { switch (x) { case "a": /*case*/x; } }
        function f2(x: ABC) { switch (x) { case "a": case "b": /*group*/x; } }
        function f3(x: ABC) { switch (x) { case "a": break; case "b": break; default: /*default*/x; } }
        function f4(x: ABC) { switch (x) { default: /*defaultFirst*/x; break; case "a": break; } }
        function f5(x: ABC) { switch (x) { case "a": break; default: /*defaultMiddle*/x; case "b": /*afterDefault*/x; } }
        function f6(x: ABC) { switch (x) { case "a": /*fallthroughFrom*/x; case "b": /*fallthroughTo*/x; break; } }
        function f28(x: ABC) { switch ((x)) { case "a": /*parentheses*/x; } }
        function f29(x: ABC) { switch (x) { case "a": case "a": /*duplicate*/x; } }
        function f30(x: ABC) { switch (x) { case "a": { /*block*/x; } } }
        function f31(x: ABC) { switch (x) { case "a": break; case "b": /*second*/x; break; default: } }
        function f36(o: { v: ABC }) { switch (o.v) { case "a": /*property*/o.v; } }
        function f37(o: { v: ABC }) { switch (o["v"]) { case "a": /*element*/o.v; } }
        function f38(x: ABC) { switch (x) { case "a": break; } switch (x) { case "b": /*secondSwitch*/x; } }
        class K { v: "a" | "b" = "a"; m() { switch (this.v) { case "a": /*this*/this.v; break; default: /*thisDefault*/this.v; } } }
        "#,
        &[
            ("case", "x", &["string: a"]),
            ("group", "x", &["string: a", "string: b"]),
            ("default", "x", &["string: c"]),
            ("defaultFirst", "x", &["string: b", "string: c"]),
            ("defaultMiddle", "x", &["string: c"]),
            ("afterDefault", "x", &["string: b", "string: c"]),
            ("fallthroughFrom", "x", &["string: a"]),
            ("fallthroughTo", "x", &["string: a", "string: b"]),
            ("parentheses", "x", &["string: a"]),
            ("duplicate", "x", &["string: a"]),
            ("block", "x", &["string: a"]),
            ("second", "x", &["string: b"]),
            ("property", "o.v", &["string: a"]),
            ("element", "o.v", &["string: a"]),
            ("secondSwitch", "x", &["string: b"]),
            ("this", "this.v", &["string: a"]),
            ("thisDefault", "this.v", &["string: b"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// The code after a `switch` joins the end of its last case, its `break`
// statements, and, without `default`, the values that no case matches.
#[test]
fn the_code_after_a_switch_joins_its_exits() {
    assert_reads(
        r#"
        function f7(x: ABC) { switch (x) { case "a": break; case "b": break; } /*breaks*/x; }
        function f8(x: ABC) { switch (x) { case "a": return; case "b": return; } /*bypass*/x; }
        function f46(x: "a" | "b" | 1) { switch (x) { case "a": break; case 1: break; } /*mixed*/x; }
        function f49(x: ABC) { switch (x) { } /*empty*/x; }
        function f10(x: "a" | "b") { switch (x) { case "a": break; case "b": break; } /*exhaustiveBreaks*/x; }
        "#,
        &[
            ("breaks", "x", &["string: a", "string: b", "string: c"]),
            ("bypass", "x", &["string: c"]),
            ("mixed", "x", &["number: 1", "string: a", "string: b"]),
            ("empty", "x", &["string: a", "string: b", "string: c"]),
            ("exhaustiveBreaks", "x", &["string: a", "string: b"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A case narrows as `===` with its value does, so `{}` keeps its type, named
// `[]` here by its empty list of members, while `default` removes only the
// literal types that a case tests.
#[test]
fn cases_narrow_broad_and_special_types() {
    assert_reads(
        r#"
        function f10(x: string) { switch (x) { case "a": /*broad*/x; break; default: /*broadDefault*/x; } }
        function f11(x: string | number) { switch (x) { case "a": case 1: /*primitives*/x; } }
        function f12(x: unknown) { switch (x) { case "a": case 1: /*unknown*/x; break; default: /*unknownDefault*/x; } }
        function f13(x: {}) { switch (x) { case "a": /*empty*/x; } }
        function f15(x: boolean) { switch (x) { case true: break; default: /*boolean*/x; } }
        function f16(x: string | null | undefined) { switch (x) { case null: /*null*/x; break; case undefined: /*undefined*/x; break; default: /*nullishDefault*/x; } }
        function f35(x: "a" | "b" | 1 | null) { switch (x) { case "a": case null: break; default: /*groupDefault*/x; } }
        function f40(x: string) { switch (x) { case "a": case "b": break; default: /*broadGroupDefault*/x; } }
        function f41(x: ABC | undefined) { switch (x) { case undefined: break; default: /*undefinedDefault*/x; } }
        function f42(x: "a" | 0 | 0n | false) { switch (x) { case 0: case 0n: case false: break; default: /*falsyDefault*/x; } }
        "#,
        &[
            ("broad", "x", &["string: a"]),
            ("broadDefault", "x", &["string"]),
            ("primitives", "x", &["number: 1", "string: a"]),
            ("unknown", "x", &["number: 1", "string: a"]),
            ("unknownDefault", "x", &["unknown"]),
            ("empty", "x", &["[]"]),
            ("boolean", "x", &["bool: false"]),
            ("null", "x", &["null"]),
            ("undefined", "x", &["undefined"]),
            ("nullishDefault", "x", &["string"]),
            ("groupDefault", "x", &["number: 1", "string: b"]),
            ("broadGroupDefault", "x", &["string"]),
            (
                "undefinedDefault",
                "x",
                &["string: a", "string: b", "string: c"],
            ),
            ("falsyDefault", "x", &["string: a"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A group that contains `default` starts where one of its cases matches and
// where no case of the statement matches.
#[test]
fn default_narrows_in_any_group() {
    assert_reads(
        r#"
        function f32(x: ABC) { switch (x) { case "a": default: /*caseThenDefault*/x; } }
        function f33(x: ABC) { switch (x) { default: case "a": /*defaultThenCase*/x; } }
        function f34(x: ABC) { switch (x) { case "a": break; default: case "b": /*laterGroup*/x; } }
        function f39(x: ABC, b: boolean) { switch (x) { case "a": if (b) break; /*beforeFallthrough*/x; default: /*fallthroughIntoDefault*/x; } }
        "#,
        &[
            (
                "caseThenDefault",
                "x",
                &["string: a", "string: b", "string: c"],
            ),
            (
                "defaultThenCase",
                "x",
                &["string: a", "string: b", "string: c"],
            ),
            ("laterGroup", "x", &["string: b", "string: c"]),
            ("beforeFallthrough", "x", &["string: a"]),
            (
                "fallthroughIntoDefault",
                "x",
                &["string: a", "string: b", "string: c"],
            ),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// Conditions and jumps inside a case narrow as anywhere else, a `break` leaves
// the `switch`, a `continue` reaches the loop around it, and a case test sees
// the state before the `switch`.
#[test]
fn statements_inside_cases_narrow() {
    assert_reads(
        r#"
        function f21(x: ABC, y: string | null) { switch (x) { case "a": if (y === null) return; break; case "b": if (y === null) return; break; } /*notExhaustive*/y; }
        function f22(x: ABC, y: string | null) { switch (x) { case "a": if (y === null) return; /*afterReturn*/y; } }
        function f23(x: ABC, y: string | null) { switch (x) { case "a": if (y === null) break; /*afterBreak*/y; } }
        function f24(x: ABC, y: string | null) { lbl: switch (x) { case "a": if (y !== null) break lbl; return; } /*labeledBreak*/y; }
        function f26(x: ABC, y: string | null) { if (y === null) return; switch (x) { case "a": break; case (/*caseTest*/y, "b"): } }
        function f27(x: ABC, y: string | null) { if (y === null) return; switch (x) { case "a": /*fromBefore*/y; } }
        function f47(x: ABC, y: string | null) { switch (x) { case "a": if (y === null) return; break; default: if (y === null) return; } /*withDefault*/y; }
        function f50(x: ABC, y: string | null) { switch (x) { default: if (y === null) return; } /*onlyDefault*/y; }
        function f2(x: ABC, y: string | null) { while (more()) { switch (x) { case "a": if (y === null) continue; /*afterContinue*/y; } } }
        function f3(n: number, y: string | null) { while (more()) { switch (n) { case 1: if (y === null) break; /*caseInLoop*/y; } /*afterSwitchInLoop*/y; } }
        function f8(x: ABC, y: string | null) { lbl: switch (x) { case "a": if (y === null) break lbl; /*labeled*/y; break; case "b": if (y === null) break; /*unlabeled*/y; } /*afterLabeled*/y; }
        function caseIf(y: string | null, kind: number) { switch (kind) { case 1: if (y !== null) { /*caseIf*/y; } } }
        "#,
        &[
            ("notExhaustive", "y", &["null", "string"]),
            ("afterReturn", "y", &["string"]),
            ("afterBreak", "y", &["string"]),
            ("labeledBreak", "y", &["null", "string"]),
            ("caseTest", "y", &["string"]),
            ("fromBefore", "y", &["string"]),
            ("withDefault", "y", &["string"]),
            ("onlyDefault", "y", &["string"]),
            ("afterContinue", "y", &["string"]),
            ("caseInLoop", "y", &["string"]),
            ("afterSwitchInLoop", "y", &["null", "string"]),
            ("labeled", "y", &["string"]),
            ("unlabeled", "y", &["string"]),
            ("afterLabeled", "y", &["null", "string"]),
            ("caseIf", "y", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m4/switch.ts` L20, L22, L49, and
// `narrowing-probes/fix-e/switch.ts` L11 to L16): a case narrows by a value
// whose type is a unit type, as `===` with it does, also when the value is
// `-1`, a `const`, an imported `const`, `void 0`, or a template literal
// without substitutions.
#[test]
fn cases_narrow_by_unit_values() {
    assert_reads_importing(
        r#"
        import { EK } from "./constants.ts";
        function f17(x: number) { switch (x) { case -1: /*negative*/x; break; default: /*negativeDefault*/x; } }
        const K = "a";
        const M = -1;
        function f18(x: ABC) { switch (x) { case K: /*constant*/x; break; default: /*constantDefault*/x; } }
        function f44(x: ABC) { switch (x) { case `a`: /*template*/x; break; default: /*templateDefault*/x; } }
        function s4(x: -1 | 1) { switch (x) { case M: /*heldNegative*/x; break; default: /*heldNegativeDefault*/x; } }
        function s5(x: string | undefined) { switch (x) { case void 0: /*void*/x; break; default: /*voidDefault*/x; } }
        function s6(x: ABC) { switch (x) { case EK: /*imported*/x; break; default: /*importedDefault*/x; } }
        "#,
        r#"export const EK = "a";"#,
        &[
            ("negative", "x", &["number: -1"]),
            ("negativeDefault", "x", &["number"]),
            ("constant", "x", &["string: a"]),
            ("constantDefault", "x", &["string: b", "string: c"]),
            ("template", "x", &["string: a"]),
            ("templateDefault", "x", &["string: b", "string: c"]),
            ("heldNegative", "x", &["number: -1"]),
            ("heldNegativeDefault", "x", &["number: 1"]),
            ("void", "x", &["undefined"]),
            ("voidDefault", "x", &["string"]),
            ("imported", "x", &["string: a"]),
            ("importedDefault", "x", &["string: b", "string: c"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) narrows `e` to `E.A` and `E.B`, and `x`
// to `"a"` at `nonNull` (`narrowing-probes/m4/switch.ts` L24, L50), and gives
// `number` at `nan` (L48). Biome doesn't infer the type of an enum, so its
// members aren't unit values, and a discriminant that isn't a subject, such
// as `x!`, narrows nothing. Biome keeps the declared type, which is wider.
#[test]
fn cases_that_biome_does_not_compare_keep_the_declared_type() {
    assert_reads(
        r#"
        enum E { A, B }
        function f19(e: E) { /*enumDeclared*/e; switch (e) { case E.A: /*enum*/e; break; default: /*enumDefault*/e; } }
        function f43(x: number) { switch (x) { case NaN: /*nan*/x; } }
        function f45(x: ABC) { switch (x!) { case "a": /*nonNull*/x; } }
        "#,
        &[
            // Biome doesn't infer the type of a variable of an enum type.
            ("enumDeclared", "e", &["Unknown"]),
            ("enum", "e", &["Unknown"]),
            ("enumDefault", "e", &["Unknown"]),
            ("nan", "x", &["number"]),
            ("nonNull", "x", &["string: a", "string: b", "string: c"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A case on a discriminant narrows the object that holds it, as `===` does,
// through a property, an element access, a tuple index, an optional chain, a
// `const` that holds the property, and an element of a destructuring pattern.
#[test]
fn discriminant_cases_narrow_the_object() {
    assert_reads(
        r#"
        function f1(s: Shape) { switch (s.kind) { case "c": /*case*/s; break; case "s": case "t": /*group*/s; break; } }
        function f2(s: Shape) { switch (s.kind) { case "c": break; default: /*default*/s; } }
        function f3(s: Shape | undefined) { switch (s?.kind) { case "c": /*optional*/s; break; default: /*optionalDefault*/s; } }
        function f4(s: Shape | undefined) { switch (s?.kind) { case undefined: /*undefined*/s; break; default: /*undefinedDefault*/s; } }
        function f5(s: Shape) { const k = s.kind; switch (k) { case "c": /*held*/s; break; default: /*heldDefault*/s; } }
        function f6(s: Shape) { const { kind } = s; switch (kind) { case "c": /*destructured*/s; } }
        function f7(s: { kind: "c"; v: number } | { kind: "s"; v: string }) { const { kind, v } = s; switch (kind) { case "c": /*element*/v; break; default: /*elementDefault*/v; } }
        function f8(w: { s: Shape }) { switch (w.s.kind) { case "c": /*path*/w.s; } }
        function f9(t: ["a", number] | ["b", string]) { switch (t[0]) { case "a": /*tuple*/t; } }
        function f10(s: Shape) { switch (s["kind"]) { case "c": /*elementAccess*/s; } }
        function f12(s: Shape) { switch (s.kind) { case "c": /*member*/s.r; } }
        function f15(s: Shape) { switch (s.kind) { case "c": /*kind*/s.kind; break; default: /*kindDefault*/s.kind; } }
        function f16(...[k, v]: ["a", number] | ["b", string]) { switch (k) { case "a": /*rest*/v; } }
        function f17(r: Ok | Err) { switch (r.ok) { case true: /*true*/r; break; default: /*trueDefault*/r; } }
        function f22(s: { kind: "c"; v: number } | { kind: "s"; v: string }) { const { kind: k, v } = s; switch (k) { case "s": /*renamed*/v; } }
        function f23(o: { a?: Circle | Square }) { switch (o.a?.kind) { case "c": /*chain*/o.a; } }
        function f11(t: readonly ["a", number] | readonly ["b", string]) { switch (t[0]) { case "a": /*readonlyTuple*/t[1]; } }
        "#,
        &[
            ("case", "s", &["Circle"]),
            ("group", "s", &["Square", "Tri"]),
            ("default", "s", &["Square", "Tri"]),
            ("optional", "s", &["Circle"]),
            ("optionalDefault", "s", &["Square", "Tri", "undefined"]),
            ("undefined", "s", &["undefined"]),
            (
                "undefinedDefault",
                "s",
                &["Circle", "Square", "Tri", "undefined"],
            ),
            ("held", "s", &["Circle"]),
            ("heldDefault", "s", &["Square", "Tri"]),
            ("destructured", "s", &["Circle"]),
            ("element", "v", &["number"]),
            ("elementDefault", "v", &["string"]),
            ("path", "w.s", &["Circle"]),
            ("tuple", "t", &["[string: a, number]"]),
            ("elementAccess", "s", &["Circle"]),
            ("member", "s.r", &["number"]),
            ("kind", "s.kind", &["string: c"]),
            ("kindDefault", "s.kind", &["string: s", "string: t"]),
            ("rest", "v", &["number"]),
            ("true", "r", &["Ok"]),
            ("trueDefault", "r", &["Err"]),
            ("renamed", "v", &["string"]),
            ("chain", "o.a", &["Circle"]),
            ("readonlyTuple", "t[1]", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `{ kind: "c"; y: 2 }` at both
// reads, because its `default` removes every member whose discriminant is one
// of the case values, even when the discriminant is a union of them. Biome
// removes a member only when a single case removes all its values, so it
// keeps both members, which is wider.
#[test]
fn default_keeps_members_whose_discriminant_is_a_union_of_case_values() {
    assert_reads(
        r#"
        interface AB { kind: "a" | "b"; x: 1 }
        interface C { kind: "c"; y: 2 }
        function f13(s: AB | C) { switch (s.kind) { case "a": case "b": break; default: /*group*/s; } }
        function f14(s: AB | C) { switch (s.kind) { case "a": break; case "b": break; default: /*separate*/s; } }
        "#,
        &[
            ("group", "s", &["AB", "C"]),
            ("separate", "s", &["AB", "C"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// except `unknownObject` and `unknownFunction`. A case of `switch (typeof x)`
// narrows `x` as `typeof x === "string"` does, and the receivers of an
// optional chain only where every case of the group shows that the operand
// is not `undefined`. TypeScript narrows `unknown` to `object | null` for
// `"object"` and to `Function` for `"function"`, where Biome keeps
// `unknown`, as for `if`.
#[test]
fn typeof_cases_narrow_the_operand() {
    assert_reads(
        r#"
        function f1(x: string | number) { switch (typeof x) { case "string": /*string*/x; break; default: /*stringDefault*/x; } }
        function f2(x: string | number | boolean) { switch (typeof x) { case "string": case "number": /*group*/x; break; default: /*groupDefault*/x; } }
        function f4(x: unknown) { switch (typeof x) { case "string": /*unknownString*/x; break; case "object": /*unknownObject*/x; break; case "function": /*unknownFunction*/x; break; default: /*unknownDefault*/x; } }
        function f5(x: string | Foo | null) { switch (typeof x) { case "object": /*object*/x; break; default: /*objectDefault*/x; } }
        function f6(x: string | Len) { switch (typeof x) { case "string": /*len*/x; break; default: /*lenDefault*/x; } }
        function f7(o: { v: string | number }) { switch (typeof o.v) { case "string": /*property*/o.v; } }
        function f10(x: { y: number } | undefined) { switch (typeof x?.y) { case "number": /*receiver*/x; break; default: /*receiverDefault*/x; } }
        function f11(x: { y: number } | undefined) { switch (typeof x?.y) { case "undefined": /*receiverUndefined*/x; break; default: /*receiverUndefinedDefault*/x; } }
        function f12(x: { y: number } | undefined) { switch (typeof x?.y) { case "number": case "undefined": /*receiverGroup*/x; } }
        function f16(x: string | number) { switch (typeof x) { case "string": case "string": /*duplicate*/x; } }
        function f19(x: string | number) { switch (typeof x) { case "string": break; default: case "number": /*defaultGroup*/x; } }
        function f20(x: string | number | boolean) { switch (typeof x) { case "string": default: /*caseDefault*/x; break; case "number": } }
        function f21(x: {}) { switch (typeof x) { case "string": /*empty*/x; } }
        function f23(x: string | number) { switch ((typeof x)) { case "string": /*parentheses*/x; } }
        function f24(x: string | number) { switch (typeof (x)) { case "string": /*operandParentheses*/x; } }
        function f25(x: string | (() => void)) { switch (typeof x) { case "function": /*function*/x; break; default: /*functionDefault*/x; } }
        "#,
        &[
            ("string", "x", &["string"]),
            ("stringDefault", "x", &["number"]),
            ("group", "x", &["number", "string"]),
            ("groupDefault", "x", &["boolean"]),
            ("unknownString", "x", &["string"]),
            ("unknownObject", "x", &["unknown"]),
            ("unknownFunction", "x", &["unknown"]),
            ("unknownDefault", "x", &["unknown"]),
            ("object", "x", &["Foo", "null"]),
            ("objectDefault", "x", &["string"]),
            ("len", "x", &["string"]),
            ("lenDefault", "x", &["Len"]),
            ("property", "o.v", &["string"]),
            ("receiver", "x", &["[\"y\": number]"]),
            ("receiverDefault", "x", &["[\"y\": number]", "undefined"]),
            ("receiverUndefined", "x", &["[\"y\": number]", "undefined"]),
            (
                "receiverUndefinedDefault",
                "x",
                &["[\"y\": number]", "undefined"],
            ),
            ("receiverGroup", "x", &["[\"y\": number]", "undefined"]),
            ("duplicate", "x", &["string"]),
            ("defaultGroup", "x", &["number"]),
            ("caseDefault", "x", &["boolean", "string"]),
            ("empty", "x", &["string"]),
            ("parentheses", "x", &["string"]),
            ("operandParentheses", "x", &["string"]),
            (
                "function",
                "x",
                &["sync Function { accepts: { params: [] type_args: [] } returns: void }"],
            ),
            ("functionDefault", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m4/typeof.ts` L7, L12, L21, L22, `m4/more.ts` L4, L5,
// and `fix-e/switch.ts` L34, L35). A case that is neither a string literal
// nor a template literal without substitutions makes no case of
// `switch (typeof x)` narrow, and a `typeof` case narrows only the operand,
// not the object that holds it or the pattern that declares it. TypeScript
// gives `never` for an invalid kind (`"strin"`), where Biome keeps the
// declared type, which is wider.
#[test]
fn typeof_switches_narrow_only_their_operand() {
    assert_reads(
        r#"
        type S = { kind: "c"; v: number } | { kind: 1; v: string };
        function f3(x: string | number, k: string) { switch (typeof x) { case "string": /*nonLiteral*/x; break; case k: break; } }
        function f8(o: { k: "a"; a: 1 } | { k: 1; b: 2 }) { switch (typeof o.k) { case "string": /*object*/o; } }
        function f17(x: string | number) { switch (typeof x) { case "strin": /*invalid*/x; } }
        function f18(x: string | number) { switch (typeof x) { case `string`: /*template*/x; break; default: /*templateDefault*/x; } }
        function t1(x: string | number) { if (typeof x === `string`) /*templateIf*/x; else /*templateElse*/x; }
        function m1(s: S) { const { kind, v } = s; switch (typeof kind) { case "string": /*pattern*/v; } }
        function m2(s: S) { const k = s.kind; switch (typeof k) { case "string": /*held*/s; } }
        "#,
        &[
            ("nonLiteral", "x", &["number", "string"]),
            (
                "object",
                "o",
                &[
                    "[\"k\": number: 1, \"b\": number: 2]",
                    "[\"k\": string: a, \"a\": number: 1]",
                ],
            ),
            ("invalid", "x", &["number", "string"]),
            ("template", "x", &["string"]),
            ("templateDefault", "x", &["number"]),
            ("templateIf", "x", &["string"]),
            ("templateElse", "x", &["number"]),
            ("pattern", "v", &["number", "string"]),
            (
                "held",
                "s",
                &[
                    "[\"kind\": number: 1, \"v\": string]",
                    "[\"kind\": string: c, \"v\": number]",
                ],
            ),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A case of `switch (true)` narrows as its test does when it is truthy, after
// every case before its group failed to match; a group with `default` sees
// only the cases outside the group fail.
#[test]
fn switch_true_narrows_by_each_case() {
    assert_reads(
        r#"
        declare function isFoo(v: unknown): v is FooClass;
        class FooClass { a = 1 }
        function f1(x: string | number | null) { switch (true) { case typeof x === "string": /*first*/x; break; case x === null: /*second*/x; break; default: /*default*/x; } }
        function f2(x: string | number | null) { switch (true) { case typeof x === "string": break; case typeof x === "number": /*afterString*/x; } }
        function f3(x: string | null) { switch (true) { case x !== null: /*notNull*/x; } }
        function f5(x: { a: 1 } | null) { switch (true) { case x: /*truthy*/x; } }
        function f6(x: FooClass | string) { switch (true) { case x instanceof FooClass: /*instanceof*/x; break; default: /*instanceofDefault*/x; } }
        function f7(x: FooClass | string) { switch (true) { case isFoo(x): /*predicate*/x; break; default: /*predicateDefault*/x; } }
        function f8(x: { a: 1 } | { b: 2 }) { switch (true) { case "a" in x: /*in*/x; break; default: /*inDefault*/x; } }
        function f9(x: string | number | null) { switch (true) { case typeof x === "string": case x === null: /*group*/x; } }
        function f10(x: string | number | null) { switch (true) { case typeof x === "string": default: /*defaultGroup*/x; break; case x === null: } }
        function f11(x: string | number | null) { switch (true) { case typeof x === "string": return; case typeof x === "number": return; } /*bypass*/x; }
        function f12(x: string | null, y: number | null) { switch (true) { case x !== null && y !== null: /*andX*/x; /*andY*/y; break; default: /*andDefaultX*/x; /*andDefaultY*/y; } }
        function f13(x: string | null, n: number) { switch (true) { case n > 0: break; case x === null: break; default: /*relational*/x; } }
        function f14(x: string | null) { switch (true) { case x === null: break; case (/*caseTest*/x, true): } }
        function f15(x: string | number | null) { switch (true) { case typeof x === "string": /*fallthroughFrom*/x; case x === null: /*fallthroughTo*/x; } }
        function f19(x: string | null, y: string | null) { switch (true) { case y !== null: /*firstX*/x; /*firstY*/y; break; case x !== null: /*secondX*/x; /*secondY*/y; } }
        function f20(x: { a: 1 } | null) { switch (true) { case !!x: break; default: /*not*/x; } }
        function f21(x: { a: 1 } | null, b: boolean) { switch (true) { case b && !x: break; default: /*and*/x; } }
        function f22(x: { a: 1 } | null, b: string) { switch (true) { case b.length > 0 && x !== null: break; default: /*andNotNull*/x; } }
        function f23(x: { kind: "a" } | { kind: "b" }) { switch (true) { case x.kind === "a": /*discriminant*/x; break; default: /*discriminantDefault*/x; } }
        function f18(x: string | null) { switch (true) { default: if (x === null) return; } /*onlyDefault*/x; }
        function f19b(x: string | number | boolean | null) { switch (true) { case x === null: break; case typeof x === "string": case typeof x === "number": /*laterGroup*/x; break; default: /*laterDefault*/x; } }
        "#,
        &[
            ("first", "x", &["string"]),
            ("second", "x", &["null"]),
            ("default", "x", &["number"]),
            ("afterString", "x", &["number"]),
            ("notNull", "x", &["string"]),
            ("truthy", "x", &["[\"a\": number: 1]"]),
            ("instanceof", "x", &["instanceof FooClass"]),
            ("instanceofDefault", "x", &["string"]),
            ("predicate", "x", &["instanceof FooClass"]),
            ("predicateDefault", "x", &["string"]),
            ("in", "x", &["[\"a\": number: 1]"]),
            ("inDefault", "x", &["[\"b\": number: 2]"]),
            ("group", "x", &["null", "string"]),
            ("defaultGroup", "x", &["number", "string"]),
            ("bypass", "x", &["null"]),
            ("andX", "x", &["string"]),
            ("andY", "y", &["number"]),
            ("andDefaultX", "x", &["null", "string"]),
            ("andDefaultY", "y", &["null", "number"]),
            ("relational", "x", &["string"]),
            ("caseTest", "x", &["null", "string"]),
            ("fallthroughFrom", "x", &["string"]),
            ("fallthroughTo", "x", &["null", "string"]),
            ("firstX", "x", &["null", "string"]),
            ("firstY", "y", &["string"]),
            ("secondX", "x", &["string"]),
            ("secondY", "y", &["null"]),
            ("not", "x", &["null"]),
            ("and", "x", &["[\"a\": number: 1]", "null"]),
            ("andNotNull", "x", &["[\"a\": number: 1]", "null"]),
            ("discriminant", "x", &["[\"kind\": string: a]"]),
            ("discriminantDefault", "x", &["[\"kind\": string: b]"]),
            ("onlyDefault", "x", &["string"]),
            ("laterGroup", "x", &["number", "string"]),
            ("laterDefault", "x", &["boolean"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) keeps the declared type at every read
// below, because only a discriminant that is the literal `true` itself
// narrows the cases.
#[test]
fn switch_on_other_constants_narrows_nothing() {
    assert_reads(
        r#"
        function f16(x: string | null) { switch ((true)) { case x !== null: /*parentheses*/x; } }
        function f17(x: string | null) { switch (!0) { case x !== null: /*not*/x; } }
        function f18(x: string | null) { switch (false) { case x === null: /*false*/x; break; default: /*falseDefault*/x; } }
        "#,
        &[
            ("parentheses", "x", &["null", "string"]),
            ("not", "x", &["null", "string"]),
            ("false", "x", &["null", "string"]),
            ("falseDefault", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `null` at `object` and
// `objectCase`, because it takes every case of `switch (true)` before
// `default` as falsy. So `case x:` with `x: { a: 1 } | null` removes
// `{ a: 1 }`, although an object is truthy without being `true` and reaches
// `default`. Biome narrows by a case that failed to match only when the case
// gives a boolean, so it keeps the declared type at those reads.
// `NARROWING.md` lists this among the deliberate choices.
#[test]
fn switch_true_keeps_the_type_after_a_case_that_is_not_boolean() {
    assert_reads(
        r#"
        function f4(x: { a: 1 } | null) { switch (true) { case x: break; default: /*object*/x; } }
        function f10(x: string | number | null, o: { a: 1 } | null) { switch (true) { case typeof x === "string": break; case o: break; default: /*booleanBefore*/x; /*objectCase*/o; } }
        "#,
        &[
            ("object", "x", &["[\"a\": number: 1]", "null"]),
            ("booleanBefore", "x", &["null", "number"]),
            ("objectCase", "o", &["[\"a\": number: 1]", "null"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `never` at the first six reads,
// where Biome gives its Unknown type, and `string` at the last two. The path
// on which no case matches narrows the discriminant by every case.
#[test]
fn the_path_that_matches_no_case_narrows_the_discriminant() {
    assert_reads(
        r#"
        function r8(s: Shape) { switch (s.kind) { case "c": break; case "s": break; case "t": break; default: /*default*/s; } }
        function r10(x: "a" | "b") { switch (x) { case "a": return 1; case "b": return 2; } /*after*/x; }
        function f13(x: string | number) { switch (typeof x) { case "string": return; case "number": return; } /*typeofAfter*/x; }
        function f22(x: string | number) { switch (typeof x) { case "string": break; case "number": break; default: /*typeofDefault*/x; } }
        function f11(s: Shape) { switch (s.kind) { case "c": return 1; case "s": return 2; case "t": return 3; } /*object*/s; }
        function f20(s: Shape) { if (s.kind === "c") return; switch (s.kind) { case "s": return; case "t": return; } /*narrowedBefore*/s; }
        function f7(x: string | null) { switch (x) { case null: return; } /*nullCase*/x; }
        function f8(x: string | null) { switch (x) { case null: return; default: } /*nullDefault*/x; }
        "#,
        &[
            ("default", "s", &["Unknown"]),
            ("after", "x", &["Unknown"]),
            ("typeofAfter", "x", &["Unknown"]),
            ("typeofDefault", "x", &["Unknown"]),
            ("object", "s", &["Unknown"]),
            ("narrowedBefore", "s", &["Unknown"]),
            ("nullCase", "x", &["string"]),
            ("nullDefault", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string` for `y` at every read
// below, and `S` for `s` and `string | number` for `r` after the switches that
// test them. A `switch` whose cases cover every value of its discriminant
// leaves no path on which no case matches, so `y` keeps the narrowing of
// every case.
#[test]
fn exhaustive_switches_narrow_other_subjects() {
    assert_reads(
        r#"
        type S = { kind: "c"; r: number } | { kind: "s"; w: number };
        function f20(x: ABC, y: string | null) { switch (x) { case "a": if (y === null) return; break; case "b": if (y === null) return; break; case "c": if (y === null) return; } /*literal*/y; }
        function f48(x: ABC, y: string | null) { switch (x) { case "a": case "b": case "c": if (y === null) return; } /*group*/y; }
        function f14(x: string | number, y: string | null) { switch (typeof x) { case "string": if (y === null) return; break; case "number": if (y === null) return; } /*typeof*/y; }
        function f15(x: unknown, y: string | null) { switch (typeof x) { case "string": case "number": case "bigint": case "boolean": case "symbol": case "undefined": case "object": case "function": if (y === null) return; } /*unknown*/y; }
        function f4(x: any, y: string | null) { switch (typeof x) { case "string": case "number": case "bigint": case "boolean": case "symbol": case "undefined": case "object": case "function": if (y === null) return; } /*any*/y; }
        function f18(s: Shape, y: string | null) { switch (s.kind) { case "c": if (y === null) return; break; case "s": if (y === null) return; break; case "t": if (y === null) return; } /*discriminant*/y; }
        function f21(x: string, y: string | null) { if (x !== "a" && x !== "b") return; switch (x) { case "a": if (y === null) return; break; case "b": if (y === null) return; } /*narrowedBefore*/y; }
        function r13(x: "a" | "b", y: string | null) { switch (x) { case "a": case "b": if (y === null) return; } /*twoValues*/y; }
        function f11(s: S, y: string | null) { switch (s.kind) { case "c": if (y === null) return; break; case "s": if (y === null) return; } /*property*/y; /*propertyObject*/s; }
        function f12(s: S, y: string | null) { const k = s.kind; switch (k) { case "c": if (y === null) return; break; case "s": if (y === null) return; } /*held*/y; }
        function f13({ kind, r }: { kind: "c"; r: number } | { kind: "s"; r: string }, y: string | null) { switch (kind) { case "c": if (y === null) return; break; case "s": if (y === null) return; } /*pattern*/y; /*patternElement*/r; }
        function f14b(x: boolean | null | undefined, y: string | null) { switch (x) { case true: case false: case null: case undefined: if (y === null) return; } /*units*/y; }
        function f17(o: { k: "a" | "b" } | undefined, y: string | null) { switch (o?.k) { case "a": case "b": case undefined: if (y === null) return; } /*optionalChain*/y; }
        class C { v: "a" | "b" = "a"; m(y: string | null) { switch (this.v) { case "a": if (y === null) return; break; case "b": if (y === null) return; } /*this*/y; } }
        "#,
        &[
            ("literal", "y", &["string"]),
            ("group", "y", &["string"]),
            ("typeof", "y", &["string"]),
            ("unknown", "y", &["string"]),
            ("any", "y", &["string"]),
            ("discriminant", "y", &["string"]),
            ("narrowedBefore", "y", &["string"]),
            ("twoValues", "y", &["string"]),
            ("property", "y", &["string"]),
            (
                "propertyObject",
                "s",
                &[
                    "[\"kind\": string: c, \"r\": number]",
                    "[\"kind\": string: s, \"w\": number]",
                ],
            ),
            ("held", "y", &["string"]),
            ("pattern", "y", &["string"]),
            ("patternElement", "r", &["number", "string"]),
            ("units", "y", &["string"]),
            ("optionalChain", "y", &["string"]),
            ("this", "y", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string | null` for `y` at every
// read below (`narrowing-probes/m4/switch.ts` L26, `m4-l1/extra.ts` L6, L14,
// L19, L23, L37). A case is missing, the discriminant is `typeof` in
// parentheses, which TypeScript checks as a string value, the discriminant is
// `never` before the switch, which no literal type is, or the read comes
// before the switch in a loop.
#[test]
fn switches_that_are_not_exhaustive_keep_the_path_that_matches_no_case() {
    assert_reads(
        r#"
        function f21(x: ABC, y: string | null) { switch (x) { case "a": if (y === null) return; break; case "b": if (y === null) return; break; } /*missing*/y; }
        function f1(x: string | number, y: string | null) { switch ((typeof x)) { case "string": if (y === null) return; break; case "number": if (y === null) return; } /*parentheses*/y; }
        function f5(x: unknown, y: string | null) { switch (typeof x) { case "string": case "number": case "bigint": case "boolean": case "symbol": case "undefined": case "object": if (y === null) return; } /*sevenKinds*/y; }
        function f7(x: "a" | "b", y: string | null) { if (x === "a") return; if (x === "b") return; switch (x) { case "a": if (y === null) return; } /*never*/y; }
        function f9(x: "a" | "b", y: string | null) { while (more()) { /*loop*/y; switch (x) { case "a": if (y === null) return; break; case "b": if (y === null) return; } } }
        function f16(o: { k: "a" | "b" } | undefined, y: string | null) { switch (o?.k) { case "a": case "b": if (y === null) return; } /*optionalChain*/y; }
        "#,
        &[
            ("missing", "y", &["null", "string"]),
            ("parentheses", "y", &["null", "string"]),
            ("sevenKinds", "y", &["null", "string"]),
            ("never", "y", &["null", "string"]),
            ("loop", "y", &["null", "string"]),
            ("optionalChain", "y", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string` for `y` at every read
// below (`narrowing-probes/m4-l1/extra.ts` L35, and `fix-e/switch.ts` L18 to
// L22, L32, L36), and `never` for `x` at `discriminant` (L27), which Biome
// reads as Unknown: a case whose value is a unit type, such as `-1`, a
// `const`, an imported `const`, `void 0`, or a template literal without
// substitutions, counts toward exhaustiveness as a literal does, also after
// a comparison with a `const` that narrows the discriminant.
#[test]
fn switches_with_cases_of_unit_values_are_exhaustive() {
    assert_reads_importing(
        r#"
        import { EK } from "./constants.ts";
        const K = "a";
        const T = `b`;
        function f15(x: "a" | "b", y: string | null) { switch (x) { case "a": if (y === null) return; break; case `b`: if (y === null) return; } /*template*/y; }
        function x1(x: ABC, y: string | null) { switch (x) { case K: if (y === null) return; break; case "b": if (y === null) return; break; case "c": if (y === null) return; } /*constant*/y; }
        function x2(x: -1 | 1, y: string | null) { switch (x) { case -1: if (y === null) return; break; case 1: if (y === null) return; } /*negative*/y; }
        function x3(x: ABC, y: string | null) { switch (x) { case `a`: case T: case "c": if (y === null) return; } /*templates*/y; }
        function x4(x: string | undefined, y: string | null) { if (x !== undefined && x !== "a") return; switch (x) { case "a": case void 0: if (y === null) return; } /*void*/y; }
        function x5(x: ABC, y: string | null) { switch (x) { case EK: case "b": case "c": if (y === null) return; } /*imported*/y; }
        function h4(x: "a" | "b", y: string | null) { if (x === K) return; switch (x) { case "b": if (y === null) return; } /*narrowedBefore*/y; }
        function t3(x: string | number, y: string | null) { switch (typeof x) { case `string`: if (y === null) return; break; case "number": if (y === null) return; } /*typeof*/y; }
        function d1(x: ABC) { switch (x) { case K: return; case "b": return; case "c": return; } /*discriminant*/x; }
        "#,
        r#"export const EK = "a";"#,
        &[
            ("template", "y", &["string"]),
            ("constant", "y", &["string"]),
            ("negative", "y", &["string"]),
            ("templates", "y", &["string"]),
            ("void", "y", &["string"]),
            ("imported", "y", &["string"]),
            ("narrowedBefore", "y", &["string"]),
            ("typeof", "y", &["string"]),
            ("discriminant", "x", &["Unknown"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string | null` for `y` at every
// read below but `function` (`narrowing-probes/fix-e/switch.ts` L23, L25,
// L29 to L31, L58): a case whose value is `-(1)`, which is `number`, or an
// imported `"a" as string` keeps a switch from being exhaustive, even when
// the other cases cover every value, and so does a discriminant that a
// comparison with a `const`, an imported `const`, or `-1` narrows to `never`
// before the switch, as no literal type is `never`. TypeScript gives `string` at
// `function` (L24), because `f()` returns `"c"`, where Biome doesn't compare
// with a call and keeps the path that matches no case, which is wider.
#[test]
fn switches_with_cases_that_are_not_unit_values_are_not_exhaustive() {
    assert_reads_importing(
        r#"
        import { EK, EP, ES } from "./constants.ts";
        const K = "a";
        const P = -(1);
        declare function f(): "c";
        const F = f();
        function x6(x: -1 | 1, y: string | null) { switch (x) { case P: case 1: if (y === null) return; } /*parenthesized*/y; }
        function x7(x: ABC, y: string | null) { switch (x) { case "a": case "b": case F: if (y === null) return; } /*function*/y; }
        function x8(x: -1 | 1, y: string | null) { switch (x) { case EP: case 1: if (y === null) return; } /*imported*/y; }
        function x9(x: "a" | "b", y: string | null) { switch (x) { case "a": case "b": case ES: if (y === null) return; } /*importedString*/y; }
        function h1(x: "a" | "b", y: string | null) { if (x === K) return; if (x === "b") return; switch (x) { case "a": if (y === null) return; } /*constant*/y; }
        function h2(x: "a" | "b", y: string | null) { if (x === EK) return; if (x === "b") return; switch (x) { case "a": if (y === null) return; } /*importedBefore*/y; }
        function h3(x: -1 | 1, y: string | null) { if (x === -1) return; if (x === 1) return; switch (x) { case -1: if (y === null) return; } /*negative*/y; }
        "#,
        r#"export const EK = "a"; export const EP = -(1); export const ES = "a" as string;"#,
        &[
            ("parenthesized", "y", &["null", "string"]),
            ("function", "y", &["null", "string"]),
            ("imported", "y", &["null", "string"]),
            ("importedString", "y", &["null", "string"]),
            ("constant", "y", &["null", "string"]),
            ("importedBefore", "y", &["null", "string"]),
            ("negative", "y", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string | null` at both reads
// (`narrowing-probes/fix-e/hazard.ts` L4, L5): the comparisons before the
// switch narrow `x` to `never`, so no case type covers it. Biome doesn't
// compare with an enum member or a variable with a type annotation, so `x`
// is `"a"` at the switch, which is exhaustive, and `y` reads as `string`,
// which is narrower than TypeScript in code that TypeScript types as
// impossible, as the Limits in `NARROWING.md` describe.
#[test]
fn switches_after_comparisons_that_biome_does_not_decompose_can_be_exhaustive() {
    assert_reads(
        r#"
        enum E { A = "a" }
        function h(x: "a" | "b", y: string | null) { if (x === E.A) return; if (x === "b") return; switch (x) { case "a": if (y === null) return; } /*enum*/y; }
        function g(x: "a" | "b", y: string | null, k: "a") { if (x === k) return; if (x === "b") return; switch (x) { case "a": if (y === null) return; } /*annotated*/y; }
        "#,
        &[("enum", "y", &["string"]), ("annotated", "y", &["string"])],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string` at every read below.
// Lookups ask whether a `switch` is exhaustive through a tracked query that
// the reads after it share, and only for a read that the path matching no
// case reaches.
#[test]
fn lookups_decide_whether_a_switch_is_exhaustive_once() {
    const SOURCE: &str = r#"
        function f(x: "a" | "b", y: string | null, z: string | null) {
            switch (x) {
                case "a":
                    if (y === null || z === null) return;
                    /*inCase*/y;
                    break;
                case "b":
                    if (y === null || z === null) return;
            }
            /*afterY*/y;
            /*afterZ*/z;
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    let read = |marker, expression| {
        infer_expression_type(
            &db,
            ExpressionTypeInput::new(&db, module, marked_range(SOURCE, marker, expression)),
        )
        .unwrap()
    };
    let decisions = |events: &[salsa::Event]| {
        function_query_will_execute_count_by_name(&db, "infer_flow_switch_exhaustive", events)
    };
    db.clear_salsa_events();
    assert_eq!(read("inCase", "y"), InferredTypeData::String);
    assert_eq!(decisions(&db.take_salsa_events()), 0);
    assert_eq!(read("afterY", "y"), InferredTypeData::String);
    assert_eq!(read("afterZ", "z"), InferredTypeData::String);
    assert_eq!(decisions(&db.take_salsa_events()), 1);
}
