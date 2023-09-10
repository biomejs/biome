type A = (a) => a is string;
type B = { test(a): a is string }
type C = { (a): a is string }
const a = { test(x): x is string { return typeof x === "string" } }
class D { test(x): x is string { return typeof x === "string"; } }
