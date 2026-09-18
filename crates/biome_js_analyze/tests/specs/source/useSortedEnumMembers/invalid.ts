/* should generate diagnostics */
enum InvalidStatus1 {
	InProgress = 'In Progress',
	Completed = 'Completed',
	/**
	 * JSDoc
	 */
	OnHold = 'On Hold',
	Cancelled = 'Cancelled', // Some inline comment
	// Pre comment
	NotStarted = 'Not Started',
}

enum InvalidStatus2 {
	Status1 = 'First',
	Status10 = 'Tenth',
	Status2 = 'Second',
}
