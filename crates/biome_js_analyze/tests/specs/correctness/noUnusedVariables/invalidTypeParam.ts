export function f<T>() {}
export class C<T> {}
export type Alias<T> = number
export type Mapped<T> = { [K in keyof T]: number }
export type Inferred<T> = T extends (infer I)[] ? number : never;