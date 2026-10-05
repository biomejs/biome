// should not generate diagnostics

interface Value {}
class ValueClass {}

function objectShape(value: {} | null) {
	if (value == null) return;
	if (value) {}
}

function interfaceShape(value: Value | null) {
	if (value == null) return;
	if (value) {}
}

function classShape(value: ValueClass | null) {
	if (value == null) return;
	if (value) {}
}
