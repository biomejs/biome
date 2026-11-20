use super::*;

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// unless a comment gives TypeScript's type.

/// Asserts the reads of `source`, the module `/src/index.ts`.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    assert_file_reads(&[("/src/index.ts", source)], "/src/index.ts", reads);
}

const DECLARATIONS: &str = r#"
declare const c: boolean;
declare const s: string;
declare const u: number | undefined;
declare const D: "a";
declare function id<T>(x: T): T;
const K = 1;
const K2: 1 = 1;
const KC = c ? "x" : 2;
"#;

#[test]
fn let_and_var_take_the_widened_type_of_their_initializer() {
    let source = format!(
        "{DECLARATIONS}{}",
        r#"
let l1 = 1; /*l1*/l1;
var v1 = "a"; /*v1*/v1;
let l2 = c ? "a" : 1; /*l2*/l2;
let l3 = true; /*l3*/l3;
let l4 = 1n; /*l4*/l4;
let l5 = -1; /*l5*/l5;
let l6 = `x`; /*l6*/l6;
let l8 = s || "d"; /*l8*/l8;
let l9 = c && "a"; /*l9*/l9;
let l10 = u ?? 0; /*l10*/l10;
let l11 = K; /*l11*/l11;
let l12 = K2; /*l12*/l12;
let l13 = D; /*l13*/l13;
let l14 = 1 as 1; /*l14*/l14;
let l15 = 1 as const; /*l15*/l15;
let l23 = KC; /*l23*/l23;
let l25 = c ? D : "b"; /*l25*/l25;
let l33 = c ? K2 : K; /*l33*/l33;
const k1 = 1; /*k1*/k1;
const k2 = c ? "a" : 1; /*k2*/k2;
"#
    );
    assert_reads(
        &source,
        &[
            ("l1", "l1", &["number"]),
            ("v1", "v1", &["string"]),
            ("l2", "l2", &["number", "string"]),
            ("l3", "l3", &["boolean"]),
            ("l4", "l4", &["BigInt"]),
            ("l5", "l5", &["number"]),
            ("l6", "l6", &["string"]),
            ("l8", "l8", &["string"]),
            // The `false` that `c` gives is not fresh.
            ("l9", "l9", &["bool: false", "string"]),
            ("l10", "l10", &["number"]),
            // A `const` declared without an annotation passes on its fresh
            // literal type.
            ("l11", "l11", &["number"]),
            ("l12", "l12", &["number: 1"]),
            ("l13", "l13", &["string: a"]),
            ("l14", "l14", &["number: 1"]),
            ("l15", "l15", &["number: 1"]),
            ("l23", "l23", &["number", "string"]),
            ("l25", "l25", &["string"]),
            // TypeScript: `1`, the declared `1` of `K2`, which absorbs the
            // fresh `1` of `K`. Biome widens every literal type of a kind
            // that has a fresh one.
            ("l33", "l33", &["number"]),
            ("k1", "k1", &["number: 1"]),
            ("k2", "k2", &["number: 1", "string: a"]),
        ],
    );
}

