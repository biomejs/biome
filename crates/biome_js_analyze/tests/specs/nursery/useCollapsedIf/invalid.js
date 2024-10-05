/**
 * Safe fixes:
 */

if (condition) {
	if (anotherCondition) {
		// ...
	}
}

if (condition) {
	if (anotherCondition) {
		// ...
	}
} else {
	// ...
}

/**
 * Suggested fixes:
 */

if (condition) { // Comment
	if (anotherCondition) {
		// ...
	}
}

if (condition) {
	// Comment
	if (anotherCondition) {
		// ...
	}
}

if (condition) {
	if (anotherCondition) {
		// ...
	} // Comment
}

if (condition) {
	if (anotherCondition) {
		// ...
	}
	// Comment
}

if (condition) { // Comment
	if (anotherCondition) {
		// ...
	}
} else {
	// ...
}

if (condition) {
	// Comment
	if (anotherCondition) {
		// ...
	}
} else {
	// ...
}

if (condition) {
	if (anotherCondition) {
		// ...
	} // Comment
} else {
	// ...
}

if (condition) {
	if (anotherCondition) {
		// ...
	}
	// Comment
} else {
	// ...
}
