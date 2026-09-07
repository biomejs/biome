use crate::assert_semantics;
use crate::{SemanticFlavor, SemanticModelOptions, semantic_model};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{AnyJsIdentifierReference, JsIdentifierAssignment};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;

// Reads

assert_semantics! {
    ok_reference_read_global,
        "let a/*#A*/ = 1; let b = a/*READ A*/ + 1;",

    ok_reference_read_escaped_binding,
        r#"let \u0065/*#E*/ = 1; e/*READ E*/;"#,

    ok_reference_read_escaped_reference,
        r#"let e/*#E*/ = 1; \u{65}/*READ E*/;"#,

    ok_reference_read_equivalent_escape_forms,
        r#"let \u0065/*#E*/ = 1; \u{65}/*READ E*/;"#,

    ok_reference_does_not_normalize_unicode,
        r#"let \u00e9/*#PRECOMPOSED*/ = 1;
        let e\u0301/*#DECOMPOSED*/ = 2;
        console.log(\u{e9}/*READ PRECOMPOSED*/, e\u{301}/*READ DECOMPOSED*/);"#,

    ok_reference_read_astral_escape,
        r#"let \u{10400}/*#LETTER*/ = 1; \u{010400}/*READ LETTER*/;"#,

    ok_reference_read_join_control_escape,
        r#"let a\u200c/*#JOINED*/ = 1; a\u{200c}/*READ JOINED*/;"#,

    ok_reference_read_inner_scope,
        r#"function f(a/*#A1*/) {
    let b = a/*READ A1*/ + 1;
    console.log(b);
    if (true) {
        let a/*#A2*/ = 2;
        let b = a/*READ A2*/ + 1;
        console.log(b);
    }
    let c = a/*READ A1*/ + 1;
    console.log(b);
}
f(1);"#,

    ok_reference_switch,
        "let b = 1;
        let a/*#A1*/ = 1;
        switch (b) {
            case 1: let a/*#A2*/ = 2; console.log(1, a/*READ A2*/);
            case 2: let c/*#C*/ = 2; console.log(2, a/*READ A2*/, c/*READ C*/);
            case 3: { let d/*#D*/ = 2; console.log(3, a/*READ A2*/, c/*READ C*/, d/*READ D*/); }
            case 4: console.log(4, a/*READ A2*/, c/*READ C*/, d/*?*/);
        }
        console.log(5, a/*READ A1*/);",
    ok_reference_recursive,
        "const fn/*#A*/ = (callback) => { callback(fn/*READ A*/) };",
}

