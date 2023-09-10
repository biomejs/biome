declare function test<A, B, R>(a: A, b: B): R;
declare function test2({ a }?: { a: "string" })
declare
function not_a_declaration() {}
