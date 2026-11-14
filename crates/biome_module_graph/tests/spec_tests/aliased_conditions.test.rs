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

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type:
// testing a `const` tests the condition that initializes it.
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

// TypeScript 6.0.3 (`tsc --strict`) gives `x` the type `{ y: string }`: the
// inlined `x?.y` shows that `x` is not nullish, as `if (x?.y)` does.
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

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type.
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

// TypeScript 6.0.3 (`tsc --strict`) inlines five aliases into one another, not
// six: it narrows `x` at `five`, but tests only the truthiness of `c1` at
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

// TypeScript 6.0.3 (`tsc --strict`) applies the inlined condition to the type
// that the subject has at the test, wherever the alias is declared. A branch
// that the alias rules out is impossible, where TypeScript gives `never` and
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

// TypeScript 6.0.3 (`tsc --strict`) gives every read below `string | null`,
// except where a comment says otherwise. TypeScript inlines only an
// unannotated `const` with an initializer that binds an identifier, and only
// when tested for truthiness.
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
            // TypeScript gives `string`: it narrows through a comparison with
            // `true`. Biome tests only the alias itself.
            ("comparison", "x", &[String, Null]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below `string | null`,
// except where a comment says otherwise: an inlined condition narrows only a
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
            // TypeScript gives `string`: only an `export` keyword on the
            // declaration stops it from inlining for a `const`.
            ("viaSpecifier", "viaSpecifier", &[String, Null]),
            ("writtenBetween", "y", &[String, Null]),
            ("writtenAfter", "x", &[String, Null]),
            ("mutable", "o.v", &[String, Null]),
            // TypeScript gives `string`: it inlines for a readonly property.
            ("readonly", "o.v", &[String, Null]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below `string`, in a
// script and in a module: none of these variables is a top-level `let` of a
// script.
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

// TypeScript 6.0.3 (`tsc --strict`) gives every read below `string`. Biome
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
