type A = { (): string; }
type B = { (a, b, c): number }
type C = { <A, B>(a: A, b: B): number }
type D = { <const A>(a: A): number }
