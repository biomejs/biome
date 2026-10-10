/* should generate diagnostics */
function typeAnnotation() {
	let value: string = "unused";
	value = "used";
	console.log(value);
}

function typeAssertionTarget() {
	let value: unknown = "used";
	console.log(value);
	(value as string) = "unused";
	value! = "unused";
}
