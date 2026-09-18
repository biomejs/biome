// Regression test for https://github.com/biomejs/biome/issues/10131
// A curried arrow in a ternary consequent where the inner arrow's parameters
// start with a destructuring pattern: `({...}: Type) =>`.
// Previously the parser emitted errors because `in_conditional_consequent`
// suppressed speculative arrow parsing inside the outer arrow's body.
const handleClick = onClick
	? (offset: number) =>
			({ photo, index, event }: ClickHandlerProps<TPhoto>) => {
				onClick({ photos: photosArray, index: offset + index, photo, event });
			}
	: undefined;

// Variant: array destructuring parameter
const handler2 = enabled
	? (n: number) =>
			([a, b]: [string, string]) => {
				use(n, a, b);
			}
	: undefined;

// Existing behaviour must be preserved: object body (not nested arrow params)
const slotFn = isFirstMount
	? (i: number) => ({ [CONTENT_SLOT]: i })
	: (i: number) => wrapSlotExpr(newExprs[i]);
