// should not generate diagnostics
function copyParamValueToLocalProperty(input) {
	const local = { a: 0, b: 0 };
	local.a = input;
	local["b"] = input;
}
