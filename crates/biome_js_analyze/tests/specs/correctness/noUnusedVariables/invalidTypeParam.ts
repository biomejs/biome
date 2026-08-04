export function f<T>() {}
export class C<T> {}
export type Alias<T> = number
export type Inferred<T> = T extends (infer I)[] ? number : never;
export type TestUnionType<T> = T extends (infer B)[] | infer B ? number : never;
// Only function/method signatures with a body-bearing implementation are exempt;
// interface members and abstract methods, which never get one, still report.
// See https://github.com/biomejs/biome/issues/11214
export interface I {
	m<T>(): void;
}
export abstract class AbstractClass {
	abstract method<T>(): void;
}