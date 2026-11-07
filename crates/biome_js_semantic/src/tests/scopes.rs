use crate::{SemanticFlavor, SemanticModelOptions, assert_semantics, semantic_model};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::JsSyntaxKind;
use biome_languages::JsFileSource;
use biome_languages::javascript::{JsEmbeddingKind, SvelteEmbeddingKind, SvelteFileKind};

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

#[test]
fn svelte_declaration_and_legacy_const_roots_have_scopes() {
    for (embedding_kind, code, root_kind) in [
        (
            SvelteEmbeddingKind::Declaration,
            "const value: number = (() => 1)()",
            JsSyntaxKind::JS_SVELTE_DECLARATION_ROOT,
        ),
        (
            SvelteEmbeddingKind::LegacyConst,
            "value: number = (() => 1)()",
            JsSyntaxKind::JS_SVELTE_LEGACY_CONST_ROOT,
        ),
    ] {
        let source = JsFileSource::ts().with_embedding_kind(JsEmbeddingKind::Svelte {
            file_kind: SvelteFileKind::Component,
            embedding_kind,
            is_class_attribute: false,
            is_module_script: false,
        });
        let parsed = biome_js_parser::parse(code, source, JsParserOptions::default());
        assert!(!parsed.has_errors());
        assert_eq!(parsed.syntax().kind(), root_kind);

        let model = semantic_model(
            &parsed.tree(),
            SemanticModelOptions {
                flavor: SemanticFlavor::Svelte,
                ..SemanticModelOptions::default()
            },
        );
        let scope = model.global_scope();
        assert_eq!(scope.syntax().kind(), root_kind);
        assert!(scope.get_binding("value").is_some());
        let closure = scope.children().next().unwrap();
        assert_eq!(
            closure.syntax().kind(),
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
        );
        assert_eq!(closure.parent(), Some(scope));
    }
}
