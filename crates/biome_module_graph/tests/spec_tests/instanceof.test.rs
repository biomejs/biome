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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A type derived from `ReadonlyArray`, such as `ReadonlyArray<string>` or an
// interface that extends it, counts as derived from `Array`. An `else` branch
// that no value reaches is `never` in TypeScript, which Biome reads as
// Unknown.
#[test]
fn instanceof_counts_readonly_arrays_as_arrays() {
    assert_reads(
        r#"
        class MyArr<T> extends Array<T> {}
        interface RO2 extends ReadonlyArray<number> { extra: 1 }
        declare const RoCtor: new () => RO2;
        function both(x: ReadonlyArray<string> | string[]) { if (x instanceof Array) /*both*/x; else /*bothElse*/x; }
        function primitive(x: ReadonlyArray<string> | string) { if (x instanceof Array) /*primitive*/x; else /*primitiveElse*/x; }
        function subclass(x: ReadonlyArray<string> | MyArr<number>) { if (x instanceof Array) /*subclass*/x; else /*subclassElse*/x; }
        function extended(x: RO2 | string) { if (x instanceof Array) /*extended*/x; else /*extendedElse*/x; }
        function instance(x: string[] | string) { if (x instanceof RoCtor) /*instance*/x; else /*instanceElse*/x; }
        "#,
        &[
            (
                "both",
                "x",
                &[
                    "instanceof Array<string>",
                    "instanceof ReadonlyArray<string>",
                ],
            ),
            ("bothElse", "x", &["unknown"]),
            ("primitive", "x", &["instanceof ReadonlyArray<string>"]),
            ("primitiveElse", "x", &["string"]),
            (
                "subclass",
                "x",
                &[
                    "instanceof MyArr<number>",
                    "instanceof ReadonlyArray<string>",
                ],
            ),
            ("subclassElse", "x", &["unknown"]),
            ("extended", "x", &["RO2"]),
            ("extendedElse", "x", &["string"]),
            // `RO2`, the instance type of `RoCtor`, is derived from the
            // member `string[]`, so it replaces that member.
            ("instance", "x", &["RO2"]),
            ("instanceElse", "x", &["instanceof Array<string>", "string"]),
        ],
    );
}

