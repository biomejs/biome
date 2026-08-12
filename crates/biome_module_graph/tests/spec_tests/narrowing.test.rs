use super::*;
use biome_rowan::TextSize;

#[test]
fn test_infer_module_types_narrows_typeof_guarded_references() {
    const SOURCE: &str = r#"
export function guarded(x: number | (() => Promise<void>)) {
    if (typeof x === "function") {
        x;
    }
    x;
}

export function reversed(y: string | undefined) {
    if (typeof y === "undefined") {
        y;
    }
    if ("string" == typeof y) {
        y;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let expression_ty_at = |offset: usize| {
        let start = TextSize::from(offset as u32);
        let range = TextRange::new(start, start + TextSize::from(1));
        inferred
            .expressions
            .get(&range)
            .copied()
            .expect("reference type must be inferred")
    };

    let narrowed_offset = SOURCE.find("x;").expect("guarded reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(narrowed_offset));
    assert!(narrowed.callable_function(&db).is_some());
    assert!(!contains_inferred_number(&db, narrowed));

    let unnarrowed_offset = SOURCE.rfind("x;").expect("trailing reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(unnarrowed_offset));
    assert!(contains_inferred_number(&db, unnarrowed));

    let undefined_offset = SOURCE
        .find("y;")
        .expect("undefined-guarded reference must exist");
    let narrowed_to_undefined = normalize_type(&db, module, expression_ty_at(undefined_offset));
    assert!(contains_inferred_undefined(&db, narrowed_to_undefined));
    assert!(!contains_inferred_string(&db, narrowed_to_undefined));

    let string_offset = SOURCE
        .rfind("y;")
        .expect("string-guarded reference must exist");
    let narrowed_to_string = normalize_type(&db, module, expression_ty_at(string_offset));
    assert!(contains_inferred_string(&db, narrowed_to_string));
    assert!(!contains_inferred_undefined(&db, narrowed_to_string));

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_typeof_guarded_references",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_narrows_typeof_guards_with_callable_interfaces() {
    const SOURCE: &str = r#"
interface AsyncFn {
    (): Promise<void>;
    tag: string;
}

export function guarded(f: AsyncFn | (() => void) | null) {
    if (typeof f === "function") {
        f;
    }
    if (typeof f === "object") {
        f;
    }
}

class Service {
    run(): void {}
}

export function classValue(c: typeof Service | number) {
    if (typeof c === "function") {
        c;
    }
}

interface Ctor {
    new (): { a: number };
}

export function constructSignature(k: Ctor | number) {
    if (typeof k === "function") {
        k;
    }
    if (typeof k === "object") {
        k;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let expression_ty_at = |offset: usize| {
        let start = TextSize::from(offset as u32);
        let range = TextRange::new(start, start + TextSize::from(1));
        inferred
            .expressions
            .get(&range)
            .copied()
            .expect("reference type must be inferred")
    };

    // An interface with a call signature is a function at runtime.
    let function_offset = SOURCE
        .find("f;")
        .expect("function-guarded reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(function_offset));
    let formatted = format_inferred_type(&db, narrowed);
    assert!(formatted.contains("interface \"AsyncFn\""), "{formatted}");
    assert!(formatted.contains("Function"), "{formatted}");
    assert!(!formatted.contains("null"), "{formatted}");

    // `typeof null` is `"object"`.
    let object_offset = SOURCE
        .rfind("f;")
        .expect("object-guarded reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(object_offset));
    let formatted = format_inferred_type(&db, narrowed);
    assert!(formatted.contains("null"), "{formatted}");
    assert!(!formatted.contains("AsyncFn"), "{formatted}");
    assert!(!formatted.contains("Function"), "{formatted}");

    // A class value is a constructor function at runtime.
    let class_offset = SOURCE.find("c;").expect("class reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(class_offset));
    assert!(!contains_inferred_number(&db, narrowed));
    let formatted = format_inferred_type(&db, narrowed);
    assert!(formatted.contains("Service"), "{formatted}");

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_typeof_guards_with_callable_interfaces",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_narrows_truthiness_guards() {
    const SOURCE: &str = r#"
export function truthiness(z: "on" | null) {
    if (z) {
        z;
    }
    if (!z) {
        z;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let expression_ty_at = |offset: usize| {
        let start = TextSize::from(offset as u32);
        let range = TextRange::new(start, start + TextSize::from(1));
        inferred
            .expressions
            .get(&range)
            .copied()
            .expect("reference type must be inferred")
    };

    // A truthiness guard drops the nullish variant.
    let truthy_offset = SOURCE
        .find("z;")
        .expect("truthy-guarded reference must exist");
    let narrowed_to_truthy = normalize_type(&db, module, expression_ty_at(truthy_offset));
    assert!(contains_inferred_string_literal(
        &db,
        narrowed_to_truthy,
        "on"
    ));
    assert!(!contains_inferred_null(&db, narrowed_to_truthy));

    // A negated truthiness guard keeps only the falsy variants.
    let falsy_offset = SOURCE
        .rfind("z;")
        .expect("falsy-guarded reference must exist");
    let narrowed_to_falsy = normalize_type(&db, module, expression_ty_at(falsy_offset));
    assert!(contains_inferred_null(&db, narrowed_to_falsy));
    assert!(!contains_inferred_string_literal(
        &db,
        narrowed_to_falsy,
        "on"
    ));

    assert_inferred_type_snapshot("test_infer_module_types_narrows_truthiness_guards", &db, &fs);
}

#[test]
fn test_infer_module_types_narrows_instanceof_guards() {
    const SOURCE: &str = r#"
class BaseCls {}
class DerivedCls extends BaseCls {
    run(): void {}
}

export function instances(v: BaseCls, w: number | DerivedCls) {
    if (v instanceof DerivedCls) {
        v;
    }
    if (w instanceof DerivedCls) {
        w;
    }
}

function WithMixin<T extends new (...args: unknown[]) => object>(base: T) {
    return class extends base {};
}
class MixedCls extends WithMixin(BaseCls) {
    onlyMixed(): void {}
}
class SiblingCls extends BaseCls {
    onlySibling(): void {}
}

export function mixins(m: MixedCls | SiblingCls) {
    if (m instanceof BaseCls) {
        m;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let expression_ty_at = |offset: usize| {
        let start = TextSize::from(offset as u32);
        let range = TextRange::new(start, start + TextSize::from(1));
        inferred
            .expressions
            .get(&range)
            .copied()
            .expect("reference type must be inferred")
    };

    // An instanceof guard over a base class reference downcasts it, making
    // the subclass member visible.
    let downcast_offset = SOURCE.find("v;").expect("downcast reference must exist");
    let narrowed_to_derived = normalize_type(&db, module, expression_ty_at(downcast_offset));
    assert!(
        inferred
            .find_member_type(&db, narrowed_to_derived, "run")
            .is_some()
    );

    // An instanceof guard strips union variants that cannot be instances.
    let union_offset = SOURCE.find("w;").expect("union reference must exist");
    let narrowed_to_instance = normalize_type(&db, module, expression_ty_at(union_offset));
    assert!(!contains_inferred_number(&db, narrowed_to_instance));
    assert!(
        inferred
            .find_member_type(&db, narrowed_to_instance, "run")
            .is_some()
    );

    // A variant whose extends chain contains a mixin call cannot be walked
    // to a proof, so it must be kept.
    let mixin_offset = SOURCE.find("m;").expect("mixin reference must exist");
    let narrowed_with_mixin = normalize_type(&db, module, expression_ty_at(mixin_offset));
    assert!(
        inferred
            .find_member_type(&db, narrowed_with_mixin, "onlyMixed")
            .is_some()
    );
    assert!(
        inferred
            .find_member_type(&db, narrowed_with_mixin, "onlySibling")
            .is_some()
    );

    assert_inferred_type_snapshot("test_infer_module_types_narrows_instanceof_guards", &db, &fs);
}

#[test]
fn test_infer_module_types_narrows_discriminant_guards() {
    const SOURCE: &str = r#"
type Choice =
    | { kind: "left"; left: string }
    | { kind: "right"; right: number };

export function discriminants(c: Choice) {
    if (c.kind === "left") {
        c;
    }
}

type Esc =
    | { kind: "o\x6e"; escaped: string }
    | { kind: "off"; plain: number };

export function escapes(e: Esc) {
    if (e.kind === "on") {
        e;
    }
}

export function mutated(m: Choice) {
    if (m.kind === "left") {
        m.kind = "right";
        m;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let expression_ty_at = |offset: usize| {
        let start = TextSize::from(offset as u32);
        let range = TextRange::new(start, start + TextSize::from(1));
        inferred
            .expressions
            .get(&range)
            .copied()
            .expect("reference type must be inferred")
    };

    // A discriminant guard strips union variants whose member is a literal
    // with a different value.
    let discriminant_offset = SOURCE
        .find("c;")
        .expect("discriminant reference must exist");
    let narrowed_to_left = normalize_type(&db, module, expression_ty_at(discriminant_offset));
    assert!(
        inferred
            .find_member_type(&db, narrowed_to_left, "left")
            .is_some()
    );
    // On the un-narrowed union, `kind` would be `"left" | "right"`.
    let kind_ty = inferred
        .find_member_type(&db, narrowed_to_left, "kind")
        .expect("kind member must be inferred");
    let kind_ty = normalize_type(&db, module, kind_ty);
    assert!(is_inferred_string_literal(&db, kind_ty, "left"));

    // A discriminant with an escape sequence is compared by its unescaped
    // value, so the guard keeps the escaped variant and strips the other.
    let escape_offset = SOURCE.find("e;").expect("escape reference must exist");
    let narrowed_to_escaped = normalize_type(&db, module, expression_ty_at(escape_offset));
    assert!(
        inferred
            .find_member_type(&db, narrowed_to_escaped, "escaped")
            .is_some()
    );

    // A write to a member of the narrowed value inside the consequent
    // declines discriminant narrowing.
    let mutated_offset = SOURCE.find("m;").expect("mutated reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(mutated_offset));
    let formatted = format_inferred_type(&db, unnarrowed);
    assert!(formatted.contains("\"left\""), "{formatted}");
    assert!(formatted.contains("\"right\""), "{formatted}");

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_discriminant_guards",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_narrows_switch_and_equality_guards() {
    const SOURCE: &str = r#"
type Choice =
    | { kind: "left"; left: string }
    | { kind: "right"; right: number };

export function switches(s: Choice) {
    switch (s.kind) {
        case "left":
            s;
            break;
    }
}

export function equality(q: number | "done") {
    if (q === "done") {
        q;
    }
}

export function promiseEquality(u: Promise<void> | "done") {
    if (u === "done") {
        u;
    }
}

interface Lengthy {
    length: number;
}

export function stringSatisfies(g: Lengthy | number) {
    if (g === "done") {
        g;
    }
}

export function genericEquality<T extends string>(o: T | Promise<void>) {
    if (o === "done") {
        o;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let expression_ty_at = |offset: usize| {
        let start = TextSize::from(offset as u32);
        let range = TextRange::new(start, start + TextSize::from(1));
        inferred
            .expressions
            .get(&range)
            .copied()
            .expect("reference type must be inferred")
    };

    // A switch case over a member discriminant narrows like the equivalent
    // `if` guard.
    let switch_offset = SOURCE.find("s;").expect("switch reference must exist");
    let narrowed_in_case = normalize_type(&db, module, expression_ty_at(switch_offset));
    let case_kind_ty = inferred
        .find_member_type(&db, narrowed_in_case, "kind")
        .expect("kind member must be inferred");
    let case_kind_ty = normalize_type(&db, module, case_kind_ty);
    assert!(is_inferred_string_literal(&db, case_kind_ty, "left"));

    // An equality guard narrows the union to the compared literal.
    let equality_offset = SOURCE.find("q;").expect("equality reference must exist");
    let narrowed_to_done = normalize_type(&db, module, expression_ty_at(equality_offset));
    assert!(is_inferred_string_literal(&db, narrowed_to_done, "done"));
    assert!(!contains_inferred_number(&db, narrowed_to_done));

    // A promise instance can never strictly equal a string.
    let promise_equality_offset = SOURCE
        .find("u;")
        .expect("promise equality reference must exist");
    let narrowed_from_promise =
        normalize_type(&db, module, expression_ty_at(promise_equality_offset));
    assert!(is_inferred_string_literal(
        &db,
        narrowed_from_promise,
        "done"
    ));

    // An interface whose members all exist on `String` may describe a
    // string, so its variant is retained.
    let satisfies_offset = SOURCE
        .find("g;")
        .expect("string-satisfies reference must exist");
    let narrowed_to_lengthy = normalize_type(&db, module, expression_ty_at(satisfies_offset));
    assert!(!contains_inferred_number(&db, narrowed_to_lengthy));
    assert!(
        inferred
            .find_member_type(&db, narrowed_to_lengthy, "length")
            .is_some()
    );

    // A generic type parameter could be instantiated with the compared
    // string, so its variant must survive while the promise is stripped.
    let generic_offset = SOURCE.find("o;").expect("generic reference must exist");
    let narrowed_generic = normalize_type(&db, module, expression_ty_at(generic_offset));
    let formatted = format_inferred_type(&db, narrowed_generic);
    assert!(formatted.contains('T'), "{formatted}");
    assert!(!formatted.contains("Promise"), "{formatted}");

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_switch_and_equality_guards",
        &db,
        &fs,
    );
}

/// The tests of preceding `case` clauses evaluate at runtime even when
/// their clauses are not entered, so a write to the discriminant inside one
/// of them must decline narrowing.
#[test]
fn test_infer_module_types_declines_switch_narrowing_on_test_side_effects() {
    const SOURCE: &str = r#"
export function precedingCaseTestWrites(x: number | "a") {
    switch (x) {
        case (x = 5, "nope" as any):
            break;
        case "a":
            x;
            break;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let offset = SOURCE.find("x;").expect("case reference must exist");
    let start = TextSize::from(offset as u32);
    let range = TextRange::new(start, start + TextSize::from(1));
    let ty = inferred
        .expressions
        .get(&range)
        .copied()
        .expect("reference type must be inferred");
    let unnarrowed = normalize_type(&db, module, ty);
    assert!(contains_inferred_number(&db, unnarrowed));

    assert_inferred_type_snapshot(
        "test_infer_module_types_declines_switch_narrowing_on_test_side_effects",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_narrows_predicate_call_guards() {
    const SOURCE: &str = r#"
type Pred = { pr: string };

function isPred(value: unknown): value is Pred {
    return typeof value === "object" && value !== null;
}

export function predicates(t: unknown) {
    if (isPred(t)) {
        t;
    }
}

function isSecond(first: unknown, value: unknown): value is Pred {
    return typeof value === "object" && value !== null;
}

export function wrongPosition(w: unknown) {
    if (isSecond(w, 0)) {
        w;
    }
}

declare const others: unknown[];

export function spreadBefore(b: unknown) {
    if (isSecond(...others, b)) {
        b;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let expression_ty_at = |offset: usize| {
        let start = TextSize::from(offset as u32);
        let range = TextRange::new(start, start + TextSize::from(1));
        inferred
            .expressions
            .get(&range)
            .copied()
            .expect("reference type must be inferred")
    };

    // A type predicate narrows its argument to the predicate's type.
    let predicate_offset = SOURCE.find("t;").expect("predicate reference must exist");
    let narrowed_by_predicate = normalize_type(&db, module, expression_ty_at(predicate_offset));
    assert!(
        inferred
            .find_member_type(&db, narrowed_by_predicate, "pr")
            .is_some()
    );

    // The predicate is over the second parameter, but the reference was
    // passed as the first argument; it must keep its declared type.
    let wrong_position_offset = SOURCE
        .find("w;")
        .expect("wrong-position reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(wrong_position_offset));
    let formatted = format_inferred_type(&db, unnarrowed);
    assert!(!formatted.contains("Pred"), "{formatted}");

    // A spread before the reference makes its runtime parameter position
    // unknowable; it must keep its declared type.
    let spread_offset = SOURCE.find("b;").expect("spread reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(spread_offset));
    let formatted = format_inferred_type(&db, unnarrowed);
    assert!(!formatted.contains("Pred"), "{formatted}");

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_predicate_call_guards",
        &db,
        &fs,
    );
}

/// A guard says nothing about a name that the guarded code rebinds or
/// reassigns, so narrowing must be declined for it -- and must survive for
/// the names the branch leaves alone.
#[test]
fn test_infer_module_types_declines_narrowing_when_invalidated() {
    const SOURCE: &str = r#"
export function reassigned(x: number | string) {
    if (typeof x === "string") {
        x = 0;
        x;
    }
}

export function compoundAssigned(x: number | string) {
    if (typeof x === "string") {
        x += "!";
        x;
    }
}

// Without a case that does narrow, this snapshot would look the same
// whether narrowing worked or was removed entirely.
export function notInvalidated(x: number | string) {
    if (typeof x === "string") {
        x;
    }
}

export function shadowed(x: number | string) {
    if (typeof x === "string") {
        const x: boolean = true;
        x;
    }
}

// A write to one name must not invalidate another name guarded in the same
// nesting, which is what the per-name invalidation cache keys on.
export function oneNameInvalidatedNotTheOther(a: number | string, b: number | string) {
    if (typeof a === "string") {
        if (typeof b === "string") {
            a = 0;
            a;
            b;
        }
    }
}

// The scan is flow-insensitive, so a write that cannot reach the reference
// still invalidates it. TypeScript narrows both of these.
export function writeAfterUse(x: number | string) {
    if (typeof x === "string") {
        x;
        x = 0;
    }
}

export function writeInNestedClosure(x: number | string) {
    if (typeof x === "string") {
        x;
        const later = () => {
            x = 0;
        };
        later;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);

    assert_inferred_type_snapshot(
        "test_infer_module_types_declines_narrowing_when_invalidated",
        &db,
        &fs,
    );
}

/// A guard must not reach into a nested function or a class body, because
/// those run after the guarded name may have changed.
#[test]
fn test_infer_module_types_does_not_narrow_across_function_boundaries() {
    const SOURCE: &str = r#"
export function closureOverGuarded(x: number | (() => void)) {
    if (typeof x === "function") {
        x;
        const callback = () => {
            x;
        };
        callback;
    }
}

export function nestedParameterShadowsName(x: number | (() => void)) {
    if (typeof x === "function") {
        function inner(x: string | boolean) {
            x;
        }
        inner;
    }
}

export function classFieldInitializer(x: number | (() => void)) {
    if (typeof x === "function") {
        x;
        return class {
            field = x;
            static staticField = x;
        };
    }
    return null;
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);

    assert_inferred_type_snapshot(
        "test_infer_module_types_does_not_narrow_across_function_boundaries",
        &db,
        &fs,
    );
}

/// Covers the `typeof` comparison strings no other test in this file guards on.
#[test]
fn test_infer_module_types_narrows_remaining_typeof_results() {
    const SOURCE: &str = r#"
export function booleanGuard(b: boolean | string) {
    if (typeof b === "boolean") {
        b;
    }
}

export function bigintGuard(n: bigint | string) {
    if (typeof n === "bigint") {
        n;
    }
}

export function symbolGuard(s: symbol | string) {
    if (typeof s === "symbol") {
        s;
    }
}

export function numberGuard(v: number | string) {
    if (typeof v === "number") {
        v;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_remaining_typeof_results",
        &db,
        &fs,
    );
}

/// Narrowing applies only to the consequent of a direct `typeof` equality
/// test, so negated tests, conjunctions, `else` branches, and code after the
/// guard keep the declared type. `elseBranch` and `afterGuard` also read
/// `x.length` from inside a handled consequent, so the snapshot still records
/// narrowing somewhere and would change if narrowing regressed.
#[test]
fn test_infer_module_types_leaves_unsupported_guard_forms_unnarrowed() {
    const SOURCE: &str = r#"
export function negated(x: number | string) {
    if (typeof x !== "string") {
        x;
    }
}

export function conjunction(x: number | string) {
    if (typeof x === "string" && x.length > 0) {
        x;
    }
}

export function elseBranch(x: number | string) {
    if (typeof x === "string") {
        x.length;
    } else {
        x;
    }
}

export function afterGuard(x: number | string) {
    if (typeof x === "string") {
        x.length;
    }
    x;
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);

    assert_inferred_type_snapshot(
        "test_infer_module_types_leaves_unsupported_guard_forms_unnarrowed",
        &db,
        &fs,
    );
}
