export function f<T>() {}
export class C<T> {}
export type Alias<T> = number
export type Inferred<T> = T extends (infer I)[] ? number : never;
export type TestUnionType<T> = T extends (infer B)[] | infer B ? number : never;