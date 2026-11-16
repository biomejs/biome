use crate::{JsDeclarationKind, SemanticModelOptions, assert_semantics, semantic_model};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_languages::JsFileSource;

// Imports
assert_semantics! {
    ok_declaration_import, "/*START GLOBAL*/ import a/*#a*//*@GLOBAL*/ from 'a'",
}

// Statements
assert_semantics! {
    ok_declaration_if, ";if(true) {/*START A*/ let b/*#b*//*@A*/ = 1; }",
    ok_declaration_at_for, ";for/*START A*/ (let a/*#a*//*@A*/;;) {/*START B*/ let b/*#b*//*@B*/ = 1; }",
    ok_declaration_at_for_of, ";for/*START A*/ (const a/*#a*//*@A*/ of []) {/*START B*/ let b/*#b*//*@B*/ = 1; }",
    ok_declaration_at_for_in, ";for/*START A*/(const a/*#a*//*@A*/ in []) {/*START B*/ let b/*#b*//*@B*/ = 1; }",
    ok_declaration_try_catch, ";try {/*START A*/ let a/*#a*//*@A*/ = 1;} catch/*START B*/ (b1/*#b1*//*@B*/) {/*START C*/ let c/*#c*//*@C*/ = 1; }",
    ok_declaration_try_catch_finally, ";try {/*START A*/ let a/*#a*//*@A*/ = 1;} catch/*START B*/ (b1/*#b1*//*@B*/) {/*START C*/ let c/*#c*//*@C*/ = 1; } finally {/*START D*/ let d/*#d*//*@D*/ = 1; }",
}

// Functions
assert_semantics! {
    ok_declaration_function, ";function/*START A*/ f(a/*#a*//*@A*/) {/*START B*/ let b/*#b*//*@B*/ = 1; }",
    ok_declaration_self_invocation, ";(function f/*#F*/() {})();",
    ok_declaration_arrow_function, ";(/*START A*/ a/*#a*//*@A*/) => {/*START B*/ let b/*#b*//*@B*/ = 1; }",
}

// Classes
assert_semantics! {
    ok_declaration_class_constructor, "class A { constructor/*START A*/ (a/*#a*//*@A*/) {/*START B*/ let b/*#b*//*@B*/ = 1; } }",
    ok_declaration_class_getter, "class A { get/*START A*/ name() {/*START B*/ let b/*#b*//*@B*/ = 1;} }",
    ok_declaration_class_setter, "class A { set/*START A*/ name(a/*#a*//*@A*/) {/*START B*/ let b/*#b*//*@B*/ = 1;} }",
}

// Others
assert_semantics! {
    ok_declaration_at_global_scope, "/*START GLOBAL*/ let b/*#b*//*@GLOBAL*/ = 1;",
    ok_declaration_with_inner_scopes, r#";
function f() {/*START SCOPE1*/
    let a/*#a1*//*@SCOPE1*/ = 1;
    console.log(a);
    if (true) {/*START SCOPE2*/
        let a/*#a2*//*@SCOPE2*/ = 2;
        console.log(a);
    }
    console.log(a);
}
f();
"#,
}

#[test]
fn imported_bindings() {
    for code in [
        "import A from 'pkg';",
        "import { A, B as C } from 'pkg';",
        "import * as A from 'pkg';",
        "import type A from 'pkg';",
        "import type { A } from 'pkg';",
        "import { type A } from 'pkg';",
        "import type * as A from 'pkg';",
        "import A = require('pkg');",
        "import type A = require('pkg');",
        "import A = N.x;",
        "export import A = require('pkg');",
    ] {
        let r = biome_js_parser::parse(code, JsFileSource::ts(), JsParserOptions::default());
        assert!(r.diagnostics().is_empty(), "at {code:?}");
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());
        assert!(model.all_bindings().next().is_some(), "at {code:?}");
        for binding in model.all_bindings() {
            assert!(binding.is_imported(), "at {code:?}: {binding:?}");
        }
    }
}

