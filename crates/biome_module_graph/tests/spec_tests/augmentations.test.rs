//! Interfaces that modules merge into declarations they don't own: module
//! augmentations, `declare global` blocks, and declaration scripts.
//!
//! TypeScript 7.0.2 (`tsc --strict`) gives every marked read the type that
//! the test expects, unless a comment gives TypeScript's type. An `else`
//! branch that no value reaches is `never` in TypeScript, which Biome reads
//! as Unknown.

use super::*;
use biome_module_graph::LocalTypeInput;

const AUG_SHAPES: &str = r#"
export class Base { base = 1 }
export interface X { x: 1 }
export class K { k = 1 }
export interface G<T> { a: T }
export interface R { readonly v: string | null }
"#;

const AUG_AUGMENT: &str = r#"
import { Base } from "./shapes";
declare module "./shapes" {
  interface X extends Base { extra: 1 }
  interface K extends Base { kx: 1 }
  interface G<T> { b: T }
  interface R { w: string | null }
  interface Fresh { f: 1 }
  interface X { self(): X }
}
"#;

const AUG_USE: &str = r#"
import type { X, K, G, R } from "./shapes";
import { Base } from "./shapes";
class C { c = 3 }
interface E { extra: 2 }
function in1(x: X | E) { if ("extra" in x) /*in1*/x; else /*notIn1*/x; }
function io1(x: X | C) { if (x instanceof Base) /*io1*/x; else /*notIo1*/x; }
function io2(x: K | C) { if (x instanceof Base) /*io2*/x; else /*notIo2*/x; }
function in2(x: K | C) { if ("kx" in x) /*in2*/x; else /*notIn2*/x; }
declare const g: G<string>;
/*ga*/g.a; /*gb*/g.b;
declare const x: X;
/*selfX*/x.self().x; /*selfExtra*/x.self().extra;
declare function isExtra(v: unknown): v is { extra: 1 };
function pr1(v: X | { extra: 1; y: 2 } | string) { if (isExtra(v)) /*pr1*/v; else /*notPr1*/v; }
"#;

const AUG_FRESH: &str = r#"
import type { Fresh } from "./shapes";
declare const f: Fresh;
/*fresh*/f.f;
"#;

const AUG_NOIMPORT: &str = r#"
import type { X } from "./shapes";
interface E { extra: 2 }
function in1(x: X | E) { if ("extra" in x) /*in1*/x; else /*notIn1*/x; }
"#;

const AUG_FILES: &[(&str, &str)] = &[
    ("/src/shapes.ts", AUG_SHAPES),
    ("/src/augment.ts", AUG_AUGMENT),
    ("/src/use.ts", AUG_USE),
    ("/src/fresh.ts", AUG_FRESH),
    ("/src/noimport.ts", AUG_NOIMPORT),
];

// TypeScript merges an augmentation into its target for the whole program,
// also for a module that doesn't import the augmenting one.
#[test]
fn module_augmentations_add_members_and_bases_to_their_targets() {
    assert_file_reads(
        AUG_FILES,
        "/src/use.ts",
        &[
            ("in1", "x", &["E", "X"]),
            ("notIn1", "x", &["unknown"]),
            ("io1", "x", &["X"]),
            ("notIo1", "x", &["instanceof C"]),
            ("io2", "x", &["instanceof K"]),
            ("notIo2", "x", &["instanceof C"]),
            // TypeScript: `K`. The `in` operator keeps class instances.
            ("in2", "x", &["instanceof C", "instanceof K"]),
            ("notIn2", "x", &["instanceof C"]),
            ("ga", "g.a", &["string"]),
            ("gb", "g.b", &["string"]),
            ("selfX", "x.self().x", &["number: 1"]),
            ("selfExtra", "x.self().extra", &["number: 1"]),
            (
                "pr1",
                "v",
                &["X", "[\"extra\": number: 1, \"y\": number: 2]"],
            ),
            ("notPr1", "v", &["string"]),
        ],
    );
    assert_file_reads(
        AUG_FILES,
        "/src/fresh.ts",
        &[("fresh", "f.f", &["number: 1"])],
    );
    assert_file_reads(
        AUG_FILES,
        "/src/noimport.ts",
        &[("in1", "x", &["E", "X"]), ("notIn1", "x", &["unknown"])],
    );
}

