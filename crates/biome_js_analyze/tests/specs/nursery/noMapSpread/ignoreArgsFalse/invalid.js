/* should generate diagnostics */
function fn(a) {
	return a.map((x) => ({ ...(x ?? y) }));
}
const arrow = (a) => a.map((x) => ({ ...(x ?? y) }));
function destructured({ items }) {
	return items.map((x) => ({ ...x }));
}
const unparenthesized = a => a.map((x) => ({ ...x }));
