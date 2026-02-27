/* should not generate diagnostics */
// Issue https://github.com/biomejs/biome/issues/2659
type Test<T> = T extends Array<infer U> ? true : false

type TestMultipleInfer<T extends readonly string[]> = T[number] extends
  | `-${infer Base}`
  | infer Base
  ? Base
  : never