// TypeScript merges an augmentation of a module that re-exports a
// declaration, with `export *` or by name, into that declaration.
#[test]
fn module_augmentations_merge_through_reexports() {
    assert_file_reads(
        &[
            (
                "/src/augment.ts",
                r#"
import "./star";
import "./named";
declare module "./star" { interface X { extra: 1 } }
declare module "./named" { interface Y { extra: 1 } }
export {};
"#,
            ),
            (
                "/src/inner.ts",
                "export interface X { x: 1 }\nexport interface Y { y: 1 }\n",
            ),
            ("/src/named.ts", "export { Y } from \"./inner\";\n"),
            ("/src/star.ts", "export * from \"./inner\";\n"),
            (
                "/src/use.ts",
                r#"
import type { X } from "./inner";
import type { Y } from "./inner";
interface E { extra: 2 }
function viaStar(v: X | E) { if ("extra" in v) /*star*/v; else /*notStar*/v; }
function viaNamed(v: Y | E) { if ("extra" in v) /*named*/v; else /*notNamed*/v; }
"#,
            ),
        ],
        "/src/use.ts",
        &[
            ("star", "v", &["E", "X"]),
            ("notStar", "v", &["unknown"]),
            ("named", "v", &["E", "Y"]),
            ("notNamed", "v", &["unknown"]),
        ],
    );
}

const GLOBAL_AUGMENT: &str = r#"
export {};
declare global {
  interface String { format(): string }
  interface Array<T> { last(): T | undefined }
  interface FreshGlobal { g: 1 }
}
interface Local { length: 1 }
function local(x: String | Local) { if ("length" in x) /*local*/x; else /*notLocal*/x; }
declare const s: String;
/*length*/s.length; /*format*/s.format();
"#;

const GLOBAL_USE: &str = r#"
interface Fmt { format(): string }
function wrapper(x: Fmt | number) { if (typeof x === "string") /*wrapper*/x; else /*notWrapper*/x; }
declare function isFormat(v: unknown): v is { format(): string };
function pred(x: string | { format(): string; a: 1 }) { if (isFormat(x)) /*pred*/x; else /*notPred*/x; }
declare const xs: string[];
/*last*/xs.last();
declare const fg: FreshGlobal;
/*fresh*/fg.g;
function falsy(x: Fmt | null) { if (!x) /*falsy*/x; else /*truthy*/x; }
export {};
"#;

const GLOBAL_FILES: &[(&str, &str)] = &[
    ("/src/augment.ts", GLOBAL_AUGMENT),
    ("/src/use.ts", GLOBAL_USE),
];

// In the module with the `declare global` block, `String` is the global
// with the members of both declarations.
#[test]
fn declare_global_merges_into_the_global_in_the_declaring_module() {
    assert_file_reads(
        GLOBAL_FILES,
        "/src/augment.ts",
        &[
            ("local", "x", &["Local", "instanceof String"]),
            ("notLocal", "x", &["unknown"]),
            ("length", "s.length", &["number"]),
            ("format", "s.format()", &["string"]),
        ],
    );
}

// Another module sees the members that `declare global` adds to the
// wrapper of `string` and to `Array`, and the global that only
// `declare global` declares.
#[test]
fn declare_global_merges_into_the_global_in_other_modules() {
    assert_file_reads(
        GLOBAL_FILES,
        "/src/use.ts",
        &[
            ("wrapper", "x", &["string"]),
            ("notWrapper", "x", &["Fmt", "number"]),
            (
                "pred",
                "x",
                &[
                    "[ \"format\": sync Function \"format\" { accepts: { params: [] type_args: [] } returns: string } , \"a\": number: 1 ]",
                    "string",
                ],
            ),
            ("notPred", "x", &["unknown"]),
            ("last", "xs.last()", &["string", "undefined"]),
            ("fresh", "fg.g", &["number: 1"]),
            // TypeScript: `null`. A falsy test keeps the object types that a
            // primitive's wrapper can satisfy, and `declare global` gives
            // `String` a `format` method.
            ("falsy", "x", &["Fmt", "null"]),
            ("truthy", "x", &["Fmt"]),
        ],
    );
}