/// How [`variant_names`] names `() => void`.
const VOID_FUNCTION: &str = "sync Function { accepts: { params: [] type_args: [] } returns: void }";

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
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
// that TypeScript 7.0.2 (`tsc --strict`) gives where it differs.
#[test]
fn instanceof_keeps_the_declared_type_where_biome_cannot_tell() {
    assert_reads(
        r#"
        declare const GenCtor: new <T>() => Box<T>;
        declare const AnyCtor: any;
        class Checker { static [Symbol.hasInstance](v: unknown): v is string { return typeof v === "string"; } }
        class SubChecker extends Checker {}
        class BoolChecker { static [Symbol.hasInstance](v: unknown): boolean { return true; } }
        function primitives(x: string | number) { if (x instanceof A) /*primitives*/x; }
        function object(x: A | C) { if (x instanceof Object) /*object*/x; else /*objectElse*/x; }
        function functionClass(x: A | string) { if (x instanceof Function) /*function*/x; else /*functionElse*/x; }
        function genericConstruct(x: Box<number> | C) { if (x instanceof GenCtor) /*genericConstruct*/x; else /*genericConstructElse*/x; }
        function anyConstructor(x: B | C) { if (x instanceof AnyCtor) /*any*/x; else /*anyElse*/x; }
        function hasInstance(x: string | B) { if (x instanceof Checker) /*hasInstance*/x; else /*hasInstanceElse*/x; }
        function inherited(x: string | B) { if (x instanceof SubChecker) /*inherited*/x; }
        function booleanHasInstance(x: B | C) { if (x instanceof BoolChecker) /*booleanHasInstance*/x; }
        "#,
        &[
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
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. Two classes with the same
// name and members in two namespaces of one module are two declarations.
#[test]
fn instanceof_tells_classes_with_the_same_contents_apart() {
    assert_reads(
        r#"
        namespace P1 { export class K { k = 1 } }
        namespace P2 { export class K { k = 1 } }
        namespace Q1 { export class K { k = 1 } export class L extends K {} }
        namespace Q2 { export class K { k = 1 } }
        function other(x: P1.K | string) { if (x instanceof P2.K) /*other*/x; else /*otherElse*/x; }
        function same(x: P1.K | string) { if (x instanceof P1.K) /*same*/x; else /*sameElse*/x; }
        function both(x: P1.K | P2.K) { /*bothDeclared*/x; if (x instanceof P1.K) /*both*/x; else /*bothElse*/x; }
        function parent(x: Q1.L | string) { if (x instanceof Q2.K) /*parent*/x; else /*parentElse*/x; }
        function ownParent(x: Q1.L | string) { if (x instanceof Q1.K) /*ownParent*/x; else /*ownParentElse*/x; }
        "#,
        &[
            // TypeScript gives `P2.K`, because it relates `P2.K` to `P1.K` by
            // their members. Biome doesn't, because a class can declare
            // private members, so the read keeps its type.
            ("other", "x", &["instanceof K", "string"]),
            ("otherElse", "x", &["instanceof K", "string"]),
            ("same", "x", &["instanceof K"]),
            ("sameElse", "x", &["string"]),
            ("bothDeclared", "x", &["instanceof K", "instanceof K"]),
            ("both", "x", &["instanceof K"]),
            ("bothElse", "x", &["instanceof K"]),
            // TypeScript gives `Q2.K`, because it relates `Q2.K` to `Q1.L` by
            // their members. Biome doesn't, so the read keeps its type.
            ("parent", "x", &["instanceof L", "string"]),
            ("parentElse", "x", &["instanceof L", "string"]),
            ("ownParent", "x", &["instanceof L"]),
            ("ownParentElse", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) merges a class and the interfaces of its
// name in one scope into one type. The bases of that type are the types in
// the `extends` clauses of the class and of the interfaces. Interfaces of one
// name merge with all their `extends` clauses. TypeScript gives every read
// below the expected type, unless a comment gives TypeScript's type. An
// `else` branch that no value reaches is `never` in TypeScript, which Biome
// reads as Unknown.
#[test]
fn instanceof_follows_the_extends_clauses_of_every_merged_declaration() {
    assert_reads(
        r#"
        class N extends C {} interface N extends A {}
        interface N2 extends A {} class N2 extends C {}
        class D2 extends C { d2 = 1 }
        class M { m = 1 } interface M extends A {}
        interface I3 extends A {} interface I3 { x: 1 } class D3 extends A { d = 1 }
        interface I2 { x: 1 } interface I2 extends A {}
        namespace Q3 { export class K { k = 1 } export interface K { j: 1 } }
        namespace Q4 { export class K { k = 1 } export interface K { j: 1 } }
        function classFirst(x: N | D2) { if (x instanceof C) /*classFirst*/x; else /*classFirstElse*/x; }
        function interfaceBase(x: N | D2) { if (x instanceof A) /*interfaceBase*/x; else /*interfaceBaseElse*/x; }
        function interfaceFirst(x: N2 | B) { if (x instanceof A) /*interfaceFirst*/x; else /*interfaceFirstElse*/x; }
        function interfaceFirstClass(x: N2 | A) { if (x instanceof C) /*interfaceFirstClass*/x; else /*interfaceFirstClassElse*/x; }
        function both(x: N | M) { if (x instanceof C) /*both*/x; else /*bothElse*/x; }
        function firstDeclaration(x: I3 | D3) { if (x instanceof A) /*firstDeclaration*/x; else /*firstDeclarationElse*/x; }
        function lastDeclaration(x: I2 | C) { if (x instanceof A) /*lastDeclaration*/x; else /*lastDeclarationElse*/x; }
        function namespaces(x: Q3.K | string) { if (x instanceof Q4.K) /*namespaces*/x; else /*namespacesElse*/x; }
        function sameNamespace(x: Q3.K | string) { if (x instanceof Q3.K) /*sameNamespace*/x; else /*sameNamespaceElse*/x; }
        function bothBases(x: N | M) { if (x instanceof A) /*bothBases*/x; else /*bothBasesElse*/x; }
        function baseAndClass(x: N | B) { if (x instanceof A) /*baseAndClass*/x; else /*baseAndClassElse*/x; }
        "#,
        &[
            ("classFirst", "x", &["instanceof D2", "instanceof N"]),
            ("classFirstElse", "x", &["unknown"]),
            ("interfaceBase", "x", &["instanceof N"]),
            ("interfaceBaseElse", "x", &["instanceof D2"]),
            ("interfaceFirst", "x", &["instanceof B", "instanceof N2"]),
            ("interfaceFirstElse", "x", &["unknown"]),
            ("interfaceFirstClass", "x", &["instanceof N2"]),
            ("interfaceFirstClassElse", "x", &["instanceof A"]),
            ("both", "x", &["instanceof N"]),
            ("bothElse", "x", &["instanceof M"]),
            ("firstDeclaration", "x", &["I3", "instanceof D3"]),
            ("firstDeclarationElse", "x", &["unknown"]),
            ("lastDeclaration", "x", &["I2"]),
            ("lastDeclarationElse", "x", &["instanceof C"]),
            // TypeScript gives `Q4.K` for `namespaces` because it relates
            // `Q4.K` to `Q3.K` by their members. Biome treats equal merged
            // declarations in separate namespaces as separate classes, so
            // `namespaces` keeps its declared type.
            ("namespaces", "x", &["instanceof K", "string"]),
            // TypeScript gives the declared type, `string | Q3.K`, for
            // `namespacesElse`.
            ("namespacesElse", "x", &["instanceof K", "string"]),
            ("sameNamespace", "x", &["instanceof K"]),
            ("sameNamespaceElse", "x", &["string"]),
            ("bothBases", "x", &["instanceof M", "instanceof N"]),
            ("bothBasesElse", "x", &["unknown"]),
            ("baseAndClass", "x", &["instanceof B", "instanceof N"]),
            ("baseAndClassElse", "x", &["unknown"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the comment
// of its line, in order. A type parameter is derived from what its constraint
// is derived from, and an instance type that no member relates to gives its
// intersection with each type parameter whose constraint it is derived from.
#[test]
fn instanceof_intersects_type_parameters() {
    assert_reads(
        r#"
        function generic<T extends A>(x: T | C) { if (x instanceof B) /*typeParameter*/x; else /*typeParameterElse*/x; } // T & B; T | C
        function derived<T extends B>(x: T | C) { if (x instanceof A) /*derived*/x; else /*derivedElse*/x; } // T; C
        function unconstrained<T>(x: T | C) { if (x instanceof A) /*unconstrained*/x; } // T & A
        "#,
        &[
            ("typeParameter", "x", &["instanceof T & instanceof B"]),
            ("typeParameterElse", "x", &["instanceof C", "instanceof T"]),
            ("derived", "x", &["instanceof T"]),
            ("derivedElse", "x", &["instanceof C"]),
            ("unconstrained", "x", &["instanceof T & instanceof A"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the comment
// of its line, in order. `#p in x` narrows `x` as `x instanceof P` would,
// where `P` declares `#p`, and a static name narrows `x` to the class
// itself. Normalization merges an instance of `P` with an object type or an
// instance of another class into an instance of an anonymous class, which
// has the same values as TypeScript's intersection.
#[test]
fn private_names_narrow_to_the_class_that_declares_them() {
    assert_reads(
        r#"
        class P {
            #p = 1;
            static #s = 2;
            m1(x: P | Date) { if (#p in x) /*m1*/x; else /*m1Else*/x; } // P; Date
            m2(x: unknown) { if (typeof x === "object" && x !== null && #p in x) /*m2*/x; } // P
            m3(x: object) { if (#p in x) /*m3*/x; } // P
            m4(x: Q | Date) { if (#p in x) /*m4*/x; else /*m4Else*/x; } // Q; Date
            m6(x: typeof P | Date) { if (#s in x) /*m6*/x; } // typeof P
        }
        class Q extends P { q = 1 }
        class R {
            #r = 1;
            m5(x: { a: 1 } | Date) { if (#r in x) /*m5*/x; else /*m5Else*/x; } // (Date | { a: 1 }) & R; Date | { a: 1 }
            m6(x: { a: 1 }) { if (#r in x) /*m6R*/x; } // { a: 1 } & R
            m7(x: { a: 1 } | string) { if (#r in x) /*m7*/x; } // (string | { a: 1 }) & R, with error TS2322
        }
        "#,
        &[
            ("m1", "x", &["instanceof P"]),
            ("m1Else", "x", &["instanceof Date"]),
            ("m2", "x", &["instanceof P"]),
            ("m3", "x", &["instanceof P"]),
            ("m4", "x", &["instanceof Q"]),
            ("m4Else", "x", &["instanceof Date"]),
            (
                "m6",
                "x",
                &[r#"class "P" { extends: none implements: [] type_args: [] }"#],
            ),
            (
                "m5",
                "x",
                &[
                    "instanceof class { extends: none implements: [] type_args: [] }",
                    "instanceof class { extends: none implements: [] type_args: [] }",
                ],
            ),
            ("m5Else", "x", &[r#"["a": number: 1]"#, "instanceof Date"]),
            (
                "m6R",
                "x",
                &["instanceof class { extends: none implements: [] type_args: [] }"],
            ),
            (
                "m7",
                "x",
                &[
                    "instanceof class { extends: none implements: [] type_args: [] }",
                    "string & instanceof R",
                ],
            ),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the comment
// of its line, in order. A static private name is a property of the class
// that declares it, not of the classes derived from it. Where `#s in x` is
// false, TypeScript removes the class only when it is the whole type of `x`;
// a union keeps every member, the class included, because it compares each
// member with the class by derivation, and a class is not derived from
// itself. Where TypeScript gives an intersection with the class, Biome keeps
// the type of `x`.
#[test]
fn static_private_names_remove_the_class_only_when_it_is_the_whole_type() {
    assert_reads(
        r#"
        class P {
            static #s = 2;
            static s1(x: typeof P | Date) { if (#s in x) /*s1*/x; else /*s1Else*/x; } // typeof P; Date | typeof P
            static s2(x: typeof P) { if (#s in x) /*s2*/x; else /*s2Else*/x; } // typeof P; never
            static s3(x: typeof P | typeof Q) { if (#s in x) /*s3*/x; else /*s3Else*/x; } // typeof P; typeof P | typeof Q
            static s4(x: typeof Q | Date) { if (#s in x) /*s4*/x; else /*s4Else*/x; } // (Date | typeof Q) & typeof P; Date | typeof Q
            static s5(x: P | typeof P) { if (#s in x) /*s5*/x; else /*s5Else*/x; } // typeof P; P | typeof P
            static s6(x: typeof P | Date) { if (x instanceof Date) return; if (#s in x) /*s6*/x; else /*s6Else*/x; } // typeof P; never
            static s7(x: typeof P | string) { if (typeof x === "string") return; if (!(#s in x)) /*s7Else*/x; } // never
            static s8<T extends object>(x: T | typeof P) { if (#s in x) /*s8*/x; else /*s8Else*/x; } // T & typeof P; T | typeof P
            static s9(x: typeof P | null) { if (x && !(#s in x)) /*s9Else*/x; } // never
        }
        class Q extends P { static q = 1 }
        "#,
        &[
            ("s1", "x", &[CLASS_P]),
            ("s1Else", "x", &[CLASS_P, "instanceof Date"]),
            ("s2", "x", &[CLASS_P]),
            ("s2Else", "x", &["unknown"]),
            ("s3", "x", &[CLASS_P]),
            // `typeof P | typeof Q`: Biome reads the declared type of `x`
            // as Unknown, as it does without the test.
            ("s3Else", "x", &["unknown"]),
            ("s4", "x", &[CLASS_Q, "instanceof Date"]),
            ("s4Else", "x", &[CLASS_Q, "instanceof Date"]),
            ("s5", "x", &[CLASS_P]),
            // `P | typeof P`: as for `s3Else`.
            ("s5Else", "x", &["unknown"]),
            ("s6", "x", &[CLASS_P]),
            ("s6Else", "x", &["unknown"]),
            ("s7Else", "x", &["unknown"]),
            ("s8", "x", &[CLASS_P]),
            ("s8Else", "x", &[CLASS_P, "instanceof T"]),
            ("s9Else", "x", &["unknown"]),
        ],
    );
}

/// How the reads below name the class `P` itself.
const CLASS_P: &str = r#"class "P" { extends: none implements: [] type_args: [] }"#;
/// How the reads below name the class `Q` itself, which extends `P`.
const CLASS_Q: &str = r#"class "Q" { extends: class "P" { extends: none implements: [] type_args: [] } implements: [] type_args: [] }"#;

// TypeScript 7.0.2 (`tsc --strict`) gives each read the type in the comment
// of its line, in order. `x.constructor === C` keeps the members that `C` may
// have constructed: an instance of the class `C` itself, but not of a class
// derived from it, or another member when it is assignable to the type of
// `C.prototype`. Only the branch where the comparison holds narrows.
#[test]
fn constructor_comparisons_narrow_where_they_hold() {
    assert_reads(
        r#"
        class K1 { k1 = 1 }
        class K2 { k2 = 2 }
        class K3 extends K1 { k3 = 3 }
        function k1(x: K1 | K2) { if (x.constructor === K1) /*k1*/x; else /*k1Else*/x; } // K1; K1 | K2
        function k2(x: K1 | K2 | K3) { if (x.constructor === K1) /*k2*/x; } // K1
        function k3(x: K1 | string) { if (x.constructor === String) /*k3*/x; } // string
        function k4(x: K1 | K2) { if (K1 === x.constructor) /*k4*/x; } // K1
        function k5(x: K1 | K2) { if (x.constructor == K1) /*k5*/x; } // K1
        function k6(x: K1 | K2) { if (x.constructor !== K1) /*k6*/x; else /*k6Else*/x; } // K1 | K2; K1
        function k7(x: unknown) { if (x && (x as object).constructor === K1) /*k7*/x; } // {}
        function k8(x: K1 | K2 | null) { if (x?.constructor === K1) /*k8*/x; } // K1
        function k9(x: Date | string) { if (x.constructor === Date) /*k9*/x; } // Date
        function k10(o: { v: K1 | K2 }) { if (o.v.constructor === K1) /*k10*/o.v; } // K1
        function k11(x: K1 | number) { if (x.constructor === Number) /*k11*/x; } // number
        function k12(x: Map<string, number> | Set<string>) { if (x.constructor === Map) /*k12*/x; } // Map<string, number>
        function k13(x: any) { if (x.constructor === K1) /*k13*/x; } // K1
        function k14(x: K1 | { k1: number }) { if (x.constructor === K1) /*k14*/x; } // K1
        "#,
        &[
            ("k1", "x", &["instanceof K1"]),
            ("k1Else", "x", &["instanceof K1", "instanceof K2"]),
            ("k2", "x", &["instanceof K1"]),
            ("k3", "x", &["string"]),
            ("k4", "x", &["instanceof K1"]),
            ("k5", "x", &["instanceof K1"]),
            ("k6", "x", &["instanceof K1", "instanceof K2"]),
            ("k6Else", "x", &["instanceof K1"]),
            ("k7", "x", &["[]"]),
            ("k8", "x", &["instanceof K1"]),
            ("k9", "x", &["instanceof Date"]),
            ("k10", "o.v", &["instanceof K1"]),
            ("k11", "x", &["number"]),
            ("k12", "x", &["instanceof Map<string, number>"]),
            ("k13", "x", &["instanceof K1"]),
            ("k14", "x", &["instanceof K1"]),
        ],
    );
}
