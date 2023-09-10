type A = number;
type B = string extends number ? string : number;
type C = A extends (B extends A ? number : string) ? void : number;
type D<T> = T extends [infer S extends string, ...unknown[]] ? S : never;
type E<U, T> = T extends (infer U extends number ? U : T ) ? U : T
type F<T> = T extends { [P in infer U extends keyof T ? 1 : 0]: 1; } ? 1 : 0;
type G<T> = T extends [unknown, infer S extends string] ? S : never;
type H = A extends () => B extends C ? D : E ? F : G;
type J<T> = T extends ((...a: any[]) => infer R extends string) ? R : never;
