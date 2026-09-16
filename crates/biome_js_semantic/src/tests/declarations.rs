use crate::{SemanticModelOptions, assert_semantics, semantic_model};
use biome_js_parser::JsParserOptions;
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