// The top-level interfaces of a declaration file without imports or exports
// are globals.
#[test]
fn declaration_scripts_declare_globals() {
    assert_file_reads(
        &[
            (
                "/src/globals.d.ts",
                "interface String { shout(): string }\ninterface ScriptOnly { s: 1 }\n",
            ),
            (
                "/src/use.ts",
                r#"
interface Loud { shout(): string }
function wrapper(x: Loud | number) { if (typeof x === "string") /*wrapper*/x; else /*notWrapper*/x; }
declare const so: ScriptOnly;
/*scriptOnly*/so.s;
export {};
"#,
            ),
        ],
        "/src/use.ts",
        &[
            ("wrapper", "x", &["string"]),
            ("notWrapper", "x", &["Loud", "number"]),
            ("scriptOnly", "so.s", &["number: 1"]),
        ],
    );
}

#[test]
fn in_sees_the_members_of_augmentations() {
    assert_file_reads(
        &[
            (
                "/src/augment.ts",
                "import './shapes';\ndeclare module './shapes' { interface X { extra: 1 } }\nexport {};\n",
            ),
            (
                "/src/index.ts",
                r#"
import type { X, Y } from './shapes';
import './augment';
interface E { extra: 2 }
function f(x: X | E) { if ('extra' in x) /*f*/x; else /*notF*/x; }
function g(x: X | Y) { if ('extra' in x) /*g*/x; else /*notG*/x; }
"#,
            ),
            (
                "/src/shapes.ts",
                "export interface X { a: 1 }\nexport interface Y { b: 2 }\n",
            ),
        ],
        "/src/index.ts",
        &[
            ("f", "x", &["E", "X"]),
            ("notF", "x", &["unknown"]),
            ("g", "x", &["X"]),
            ("notG", "x", &["Y"]),
        ],
    );
}

#[test]
fn instanceof_follows_the_bases_that_augmentations_add() {
    assert_file_reads(
        &[
            (
                "/src/augment.ts",
                r#"
import { Base } from "./shapes";
declare module "./shapes" {
  interface X extends Base {}
  interface K extends Base {}
}
"#,
            ),
            (
                "/src/shapes.ts",
                "export class Base { base = 1 }\nexport interface X { x: 1 }\nexport class K { k = 1 }\n",
            ),
            (
                "/src/use.ts",
                r#"
import "./augment";
import { Base, X, K } from "./shapes";
class C { c = 3 }
export function f1(x: X | C) { if (x instanceof Base) /*f1*/x; else /*notF1*/x; }
export function f2(x: K | C) { if (x instanceof Base) /*f2*/x; else /*notF2*/x; }
"#,
            ),
        ],
        "/src/use.ts",
        &[
            ("f1", "x", &["X"]),
            ("notF1", "x", &["instanceof C"]),
            ("f2", "x", &["instanceof K"]),
            ("notF2", "x", &["instanceof C"]),
        ],
    );
    assert_file_reads(
        &[
            (
                "/src/a.ts",
                "export class K { k = 1 }\nexport class Base { base = 1 }\nexport interface X { x: 1 }\nexport class Sub extends Base { s = 1 }\n",
            ),
            (
                "/src/augment.ts",
                "import { Base } from \"./a\";\ndeclare module \"./a\" { interface X extends Base {} }\n",
            ),
            (
                "/src/index.ts",
                r#"
import "./augment";
import { Base, X, Sub } from "./a";
export function f3(x: X | Sub) { if (x instanceof Base) /*f3*/x; else /*notF3*/x; }
"#,
            ),
        ],
        "/src/index.ts",
        &[
            ("f3", "x", &["X", "instanceof Sub"]),
            ("notF3", "x", &["unknown"]),
        ],
    );
}

