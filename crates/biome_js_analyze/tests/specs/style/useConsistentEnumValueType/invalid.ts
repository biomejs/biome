/* should generate diagnostics */
enum Invalid1 {
	Unknown,
	Closed = 1,
	Open = 'open',
}

function getInvalidValue() {
	return 0
}

enum Invalid2 {
	Unknown = getInvalidValue(),
	Closed = "closed",
	Open = getInvalidValue(),
}
