/**
 * Safe fixes:
 */

if (condition) {
	// ...
} else {
	if (anotherCondition) {
		// ...
	}
}

if (condition) {
	// ...
} else {
	if (anotherCondition) {
		// ...
	} else {
		// ...
	}
}

/**
 * Suggested fixes:
 */

if (condition) {
	// ...
} else { // Comment
	if (anotherCondition) {
		// ...
	}
}

if (condition) {
	// ...
} else {
	// Comment
	if (anotherCondition) {
		// ...
	}
}

if (condition) {
	// ...
} else {
	if (anotherCondition) {
		// ...
	} // Comment
}

if (condition) {
	// ...
} else {
	if (anotherCondition) {
		// ...
	}
	// Comment
}

if (condition) {
	// ...
} else { // Comment
	if (anotherCondition) {
		// ...
	} else {
		// ...
	}
}

if (condition) {
	// ...
} else {
	// Comment
	if (anotherCondition) {
		// ...
	} else {
		// ...
	}
}

if (condition) {
	// ...
} else {
	if (anotherCondition) {
		// ...
	} else {
		// ...
	} // Comment
}

if (condition) {
	// ...
} else {
	if (anotherCondition) {
		// ...
	} else {
		// ...
	}
	// Comment
}
