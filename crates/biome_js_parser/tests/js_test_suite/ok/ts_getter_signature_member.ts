type A = { get a(): number }
type B = { get a() }
// members that look like getters but aren't getters
type C = { get(): number }
type D = { get: number }
type E = { get }
