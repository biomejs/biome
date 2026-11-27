/* should generate diagnostics */
const value = 1;
enum Color {
	Red,
}

makeSynchronous((): number => value);
makeSynchronous(() => Color.Red);
makeSynchronous(() => (value as number) + 1);
makeSynchronous((() => value) as () => number);

// Decorators inside an isolated method run inside it.
const dec = (value: unknown) => () => {};
class Decorated {
	/** @isolated */
	method() {
		@dec(value)
		class Inner {
			@dec(value)
			field = 1;
		}
		return Inner;
	}
}
