type A = A extends infer B ? B : never;
type B = A extends { a: infer U; b: infer U} ? U : never;