// A type predicate keeps the union members that satisfy its target: an
// interface that an augmentation completes, an interface merged from two
// declarations, and `string`, whose wrapper `declare global` gives a
// `format` method.
#[test]
fn type_predicates_keep_members_that_augmentations_complete() {
    const FILES: &[(&str, &str)] = &[
        (
            "/src/augment.ts",
            "import \"./shapes\";\ndeclare module \"./shapes\" { interface X { extra: 1 } }\nexport {};\n",
        ),
        (
            "/src/global.ts",
            r#"
export {};
declare global { interface String { format(): string } }
declare function isFormat(v: unknown): v is { format(): string };
function g(x: string | { format(): string; a: 1 }) { if (isFormat(x)) /*g*/x; else /*notG*/x; }
"#,
        ),
        (
            "/src/index.ts",
            r#"
import type { X } from "./shapes";
import "./augment";
declare function isExtra(v: unknown): v is { extra: 1 };
function aug(x: X | { extra: 1; y: 2 } | string) { if (isExtra(x)) /*aug*/x; else /*notAug*/x; }
interface M { m: 1 } interface M { n: 2 }
declare function isM(v: unknown): v is { m: 1 };
function merged(x: M | { m: 1; y: 2 } | string) { if (isM(x)) /*merged*/x; else /*notMerged*/x; }
"#,
        ),
        ("/src/shapes.ts", "export interface X { x: 1 }\n"),
    ];
    assert_file_reads(
        FILES,
        "/src/index.ts",
        &[
            (
                "aug",
                "x",
                &["X", "[\"extra\": number: 1, \"y\": number: 2]"],
            ),
            ("notAug", "x", &["string"]),
            (
                "merged",
                "x",
                &["M", "[\"m\": number: 1, \"y\": number: 2]"],
            ),
            ("notMerged", "x", &["string"]),
        ],
    );
    assert_file_reads(
        FILES,
        "/src/global.ts",
        &[
            (
                "g",
                "x",
                &[
                    "[ \"format\": sync Function \"format\" { accepts: { params: [] type_args: [] } returns: string } , \"a\": number: 1 ]",
                    "string",
                ],
            ),
            ("notG", "x", &["unknown"]),
        ],
    );
}

// `declare global` adds the member to `String` in the module that reads it.
#[test]
fn typeof_sees_wrapper_members_that_declare_global_adds() {
    assert_file_reads(
        &[(
            "/src/augmented.ts",
            r#"
export {};
declare global { interface String { format(): string } }
interface Fmt { format(): string }
function f(x: Fmt | number) { if (typeof x === "string") /*string*/x; }
"#,
        )],
        "/src/augmented.ts",
        &[("string", "x", &["string"])],
    );
}

// The first declaration of a property decides whether it is readonly
// (TS2687 reports the other).
#[test]
fn readonly_paths_take_readonly_from_the_declaration_that_an_augmentation_redeclares() {
    assert_file_reads(
        &[
            (
                "/src/augment.ts",
                r#"
import "./shapes";
declare module "./shapes" {
  interface X { v: string | null }
  interface Y { readonly v: string | null }
}
"#,
            ),
            (
                "/src/shapes.ts",
                "export interface X { readonly v: string | null }\nexport interface Y { v: string | null }\n",
            ),
            (
                "/src/use.ts",
                r#"
import "./augment";
import type { X, Y } from "./shapes";
function a1(o: X) { const ok = o.v !== null; if (ok) /*a1*/o.v; }
function a2(o: Y) { const ok = o.v !== null; if (ok) /*a2*/o.v; }
"#,
            ),
        ],
        "/src/use.ts",
        &[
            ("a1", "o.v", &["string"]),
            ("a2", "o.v", &["null", "string"]),
        ],
    );
}

// TypeScript merges augmentations after the declarations they augment, in
// the order of the program's files, and tries the overloads of a later
// declaration first. Biome orders the augmenting modules by path, so the
// expected types match TypeScript when the program lists the files in path
// order.
#[test]
fn methods_of_contributions_are_tried_before_those_of_their_targets() {
    assert_file_reads(
        &[
            (
                "/src/a.ts",
                "export interface X { f(x: unknown): 1 }\nexport class C { g(x: unknown): 1 { return 1; } }\n",
            ),
            (
                "/src/b.ts",
                "declare module \"./a\" { interface X { f(x: unknown): 2 } interface C { g(x: unknown): 2 } }\nexport {};\n",
            ),
            (
                "/src/c.ts",
                "declare module \"./a\" { interface X { f(x: unknown): 3 } }\nexport {};\n",
            ),
            (
                "/src/use.ts",
                r#"
import type { X, C } from "./a";
declare const x: X; declare const c: C;
/*f*/x.f(0); /*g*/c.g(0);
"#,
            ),
        ],
        "/src/use.ts",
        &[
            ("f", "x.f(0)", &["number: 3"]),
            ("g", "c.g(0)", &["number: 2"]),
        ],
    );
    assert_file_reads(
        &[
            (
                "/src/augment.ts",
                r#"
export {};
declare global {
  interface String { charAt(pos: number): 1 }
  interface Array<T> { at(index: number): 2 }
}
"#,
            ),
            (
                "/src/use.ts",
                r#"
declare const xs: string[];
/*charAt*/"".charAt(0); /*at*/xs.at(0);
export {};
"#,
            ),
        ],
        "/src/use.ts",
        &[
            ("charAt", "\"\".charAt(0)", &["number: 1"]),
            ("at", "xs.at(0)", &["number: 2"]),
        ],
    );
}

