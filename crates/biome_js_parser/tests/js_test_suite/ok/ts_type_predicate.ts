type A = (a) => a is string;
type B = (a) => asserts a is string;
type C = (a) => asserts a;
type asserts = string;
type D = () => asserts;
