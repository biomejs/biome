use crate::{SemanticModel, assert_semantics, semantic_model};
use biome_js_parser::parse;
use biome_js_syntax::AnyJsRoot;
use biome_languages::JsFileSource;

// Statements
assert_semantics! {
    ok_scope_if, "if(true) {/*START A*/}/*END A*/",
    ok_scope_if_no_block, "if(true) ;/*NOEVENT*/;",
    ok_scope_if_without_block_else_with_block, "if(true) ;/*NOEVENT*/ else {/*START A*/}/*END A*/;",
    ok_scope_if_without_block_else_without_block, "if(true) ;/*NOEVENT*/ else ;/*NOEVENT*/;",
    ok_scope_for_with_block, ";for/*START A*/(;;) {/*START B*/}/*END A*//*END B*/;",
    ok_scope_for_without_block, "for/*START A*/(;;) ;/*END A*//*UNIQUE*/;",
    ok_scope_for_of, "for/*START A*/(const a of []) {/*START B*/}/*END A*//*END B*/;",
    ok_scope_for_of_without_block, "for/*START A*/(const a of []) ;/*END A*//*UNIQUE*/;",
    ok_scope_for_in, "for/*START A*/(const a in []) {/*START B*/}/*END A*//*END B*/;",
    ok_scope_for_in_without_block, "for/*START A*/(const a in []) ;/*END A*//*UNIQUE*/;",
    ok_scope_try_catch, "try {/*START A*/}/*END A*/ catch/*START B*/ (e) {}/*END B*/",
    ok_scope_try_catch_finally, "try {/*START A*/}/*END A*/ catch/*START B1*/ (e) {/*START B2*/}/*END B1*//*END B2*/ finally {/*START C*/}/*END C*/",
}

// Functions
assert_semantics! {
    ok_scope_function, ";function/*START A*/ f() {}/*END A*/",
    ok_scope_function_with_export_default, ";export default function/*START A*/ f() {}/*END A*/",
    ok_scope_function_expression, ";var a = function/*START A*/ f() {}/*END A*/",
    ok_scope_arrow_function, ";(/*START A*/) => {}/*END A*/",
    ok_scope_js_function_export_default_declaration, "export default function/*START A*/ () {}/*END A*/",
    ok_scope_overloaded_functions, "
        function/*START A*/ add(a:string, b:string):string;/*END A*/
        function/*START B*/ add(a:number, b:number):number;/*END B*/
        function/*START C*/ add(a: any, b:any): any {
            return a + b;
        }/*END C*/;",
}

// Classes
assert_semantics! {
    ok_scope_class_constructor, ";class A { constructor/*START A*/ () {}/*END A*/ }",
    ok_scope_class_getter, ";class A { get/*START A*/ name() {}/*END A*/ }",
    ok_scope_class_setter, ";class A { set/*START A*/ name(v) {}/*END A*/ }",
}

// Static Initialization Block
assert_semantics! {
    ok_scope_static_initialization_block,
        "class A {
            static/*START A*/ {
                const a/*@ A*/ = 2;
            }/*END A*/
        };",
}

// Type parameters
assert_semantics! {
    ok_type_parameter, "export type /*START A*/ EventHandler<Event /*# Event */ /*@ A */ extends string> = `on${ Event /*READ Event */ }` /*END A*/",
    ok_type_parameter_with_default, "export type /*START A */ EventHandler<Event /*# Event */ /*@ A */ extends string = 'click'> = `on${ Event /*READ Event */  }` /*END A*/",
    ok_type_parameter_multiple_declaration, "
        export type /*START ScopeA */ EventHandler<Event /*# EventA */ /*@ ScopeA */ extends string> = `on${ Event /*READ EventA */ }`; /*END ScopeA */
        export type /*START ScopeB */ EventHandlerDefault<Event /*# EventB */ /*@ ScopeB */ extends string = 'click'> = `on${ Event /*READ EventB */  }`; /*END ScopeB */
    ",
    ok_type_parameter_interface, "
        export interface /*START A*/ EventHandler<Event /*# Event */ /*@ A */ extends string> {
            on: (event: Event /*READ Event */, data: unknown) => void;
        } /*END A*/
    ",
}

// Modules
assert_semantics! {
    ok_scope_module, "module/*START M*/ M {}/*END M*/;",
}

// Others
assert_semantics! {
    ok_scope_global, "/*START GLOBAL*//*END GLOBAL*/",
}

fn build_ts_model(source: &str) -> SemanticModel {
    let parsed = parse(source, JsFileSource::ts(), Default::default());
    let parsed: AnyJsRoot = parsed.tree();
    semantic_model(&parsed, Default::default())
}

const STORED_DATA_SOURCE: &str = "
    let a;
    var \\u0062;
    function f<T>(c) {
        if (c) { var d; function g() {} }
        const h = () => { let i; };
    }
    class C { static { var j; } method(k) {} }
    namespace N { export const l = 1; }
    type U<V> = V extends (infer W)[] ? W : never;
    enum E { M }
";

#[test]
fn binding_scope_hoisted_to_matches_syntax_lookup() {
    let model = build_ts_model(STORED_DATA_SOURCE);
    let mut hoisted = 0;
    for binding in model.all_bindings() {
        let expected = model.scope_hoisted_to(&binding.syntax());
        hoisted += usize::from(expected.is_some());
        assert_eq!(
            binding.scope_hoisted_to(),
            expected,
            "{:?}",
            binding.syntax()
        );
    }
    assert!(hoisted > 0);
}

#[test]
fn scope_syntax_kind_matches_syntax() {
    let model = build_ts_model(STORED_DATA_SOURCE);
    for scope in model.global_scope().descendents() {
        assert_eq!(scope.syntax_kind(), scope.syntax().kind());
    }
}

#[test]
fn scope_binding_names_are_distinct_and_decoded() {
    let model = build_ts_model("let a; let \\u0061; var b; function c(d) {}");
    let global_scope = model.global_scope();
    let mut names: Vec<_> = global_scope.binding_names().collect();
    names.sort_unstable();
    assert_eq!(names, ["a", "b", "c"]);
    assert_eq!(global_scope.bindings().len(), 4);
}
