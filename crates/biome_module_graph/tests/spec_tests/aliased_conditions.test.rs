use super::*;
use biome_module_graph::type_inference::{
    NormalizedExpressionTypeRequest, TypeInferenceCaller, execute_type_inference_request,
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

fn normalized_type_at<'db>(
    db: &'db TestModuleDb,
    module: ModuleInfo,
    source: &str,
    marker: &str,
    expression: &str,
) -> InferredTypeData<'db> {
    let range = marked_range(source, marker, expression);
    execute_type_inference_request(
        db,
        TypeInferenceCaller::new("test", "aliasedConditions"),
        NormalizedExpressionTypeRequest::new(module, range),
    )
    .unwrap_or_else(|| panic!("expression at {marker} {range:?} must be inferred"))
}

/// Asserts the type of each `(marker, expression, expected)` read in `source`,
/// where `expected` lists the members of a union.
fn assert_reads(source: &str, reads: &[(&str, &str, &[InferredTypeData<'static>])]) {
    let (db, module) = narrowing_db(source);
    for (marker, expression, expected) in reads {
        let ty = normalized_type_at(&db, module, source, marker, expression);
        let actual = match ty {
            InferredTypeData::Union(union) => union.types(&db).to_vec(),
            _ => vec![ty],
        };
        assert!(
            actual.len() == expected.len() && expected.iter().all(|ty| actual.contains(ty)),
            "{marker}: expected {expected:?}, got {}",
            format_inferred_type(&db, ty)
        );
    }
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// because testing a `const` tests the condition that initializes it.
#[test]
fn aliased_conditions_narrow_like_their_initializers() {
    use InferredTypeData::{Null, Number, String};
    const SOURCE: &str = r#"
        declare function isString(value: unknown): value is string;
        declare function assert(value: unknown): asserts value;
        function typeOf(x: string | number) {
            const isS = typeof x === "string";
            if (isS) /*typeof*/x; else /*typeofElse*/x;
        }
        function both(a: string | null, b: string | null) {
            const ok = a !== null && b !== null;
            if (ok) { /*bothA*/a; /*bothB*/b; } else /*bothElse*/a;
        }
        function earlyReturn(x: string | null) { const ok = x !== null; if (!ok) return; /*early*/x; }
        function identifier(x: string | null) { const ok = x; if (ok) /*identifier*/x; }
        function itself(x: string | null) { const ok = x; if (ok) /*itself*/ok; }
        function notTaken(x: string | null) { const ok = x === null; if (ok) {} else /*else*/x; }
        function parenthesized(x: string | null) { const ok = (x !== null); if ((ok)) /*parens*/x; }
        function guard(x: string | number) { const ok = isString(x); if (ok) /*guard*/x; }
        function assertion(x: string | null) { const ok = x !== null; assert(ok); /*assert*/x; }
        function neverWritten(x: string | null) { let y = x; const ok = y !== null; if (ok) /*let*/y; }
        function destructured({ x }: { x: string | null }) { const ok = x !== null; if (ok) /*destructured*/x; }
        function destructuredConst(o: { a: string | null }) { const { a } = o; const ok = a !== null; if (ok) /*destructuredConst*/a; }
        class K { constructor(private x: string | null) { const ok = x !== null; if (ok) /*property*/x; } }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("typeof", "x", &[String]),
            ("typeofElse", "x", &[Number]),
            ("bothA", "a", &[String]),
            ("bothB", "b", &[String]),
            ("bothElse", "a", &[String, Null]),
            ("early", "x", &[String]),
            ("identifier", "x", &[String]),
            ("itself", "ok", &[String]),
            ("else", "x", &[String]),
            ("parens", "x", &[String]),
            ("guard", "x", &[String]),
            ("assert", "x", &[String]),
            ("let", "y", &[String]),
            ("destructured", "x", &[String]),
            ("destructuredConst", "a", &[String]),
            ("property", "x", &[String]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `x` the type `{ y: string }`,
// because the inlined `x?.y` shows that `x` is not nullish, as `if (x?.y)`
// does.
#[test]
fn aliased_conditions_narrow_optional_chain_receivers() {
    const SOURCE: &str = r#"
        function receiver(x: { y: string } | undefined) { const ok = x?.y; if (ok) /*receiver*/x; }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    let ty = normalized_type_at(&db, module, SOURCE, "receiver", "x");
    assert_ne!(ty, InferredTypeData::Unknown);
    assert!(
        !contains_inferred_undefined(&db, ty),
        "{}",
        format_inferred_type(&db, ty)
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
#[test]
fn aliases_combine_with_negation_logical_operators_and_loops() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        function not(x: string | null) { const ok = x !== null; if (!ok) /*not*/x; else /*notElse*/x; }
        function and(x: string | null, y: boolean) { const ok = x !== null; if (ok && y) /*and*/x; else /*andElse*/x; }
        function or(x: string | null, y: boolean) { const ok = x !== null; if (ok || y) /*or*/x; else /*orElse*/x; }
        function operand(x: string | null) { const ok = x !== null; ok && /*operand*/x; }
        function ternary(x: string | null) { const ok = x !== null; ok ? /*ternary*/x : /*ternaryElse*/x; }
        function loop(x: string | null) { const ok = x !== null; while (ok) { /*while*/x; break; } /*afterWhile*/x; }
        function forLoop(x: string | null) { const ok = x !== null; for (; ok; ) { /*for*/x; break; } }
        function doWhile(x: string | null) { const ok = x !== null; do {} while (ok); /*doWhile*/x; }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("not", "x", &[Null]),
            ("notElse", "x", &[String]),
            ("and", "x", &[String]),
            ("andElse", "x", &[String, Null]),
            ("or", "x", &[String, Null]),
            ("orElse", "x", &[Null]),
            ("operand", "x", &[String]),
            ("ternary", "x", &[String]),
            ("ternaryElse", "x", &[Null]),
            ("while", "x", &[String]),
            ("afterWhile", "x", &[String, Null]),
            ("for", "x", &[String]),
            ("doWhile", "x", &[Null]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) inlines five aliases into one another, not
// six. It narrows `x` at `five`, but tests only the truthiness of `c1` at
// `six`, so `x` keeps `string | null` there.
#[test]
fn aliases_inline_up_to_five_levels() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        function depth(x: string | null) {
            const c1 = x !== null; const c2 = c1; const c3 = c2; const c4 = c3; const c5 = c4; const c6 = c5;
            if (c5) /*five*/x;
            if (c6) /*six*/x;
        }
    "#;
    assert_reads(
        SOURCE,
        &[("five", "x", &[String]), ("six", "x", &[String, Null])],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) applies the inlined condition to the type
// that the subject has at the test, wherever the alias is declared. A branch
// that the alias rules out is impossible. TypeScript gives `never` there, and
// Biome gives Unknown.
#[test]
fn aliases_apply_their_initializer_where_they_are_tested() {
    use InferredTypeData::{Number, String, Unknown};
    const SOURCE: &str = r#"
        function before(x: string | number | null) {
            const isS = typeof x === "string";
            if (x === null) return;
            if (isS) /*before*/x; else /*beforeElse*/x;
        }
        function inside(x: string | number | null) {
            if (x !== null) { const isS = typeof x === "string"; if (isS) /*inside*/x; else /*insideElse*/x; }
        }
        function impossible(x: string | number | null) {
            const isS = typeof x === "string";
            if (typeof x === "number") { if (isS) /*impossible*/x; }
        }
        function contradicted(x: string | null) { const ok = x !== null; if (ok) { if (x === null) /*contradicted*/x; } }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("before", "x", &[String]),
            ("beforeElse", "x", &[Number]),
            ("inside", "x", &[String]),
            ("insideElse", "x", &[Number]),
            ("impossible", "x", &[Unknown]),
            ("contradicted", "x", &[Unknown]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below `string | null`,
// except where a comment says otherwise. TypeScript inlines only an
// unannotated `const` that binds an identifier to an initializer, and only
// when the `const` is tested for truthiness.
#[test]
fn other_declarations_are_not_aliases() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        function annotated(x: string | null) { const ok: boolean = x !== null; if (ok) /*annotated*/x; }
        function mutable(x: string | null) { let ok = x !== null; if (ok) /*let*/x; }
        function destructured(x: string | null) { const { ok } = { ok: x !== null }; if (ok) /*destructured*/x; }
        function asserted(x: string | null) { const ok = (x !== null) as boolean; if (ok) /*as*/x; }
        function coalesced(x: string | null) { const ok = x !== null; ok ?? /*coalesced*/x; }
        function conditional(x: string | null) { const ok = x !== null; if (ok ? true : false) /*conditional*/x; }
        function comparison(x: string | null) { const ok = x !== null; if (ok === true) /*comparison*/x; }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("annotated", "x", &[String, Null]),
            ("let", "x", &[String, Null]),
            ("destructured", "x", &[String, Null]),
            ("as", "x", &[String, Null]),
            ("coalesced", "x", &[String, Null]),
            ("conditional", "x", &[String, Null]),
            // TypeScript gives `string` because it narrows through a comparison
            // with `true`. Biome tests only the alias itself.
            ("comparison", "x", &[String, Null]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below `string | null`,
// except where a comment says otherwise. An inlined condition narrows only a
// variable that cannot change between the alias and its test.
#[test]
fn inlined_conditions_narrow_only_variables_that_cannot_change() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        declare function get(): string | null;
        export let exportedLet = get();
        export const exportedConst = get();
        let moduleLet = get();
        const viaSpecifier = get();
        export { viaSpecifier };
        { const ok = exportedLet !== null; if (ok) /*exportedLet*/exportedLet; }
        { const ok = exportedConst !== null; if (ok) /*exportedConst*/exportedConst; }
        { const ok = moduleLet !== null; if (ok) /*moduleLet*/moduleLet; }
        { const ok = viaSpecifier !== null; if (ok) /*viaSpecifier*/viaSpecifier; }
        function writtenBetween(x: string | null) { let y = x; const ok = y !== null; y = get(); if (ok) /*writtenBetween*/y; }
        function writtenAfter(x: string | null) { const ok = x !== null; if (ok) /*writtenAfter*/x; x = get(); }
        function mutableProperty(o: { v: string | null }) { const ok = o.v !== null; if (ok) /*mutable*/o.v; }
        function readonlyProperty(o: { readonly v: string | null }) { const ok = o.v !== null; if (ok) /*readonly*/o.v; }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("exportedLet", "exportedLet", &[String, Null]),
            ("exportedConst", "exportedConst", &[String, Null]),
            // TypeScript gives `string` in a module, but `string | null` for a
            // top-level `let` of a script, which other scripts can write.
            // Biome cannot tell a script from a module.
            ("moduleLet", "moduleLet", &[String, Null]),
            // TypeScript gives `string` because, for a `const`, only an
            // `export` keyword on the declaration stops it from inlining.
            ("viaSpecifier", "viaSpecifier", &[String, Null]),
            ("writtenBetween", "y", &[String, Null]),
            ("writtenAfter", "x", &[String, Null]),
            ("mutable", "o.v", &[String, Null]),
            // A readonly property cannot change either, so TypeScript gives
            // `string`.
            ("readonly", "o.v", &[String]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below `string`, in a
// script and in a module, because none of these variables is a top-level
// `let` of a script.
#[test]
fn inlined_conditions_narrow_module_constants_and_nested_lets() {
    use InferredTypeData::String;
    const SOURCE: &str = r#"
        declare function get(): string | null;
        const moduleConst = get();
        { const ok = moduleConst !== null; if (ok) /*moduleConst*/moduleConst; }
        { let l = get(); const ok = l !== null; if (ok) /*block*/l; }
        for (let i = get(); ; ) { const ok = i !== null; if (ok) /*for*/i; break; }
        function local() { let l = get(); const ok = l !== null; if (ok) /*local*/l; }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("moduleConst", "moduleConst", &[String]),
            ("block", "l", &[String]),
            ("for", "i", &[String]),
            ("local", "l", &[String]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below `string`. Biome
// keeps `string | null` where the read is in a closure and its variable is
// declared outside it, because no fact enters a nested function.
#[test]
fn aliases_inside_and_around_nested_functions() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        declare function get(): string | null;
        const outer = get();
        const outerOk = outer !== null;
        function moduleAlias() { if (outerOk) /*moduleAlias*/outer; }
        function testedInClosure(x: string | null) { const ok = x !== null; const g = () => { if (ok) /*testedInClosure*/x; }; }
        function readInClosure(x: string | null) { const ok = x !== null; if (ok) { const g = () => /*readInClosure*/x; } }
        function nested(x: string | null) { const g = (y: string | null) => { const ok = y !== null; if (ok) /*nested*/y; }; }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("moduleAlias", "outer", &[String, Null]),
            ("testedInClosure", "x", &[String, Null]),
            ("readInClosure", "x", &[String, Null]),
            ("nested", "y", &[String]),
        ],
    );
}

/// Names each member of `ty`, sorted: an interface by its name, and any other
/// type by its formatted text on one line.
fn member_names(db: &TestModuleDb, ty: InferredTypeData) -> Vec<String> {
    let members = match ty {
        InferredTypeData::Union(union) => union.types(db).to_vec(),
        _ => vec![ty],
    };
    let mut names: Vec<_> = members
        .into_iter()
        .map(|ty| {
            let text = format_inferred_type(db, ty);
            match text
                .lines()
                .next()
                .and_then(|line| line.strip_prefix("interface \""))
                .and_then(|name| name.split('"').next())
            {
                Some(name) => name.to_owned(),
                None => text.lines().map(str::trim).collect::<Vec<_>>().join(" "),
            }
        })
        .collect();
    names.sort();
    names
}

/// Asserts the type of each `(marker, expression, expected)` read in `source`
/// by the sorted names of its members. See [`member_names`] for how a member
/// is named.
fn assert_named_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    let (db, module) = narrowing_db(source);
    let mismatches: Vec<_> = reads
        .iter()
        .filter_map(|(marker, expression, expected)| {
            let ty = normalized_type_at(&db, module, source, marker, expression);
            let actual = member_names(&db, ty);
            (actual != *expected)
                .then(|| format!("{marker}: expected {expected:?}, got {actual:?}"))
        })
        .collect();
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// because an inlined condition narrows a property path when every property
// along it is readonly. TypeScript also narrows an element of a readonly
// tuple, which Biome types as Unknown even without a condition.
#[test]
fn readonly_property_paths_narrow_through_aliased_conditions() {
    use InferredTypeData::{Null, Number, String};
    const SOURCE: &str = r#"
        declare function isString(value: unknown): value is string;
        interface RI { readonly v: string | null }
        class R { readonly v: string | null = null; m() { const ok = this.v !== null; if (ok) /*this*/this.v; } }
        class P { readonly #v: string | null = null; m() { const ok = this.#v !== null; if (ok) /*private*/this.#v; } }
        class A { readonly v: string | null = null; m() { const g = () => { const ok = this.v !== null; if (ok) /*arrow*/this.v; }; } }
        function property(o: { readonly v: string | null }) { const ok = o.v !== null; if (ok) /*property*/o.v; }
        function asConst() { const o = { v: "a" as string | null } as const; const ok = o.v !== null; if (ok) /*asConst*/o.v; }
        function nested(o: { readonly a: { readonly v: string | null } }) { const ok = o.a.v !== null; if (ok) /*nested*/o.a.v; }
        function mapped(o: Readonly<{ v: string | null }>) { const ok = o.v !== null; if (ok) /*mapped*/o.v; }
        function element(o: { readonly v: string | null }) { const ok = o["v"] !== null; if (ok) /*element*/o.v; }
        function intersection(o: { readonly v: string | null } & { w: 1 }) { const ok = o.v !== null; if (ok) /*intersection*/o.v; }
        function typeOf(o: { readonly v: string | number }) { const ok = typeof o.v === "string"; if (ok) /*typeof*/o.v; else /*typeofElse*/o.v; }
        function iface(o: RI) { const ok = o.v !== null; if (ok) /*interface*/o.v; }
        function negated(o: { readonly v: string | null }) { const ok = o.v !== null; if (!ok) /*negated*/o.v; }
        function and(o: { readonly v: string | null; readonly w: string | null }) { const ok = o.v !== null && o.w !== null; if (ok) { /*andV*/o.v; /*andW*/o.w; } }
        function or(o: { readonly v: string | null }, q: boolean) { const ok = o.v !== null || q; if (ok) /*or*/o.v; else /*orElse*/o.v; }
        function chained(o: { readonly v: string | null }) { const a = o.v !== null; const b = a; if (b) /*chained*/o.v; }
        function optional(o: { readonly v?: string }) { const ok = o.v !== undefined; if (ok) /*optional*/o.v; }
        function nullable(o: { readonly v: string | null } | null) { const ok = o?.v !== null && o !== null; if (ok) /*nullable*/o.v; }
        function predicate(o: { readonly v: string | number }) { const ok = isString(o.v); if (ok) /*predicate*/o.v; }
        function truthy(o: { readonly v: string | null }) { const ok = !!o.v; if (ok) /*truthy*/o.v; }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("this", "this.v", &[String]),
            ("private", "this.#v", &[String]),
            ("arrow", "this.v", &[String]),
            ("property", "o.v", &[String]),
            ("asConst", "o.v", &[String]),
            ("nested", "o.a.v", &[String]),
            ("mapped", "o.v", &[String]),
            ("element", "o.v", &[String]),
            ("intersection", "o.v", &[String]),
            ("typeof", "o.v", &[String]),
            ("typeofElse", "o.v", &[Number]),
            ("interface", "o.v", &[String]),
            ("negated", "o.v", &[Null]),
            ("andV", "o.v", &[String]),
            ("andW", "o.w", &[String]),
            ("or", "o.v", &[String, Null]),
            ("orElse", "o.v", &[Null]),
            ("chained", "o.v", &[String]),
            ("optional", "o.v", &[String]),
            ("nullable", "o.v", &[String]),
            ("predicate", "o.v", &[String]),
            ("truthy", "o.v", &[String]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A readonly path narrows through an aliased condition by `instanceof`, by a
// test of a discriminant, by `in`, and as the receiver of an optional chain.
#[test]
fn readonly_objects_narrow_through_aliased_conditions() {
    const SOURCE: &str = r#"
        interface X { kind: "x"; y: 1 }
        interface Z { kind: "z"; w: 2 }
        interface V { readonly v: string | null }
        function instance(o: { readonly v: Date | string }) { const ok = o.v instanceof Date; if (ok) /*instanceof*/o.v; }
        function discriminant(o: { readonly a: X | Z }) { const ok = o.a.kind === "x"; if (ok) /*discriminant*/o.a; }
        function held(o: { readonly a: X | Z }) { const k = o.a.kind; const ok = k === "x"; if (ok) /*held*/o.a; }
        function inOperator(o: { readonly a: X | Z }) { const ok = "y" in o.a; if (ok) /*in*/o.a; }
        function receiver(o: { readonly a?: V }) { const ok = o.a?.v === "x"; if (ok) /*receiver*/o.a; }
        function mutableDiscriminant(o: { a: X | Z }) { const ok = o.a.kind === "x"; if (ok) /*mutableDiscriminant*/o.a; }
        function mutableReceiver(o: { a?: V }) { const ok = o.a?.v === "x"; if (ok) /*mutableReceiver*/o.a; }
    "#;
    assert_named_reads(
        SOURCE,
        &[
            ("instanceof", "o.v", &["instanceof Date"]),
            ("discriminant", "o.a", &["X"]),
            ("held", "o.a", &["X"]),
            ("in", "o.a", &["X"]),
            ("receiver", "o.a", &["V"]),
            ("mutableDiscriminant", "o.a", &["X", "Z"]),
            ("mutableReceiver", "o.a", &["V", "undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below `string | null`.
// In each case, the path, or an object on it, can change between the alias
// and its test, or the read is outside the branch. A property of an
// intersection is readonly only when every operand that declares it declares
// it readonly.
#[test]
fn mutable_property_paths_keep_their_declared_types() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        class GS { get v(): string | null { return null; } set v(x) {} }
        class K { #v: string | null = null; m() { const ok = this.#v !== null; if (ok) /*private*/this.#v; } }
        function mutable(o: { v: string | null }) { const ok = o.v !== null; if (ok) /*mutable*/o.v; }
        function getterSetter(g: GS) { const ok = g.v !== null; if (ok) /*getterSetter*/g.v; }
        function mutableObject(o: { a: { readonly v: string | null } }) { const ok = o.a.v !== null; if (ok) /*mutableObject*/o.a.v; }
        function letAlias(o: { readonly v: string | null }) { let ok = o.v !== null; if (ok) /*let*/o.v; }
        function indexSignature(o: { readonly [k: string]: string | null }) { const ok = o.a !== null; if (ok) /*index*/o.a; }
        function after(o: { readonly v: string | null }) { const ok = o.v !== null; if (ok) {} /*after*/o.v; }
        function mixedIntersection(o: { readonly v: string | null } & { v: string | null }) { const ok = o.v !== null; if (ok) /*intersection*/o.v; }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("private", "this.#v", &[String, Null]),
            ("mutable", "o.v", &[String, Null]),
            ("getterSetter", "g.v", &[String, Null]),
            ("mutableObject", "o.a.v", &[String, Null]),
            ("let", "o.v", &[String, Null]),
            ("index", "o.a", &[String, Null]),
            ("after", "o.v", &[String, Null]),
            ("intersection", "o.v", &[String, Null]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below `string`. Biome
// keeps the declared type where it can't tell that the path is readonly, for
// the reasons that the comments give.
#[test]
fn paths_that_biome_cannot_prove_readonly_keep_their_declared_types() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        class G { get v(): string | null { return null; } }
        class Base { readonly v: string | null = null }
        class Sub extends Base {}
        class Derived extends Base { v: string | null = null }
        type DA = { kind: "a"; readonly v: string | null };
        type DB = { kind: "b"; v: string | null };
        const C = class { static readonly v: string | null = null };
        function getter(g: G) { const ok = g.v !== null; if (ok) /*getter*/g.v; }
        function oneReadonlyMember(o: { readonly v: string | null } | { v: string | null }) { const ok = o.v !== null; if (ok) /*oneReadonly*/o.v; }
        function inherited(o: Sub) { const ok = o.v !== null; if (ok) /*inherited*/o.v; }
        function generic<T extends { readonly v: string | null }>(o: T) { const ok = o.v !== null; if (ok) /*generic*/o.v; }
        function asConstArray() { const t = [("a" as string | null)] as const; const ok = t[0] !== null; if (ok) /*asConstArray*/t[0]; }
        function staticField() { const ok = C.v !== null; if (ok) /*static*/C.v; }
        function discriminated(o: DA | DB) { if (o.kind === "a") { const ok = o.v !== null; if (ok) /*discriminated*/o.v; } }
        function unionStep(o: { readonly a: { readonly v: string | null } } | { readonly a: { v: string | null } }) { const ok = o.a.v !== null; if (ok) /*unionStep*/o.a.v; }
        class GT { get v(): string | null { return null; } m() { const ok = this.v !== null; if (ok) /*thisGetter*/this.v; } }
        class Outer { readonly v: string | null = null; m() { const ok = this.v !== null; const g = () => { if (ok) /*outerAlias*/this.v; }; } }
        function testedElsewhere(o: Base) { const ok = o.v !== null; if (ok) /*testedElsewhere*/o.v; if (o instanceof Derived) {} }
    "#;
    assert_reads(
        SOURCE,
        &[
            // A setter may change a getter's value, and Biome doesn't
            // collect setters.
            ("getter", "g.v", &[String, Null]),
            ("thisGetter", "this.v", &[String, Null]),
            // TypeScript needs the property to be readonly in only one
            // member, but a write through another member could change it.
            ("oneReadonly", "o.v", &[String, Null]),
            ("unionStep", "o.a.v", &[String, Null]),
            // Biome can't look up a property of a generic.
            ("generic", "o.v", &[String, Null]),
            // An inherited property can be hidden by a declaration that
            // Biome doesn't see.
            ("inherited", "o.v", &[String, Null]),
            // Biome types an array under `as const` as a tuple whose
            // elements are not readonly, and doesn't record `static
            // readonly`.
            ("asConstArray", "t[0]", &[String, Null]),
            ("static", "C.v", &[String, Null]),
            // Biome decides readonly from the declared type of `o`, where
            // TypeScript uses its type at the read.
            ("discriminated", "o.v", &[String, Null]),
            // An aliased condition narrows `this` only in the function that
            // declares it.
            ("outerAlias", "this.v", &[String, Null]),
            // A test that can replace the type of `o` anywhere in the
            // function keeps every read of its paths from narrowing.
            ("testedElsewhere", "o.v", &[String, Null]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below `string | null`,
// except `outerThis`, where it gives `string`. In the other cases, the type of
// the object at the read, or a declaration that Biome doesn't see, has a
// property that isn't readonly, although Biome's declared type has a readonly
// one. Biome also keeps `string | null` at `outerThis`, because an aliased
// condition that an enclosing function declares reads that function's `this`.
#[test]
fn readonly_paths_never_narrow_where_typescript_sees_a_mutable_property() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        class Base { readonly v: string | null = null }
        class Derived extends Base { v: string | null = null }
        declare function isMutable(x: unknown): x is { v: string | null };
        interface RB { readonly v: string | null }
        interface Merged { v: string | null }
        interface Merged extends RB {}
        function replacedByInstanceof(o: Base) { const ok = o.v !== null; if (o instanceof Derived) { if (ok) /*instanceof*/o.v; } }
        function replacedByPredicate(o: { readonly v: string | null }) { const ok = o.v !== null; if (isMutable(o)) { if (ok) /*predicate*/o.v; } }
        function merged(o: Merged) { const ok = o.v !== null; if (ok) /*merged*/o.v; }
        function outerThis(this: { readonly v: string | null }) { const ok = this.v !== null; class K { readonly v: string | null = null; m() { if (ok) /*outerThis*/this.v; } } }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("instanceof", "o.v", &[String, Null]),
            ("predicate", "o.v", &[String, Null]),
            ("merged", "o.v", &[String, Null]),
            ("outerThis", "this.v", &[String, Null]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string | null` at the read, because
// the augmentation declares `v` on `X` without `readonly`. That declaration
// hides the readonly `v` that `X` inherits.
#[test]
fn readonly_paths_never_narrow_through_properties_that_an_augmentation_can_hide() {
    use InferredTypeData::{Null, String};
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/shapes.ts".into(),
        "export interface RB { readonly v: string | null }\nexport interface X extends RB {}\n",
    );
    fs.insert(
        "/src/augment.ts".into(),
        "import \"./shapes\";\ndeclare module \"./shapes\" { interface X { v: string | null } }\n",
    );
    const SOURCE: &str = r#"
        import "./augment";
        import type { X } from "./shapes";
        function f(o: X) { const ok = o.v !== null; if (ok) /*read*/o.v; }
    "#;
    fs.insert("/src/index.ts".into(), SOURCE);
    let db = build_js_test_module_db(
        &fs,
        &["/src/shapes.ts", "/src/augment.ts", "/src/index.ts"],
        true,
    );
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let ty = normalized_type_at(&db, module, SOURCE, "read", "o.v");
    let actual = match ty {
        InferredTypeData::Union(union) => union.types(&db).to_vec(),
        _ => vec![ty],
    };
    assert!(
        actual.len() == 2 && actual.contains(&String) && actual.contains(&Null),
        "{}",
        format_inferred_type(&db, ty)
    );
}

// TypeScript 7.0.2 (`tsc --strict`) reports TS2540 for the write and gives
// `never` at the read, because the write assigns `null` after the alias.
// Biome keeps the declared type, because a path that the function writes never
// narrows.
#[test]
fn written_readonly_paths_keep_their_declared_types() {
    use InferredTypeData::{Null, String};
    assert_reads(
        r#"
        function written(o: { readonly v: string | null }) { const ok = o.v !== null; o.v = null; if (ok) /*written*/o.v; }
        "#,
        &[("written", "o.v", &[String, Null])],
    );
}
