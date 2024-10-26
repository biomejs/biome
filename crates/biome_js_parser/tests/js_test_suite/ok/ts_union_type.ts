type A = string | number;
type B = | A | void | null;
type C = A & C | C;
