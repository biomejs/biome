type A = { [a: number]: string }
type B = { readonly [a: number]: string }
// not an index signature
type C = { [a]: string }
type D = { readonly [a]: string }