#[test]
fn classifies_global_and_unresolved_references() {
    let parse = biome_js_parser::parse(
        "configured; missing;",
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    let mut options = SemanticModelOptions::default();
    options.globals.insert("configured".into());
    let model = semantic_model(&parse.tree(), options);
    let mut references = parse
        .syntax()
        .descendants()
        .filter_map(AnyJsIdentifierReference::cast);
    let configured = references.next().expect("configured reference");
    let missing = references.next().expect("unresolved reference");

    assert!(model.is_global_reference(&configured));
    assert!(!model.is_unresolved_reference(&configured));
    assert!(model.is_unresolved_reference(&missing));
    assert!(!model.is_global_reference(&missing));
}

#[test]
fn escaped_identifier_scope_lookup_uses_decoded_name() {
    let parse = biome_js_parser::parse(
        r#"let \u0065 = 1;"#,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    let model = semantic_model(&parse.tree(), SemanticModelOptions::default());
    let scope = model.global_scope();

    assert!(scope.get_binding("e").is_some());
    assert!(scope.get_binding(r#"\u{65}"#).is_some());
    assert!(scope.get_binding_reference("e").is_some());
    assert!(scope.get_binding_reference(r#"\u0065"#).is_some());
}

#[test]
fn escaped_identifier_resolves_configured_global() {
    let parse = biome_js_parser::parse(
        r#"\u0065;"#,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    let mut options = SemanticModelOptions::default();
    options.globals.insert("e".into());
    let model = semantic_model(&parse.tree(), options);

    assert_eq!(model.all_unresolved_references().count(), 0);
    assert_eq!(model.all_global_references().count(), 1);
}

#[test]
fn escaped_configured_global_uses_decoded_name() {
    let parse = biome_js_parser::parse("e;", JsFileSource::js_module(), JsParserOptions::default());
    let mut options = SemanticModelOptions::default();
    options.globals.insert(r#"\u0065"#.into());
    let model = semantic_model(&parse.tree(), options);

    assert_eq!(model.all_unresolved_references().count(), 0);
    assert_eq!(model.all_global_references().count(), 1);
}

#[test]
fn escaped_identifier_overloads_share_one_name() {
    let parse = biome_js_parser::parse(
        r#"function \u0066(value: number): number;
        function f(value: string): string;
        function \u{66}(value: number | string): number | string { return value; }"#,
        JsFileSource::ts(),
        JsParserOptions::default(),
    );
    let model = semantic_model(&parse.tree(), SemanticModelOptions::default());
    let scope = model.global_scope();

    assert!(scope.get_binding_reference("f").is_some());
    let overload_sets = scope.overload_sets();
    assert_eq!(overload_sets.len(), 1);
    assert_eq!(overload_sets[0].len(), 3);
}

fn svelte_options() -> SemanticModelOptions {
    SemanticModelOptions {
        flavor: SemanticFlavor::Svelte,
        ..SemanticModelOptions::default()
    }
}

#[test]
fn svelte_store_dereference_with_escape() {
    let parse = biome_js_parser::parse(
        r#"const store = 1; $st\u006fre;"#,
        JsFileSource::ts(),
        JsParserOptions::default(),
    );
    let model = semantic_model(&parse.tree(), svelte_options());

    assert_eq!(model.all_unresolved_references().count(), 0);
}

#[test]
fn svelte_store_binding_with_escape() {
    let parse = biome_js_parser::parse(
        r#"const st\u006fre = 1; $store;"#,
        JsFileSource::ts(),
        JsParserOptions::default(),
    );
    let model = semantic_model(&parse.tree(), svelte_options());

    assert_eq!(model.all_unresolved_references().count(), 0);
}

#[test]
fn escaped_svelte_rune_is_not_a_store_dereference() {
    let parse = biome_js_parser::parse(
        r#"const state = 1; $st\u0061te;"#,
        JsFileSource::ts(),
        JsParserOptions::default(),
    );
    let model = semantic_model(&parse.tree(), svelte_options());

    assert_eq!(model.all_unresolved_references().count(), 1);
}

#[test]
fn svelte_store_assignment_with_escaped_dollar_is_not_a_binding_write() {
    let parse = biome_js_parser::parse(
        r#"const store = 1; \u0024store = 2;"#,
        JsFileSource::ts(),
        JsParserOptions::default(),
    );
    let model = semantic_model(&parse.tree(), svelte_options());

    let store_binding = model
        .global_scope()
        .get_binding("store")
        .expect("expected store binding");
    assert_eq!(store_binding.all_writes().count(), 0);
    assert_eq!(store_binding.all_reads().count(), 1);

    let assignment = parse
        .syntax()
        .descendants()
        .find_map(JsIdentifierAssignment::cast)
        .expect("expected an assignment");
    assert!(model.binding(&assignment).is_none());
}

// Read Hoisting

assert_semantics! {
    ok_hoisting_read_inside_function, "function f() {
    a = 2;
    let b = a/*READ A*/ + 1;
    console.log(a, b);

    var a/*#A*/;
}
f();",
    ok_hoisting_read_var_inside_if, r#"function f() {
    a = 1;
    let b = a/*READ A*/ + 1;
    console.log(a, b);
    if (true) {
        var a/*#A*/;
    }
}
f();"#,
    ok_hoisting_read_redeclaration_before_use, r#"var a/*#A1*/ = 1;
function f() {
    var a/*#A2*/ = 10;
    console.log(a/*READ A2*/);
}
f();"#,

    ok_hoisting_read_redeclaration_after_use, r#"var a/*#A1*/ = 1;
function f() {
    console.log(a/*READ A2*/);
    var a/*#A2*/ = 10;
}
f();"#,

    ok_hoisting_read_for_of, r#"function f() {
    for (var a/*#A*/ of [1,2,3]) {
        console.log(a/*READ A*/)
    }
    console.log(a/*READ A*/);
}
f()"#,

    ok_hoisting_read_for_in, r#"function f() {
    for (var a/*#A*/ in [1,2,3]) {
        console.log(a/*READ A*/)
    }
    console.log(a/*READ A*/);
}
f()"#,

    ok_hoisting_read_let_after_reference_same_scope, r#"var a = 1;
function f() {
    console.log(a/*READ A*/);
    let a/*#A*/ = 2;
}
f()"#,

    ok_hoisting_read_let_after_reference_different_scope, r#"var a/*#A*/ = 1;
function f() {
    console.log(a/*READ A*/);
    if (true) {
        let a = 2;
    }
}
f()"#,

    ok_hoisting_inside_switch,
    "var a/*#A1*/ = 1;
switch (a) {
    case 1: var a/*#A2*/ = 2;
};
console.log(a/*READ A2*/);",
}

// Write

assert_semantics! {
    ok_reference_write_global, "let a/*#A*/; a/*WRITE A*/ = 1;",
    ok_reference_write_escaped_reference,
        r#"let e/*#E*/; \u0065/*WRITE E*/ = 1;"#,
    ok_reference_write_inner_scope, r#"function f(a/*#A1*/) {
    a/*WRITE A1*/ = 1;
    console.log(a);
    if (true) {
        let a/*#A2*/;
        a/*WRITE A2*/ = 2;
        console.log(a);
    }
    a/*WRITE A1*/ = 3;
    console.log(3);
}
f(1);"#,
    ok_reference_write_expression, "let a/*#A*/ = 1; let b = a/*WRITE A*/ = 2;",
    ok_reference_write_object_assignment_pattern,
        "let a/*#A*/, b/*#B*/; ({a/*WRITE A*/, b/*WRITE B*/} = obj);",
}

// Write Hoisting

assert_semantics! {
    ok_hoisting_write_inside_function, "function f() {
    a/*WRITE A*/ = 2;
    console.log(a);
    var a/*#A*/;
}
f();",
}

// Functions
assert_semantics! {
    ok_read_function,
        r#"function f/*#F*/() {} console.log(f/*READ F*/);"#,
    ok_write_function,
        r#"function f/*#F*/() {} f/*WRITE F*/ = null;"#,

    ok_scope_function_expression_read,
        "var f/*#F1*/ = function f/*#F2*/() {console.log(f/*READ F2*/);}; f/*READ F1*/();",
    ok_scope_function_expression_read1 ,
        "var f/*#F1*/ = function () {console.log(f/*READ F1*/);}",
    ok_scope_function_expression_read2,
        "let f/*#F1*/ = 1; let g = function f/*#F2*/() {console.log(2, f/*READ F2*/);}; console.log(f/*READ F1*/);",
    ok_function_parameter,
        "function t({ a/*#A*/ = 0, b/*#B*/ = a/*READ A*/ }, c = a/*READ A*/) { console.log(a/*READ A*/, b/*READ B*/); }",
    ok_function_parameter_array,
        "let b/*#B*/ = 5;
let c/*#C*/ = 6;
let d/*#D*/ = 7;
function f({a/*#A*/} = {a: [b/*READ B*/,c/*READ C*/,d/*READ D*/]}) {
    console.log(a/*READ A*/, b/*READ B*/);
}
f()",
    ok_function_parameter_array_with_name_conflict,
        "let b/*#B1*/ = 5;
let c/*#C*/ = 6;
let d/*#D*/ = 7;
function f({a/*#A*/} = {a: [b/*READ B2*/,c/*READ C*/,d/*READ D*/]}, b/*#B2*/) {
    var b/*#B3*/;
    console.log(a/*READ A*/, b/*READ B3*/);
}
f()",
    ok_function_overloading,
        "function overloaded/*#A*/(): number;
        function overloaded/*#B*/(s: string): string;
        function overloaded/*#C*/(s?: string) {
            return s;
        }
        overloaded/*READ C*/();",
    ok_function_overloading_2,
        "function a/*#A*/() {}
        a/*READ A*/();
        function add(a: string, b: string): string;
        console.log(a/*READ A*/);",
}

// Imports
assert_semantics! {
    ok_import_used_in_jsx, r#"import A/*#A*/ from 'a.js'; console.log(<A/*READ A*//>);"#,
}

assert_semantics! {
    ok_unresolved_reference, r#"a/*?*/"#,
    ok_unresolved_function_expression_read,"let f/*#F*/ = function g/*#G*/(){}; g/*?*/();",
    ok_unresolved_reference_arguments,
        r#"function f() {
            console.log(arguments/*?*/);

            for(let i = 0;i < arguments/*?*/.length; ++i) {
                console.log(arguments/*?*/[i]);
            }
        }"#,
}

// Exports
assert_semantics! {
    ok_export_hoisted_variable,
        "var a/*#A1*/ = 2; export {a/*READ A2*/}; var a/*#A2*/ = 1;",
}

// Classes
assert_semantics! {
    ok_class_reference,
        "class A/*#A*/ {} new A/*READ A*/();",
    ok_class_expression_1,
        "const A/*#A*/ = class B/*#B*/ {}; console.log(A/*READ A*/, B/*?*/);",
    //https://github.com/rome/tools/issues/3779
    ok_class_expression_2,
        "const A/*#A1*/ = print(class A/*#A2*/ {}); console.log(A/*READ A1*/);",
    ok_class_static_init,
        "class C { static { () => a/*READ A*/; let a/*#A*/ = 1; } };",
}

// Static Initialization Block
assert_semantics! {
    ok_reference_static_initialization_block,
        "const a/*#A1*/ = 1;
        console.log(a/*READ A1*/);

        class A {
            static {
                console.log(a/*READ A2*/);
                const a/*#A2*/ = 2;
                console.log(a/*READ A2*/);
            }
        };

        console.log(a/*READ A1*/);",
}

// Typescript types
assert_semantics! {
    ok_typescript_function_type,
        "function f (a/*#A1*/, b: (a/*#A2*/) => any) { return b(a/*READ A1*/); };",
    ok_typescript_type_parameter_name,
        "type A = { [key/*#A1*/ in P]: key/*READ A1*/ }",
    ok_typescript_escaped_type_name,
        r#"type \u0054/*#T*/ = string; let value: T/*READ T*/;"#,
    ok_typescript_escaped_infer_name,
        r#"type Element<T> = T extends Array<infer \u0055/*#U*/> ? U/*READ U*/ : never;"#,
    ok_typescript_escaped_enum_member_name,
        r#"enum E { \u0041/*#A*/ = 1, B = A/*READ A*/ }"#,
}
