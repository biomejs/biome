/* should generate diagnostics */
// The fix isn't offered because `Object` doesn't refer to the global object.
function withShadowedObject(Object) {
	return b.map(({ x, y }) => ({ ...x, y }));
}
