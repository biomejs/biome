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
 * NOTE: For the following cases: Show diagnostic but don't suggest a fix.
 */

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
	}
	// Comment
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
	}
	// Comment
}
