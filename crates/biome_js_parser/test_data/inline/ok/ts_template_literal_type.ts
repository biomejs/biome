type A = `abcd`
type B = `a${A}`
type C<X extends string> = `c${X}`
