/* should not generate diagnostics */
// The decorators of an isolated method run outside of it.
const dec = (value: unknown) => () => {};
const outer = 1;
class Decorated {
	/** @isolated */
	@dec(outer)
	method() {
		return 1;
	}

	/** @isolated */
	@dec(outer)
	static other(x: number) {
		return x;
	}
}
