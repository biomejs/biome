/* should generate diagnostics */

(() => {
	const value = getValue() satisfies number;
	run(value);
})();

((): void => {
	run();
})();

(function (): void {
	run();
})();

(() => {
	enum Direction {
		Up,
		Down,
	}
	type Value = Direction;
	interface Options {
		value: Value;
	}
	run(Direction.Up satisfies Value);
})();
