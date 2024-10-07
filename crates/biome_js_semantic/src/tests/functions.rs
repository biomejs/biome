use crate::assert_semantics;

// functions

assert_semantics! {
    ok_function_declaration, "function f/*#F*/ () {}",
    ok_function_call, "function f/*#F*/ () {} f/*READ F*/();",
    ok_function_hoisted_call, "function f/*#F*/ () { g/*READ G*/(); } function g/*#G*/() {}",
    ok_function_inner_function,
        "function b/*#B1*/() { function b/*#B2*/() {console.log(2)}; console.log(1); b/*READ B2*/(); } b/*READ B1*/();",
    ok_function_inner_function2,
        "function b/*#B1*/(a=b/*READ B1*/()) {
            function b/*#B2*/() {console.log(2)}
            console.log(a);  
          }
          b(1);
        ",
}

// modules

assert_semantics! {
    ok_function_inside_module,
      "declare module M {
        function f/*#F*/();
      }
      console.log(f/*?*/());",
}
