/* should not generate diagnostics */
enum ValidStatus1 {
	InProgress,
	Completed,
	OnHold,
	Cancelled,
	NotStarted,
}

enum ValidStatus2 {
	InProgress = 0,
	Completed = 1,
	OnHold = 2,
	Cancelled = 3,
	NotStarted = 4,
}

enum ValidStatus3 {
	InProgress,
	Completed,
	OnHold = 'On Hold',
	Cancelled = 'Cancelled',
	NotStarted = 'Not Started',
}

enum ValidStatus4 {
	Cancelled = 'Cancelled',
	Completed = 'Completed',
	InProgress = 'In Progress',
	NotStarted = 'Not Started',
	OnHold = 'On Hold',
}

enum ValidStatus5 {
	Status1 = 'First',
	Status2 = 'Second',
	Status10 = 'Tenth',
}
