/* should generate diagnostics */
function typed(bar: Bar): Bar {
	return typed(bar.baz as Bar);
}

function generic<T>(value: T): T {
	return generic<T>(value);
}
