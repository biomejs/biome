// Regression test for https://github.com/biomejs/biome/issues/10131 (JavaScript variant)
// Curried arrow in a ternary consequent where the inner arrow's parameters
// start with a destructuring pattern.
const handleClick = onClick
	? offset =>
			({ photo, index, event }) => {
				onClick(offset);
			}
	: undefined;

// Variant: array destructuring parameter
const handler2 = enabled
	? n =>
			([a, b]) => {
				use(n, a, b);
			}
	: undefined;

// Existing behaviour must be preserved: object body (not nested arrow params)
const slotFn = isFirstMount
	? i => ({ [CONTENT_SLOT]: i })
	: i => wrapSlotExpr(newExprs[i]);
