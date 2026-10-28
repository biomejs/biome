/* should not generate diagnostics */
function asserted(value: Value): Value {
	return asserted(value.next) as Value;
}

function nonNull(value: Value): Value {
	return nonNull(value.next)!;
}

function satisfied(value: Value): Value {
	return satisfied(value.next) satisfies Value;
}
