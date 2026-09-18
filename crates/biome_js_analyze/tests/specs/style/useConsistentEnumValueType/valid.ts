/* should not generate diagnostics */

enum Valid1 {
	Unknown = 0,
	Closed = 1,
	Open = 2,
}

enum Valid2 {
	Unknown,
	Closed,
	Open,
}

enum Valid3 {
	Unknown = 'unknown',
	Closed = 'closed',
	Open = 'open',
}

function getValidValue() {
	return 0
}

enum Valid4 {
	Unknown = getValidValue(),
	Closed = 1,
	Open = getValidValue(),
}
