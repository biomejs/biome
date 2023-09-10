use crate::assert_semantics;

// Reads

assert_semantics! {
    ok_reference_read_global,
        "let a/*#A*/ = 1; let b = a/*READ A*/ + 1;",

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
    ok_read_self_invoking_function,
        r#"(function f/*#F*/(){console.log(1)})/*READ F*/()"#,
    ok_read_self_invoking_function2,
        r#"(1,2,3,function f/*#F*/(){console.log(1)})/*READ F*/()"#,

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
}
