// should not generate diagnostics

function f(visitor) {
	let ctrl = visitor();
	for (const x of [0]) ctrl = ctrl();
}
