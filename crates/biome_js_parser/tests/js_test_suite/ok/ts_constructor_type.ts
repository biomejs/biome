type A = new(a: string, b: number) => string;
type B = abstract new(a: string, b: number) => string;
type C = new<A, B>(a: A, b: B) => string;
type D = abstract new<A, B>(a: A, b: B) => string;