#[test]
fn object_literals_widen_the_properties_that_no_const_assertion_asserts() {
    let source = format!(
        "{DECLARATIONS}{}",
        r#"
declare function idc<const T>(x: T): T;
const o1 = { a: 1 }; /*o1*/o1;
const o2 = { a: 1 as const }; /*o2*/o2;
const o4 = { a: c ? "x" : "y" }; /*o4*/o4;
const o5 = { a: K, d: D }; /*o5*/o5;
const o6 = { a: { b: 1 } }; /*o6*/o6;
const o8 = { a: -1, b: true, n: 1n, t: `t` }; /*o8*/o8;
const o11 = { K }; /*o11*/o11;
const o14 = id({ a: 1 }); /*o14*/o14;
const o15 = idc({ a: 1 }); /*o15*/o15;
const o18 = { a: 1 as 1 }; /*o18*/o18;
const { e1 } = { e1: 1 }; /*e1*/e1;
const [d1] = [1]; /*d1*/d1;
let [d2] = [1]; /*d2*/d2;
"#
    );
    assert_reads(
        &source,
        &[
            ("o1", "o1", &[r#"["a": number]"#]),
            ("o2", "o2", &[r#"["a": number: 1]"#]),
            ("o4", "o4", &[r#"["a": string]"#]),
            ("o5", "o5", &[r#"["a": number, "d": instanceof string: a]"#]),
            (
                "o6",
                "o6",
                &[r#"[ "a": Object { prototype: No prototype members: ["b": number] } ]"#],
            ),
            (
                "o8",
                "o8",
                &[r#"["a": number, "b": boolean, "n": BigInt, "t": string]"#],
            ),
            ("o11", "o11", &[r#"["K": number]"#]),
            // `T` is inferred from the widened object.
            ("o14", "o14", &[r#"["a": number]"#]),
            // TypeScript: `{ readonly a: 1 }`, a `const` type parameter.
            ("o15", "o15", &[r#"["a": number: 1]"#]),
            ("o18", "o18", &[r#"["a": number: 1]"#]),
            ("e1", "e1", &["number"]),
            ("d1", "d1", &["number"]),
            ("d2", "d2", &["number"]),
        ],
    );
}

#[test]
fn class_fields_widen_unless_they_are_readonly() {
    let source = format!(
        "{DECLARATIONS}{}",
        r#"
class A { a = 1; readonly r = 1; static s = 1; static readonly sr = 1; b = c ? "x" : 1; k = K; k2 = K2; o? = 1; }
class B { constructor(public x = 1, readonly y = 1) {} }
const a = new A();
const b = new B();
/*a*/a.a; /*r*/a.r; /*s*/A.s; /*sr*/A.sr; /*b*/a.b; /*k*/a.k; /*k2*/a.k2; /*o*/a.o;
/*x*/b.x; /*y*/b.y;
"#
    );
    assert_reads(
        &source,
        &[
            ("a", "a.a", &["number"]),
            ("r", "a.r", &["number: 1"]),
            ("s", "A.s", &["number"]),
            ("sr", "A.sr", &["number: 1"]),
            ("b", "a.b", &["number", "string"]),
            ("k", "a.k", &["number"]),
            ("k2", "a.k2", &["number: 1"]),
            ("o", "a.o", &["number", "undefined"]),
            // A parameter property takes the type of its parameter, whose
            // default value widens, also when it is `readonly`.
            ("x", "b.x", &["number"]),
            ("y", "b.y", &["number"]),
        ],
    );
}

#[test]
fn array_literals_widen_their_fresh_elements() {
    let source = format!(
        "{DECLARATIONS}{}",
        r#"
const a1 = [1]; /*a1*/a1;
const a2 = [K]; /*a2*/a2;
const a3 = [D, K2]; /*a3*/a3;
const a4 = [1 as 1]; /*a4*/a4;
const a5 = [c ? 1 : 2]; /*a5*/a5;
"#
    );
    assert_reads(
        &source,
        &[
            ("a1", "a1", &["instanceof Array<number>"]),
            ("a2", "a2", &["instanceof Array<number>"]),
            ("a3", "a3", &["instanceof Array<string: a | number: 1>"]),
            ("a4", "a4", &["instanceof Array<number: 1>"]),
            ("a5", "a5", &["instanceof Array<number>"]),
        ],
    );
}

#[test]
fn inferred_return_types_widen_a_single_fresh_literal_type() {
    let source = format!(
        "{DECLARATIONS}{}",
        r#"
declare function pick<T extends "a" | "b">(f: () => T): T;
declare function produce<T>(f: () => T): T;
function f1() { return 1; }
function f2(): 1 { return 1; }
const f3 = () => 1;
const f4 = () => c ? "a" : "b";
function f5() { if (c) return "a"; return "b"; }
function f6() { return K; }
function f7() { return D; }
function f8() { return 1 as const; }
async function f9() { return 1; }
function f12() { return c ? 1 : undefined; }
function f13() { return true; }
class G { get g() { return 1; } m() { return 1; } }
/*f1*/f1(); /*f2*/f2(); /*f3*/f3(); /*f4*/f4(); /*f5*/f5(); /*f6*/f6(); /*f7*/f7(); /*f8*/f8();
/*f9*/f9(); /*f12*/f12(); /*f13*/f13();
/*g*/new G().g; /*m*/new G().m();
/*pick*/pick(() => "a"); /*produce*/produce(() => 1);
"#
    );
    assert_reads(
        &source,
        &[
            ("f1", "f1()", &["number"]),
            ("f2", "f2()", &["number: 1"]),
            ("f3", "f3()", &["number"]),
            ("f4", "f4()", &["string: a", "string: b"]),
            ("f5", "f5()", &["string: a", "string: b"]),
            ("f6", "f6()", &["number"]),
            ("f7", "f7()", &["string: a"]),
            ("f8", "f8()", &["number: 1"]),
            ("f9", "f9()", &["instanceof Promise<number>"]),
            ("f12", "f12()", &["number: 1", "undefined"]),
            ("f13", "f13()", &["boolean"]),
            ("g", "new G().g", &["number"]),
            ("m", "new G().m()", &["number"]),
            // TypeScript: `"a"`, because `T` gives the callback a contextual
            // literal type, which Biome doesn't see.
            ("pick", "pick(() => \"a\")", &["string"]),
            ("produce", "produce(() => 1)", &["number"]),
        ],
    );
}

#[test]
fn parameters_take_the_widened_type_of_their_default_value() {
    let source = format!(
        "{DECLARATIONS}{}",
        r#"
function g1(x = 1) { /*g1*/x; }
function g2(x = c ? "a" : "b") { /*g2*/x; }
function g3(x = K) { /*g3*/x; }
function g4(x: 1 = 1) { /*g4*/x; }
function g7(x = K2) { /*g7*/x; }
function g8(x = true) { /*g8*/x; }
"#
    );
    assert_reads(
        &source,
        &[
            ("g1", "x", &["number"]),
            ("g2", "x", &["string"]),
            ("g3", "x", &["number"]),
            ("g4", "x", &["number: 1"]),
            ("g7", "x", &["number: 1"]),
            ("g8", "x", &["boolean"]),
        ],
    );
}

// The `const` that an import names passes on its fresh literal type, also
// through a renaming export and as the value of `export default`.
#[test]
fn imported_constants_pass_on_their_fresh_literal_types() {
    assert_file_reads(
        &[
            (
                "/src/k.ts",
                r#"
export const K = 1;
export const KA: 1 = 1;
export const KC = Math.random() > 0.5 ? "x" : 2;
const local = "l";
export { local as renamed };
export default 1;
"#,
            ),
            (
                "/src/main.ts",
                r#"
import D, { K, KA, KC, renamed } from "./k";
let l1 = K; /*l1*/l1;
let l2 = KA; /*l2*/l2;
let l3 = KC; /*l3*/l3;
let l4 = renamed; /*l4*/l4;
const o = { K, a: KA }; /*o*/o;
let l6 = D; /*l6*/l6;
"#,
            ),
        ],
        "/src/main.ts",
        &[
            ("l1", "l1", &["number"]),
            ("l2", "l2", &["number: 1"]),
            ("l3", "l3", &["number", "string"]),
            ("l4", "l4", &["string"]),
            ("o", "o", &[r#"["K": number, "a": instanceof number: 1]"#]),
            ("l6", "l6", &["number"]),
        ],
    );
}

// The search for the fresh literal types of a value visits a bounded number
// of types. When it stops before it reaches every operand or `const`, every
// literal kind widens.
#[test]
fn values_too_large_to_search_widen_every_literal_kind() {
    let arms: String = (0..40)
        .map(|index| format!("c ? \"s{index}\" : "))
        .collect();
    let source = format!(
        "{DECLARATIONS}let m = {arms}7; /*m*/m;\n{}{}let n = c ? s40 : a40; /*n*/n;\n",
        const_chain("s", "\"x\""),
        const_chain("a", "1"),
    );
    assert_reads(
        &source,
        &[
            ("m", "m", &["number", "string"]),
            ("n", "n", &["number", "string"]),
        ],
    );
}

/// Returns the declarations of `{name}0`, initialized to `value`, and of
/// `{name}1` to `{name}40`, each initialized to the one before.
fn const_chain(name: &str, value: &str) -> String {
    let aliases: String = (1..=40)
        .map(|index| format!("const {name}{index} = {name}{};\n", index - 1))
        .collect();
    format!("const {name}0 = {value};\n{aliases}")
}

// Biome can't tell that these values are fresh literals, so it keeps their
// literal types where TypeScript widens them.
#[test]
fn values_whose_freshness_biome_cannot_tell_keep_their_literal_types() {
    let source = format!(
        "{DECLARATIONS}{}",
        r#"
class A { static readonly X = 1; }
namespace N { export const K = 1; }
const o = { a: id(1) }; /*o*/o;
let l1 = A.X; /*l1*/l1;
let l2 = N.K; /*l2*/l2;
"#
    );
    assert_reads(
        &source,
        &[
            // TypeScript: `{ a: number }`.
            ("o", "o", &[r#"["a": number: 1]"#]),
            // TypeScript: `number`.
            ("l1", "l1", &["number: 1"]),
            // TypeScript: `number`.
            ("l2", "l2", &["number: 1"]),
        ],
    );
}
