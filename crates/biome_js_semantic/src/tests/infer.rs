use crate::assert_semantics;

assert_semantics! {
    ok_conditional_true_type_scope, "type A<T> = T extends string ? (/*START Scope*/number)/*END Scope*/: boolean;",
    ok_conditional_true_type_infer_simple, "type A<T> = T extends infer /*@ Scope */T ? (/*START Scope*/number)/*END Scope*/: boolean;",
    ok_conditional_true_type_infer_function, "type A<T> = T extends { getState: () => infer /*@ Scope */ T } ? (/*START Scope*/number)/*END Scope*/: boolean;",
    ok_conditional_true_type_infer_nested, "type A = MyType extends (OtherType extends infer /*@ InnerScope */T ? (/*START InnerScope*/infer /*@ OuterScope */ U)/*END InnerScope*/ : InnerFalse) ? (/*START OuterScope*/OuterTrue)/*END OuterScope*/ : OuterFalse;",
}
