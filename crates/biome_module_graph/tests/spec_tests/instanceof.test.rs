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
        TypeInferenceCaller::new("test", "instanceof"),
        NormalizedExpressionTypeRequest::new(module, range),
    )
    .unwrap_or_else(|| panic!("expression at {marker} {range:?} must be inferred"))
}

/// Names each member of `ty`, sorted: an interface by its name, an object
/// type literal by its members, and any other type by its formatted text on
/// one line.
fn variant_names(db: &TestModuleDb, ty: InferredTypeData) -> Vec<String> {
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

/// Classes and interfaces shared by the tests.
const CLASSES: &str = r#"
    class A { a = 1 }
    class B extends A { b = 2 }
    class C { c = 3 }
    class D extends B { d = 4 }
    class E extends Error { e = 1 }
    class Box<T> { v!: T }
    interface Foo { a: number }
    interface IA extends A { z: 1 }
    interface IB extends IA { y: 1 }
    declare const Ctor: new () => Foo;
    declare const Ctors: { new (): Foo; new (n: number): Foo };
"#;

/// Asserts the type of each `(marker, expression, expected)` read in `source`,
/// which follows [`CLASSES`]. `expected` lists the member names that
/// [`variant_names`] returns.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    let source = format!("{CLASSES}{source}");
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source.as_str());
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .filter_map(|(marker, expression, expected)| {
            let ty = normalized_type_at(&db, module, &source, marker, expression);
            let actual = variant_names(&db, ty);
            (actual != *expected)
                .then(|| format!("{marker}: expected {expected:?}, got {actual:?}"))
        })
        .collect();
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type.
// An `else` branch that no value reaches is `never` in TypeScript, which Biome
// reads as Unknown. `x instanceof A` with a primitive member is an error in
// TypeScript (TS2358) only when no member is an object type.
#[test]
fn instanceof_narrows_by_declarations() {
    assert_reads(
        r#"
        function builtin(x: Date | string) { if (x instanceof Date) /*builtin*/x; else /*builtinElse*/x; }
        function derived(x: B | C) { if (x instanceof A) /*derived*/x; else /*derivedElse*/x; }
        function parent(x: A | string) { if (x instanceof B) /*parent*/x; else /*parentElse*/x; }
        function same(x: B | C) { if (x instanceof B) /*same*/x; else /*sameElse*/x; }
        function grandparent(x: A | C) { if (x instanceof D) /*grandparent*/x; else /*grandparentElse*/x; }
        function nullish(x: D | C | null) { if (x instanceof A) /*nullish*/x; else /*nullishElse*/x; }
        function undefinedMember(x: B | C | undefined) { if (x instanceof A) /*undefined*/x; else /*undefinedElse*/x; }
        function single(x: A) { if (x instanceof B) /*single*/x; else /*singleElse*/x; }
        function impossible(x: Date) { if (x instanceof Date) /*impossible*/x; else /*impossibleElse*/x; }
        function structural(x: B | { c: number }) { if (x instanceof A) /*structural*/x; else /*structuralElse*/x; }
        function literal(x: { a: number } | string) { if (x instanceof A) /*literal*/x; else /*literalElse*/x; }
        function interfaceMember(x: Foo | string) { if (x instanceof A) /*interfaceMember*/x; else /*interfaceMemberElse*/x; }
        function unrelated(x: Foo | C) { if (x instanceof A) /*unrelated*/x; else /*unrelatedElse*/x; }
        function objectKeyword(x: object) { if (x instanceof A) /*object*/x; else /*objectElse*/x; }
        function objectOrString(x: object | string) { if (x instanceof A) /*objectOrString*/x; else /*objectOrStringElse*/x; }
        function emptyObject(x: {} | undefined) { if (x instanceof A) /*empty*/x; else /*emptyElse*/x; }
        function unknownValue(x: unknown) { if (x instanceof A) /*unknown*/x; else /*unknownElse*/x; }
        function unknownBuiltin(x: unknown) { if (x instanceof Date) /*unknownBuiltin*/x; }
        function generic(x: Box<string> | number) { if (x instanceof Box) /*generic*/x; else /*genericElse*/x; }
        function generics(x: Box<string> | Box<number>) { if (x instanceof Box) /*generics*/x; else /*genericsElse*/x; }
        function map(x: Map<string, number> | string) { if (x instanceof Map) /*map*/x; else /*mapElse*/x; }
        function mapOrSet(x: Map<string, number> | Set<string>) { if (x instanceof Map) /*mapOrSet*/x; else /*mapOrSetElse*/x; }
        function promise(x: Promise<number> | number) { if (x instanceof Promise) /*promise*/x; else /*promiseElse*/x; }
        function regExp(x: RegExp | string) { if (x instanceof RegExp) /*regExp*/x; else /*regExpElse*/x; }
        function error(x: Error | { message: string }) { if (x instanceof Error) /*error*/x; else /*errorElse*/x; }
        function typeError(x: TypeError | string) { if (x instanceof Error) /*typeError*/x; else /*typeErrorElse*/x; }
        function parentError(x: Error) { if (x instanceof TypeError) /*parentError*/x; else /*parentErrorElse*/x; }
        function subError(x: E | string) { if (x instanceof Error) /*subError*/x; else /*subErrorElse*/x; }
        function userError(x: Error | string) { if (x instanceof E) /*userError*/x; else /*userErrorElse*/x; }
        function array(x: string[] | string) { if (x instanceof Array) /*array*/x; else /*arrayElse*/x; }
        function arrayOrMap(x: number[] | Map<string, number>) { if (x instanceof Array) /*arrayOrMap*/x; else /*arrayOrMapElse*/x; }
        function tuple(x: [1, 2] | string) { if (x instanceof Array) /*tuple*/x; else /*tupleElse*/x; }
        function readonlyArray(x: readonly string[] | string) { if (x instanceof Array) /*readonlyArray*/x; else /*readonlyArrayElse*/x; }
        function wrapper(x: string | String) { if (x instanceof String) /*wrapper*/x; else /*wrapperElse*/x; }
        function functionMember(x: (() => void) | Date) { if (x instanceof Date) /*function*/x; else /*functionElse*/x; }
        function interfaceOfClass(x: IA | C) { if (x instanceof A) /*interfaceOfClass*/x; else /*interfaceOfClassElse*/x; }
        function interfaceChain(x: IB | C) { if (x instanceof A) /*interfaceChain*/x; else /*interfaceChainElse*/x; }
        function intersection(x: (A & { q: 1 }) | C) { if (x instanceof A) /*intersection*/x; else /*intersectionElse*/x; }
        function constructSignature(x: Foo | string) { if (x instanceof Ctor) /*construct*/x; else /*constructElse*/x; }
        function constructSignatures(x: Foo | C) { if (x instanceof Ctors) /*constructs*/x; else /*constructsElse*/x; }
        function constructors(x: A | C, k: typeof A | typeof C) { if (x instanceof k) /*constructors*/x; else /*constructorsElse*/x; }
        function functionConstructor(x: B | C, F: Function) { if (x instanceof F) /*functionConstructor*/x; else /*functionConstructorElse*/x; }
        "#,
        &[
            ("builtin", "x", &["instanceof Date"]),
            ("builtinElse", "x", &["string"]),
            ("derived", "x", &["instanceof B"]),
            ("derivedElse", "x", &["instanceof C"]),
            ("parent", "x", &["instanceof B"]),
            ("parentElse", "x", &["instanceof A", "string"]),
            ("same", "x", &["instanceof B"]),
            ("sameElse", "x", &["instanceof C"]),
            ("grandparent", "x", &["instanceof D"]),
            ("grandparentElse", "x", &["instanceof A", "instanceof C"]),
            ("nullish", "x", &["instanceof D"]),
            ("nullishElse", "x", &["instanceof C", "null"]),
            ("undefined", "x", &["instanceof B"]),
            ("undefinedElse", "x", &["instanceof C", "undefined"]),
            ("single", "x", &["instanceof B"]),
            ("singleElse", "x", &["instanceof A"]),
            ("impossible", "x", &["instanceof Date"]),
            ("impossibleElse", "x", &["unknown"]),
            // Biome matches TypeScript, which is unsound here because an
            // instance of `A` can also have the removed type `{ c: number }`.
            ("structural", "x", &["instanceof B"]),
            ("structuralElse", "x", &[r#"["c": number]"#]),
            // No member is related to `A`, so the result is `A`, a subtype of
            // `{ a: number }`.
            ("literal", "x", &["instanceof A"]),
            ("literalElse", "x", &[r#"["a": number]"#, "string"]),
            ("interfaceMember", "x", &["instanceof A"]),
            ("interfaceMemberElse", "x", &["Foo", "string"]),
            ("unrelated", "x", &["instanceof A"]),
            ("unrelatedElse", "x", &["Foo", "instanceof C"]),
            ("object", "x", &["instanceof A"]),
            ("objectElse", "x", &["object"]),
            ("objectOrString", "x", &["instanceof A"]),
            ("objectOrStringElse", "x", &["object", "string"]),
            ("empty", "x", &["instanceof A"]),
            ("emptyElse", "x", &["[]", "undefined"]),
            ("unknown", "x", &["instanceof A"]),
            ("unknownElse", "x", &["unknown"]),
            ("unknownBuiltin", "x", &["instanceof Date"]),
            ("generic", "x", &["instanceof Box<string>"]),
            ("genericElse", "x", &["number"]),
            (
                "generics",
                "x",
                &["instanceof Box<number>", "instanceof Box<string>"],
            ),
            ("genericsElse", "x", &["unknown"]),
            ("map", "x", &["instanceof Map<string, number>"]),
            ("mapElse", "x", &["string"]),
            ("mapOrSet", "x", &["instanceof Map<string, number>"]),
            ("mapOrSetElse", "x", &["instanceof Set<string>"]),
            ("promise", "x", &["instanceof Promise<number>"]),
            ("promiseElse", "x", &["number"]),
            ("regExp", "x", &["instanceof RegExp"]),
            ("regExpElse", "x", &["string"]),
            ("error", "x", &["instanceof Error"]),
            ("errorElse", "x", &[r#"["message": string]"#]),
            ("typeError", "x", &["instanceof TypeError"]),
            ("typeErrorElse", "x", &["string"]),
            ("parentError", "x", &["instanceof TypeError"]),
            ("parentErrorElse", "x", &["instanceof Error"]),
            ("subError", "x", &["instanceof E"]),
            ("subErrorElse", "x", &["string"]),
            ("userError", "x", &["instanceof E"]),
            ("userErrorElse", "x", &["instanceof Error", "string"]),
            ("array", "x", &["instanceof Array<string>"]),
            ("arrayElse", "x", &["string"]),
            ("arrayOrMap", "x", &["instanceof Array<number>"]),
            ("arrayOrMapElse", "x", &["instanceof Map<string, number>"]),
            ("tuple", "x", &["[number: 1, number: 2]"]),
            ("tupleElse", "x", &["string"]),
            ("readonlyArray", "x", &["Readonly instanceof Array<string>"]),
            ("readonlyArrayElse", "x", &["string"]),
            ("wrapper", "x", &["instanceof String"]),
            ("wrapperElse", "x", &["string"]),
            ("function", "x", &["instanceof Date"]),
            ("functionElse", "x", &[VOID_FUNCTION]),
            ("interfaceOfClass", "x", &["IA"]),
            ("interfaceOfClassElse", "x", &["instanceof C"]),
            ("interfaceChain", "x", &["IB"]),
            ("interfaceChainElse", "x", &["instanceof C"]),
            // Biome prints `A & { q: 1 }` as an anonymous class.
            (
                "intersection",
                "x",
                &["instanceof class { extends: none implements: [] type_args: [] }"],
            ),
            ("intersectionElse", "x", &["instanceof C"]),
            ("construct", "x", &["Foo"]),
            ("constructElse", "x", &["string"]),
            ("constructs", "x", &["Foo"]),
            ("constructsElse", "x", &["instanceof C"]),
            ("constructors", "x", &["instanceof A", "instanceof C"]),
            ("constructorsElse", "x", &["instanceof A", "instanceof C"]),
            (
                "functionConstructor",
                "x",
                &["instanceof B", "instanceof C"],
            ),
            (
                "functionConstructorElse",
                "x",
                &["instanceof B", "instanceof C"],
            ),
        ],
    );
}

/// How [`variant_names`] names `() => void`.
const VOID_FUNCTION: &str = "sync Function { accepts: { params: [] type_args: [] } returns: void }";

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type.
#[test]
fn instanceof_narrows_through_the_flow() {
    assert_reads(
        r#"
        function aliased(x: B | C) { const ok = x instanceof B; if (ok) /*aliased*/x; else /*aliasedElse*/x; }
        function operand(x: B | C) { x instanceof B && /*operand*/x; }
        function conditional(x: B | C) { x instanceof B ? /*conditional*/x : /*conditionalElse*/x; }
        function loop(x: B | C) { while (x instanceof B) { /*loop*/x; } /*loopExit*/x; }
        function returned(x: B | C) { if (x instanceof B) { return; } /*returned*/x; }
        function thrown(x: unknown) { if (!(x instanceof B)) throw 0; /*thrown*/x; }
        function parenthesized(x: B | C) { if ((x) instanceof (B)) /*parenthesized*/x; }
        function either(x: A | C) { if (x instanceof A || x instanceof C) /*either*/x; else /*eitherElse*/x; }
        function both(x: A | B | C) { if (!(x instanceof B) && x instanceof A) /*both*/x; }
        function two(x: B | C, y: B | C) { if (x instanceof B && y instanceof C) { /*twoX*/x; /*twoY*/y; } }
        function truthiness(x: B | C | null) { if (x instanceof B) /*truthiness*/x; if (!x) return; /*truthinessAfter*/x; }
        function element(xs: (B | C)[]) { const x = xs[0]; if (x instanceof B) /*element*/x; }
        function path(o: { v: Date | string }) { if (o.v instanceof Date) /*path*/o.v; else /*pathElse*/o.v; }
        class K { x: B | C = new C(); m() { if (this.x instanceof B) /*this*/this.x; else /*thisElse*/this.x; } }
        function chain(x: { y: unknown } | undefined) { if (x?.y instanceof Date) /*chain*/x; else /*chainElse*/x; }
        function chainPath(o: { v: B | C } | undefined) { if (o?.v instanceof B) { /*chainPath*/o; /*chainPathV*/o.v; } else /*chainPathElse*/o; }
        function chainCall(o: { f(): B | C } | undefined) { if (o?.f() instanceof B) /*chainCall*/o; }
        type S = { k: B; v: number } | { k: C; v: string };
        function discriminant(s: S) { if (s.k instanceof B) /*discriminant*/s; }
        function pattern({ k, v }: S) { if (k instanceof B) { /*patternK*/k; /*patternV*/v; } }
        "#,
        &[
            ("aliased", "x", &["instanceof B"]),
            ("aliasedElse", "x", &["instanceof C"]),
            ("operand", "x", &["instanceof B"]),
            ("conditional", "x", &["instanceof B"]),
            ("conditionalElse", "x", &["instanceof C"]),
            ("loop", "x", &["instanceof B"]),
            ("loopExit", "x", &["instanceof C"]),
            ("returned", "x", &["instanceof C"]),
            ("thrown", "x", &["instanceof B"]),
            ("parenthesized", "x", &["instanceof B"]),
            ("either", "x", &["instanceof A", "instanceof C"]),
            ("eitherElse", "x", &["unknown"]),
            ("both", "x", &["instanceof A"]),
            ("twoX", "x", &["instanceof B"]),
            ("twoY", "y", &["instanceof C"]),
            ("truthiness", "x", &["instanceof B"]),
            ("truthinessAfter", "x", &["instanceof B", "instanceof C"]),
            ("element", "x", &["instanceof B"]),
            ("path", "o.v", &["instanceof Date"]),
            ("pathElse", "o.v", &["string"]),
            ("this", "this.x", &["instanceof B"]),
            ("thisElse", "this.x", &["instanceof C"]),
            // An instance is never nullish, so the receiver narrows in the
            // true branch.
            ("chain", "x", &[r#"["y": unknown]"#]),
            ("chainElse", "x", &[r#"["y": unknown]"#, "undefined"]),
            ("chainPath", "o", &[r#"["v": instanceof B | instanceof C]"#]),
            ("chainPathV", "o.v", &["instanceof B"]),
            (
                "chainPathElse",
                "o",
                &[r#"["v": instanceof B | instanceof C]"#, "undefined"],
            ),
            (
                "chainCall",
                "o",
                &[
                    r#"[ "f": sync Function "f" { accepts: { params: [] type_args: [] } returns: instanceof B | instanceof C } ]"#,
                ],
            ),
            // `instanceof` narrows no object that holds the tested value, and
            // no other element of a destructuring pattern.
            (
                "discriminant",
                "s",
                &[
                    r#"["k": instanceof B, "v": number]"#,
                    r#"["k": instanceof C, "v": string]"#,
                ],
            ),
            ("patternK", "k", &["instanceof B"]),
            ("patternV", "v", &["number", "string"]),
        ],
    );
}

// Biome keeps the declared type of every read below. A comment gives the type
// that TypeScript 6.0.3 (`tsc --strict`) gives where it differs.
#[test]
fn instanceof_keeps_the_declared_type_where_biome_cannot_tell() {
    assert_reads(
        r#"
        declare const GenCtor: new <T>() => Box<T>;
        declare const AnyCtor: any;
        class Checker { static [Symbol.hasInstance](v: unknown): v is string { return typeof v === "string"; } }
        class SubChecker extends Checker {}
        class BoolChecker { static [Symbol.hasInstance](v: unknown): boolean { return true; } }
        function genericClass(x: unknown) { if (x instanceof Map) /*genericClass*/x; }
        function genericBox(x: unknown) { if (x instanceof Box) /*genericBox*/x; }
        function primitives(x: string | number) { if (x instanceof A) /*primitives*/x; }
        function object(x: A | C) { if (x instanceof Object) /*object*/x; else /*objectElse*/x; }
        function functionClass(x: A | string) { if (x instanceof Function) /*function*/x; else /*functionElse*/x; }
        function genericConstruct(x: Box<number> | C) { if (x instanceof GenCtor) /*genericConstruct*/x; else /*genericConstructElse*/x; }
        function anyConstructor(x: B | C) { if (x instanceof AnyCtor) /*any*/x; else /*anyElse*/x; }
        function hasInstance(x: string | B) { if (x instanceof Checker) /*hasInstance*/x; else /*hasInstanceElse*/x; }
        function inherited(x: string | B) { if (x instanceof SubChecker) /*inherited*/x; }
        function booleanHasInstance(x: B | C) { if (x instanceof BoolChecker) /*booleanHasInstance*/x; }
        function generic<T extends A>(x: T | C) { if (x instanceof B) /*typeParameter*/x; else /*typeParameterElse*/x; }
        "#,
        &[
            // `Map<any, any>`
            ("genericClass", "x", &["unknown"]),
            // `Box<any>`
            ("genericBox", "x", &["unknown"]),
            // `(string | number) & A`
            ("primitives", "x", &["number", "string"]),
            ("object", "x", &["instanceof A", "instanceof C"]),
            // `never`
            ("objectElse", "x", &["instanceof A", "instanceof C"]),
            // `(string | A) & Function`
            ("function", "x", &["instanceof A", "string"]),
            ("functionElse", "x", &["instanceof A", "string"]),
            // `Box<number>`
            (
                "genericConstruct",
                "x",
                &["instanceof Box<number>", "instanceof C"],
            ),
            // `C`
            (
                "genericConstructElse",
                "x",
                &["instanceof Box<number>", "instanceof C"],
            ),
            ("any", "x", &["instanceof B", "instanceof C"]),
            ("anyElse", "x", &["instanceof B", "instanceof C"]),
            // `string`, the target of the type predicate of
            // `[Symbol.hasInstance]`
            ("hasInstance", "x", &["instanceof B", "string"]),
            ("hasInstanceElse", "x", &["instanceof B", "string"]),
            // `string`
            ("inherited", "x", &["instanceof B", "string"]),
            ("booleanHasInstance", "x", &["instanceof B", "instanceof C"]),
            // `T & B`
            ("typeParameter", "x", &["instanceof C", "instanceof T"]),
            ("typeParameterElse", "x", &["instanceof C", "instanceof T"]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type.
#[test]
fn instanceof_compares_classes_of_other_modules() {
    let files = [
        ("/src/a.ts", "export class K { k = 1 }\n"),
        ("/src/b.ts", "export class K { k = 1 }\n"),
        ("/src/barrel.ts", "export { K } from \"./a\";\n"),
        (
            "/src/index.ts",
            r#"import { K as KA } from "./a";
import { K as KB } from "./b";
import { K as KC } from "./barrel";
function same(x: KA | string) { if (x instanceof KA) /*same*/x; else /*sameElse*/x; }
function barrel(x: KA | string) { if (x instanceof KC) /*barrel*/x; else /*barrelElse*/x; }
function copy(x: KA | string) { if (x instanceof KB) /*copy*/x; else /*copyElse*/x; }
"#,
        ),
    ];
    let fs = MemoryFileSystem::default();
    for (path, source) in files {
        fs.insert(path.into(), source);
    }
    let paths: Vec<_> = files.iter().map(|(path, _)| *path).collect();
    let db = build_js_test_module_db(&fs, &paths, true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let source = files[3].1;
    for (marker, expected) in [
        ("same", &["instanceof K"][..]),
        ("sameElse", &["string"]),
        ("barrel", &["instanceof K"]),
        ("barrelElse", &["string"]),
        // `K` of `./b`, a subtype of `K` of `./a`. Two classes with the same
        // name and members in two modules can't be told from one class that
        // Biome reaches through two modules, so the read keeps its type.
        ("copy", &["instanceof K", "string"]),
        // `string | K`
        ("copyElse", &["instanceof K", "string"]),
    ] {
        let ty = normalized_type_at(&db, module, source, marker, "x");
        assert_eq!(variant_names(&db, ty), expected, "{marker}");
    }
}
