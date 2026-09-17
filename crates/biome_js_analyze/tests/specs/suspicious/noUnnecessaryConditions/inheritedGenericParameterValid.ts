// should not generate diagnostics
interface Base<T> {
	value: T;
}
interface Derived<T> extends Base<null> {}

declare const derived: Derived<number>;
export const result = derived.value ?? 1;
