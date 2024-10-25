if (condition && anotherCondition) {
	// ...
}

if (condition) {
  if (anotherCondition) {
		// ...
  }
  doSomething();
}

if (condition) {
	if (anotherCondition) {
		// ...
	} else {
		// ...
	}
}

if (condition) {
	if (anotherCondition) {
		// ...
	}
	doSomething();
} else {
	// ...
}

if (condition) {
	anotherCondition ? c() : d()
}

// Covered by `useCollapsedElseIf`
if (condition) {
	// ...
} else {
	if (anotherCondition) {
		// ...
	}
}

// Ignore `if` with an `else` clause
if (condition) { // Comment
	if (anotherCondition) {
		// ...
	}
} else {
	// ...
}
if (condition) { // Comment
	if (anotherCondition) {
		// ...
	}
} else if(condition) {
	// ...
}
