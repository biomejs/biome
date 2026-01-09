/* should not generate diagnostics */
enum One {
	Unknown = 0,
	Closed = 1,
	Open = 2,
}

enum Two {
	Unknown,
	Closed,
	Open,
}

enum Three {
	Unknown = 'unknown',
	Closed = 'closed',
	Open = 'open',
}
