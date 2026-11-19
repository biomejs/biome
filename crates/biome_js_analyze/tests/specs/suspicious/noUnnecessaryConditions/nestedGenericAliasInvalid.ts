interface Box<U> {
	value: U;
}
interface Wrapper<X> {
	inner: X;
}

type Nested<T> = Box<Wrapper<T>>;
declare const nested: Nested<number>;
export const inner = nested.value.inner ?? 1;