#[test]
fn local_bindings_are_not_imported() {
    let r = biome_js_parser::parse(
        "const A = 1; function B<T>(C: T) {} class D {} interface E {} type F = number;",
        JsFileSource::ts(),
        JsParserOptions::default(),
    );
    assert!(r.diagnostics().is_empty());
    let model = semantic_model(&r.tree(), SemanticModelOptions::default());
    assert_eq!(model.all_bindings().count(), 7);
    for binding in model.all_bindings() {
        assert!(!binding.is_imported(), "{binding:?}");
    }
}

#[test]
fn imported_bindings_in_malformed_imports() {
    for code in [
        "import { A };",
        "import A;",
        "import { A } 'pkg';",
        "import { A, A } from 'pkg';",
        "import type A, { B, C } from './a';",
    ] {
        let r = biome_js_parser::parse(code, JsFileSource::ts(), JsParserOptions::default());
        assert!(!r.diagnostics().is_empty(), "at {code:?}");
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());
        assert!(model.all_bindings().next().is_some(), "at {code:?}");
        for binding in model.all_bindings() {
            assert!(binding.is_imported(), "at {code:?}: {binding:?}");
        }
    }
}

#[test]
fn class_declaration_bindings_are_classes() {
    for code in [
        "class A {}",
        "export class A {}",
        "abstract class A {}",
        "declare class A {}",
        "export default class A {}",
        "function f() { class A {} }",
        "namespace N { export class A {} }",
        "interface A {} class A {}",
    ] {
        let r = biome_js_parser::parse(code, JsFileSource::ts(), JsParserOptions::default());
        assert!(r.diagnostics().is_empty(), "at {code:?}");
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());
        let mut classes = 0;
        for binding in model.all_bindings() {
            if matches!(
                binding.tree().declaration(),
                Some(AnyJsBindingDeclaration::JsClassDeclaration(_))
            ) {
                classes += 1;
                assert_eq!(
                    binding.declaration_kind(),
                    JsDeclarationKind::Class,
                    "at {code:?}: {binding:?}"
                );
            }
        }
        assert_eq!(
            classes,
            usize::from(!code.contains("default")),
            "at {code:?}"
        );
    }
}

#[test]
fn binding_syntax_near_returns_the_node_of_syntax() {
    let parse = biome_js_parser::parse(
        r#"import { imported } from "mod";
let a = 1, [b, { c }] = [];
function f(p, ...q) {
    var hoisted;
    if (p) { var hoisted = 2; let inner; }
    const g = function named() {};
    return <Component value={imported}>{a}</Component>;
}
class C<T> { m(x) { return f(C); } }
interface C {}
enum E { A, B }
namespace N { export const n = 1; }
type M<U> = { [K in keyof U]: U[K] extends infer V ? V : never };
try {} catch (error) {}"#,
        JsFileSource::tsx(),
        JsParserOptions::default(),
    );
    let model = semantic_model(&parse.tree(), SemanticModelOptions::default());
    let nodes: Vec<_> = parse.syntax().descendants().collect();
    let mut binding_count = 0;
    for binding in model.all_bindings() {
        binding_count += 1;
        let expected = binding.syntax();
        for node in &nodes {
            let actual = binding.syntax_near(node);
            assert_eq!(actual, expected, "from {node:?}");
            assert_eq!(actual.kind(), expected.kind());
            assert_eq!(
                actual.text_range_with_trivia(),
                expected.text_range_with_trivia()
            );
            assert_eq!(binding.tree_near(node), binding.tree(), "from {node:?}");
        }
    }
    assert!(binding_count >= 25, "{binding_count} bindings");

    let mut scope_count = 0;
    for scope in model.scopes() {
        scope_count += 1;
        assert_eq!(scope.syntax_kind(), scope.syntax().kind(), "{scope:?}");
    }
    assert!(scope_count >= 10, "{scope_count} scopes");
}

