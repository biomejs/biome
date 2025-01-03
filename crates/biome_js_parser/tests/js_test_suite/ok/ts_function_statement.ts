function test(a: string, b?: number, c="default") {}
function test2<A, B extends A, C = A>(a: A, b: B, c: C) {}