// The members that an augmentation declares on a class are the class's own
// in TypeScript, so they come before those of its parent class.
#[test]
fn class_contributions_come_before_the_parent_class() {
    assert_file_reads(
        &[
            (
                "/src/a.ts",
                "export class P { m: number = 1 }\nexport class C extends P {}\n",
            ),
            (
                "/src/b.ts",
                "declare module \"./a\" { interface C { m: 1 } }\nexport {};\n",
            ),
            (
                "/src/use.ts",
                r#"
import type { C } from "./a";
declare const c: C;
/*member*/c.m;
"#,
            ),
        ],
        "/src/use.ts",
        &[("member", "c.m", &["number: 1"])],
    );
}

// #region SALSA EXECUTION

const SALSA_SHAPES: &str = "export interface X { x: 1 }\n";
const SALSA_AUGMENT: &str =
    "import \"./shapes\";\ndeclare module \"./shapes\" { interface X { extra: 1 } }\nexport {};\n";
const SALSA_USE: &str = r#"
import type { X } from "./shapes";
declare const x: X;
/*read*/x.extra;
"#;

fn salsa_db(augment: &str) -> (MemoryFileSystem, TestModuleDb) {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/shapes.ts".into(), SALSA_SHAPES);
    fs.insert("/src/augment.ts".into(), augment);
    fs.insert("/src/unrelated.ts".into(), "export const unrelated = 1;\n");
    fs.insert("/src/use.ts".into(), SALSA_USE);
    let db = build_js_test_module_db(
        &fs,
        &[
            "/src/shapes.ts",
            "/src/augment.ts",
            "/src/unrelated.ts",
            "/src/use.ts",
        ],
        true,
    );
    (fs, db)
}

/// Returns the names of the type of `expression`, which follows the comment
/// `/*marker*/` in `source`, the source of the module at `path`.
fn read_at(
    db: &TestModuleDb,
    path: &str,
    source: &str,
    marker: &str,
    expression: &str,
) -> Vec<String> {
    let module = db.module_for_path(Utf8Path::new(path)).unwrap();
    let marker = format!("/*{marker}*/");
    let start = source.find(&marker).unwrap() + marker.len();
    let range = TextRange::at(
        start.try_into().unwrap(),
        expression.len().try_into().unwrap(),
    );
    let ty = biome_module_graph::type_inference::execute_type_inference_request(
        db,
        biome_module_graph::type_inference::TypeInferenceCaller::new("test", "augmentations"),
        biome_module_graph::type_inference::NormalizedExpressionTypeRequest::new(module, range),
    )
    .expect("the read must be inferred");
    type_names(db, ty)
}

fn read_extra(db: &TestModuleDb) -> Vec<String> {
    read_at(db, "/src/use.ts", SALSA_USE, "read", "x.extra")
}

fn set_source(fs: &MemoryFileSystem, db: &mut TestModuleDb, path: &str, source: &str) {
    fs.insert(path.into(), source);
    let module = db.module_for_path(Utf8Path::new(path)).unwrap();
    let kind = resolve_js_module_kind_for_test(fs, path, true);
    salsa::Setter::to(module.set_kind(db), kind);
}

fn executions(db: &TestModuleDb, query: &str, events: &[salsa::Event]) -> usize {
    function_query_will_execute_count_by_name(db, query, events)
}

#[test]
fn editing_an_unrelated_module_reuses_the_index_and_the_target() {
    let (fs, mut db) = salsa_db(SALSA_AUGMENT);
    assert_eq!(read_extra(&db), ["number: 1"]);

    set_source(
        &fs,
        &mut db,
        "/src/unrelated.ts",
        "export const unrelated = 2;\n",
    );
    db.clear_salsa_events();
    assert_eq!(read_extra(&db), ["number: 1"]);
    let events = db.take_salsa_events();
    assert_eq!(executions(&db, "module_augmentations", &events), 1);
    assert_eq!(executions(&db, "augmentation_index", &events), 0);
    assert_eq!(executions(&db, "declaration_contributions", &events), 0);
    assert_eq!(executions(&db, "infer_local_type", &events), 0);
    assert_eq!(executions(&db, "infer_module_types", &events), 0);
}

