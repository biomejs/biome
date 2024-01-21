type A = { a?: string; b?(): number }
type B = { a: string, b(): number }
type C = { m(a: string, b: number, c: string): any }
type D = { readonly: string, readonly a: number }
type E = { m<A, B>(a: A, b: B): never }
type F = { m<const A>(a: A): never }