/// Returns the offsets of the declaration names in each merge set of the
/// global scope of the TypeScript `source`.
fn merged_type_declaration_offsets(source: &str) -> Vec<Vec<u32>> {
    let r = biome_js_parser::parse(source, JsFileSource::ts(), JsParserOptions::default());
    assert!(r.diagnostics().is_empty(), "at {source:?}");
    let model = semantic_model(&r.tree(), SemanticModelOptions::default());
    model
        .global_scope()
        .merged_type_declarations()
        .into_iter()
        .map(|set| {
            set.into_iter()
                .map(|id| {
                    let binding = model.binding_by_id(id).expect("binding must exist");
                    binding.range().start().into()
                })
                .collect()
        })
        .collect()
}

/// Returns the offsets of the names of the classes and interfaces called
/// `name` in `source`, in source order.
fn declaration_offsets(source: &str, name: &str) -> Vec<u32> {
    let mut offsets: Vec<_> = ["class ", "interface "]
        .iter()
        .flat_map(|keyword| {
            let pattern = format!("{keyword}{name} ");
            source
                .match_indices(&pattern)
                .map(|(offset, _)| u32::try_from(offset + keyword.len()).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    offsets.sort_unstable();
    offsets
}

/// Interfaces of one name form one merge set, and a class joins it whether
/// it comes before or after them. Each set lists its declarations in source
/// order. Only names with two or more such declarations form a set.
#[test]
fn merged_type_declarations_record_interfaces_and_classes_of_one_name() {
    let source = "interface A { a: 1 }\n\
        interface A { b: 2 }\n\
        interface A { c: 3 }\n\
        class B { b = 1 }\n\
        interface B { c: 3 }\n\
        interface C { c: 3 }\n\
        export class C { c = 3 }\n\
        interface D { d: 4 }\n\
        class E { e = 5 }\n";
    assert_eq!(
        merged_type_declaration_offsets(source),
        [
            declaration_offsets(source, "A"),
            declaration_offsets(source, "B"),
            declaration_offsets(source, "C"),
        ]
    );
}

/// A namespace between the declarations of a set is not part of it and
/// doesn't split it, although it merges into the name's binding reference.
#[test]
fn merged_type_declarations_survive_declaration_merging() {
    let source = "interface A { a: 1 }\n\
        namespace A { export const v = 1; }\n\
        class A { b = 2 }\n\
        interface A { c: 3 }\n";
    assert_eq!(
        merged_type_declaration_offsets(source),
        [declaration_offsets(source, "A")]
    );
    let r = biome_js_parser::parse(source, JsFileSource::ts(), JsParserOptions::default());
    let model = semantic_model(&r.tree(), SemanticModelOptions::default());
    assert!(matches!(
        model.global_scope().get_binding_reference("A"),
        Some(crate::TsBindingReference::Merged {
            namespace_ty: Some(_),
            ..
        })
    ));
}

/// Two classes of one name form no merge set, because TypeScript rejects the
/// pair (TS2300). A class expression joins no set because its name is local
/// to it, and declarations of one name in two scopes form no set either.
#[test]
fn merged_type_declarations_leave_out_what_typescript_does_not_merge() {
    let r = biome_js_parser::parse(
        "class A { a = 1 }\n\
         class A { b = 2 }\n",
        JsFileSource::ts(),
        JsParserOptions::default(),
    );
    let model = semantic_model(&r.tree(), SemanticModelOptions::default());
    assert!(model.global_scope().merged_type_declarations().is_empty());

    assert!(
        merged_type_declaration_offsets(
            "interface B { b: 1 }\n\
             const B = class B { c = 3 };\n\
             interface C { c: 1 }\n\
             namespace N { export interface C { d: 2 } }\n"
        )
        .is_empty()
    );
}