#[test]
fn editing_a_contributed_member_recomputes_only_the_contribution() {
    let (fs, mut db) = salsa_db(SALSA_AUGMENT);
    assert_eq!(read_extra(&db), ["number: 1"]);

    // The contribution keeps its type ID, so the index stays the same.
    set_source(
        &fs,
        &mut db,
        "/src/augment.ts",
        "import \"./shapes\";\ndeclare module \"./shapes\" { interface X { extra: 2 } }\nexport {};\n",
    );
    db.clear_salsa_events();
    assert_eq!(read_extra(&db), ["number: 2"]);
    let events = db.take_salsa_events();
    let augment = db
        .module_for_path(Utf8Path::new("/src/augment.ts"))
        .unwrap();
    let shapes = db.module_for_path(Utf8Path::new("/src/shapes.ts")).unwrap();
    assert_eq!(executions(&db, "augmentation_index", &events), 0);
    assert_eq!(executions(&db, "declaration_contributions", &events), 0);
    let local_type_modules: Vec<_> = events
        .iter()
        .filter_map(|event| {
            let salsa::EventKind::WillExecute { database_key } = event.kind else {
                return None;
            };
            (salsa::Database::ingredient_debug_name(&db, database_key.ingredient_index())
                == "infer_local_type")
                .then(|| {
                    let input = LocalTypeInput::from_id(database_key.key_index());
                    input.module(&db)
                })
        })
        .collect();
    assert!(!local_type_modules.is_empty());
    assert!(local_type_modules.iter().all(|module| *module == augment));
    assert!(!local_type_modules.contains(&shapes));
    assert_eq!(executions(&db, "infer_module_types", &events), 0);
}

#[test]
fn adding_an_augmentation_recomputes_the_target() {
    let (fs, mut db) = salsa_db("import \"./shapes\";\nexport {};\n");
    assert_eq!(read_extra(&db), ["unknown"]);

    set_source(&fs, &mut db, "/src/augment.ts", SALSA_AUGMENT);
    db.clear_salsa_events();
    assert_eq!(read_extra(&db), ["number: 1"]);
    let events = db.take_salsa_events();
    let shapes = db.module_for_path(Utf8Path::new("/src/shapes.ts")).unwrap();
    assert_eq!(executions(&db, "augmentation_index", &events), 1);
    assert_function_query_was_run(
        &db,
        biome_module_graph::infer_local_type,
        LocalTypeInput::new(
            &db,
            shapes,
            InferredLocalTypeId::new(local_type_index(&db, shapes)),
        ),
        &events,
    );
    assert_eq!(executions(&db, "infer_module_types", &events), 0);
}

#[test]
fn lookups_in_modules_without_augmentations_skip_whole_module_inference() {
    const USE: &str = "import type { X } from \"./shapes\";\ndeclare const x: X;\n/*read*/x.x;\n";
    let fs = MemoryFileSystem::default();
    fs.insert("/src/shapes.ts".into(), SALSA_SHAPES);
    fs.insert("/src/use.ts".into(), USE);
    let db = build_js_test_module_db(&fs, &["/src/shapes.ts", "/src/use.ts"], true);
    db.clear_salsa_events();
    assert_eq!(
        read_at(&db, "/src/use.ts", USE, "read", "x.x"),
        ["number: 1"]
    );
    let events = db.take_salsa_events();
    assert_eq!(executions(&db, "augmentation_index", &events), 1);
    assert_eq!(executions(&db, "infer_module_types", &events), 0);
}

/// Returns the index of the raw type that the binding `X` of `module` has.
fn local_type_index(db: &TestModuleDb, module: ModuleInfo) -> usize {
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        panic!("module must contain JavaScript information");
    };
    let range = binding_range_by_name(db, module, "X");
    match info.raw_binding_types.get(&range) {
        Some(biome_js_type_info::TypeReference::Resolved(
            biome_js_type_info::RawTypeId::Local(type_id),
        )) => type_id.index(),
        other => panic!("X must have a local type, not {other:?}"),
    }
}

// #endregion
