use super::*;
use biome_rowan::TextSize;

/// Returns the inferred type of the one-character expression at `offset`,
/// where the narrowing tests place their `x;` references.
fn expression_ty_at<'db>(
    inferred: &InferredModuleTypes<'db>,
    offset: usize,
) -> InferredTypeData<'db> {
    let start = TextSize::from(offset as u32);
    let range = TextRange::new(start, start + TextSize::from(1));
    inferred
        .expressions
        .get(&range)
        .copied()
        .expect("reference type must be inferred")
}

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

    let narrowed_offset = SOURCE.find("x;").expect("guarded reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(inferred, narrowed_offset));
    assert!(narrowed.callable_function(&db).is_some());
    assert!(!contains_inferred_number(&db, narrowed));

    let unnarrowed_offset = SOURCE.rfind("x;").expect("trailing reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(inferred, unnarrowed_offset));
    assert!(contains_inferred_number(&db, unnarrowed));

    let undefined_offset = SOURCE
        .find("y;")
        .expect("undefined-guarded reference must exist");
    let narrowed_to_undefined =
        normalize_type(&db, module, expression_ty_at(inferred, undefined_offset));
    assert!(contains_inferred_undefined(&db, narrowed_to_undefined));
    assert!(!contains_inferred_string(&db, narrowed_to_undefined));

    let string_offset = SOURCE
        .rfind("y;")
        .expect("string-guarded reference must exist");
    let narrowed_to_string = normalize_type(&db, module, expression_ty_at(inferred, string_offset));
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

    // An interface with a call signature is a function at runtime.
    let function_offset = SOURCE
        .find("f;")
        .expect("function-guarded reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(inferred, function_offset));
    let formatted = format_inferred_type(&db, narrowed);
    assert!(formatted.contains("interface \"AsyncFn\""), "{formatted}");
    assert!(formatted.contains("Function"), "{formatted}");
    assert!(!formatted.contains("null"), "{formatted}");

    // `typeof null` is `"object"`.
    let object_offset = SOURCE
        .rfind("f;")
        .expect("object-guarded reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(inferred, object_offset));
    let formatted = format_inferred_type(&db, narrowed);
    assert!(formatted.contains("null"), "{formatted}");
    assert!(!formatted.contains("AsyncFn"), "{formatted}");
    assert!(!formatted.contains("Function"), "{formatted}");

    // A class value is a constructor function at runtime.
    let class_offset = SOURCE.find("c;").expect("class reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(inferred, class_offset));
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

    // A truthiness guard drops the nullish variant.
    let truthy_offset = SOURCE
        .find("z;")
        .expect("truthy-guarded reference must exist");
    let narrowed_to_truthy = normalize_type(&db, module, expression_ty_at(inferred, truthy_offset));
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
    let narrowed_to_falsy = normalize_type(&db, module, expression_ty_at(inferred, falsy_offset));
    assert!(contains_inferred_null(&db, narrowed_to_falsy));
    assert!(!contains_inferred_string_literal(
        &db,
        narrowed_to_falsy,
        "on"
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_truthiness_guards",
        &db,
        &fs,
    );
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

    // An instanceof guard over a base class reference downcasts it, making
    // the subclass member visible.
    let downcast_offset = SOURCE.find("v;").expect("downcast reference must exist");
    let narrowed_to_derived =
        normalize_type(&db, module, expression_ty_at(inferred, downcast_offset));
    assert!(
        inferred
            .find_member_type(&db, narrowed_to_derived, "run")
            .is_some()
    );

    // An instanceof guard strips union variants that cannot be instances.
    let union_offset = SOURCE.find("w;").expect("union reference must exist");
    let narrowed_to_instance =
        normalize_type(&db, module, expression_ty_at(inferred, union_offset));
    assert!(!contains_inferred_number(&db, narrowed_to_instance));
    assert!(
        inferred
            .find_member_type(&db, narrowed_to_instance, "run")
            .is_some()
    );

    // A variant whose extends chain contains a mixin call cannot be walked
    // to a proof, so it must be kept.
    let mixin_offset = SOURCE.find("m;").expect("mixin reference must exist");
    let narrowed_with_mixin = normalize_type(&db, module, expression_ty_at(inferred, mixin_offset));
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

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_instanceof_guards",
        &db,
        &fs,
    );
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

    // A discriminant guard strips union variants whose member is a literal
    // with a different value.
    let discriminant_offset = SOURCE
        .find("c;")
        .expect("discriminant reference must exist");
    let narrowed_to_left =
        normalize_type(&db, module, expression_ty_at(inferred, discriminant_offset));
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
    let narrowed_to_escaped =
        normalize_type(&db, module, expression_ty_at(inferred, escape_offset));
    assert!(
        inferred
            .find_member_type(&db, narrowed_to_escaped, "escaped")
            .is_some()
    );

    // A write to a member of the narrowed value inside the consequent
    // declines discriminant narrowing.
    let mutated_offset = SOURCE.find("m;").expect("mutated reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(inferred, mutated_offset));
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

// Biome does not model the members of `String`, so an object-like variant
// is never ruled out by a string comparison.
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

    // A switch case over a member discriminant narrows like the equivalent
    // `if` guard.
    let switch_offset = SOURCE.find("s;").expect("switch reference must exist");
    let narrowed_in_case = normalize_type(&db, module, expression_ty_at(inferred, switch_offset));
    let case_kind_ty = inferred
        .find_member_type(&db, narrowed_in_case, "kind")
        .expect("kind member must be inferred");
    let case_kind_ty = normalize_type(&db, module, case_kind_ty);
    assert!(is_inferred_string_literal(&db, case_kind_ty, "left"));

    // An equality guard narrows the union to the compared literal.
    let equality_offset = SOURCE.find("q;").expect("equality reference must exist");
    let narrowed_to_done = normalize_type(&db, module, expression_ty_at(inferred, equality_offset));
    assert!(is_inferred_string_literal(&db, narrowed_to_done, "done"));
    assert!(!contains_inferred_number(&db, narrowed_to_done));

    // A promise instance can never strictly equal a string.
    let promise_equality_offset = SOURCE
        .find("u;")
        .expect("promise equality reference must exist");
    let narrowed_from_promise = normalize_type(
        &db,
        module,
        expression_ty_at(inferred, promise_equality_offset),
    );
    assert!(is_inferred_string_literal(
        &db,
        narrowed_from_promise,
        "done"
    ));

    // An interface may describe a string, so its variant is retained.
    let satisfies_offset = SOURCE
        .find("g;")
        .expect("string-satisfies reference must exist");
    let narrowed_to_lengthy =
        normalize_type(&db, module, expression_ty_at(inferred, satisfies_offset));
    assert!(!contains_inferred_number(&db, narrowed_to_lengthy));
    assert!(
        inferred
            .find_member_type(&db, narrowed_to_lengthy, "length")
            .is_some()
    );

    // A generic type parameter could be instantiated with the compared
    // string, so its variant must survive while the promise is stripped.
    let generic_offset = SOURCE.find("o;").expect("generic reference must exist");
    let narrowed_generic = normalize_type(&db, module, expression_ty_at(inferred, generic_offset));
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

type Kinded =
    | { kind: "a"; a: number }
    | { kind: "b"; b: number };

export function precedingCaseTestWritesMember(y: Kinded) {
    switch (y.kind) {
        case (y.kind = "a", "zzz" as any):
            break;
        case "b":
            y;
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
    let unnarrowed = normalize_type(&db, module, expression_ty_at(inferred, offset));
    assert!(contains_inferred_number(&db, unnarrowed));

    // A preceding case test that writes to a member of the discriminant
    // object must also decline member-discriminant narrowing.
    let member_offset = SOURCE
        .find("y;")
        .expect("member-discriminant reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(inferred, member_offset));
    let formatted = format_inferred_type(&db, unnarrowed);
    assert!(formatted.contains("\"a\""), "{formatted}");
    assert!(formatted.contains("\"b\""), "{formatted}");

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

declare function isFirstWithThis(this: void, first: unknown, second: unknown): first is Pred;

export function thisParameter(p: unknown, q: unknown) {
    if (isFirstWithThis(p, q)) {
        p;
        q;
    }
}

function looksRight(value: unknown): boolean {
    return typeof value === "object";
}

export function plainBoolean(n: unknown) {
    if (looksRight(n)) {
        n;
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

    // A type predicate narrows its argument to the predicate's type.
    let predicate_offset = SOURCE.find("t;").expect("predicate reference must exist");
    let narrowed_by_predicate =
        normalize_type(&db, module, expression_ty_at(inferred, predicate_offset));
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
    let unnarrowed = normalize_type(
        &db,
        module,
        expression_ty_at(inferred, wrong_position_offset),
    );
    let formatted = format_inferred_type(&db, unnarrowed);
    assert!(!formatted.contains("Pred"), "{formatted}");

    // A spread before the reference makes its runtime parameter position
    // unknowable; it must keep its declared type.
    let spread_offset = SOURCE.find("b;").expect("spread reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(inferred, spread_offset));
    let formatted = format_inferred_type(&db, unnarrowed);
    assert!(!formatted.contains("Pred"), "{formatted}");

    // A `this` parameter occupies a parameter slot but no argument
    // position: the predicate over the first real parameter narrows the
    // first argument, not the second.
    let this_param_offset = SOURCE.find("p;").expect("this-param reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(inferred, this_param_offset));
    assert!(inferred.find_member_type(&db, narrowed, "pr").is_some());
    let second_offset = SOURCE.find("q;").expect("second reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(inferred, second_offset));
    let formatted = format_inferred_type(&db, unnarrowed);
    assert!(!formatted.contains("Pred"), "{formatted}");

    // A callee returning plain `boolean` is not a type predicate.
    let plain_offset = SOURCE
        .find("n;")
        .expect("plain-boolean reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(inferred, plain_offset));
    assert!(!contains_inferred_instance(&db, unnarrowed));

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_predicate_call_guards",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_narrows_assigned_references() {
    const SOURCE: &str = r#"
export function assigned(x: number | (() => Promise<void>)) {
    x = async () => {};
    x;
}

export function nearestWins(y: string | number | undefined) {
    y = "first";
    y = 1;
    y;
}

export function conditionalWrite(z: string | undefined, flag: boolean) {
    z = "on";
    if (flag) {
        z = undefined;
    }
    z;
}

export function closureWrite(a: string | undefined) {
    a = "on";
    const reset = () => {
        a = undefined;
    };
    reset();
    a;
}

export function closureBeforeAssignment(f: string | undefined) {
    const reset = () => {
        f = undefined;
    };
    f = "on";
    reset();
    f;
}

export function insideClosure(b: string | undefined) {
    b = "on";
    return () => {
        b;
    };
}

export function compoundWrite(c: number | undefined) {
    c = 1;
    c += 1;
    c;
}

export function nestedBlockWrite(q: string | undefined) {
    q = "on";
    {
        q = undefined;
    }
    q;
}

export function loopTest(e: string | undefined) {
    e = "on";
    for (; e; ) {
        e = undefined;
    }
}

export function reassignedInGuard(m: number | string) {
    if (typeof m === "string") {
        m = 7;
        m;
    }
}

export function selfReference(n: number | undefined) {
    n = 1;
    n = n;
    n;
}

export function unresolvedRhs(w: string | undefined) {
    w = missing;
    w;
}

export function destructured(p: string | undefined, arr: [string]) {
    p = "on";
    [p] = arr;
    p;
}

export function scanPast(v: string | undefined, log: (value: string) => void) {
    v = "on";
    log(v);
    v;
}

export function caseAssignment(t: string | undefined, choice: string) {
    switch (choice) {
        case "set":
            t = "on";
            t;
            break;
    }
}

export function updateWrite(j: number | undefined) {
    j = 1;
    j++;
    j;
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    // After an assignment, the reference takes the assigned type.
    let assigned_offset = SOURCE.find("x;").expect("assigned reference must exist");
    let narrowed_to_assigned =
        normalize_type(&db, module, expression_ty_at(inferred, assigned_offset));
    assert!(narrowed_to_assigned.callable_function(&db).is_some());
    assert!(!contains_inferred_number(&db, narrowed_to_assigned));

    // The nearest preceding assignment wins.
    let nearest_offset = SOURCE.find("y;").expect("nearest reference must exist");
    let narrowed_to_nearest =
        normalize_type(&db, module, expression_ty_at(inferred, nearest_offset));
    assert!(is_inferred_number(&db, narrowed_to_nearest));

    // A conditional write between the assignment and the reference cancels
    // the narrowing.
    let conditional_offset = SOURCE
        .rfind("z;")
        .expect("conditional reference must exist");
    let unnarrowed_conditional =
        normalize_type(&db, module, expression_ty_at(inferred, conditional_offset));
    assert!(contains_inferred_undefined(&db, unnarrowed_conditional));

    // A closure between the assignment and the reference that writes the
    // value cancels the narrowing.
    let closure_offset = SOURCE.rfind("a;").expect("closure reference must exist");
    let unnarrowed_closure =
        normalize_type(&db, module, expression_ty_at(inferred, closure_offset));
    assert!(contains_inferred_undefined(&db, unnarrowed_closure));

    // A call to a closure that writes the name is not a write in the
    // statement list, so the assignment still narrows. TypeScript agrees.
    let closure_first_offset = SOURCE
        .rfind("f;")
        .expect("closure-before-assignment reference must exist");
    let narrowed_past_closure_call = normalize_type(
        &db,
        module,
        expression_ty_at(inferred, closure_first_offset),
    );
    assert!(is_inferred_string(&db, narrowed_past_closure_call));

    // A reference inside a closure is never narrowed by outer assignments.
    let inside_closure_offset = SOURCE
        .find("b;")
        .expect("inside-closure reference must exist");
    let unnarrowed_inside = normalize_type(
        &db,
        module,
        expression_ty_at(inferred, inside_closure_offset),
    );
    assert!(contains_inferred_undefined(&db, unnarrowed_inside));

    // A compound assignment is not a narrowing source, and cancels scanning
    // past it.
    let compound_offset = SOURCE.find("c;").expect("compound reference must exist");
    let unnarrowed_compound =
        normalize_type(&db, module, expression_ty_at(inferred, compound_offset));
    assert!(contains_inferred_undefined(&db, unnarrowed_compound));

    // A write inside a nested block between the assignment and the
    // reference cancels the narrowing.
    let nested_offset = SOURCE.rfind("q;").expect("nested reference must exist");
    let unnarrowed_nested = normalize_type(&db, module, expression_ty_at(inferred, nested_offset));
    assert!(contains_inferred_undefined(&db, unnarrowed_nested));

    // A loop test that the loop body writes to is re-evaluated, so it must
    // not be narrowed.
    let loop_offset = SOURCE.find("e;").expect("loop reference must exist");
    let unnarrowed_loop = normalize_type(&db, module, expression_ty_at(inferred, loop_offset));
    assert!(contains_inferred_undefined(&db, unnarrowed_loop));

    // An assignment inside a guarded consequent cancels the guard but
    // establishes its own narrowing.
    let guard_offset = SOURCE.find("m;").expect("guard reference must exist");
    let narrowed_in_guard = normalize_type(&db, module, expression_ty_at(inferred, guard_offset));
    assert!(is_inferred_number(&db, narrowed_in_guard));

    // A self-referencing assignment resolves without narrowing its own
    // right-hand side.
    let self_offset = SOURCE.rfind("n;").expect("self reference must exist");
    let narrowed_self = normalize_type(&db, module, expression_ty_at(inferred, self_offset));
    assert!(contains_inferred_undefined(&db, narrowed_self));
    assert!(contains_inferred_number(&db, narrowed_self));

    // An unresolvable assigned type falls back to the declared type.
    let unresolved_offset = SOURCE.find("w;").expect("unresolved reference must exist");
    let unnarrowed_unresolved =
        normalize_type(&db, module, expression_ty_at(inferred, unresolved_offset));
    assert!(contains_inferred_undefined(&db, unnarrowed_unresolved));
    assert!(contains_inferred_string(&db, unnarrowed_unresolved));

    // A destructuring assignment is not a narrowing source, and cancels
    // scanning past it.
    let destructured_offset = SOURCE
        .rfind("p;")
        .expect("destructured reference must exist");
    let unnarrowed_destructured =
        normalize_type(&db, module, expression_ty_at(inferred, destructured_offset));
    assert!(contains_inferred_undefined(&db, unnarrowed_destructured));

    // The scan walks past intervening statements that cannot write the
    // value.
    let scan_past_offset = SOURCE.find("v;").expect("scan-past reference must exist");
    let narrowed_past_call =
        normalize_type(&db, module, expression_ty_at(inferred, scan_past_offset));
    assert!(is_inferred_string(&db, narrowed_past_call));

    // A case clause is its own statement list, and an assignment inside it
    // narrows references that follow in the same clause.
    let case_offset = SOURCE.find("t;").expect("case reference must exist");
    let narrowed_in_clause = normalize_type(&db, module, expression_ty_at(inferred, case_offset));
    assert!(is_inferred_string(&db, narrowed_in_clause));

    // An update expression writes the value, and cancels scanning past it.
    let update_offset = SOURCE.find("j;").expect("update reference must exist");
    let unnarrowed_update = normalize_type(&db, module, expression_ty_at(inferred, update_offset));
    assert!(contains_inferred_undefined(&db, unnarrowed_update));

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_assigned_references",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_treats_object_values_as_truthy() {
    const SOURCE: &str = r#"
class Service {
    run(): void {}
}

interface Task {
    id: string;
}

export function promiseInstance(x: Promise<void> | undefined) {
    if (!x) {
        x;
    }
    if (x) {
        x;
    }
}

export function classInstance(y: Service | null) {
    if (!y) {
        y;
    }
}

export function interfaceValue(z: Task | undefined) {
    if (!z) {
        z;
    }
}

export function genericValue<T>(k: T | undefined) {
    if (!k) {
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

    // A promise instance can never be falsy, so a negated guard narrows the
    // union to `undefined`.
    let falsy_promise_offset = SOURCE.find("x;").expect("falsy reference must exist");
    let narrowed_to_undefined = normalize_type(
        &db,
        module,
        expression_ty_at(inferred, falsy_promise_offset),
    );
    assert!(!contains_inferred_instance(&db, narrowed_to_undefined));
    assert!(contains_inferred_undefined(&db, narrowed_to_undefined));

    // The truthy branch keeps the promise instance.
    let truthy_promise_offset = SOURCE.rfind("x;").expect("truthy reference must exist");
    let narrowed_to_promise = normalize_type(
        &db,
        module,
        expression_ty_at(inferred, truthy_promise_offset),
    );
    assert!(is_inferred_promise_instance(&db, narrowed_to_promise));

    // A class instance can never be falsy either.
    let class_offset = SOURCE.find("y;").expect("class reference must exist");
    let narrowed_to_null = normalize_type(&db, module, expression_ty_at(inferred, class_offset));
    assert!(!contains_inferred_instance(&db, narrowed_to_null));
    assert!(contains_inferred_null(&db, narrowed_to_null));

    // A value of an interface type is an object at runtime, following
    // TypeScript's narrowing semantics.
    let interface_offset = SOURCE.find("z;").expect("interface reference must exist");
    let narrowed_interface =
        normalize_type(&db, module, expression_ty_at(inferred, interface_offset));
    assert!(contains_inferred_undefined(&db, narrowed_interface));
    assert!(
        inferred
            .find_member_type(&db, narrowed_interface, "id")
            .is_none()
    );

    // A generic value could be instantiated with a falsy type, so it must
    // be kept.
    let generic_offset = SOURCE.find("k;").expect("generic reference must exist");
    let kept_generic = normalize_type(&db, module, expression_ty_at(inferred, generic_offset));
    assert!(contains_inferred_instance(&db, kept_generic));
    assert!(contains_inferred_undefined(&db, kept_generic));

    assert_inferred_type_snapshot(
        "test_infer_module_types_treats_object_values_as_truthy",
        &db,
        &fs,
    );
}

/// A guard says nothing about a name the guarded code rebinds or reassigns;
/// the reassigned rows in the snapshot show the assigned type instead.
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

// A getter, a setter and a static initialization block are function-like
// scopes too, even though `async` is never valid in them.
export function accessorsAndStaticBlock(x: number | (() => void)) {
    if (typeof x === "function") {
        x;
        return class {
            static {
                x;
            }
            get value() {
                return x;
            }
            set value(_v: unknown) {
                x;
            }
        };
    }
    return null;
}

export function methodsAndConstructor(x: number | (() => void)) {
    if (typeof x === "function") {
        x;
        return class {
            constructor() {
                x;
            }
            method() {
                x;
            }
        };
    }
    return null;
}

export function objectAccessors(x: number | (() => void)) {
    if (typeof x === "function") {
        x;
        return {
            get value() {
                return x;
            },
            set value(_v: unknown) {
                x;
            },
            method() {
                x;
            },
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

/// Covers the shapes whose `typeof` tag is decided by something other than a
/// plain keyword: a callable interface's bases, literals, and tuples.
#[test]
fn test_infer_module_types_reads_typeof_tags_from_type_shapes() {
    const SOURCE: &str = r#"
interface CallableBase {
    (): void;
}

// A base can still contribute a call signature, so the interface's own tag
// is unknown and it survives a guard for any tag.
interface DerivedFromCallable extends CallableBase {
    tag: string;
}

export function interfaceWithBases(a: DerivedFromCallable | number) {
    if (typeof a === "object") {
        a;
    }
}

export function tupleValue(b: [string, number] | number) {
    if (typeof b === "object") {
        b;
    }
}

export function regexpLiteral(c: number) {
    const value = c > 0 ? /re/ : c;
    if (typeof value === "object") {
        value;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);

    assert_inferred_type_snapshot(
        "test_infer_module_types_reads_typeof_tags_from_type_shapes",
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

/// Negated tests, conjunctions, `else` branches, and code after the guard
/// keep the declared type.
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

#[test]
fn test_infer_module_types_narrows_contradictory_equality_guards_to_never() {
    const SOURCE: &str = r#"
export function stringEquals(x: "a" | "b" | "c") {
    if (x === "a") {
        if (x === "b") {
            x;
        }
    }
}

type Choice =
    | { kind: "left"; left: string }
    | { kind: "right"; right: number };

export function memberEquals(s: Choice) {
    if (s.kind === "left") {
        if (s.kind === "right") {
            s;
        }
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);

    // No value passes both guards, so applying them in turn leaves nothing,
    // the same way contradictory `typeof` guards already resolve to `never`.
    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_contradictory_equality_guards_to_never",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_declines_member_narrowing_after_delete() {
    const SOURCE: &str = r#"
type Choice =
    | { kind: "left"; left: string }
    | { kind: "right"; right: number };

export function example(s: Choice) {
    if (s.kind === "left") {
        delete s.kind;
        s;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);

    // `delete s.kind` can change the discriminant, so the MemberEquals guard
    // must be declined here just as it is for a plain assignment to `s.kind`.
    assert_inferred_type_snapshot(
        "test_infer_module_types_declines_member_narrowing_after_delete",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_keeps_number_under_falsy_guards() {
    const SOURCE: &str = r#"
export function example(n: number) {
    if (!n) {
        n;
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

    let offset = SOURCE.find("n;").expect("guarded reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(inferred, offset));

    // `0` is not the only falsy number: `-0` and `NaN` are falsy too, so the
    // guard cannot narrow `number` down to a single literal.
    assert!(contains_inferred_number(&db, narrowed));
    assert!(!contains_inferred_number_literal(&db, narrowed, "0"));
}

/// Guards of different kinds nest just like guards of the same kind: the
/// value inside passed every test, so every enclosing guard narrows it.
#[test]
fn test_infer_module_types_narrows_nested_guards_of_different_kinds() {
    const SOURCE: &str = r#"
export function typeofThenTruthy(x: string | undefined) {
    if (typeof x === "undefined") {
        if (x) {
            x;
        }
    }
}

export function typeofThenTruthyKeepsSubset(y: string | Promise<void>) {
    if (typeof y === "string") {
        if (y) {
            y;
        }
    }
}

export function truthyThenTypeof(z: string | Promise<void>) {
    if (z) {
        if (typeof z === "string") {
            z;
        }
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

    let ty_at = |needle: &str| {
        let offset = SOURCE.find(needle).expect("reference must exist");
        normalize_type(&db, module, expression_ty_at(inferred, offset))
    };

    // Nothing is both `undefined` and truthy. Keeping only the inner guard
    // would answer `string` instead.
    let outer_undefined = ty_at("x;");
    assert!(matches!(outer_undefined, InferredTypeData::NeverKeyword));

    // Keeping only the inner truthiness guard would put the promise back,
    // which the outer `typeof` guard had already ruled out.
    let outer_string = ty_at("y;");
    assert!(contains_inferred_string(&db, outer_string));
    assert!(!contains_inferred_instance(&db, outer_string));

    // The same holds when the more specific guard is the inner one.
    let inner_string = ty_at("z;");
    assert!(contains_inferred_string(&db, inner_string));
    assert!(!contains_inferred_instance(&db, inner_string));

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_nested_guards_of_different_kinds",
        &db,
        &fs,
    );
}

/// A `case` narrows only when every clause before it provably exits, so each
/// statement kind that ends a clause has to be recognised.
#[test]
fn test_infer_module_types_narrows_after_every_case_exit_kind() {
    const SOURCE: &str = r#"
type Choice =
    | { kind: "left"; left: string }
    | { kind: "right"; right: number };

export function afterReturn(v1: Choice) {
    switch (v1.kind) {
        case "right":
            return;
        case "left":
            v1;
            break;
    }
}

export function afterThrow(v2: Choice) {
    switch (v2.kind) {
        case "right":
            throw new Error();
        case "left":
            v2;
            break;
    }
}

export function afterContinue(v3: Choice) {
    while (true) {
        switch (v3.kind) {
            case "right":
                continue;
            case "left":
                v3;
                break;
        }
    }
}

export function afterDefault(v4: Choice) {
    switch (v4.kind) {
        default:
            break;
        case "left":
            v4;
            break;
    }
}

export function afterNestedExit(v5: Choice) {
    switch (v5.kind) {
        case "right": {
            break;
        }
        case "left":
            v5;
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

    // `needle` is the reference followed by its `;`, so the reference itself
    // is one character shorter.
    let formatted_at = |needle: &str| {
        let offset = SOURCE.find(needle).expect("reference must exist");
        let start = TextSize::from(offset as u32);
        let range = TextRange::new(start, start + TextSize::from(needle.len() as u32 - 1));
        let ty = normalize_type(
            &db,
            module,
            inferred
                .expressions
                .get(&range)
                .copied()
                .expect("reference type must be inferred"),
        );
        format_inferred_type(&db, ty)
    };

    for (needle, kind) in [("v1;", "return"), ("v2;", "throw"), ("v3;", "continue")] {
        let formatted = formatted_at(needle);
        assert!(!formatted.contains("right"), "after {kind}: {formatted}");
    }

    // A `default` clause that breaks cannot fall through either.
    let formatted = formatted_at("v4;");
    assert!(!formatted.contains("right"), "{formatted}");

    // The exit sits inside a block, which the scan does not look into, so the
    // clause conservatively declines narrowing for its successors.
    let formatted = formatted_at("v5;");
    assert!(formatted.contains("right"), "{formatted}");
}

/// Covers the `instanceof` outcomes no other test reaches: two classes that
/// are provably unrelated, an `extends` chain that loops, and a guard that is
/// not a class at all.
#[test]
fn test_infer_module_types_narrows_unrelated_and_cyclic_instanceof_guards() {
    const SOURCE: &str = r#"
class Left {
    left(): void {}
}
class Right {
    right(): void {}
}

// Both chains reach their root without meeting the other, so the guard
// leaves nothing of the other variant.
export function unrelatedClasses(v: Left | Right) {
    if (v instanceof Right) {
        v;
    }
}

class SelfExtending extends SelfExtending {}

// Walking the chain must terminate rather than follow the loop forever.
export function cyclicChain(w: SelfExtending | number) {
    if (w instanceof SelfExtending) {
        w;
    }
}

interface NotAClass {
    tag: string;
}
declare const notAClass: NotAClass;

// The right-hand side never resolves to a class, so nothing is narrowed.
export function nonClassGuard(y: Left | number) {
    if (y instanceof (notAClass as unknown as typeof Left)) {
        y;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_unrelated_and_cyclic_instanceof_guards",
        &db,
        &fs,
    );
}

/// A falsy or truthy guard maps a few types to the only literal they can hold
/// there; every other arm keeps or drops the type as a whole.
#[test]
fn test_infer_module_types_maps_types_to_their_only_literal() {
    const SOURCE: &str = r#"
export function falsyMappings(a: bigint | boolean | string) {
    if (!a) {
        a;
    }
}

export function truthyBoolean(b: boolean | null) {
    if (b) {
        b;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);

    assert_inferred_type_snapshot(
        "test_infer_module_types_maps_types_to_their_only_literal",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_narrows_unreachable_branches_to_never() {
    const SOURCE: &str = r#"
class Cls {}

export function falsyInstance(p: Promise<void>) {
    if (!p) {
        p;
    }
}

export function instanceofPrimitive(v: number | string) {
    if (v instanceof Cls) {
        v;
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

    // No variant survives either guard, so the branch cannot run.
    for needle in ["p;", "v;"] {
        let offset = SOURCE.find(needle).expect("reference must exist");
        let narrowed = normalize_type(&db, module, expression_ty_at(inferred, offset));
        assert!(
            matches!(narrowed, InferredTypeData::NeverKeyword),
            "{needle} must be never, got {narrowed:?}"
        );
    }

    assert_inferred_type_snapshot(
        "test_infer_module_types_narrows_unreachable_branches_to_never",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_widens_assigned_literals_like_typescript() {
    const SOURCE: &str = r#"
declare function pick(): string | number;

export let top: string | undefined;
top = "on";
top;

export function widens(y: string | number | undefined) {
    y = 1;
    y;
}

export function keepsLiteral(z: "a" | "b" | undefined) {
    z = "a";
    z;
}

export function guardedAfterAssignment(v: string | number | undefined) {
    v = pick();
    if (typeof v === "string") v;
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    // Module-level statements narrow like statements in a function body.
    let top_offset = SOURCE.find("top;").expect("top-level reference must exist");
    let start = TextSize::from(top_offset as u32);
    let top_ty = inferred
        .expressions
        .get(&TextRange::new(start, start + TextSize::from(3)))
        .copied()
        .expect("top-level reference type must be inferred");
    let narrowed_top = normalize_type(&db, module, top_ty);
    assert!(is_inferred_string(&db, narrowed_top), "{narrowed_top:?}");

    // `y: string | number | undefined` admits `number`, so `1` widens to it.
    let widened_offset = SOURCE.find("y;").expect("widened reference must exist");
    let widened = normalize_type(&db, module, expression_ty_at(inferred, widened_offset));
    assert!(is_inferred_number(&db, widened), "{widened:?}");

    // `z` admits no `string`, so the literal stays.
    let literal_offset = SOURCE.find("z;").expect("literal reference must exist");
    let literal = normalize_type(&db, module, expression_ty_at(inferred, literal_offset));
    assert!(is_inferred_string_literal(&db, literal, "a"));

    // The guard applies on top of the assigned type.
    let guarded_offset = SOURCE.find("v;").expect("guarded reference must exist");
    let guarded = normalize_type(&db, module, expression_ty_at(inferred, guarded_offset));
    assert!(is_inferred_string(&db, guarded), "{guarded:?}");

    assert_inferred_type_snapshot(
        "test_infer_module_types_widens_assigned_literals_like_typescript",
        &db,
        &fs,
    );
}
