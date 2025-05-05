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

if (condition) // Comment
	if (anotherCondition)
		doSomething();

// Inner one is `JsBlockStatement`
if (condition) if (anotherCondition) {
	// ...
}

// Outer one is `JsBlockStatement`
if (condition) {
	if (anotherCondition) doSomething();
}

// No `JsBlockStatement`
if (condition) if (anotherCondition) doSomething();

// `JsEmptyStatement`
if (condition) if (anotherCondition);

// Nested
if (a) {
	if (b) {
		// ...
	}
} else if (c) {
	if (d) {
		// ...
	}
}

// Need parenthesis
function* foo() {
	if (a || b)
		if (a ?? b)
			if (a ? b : c)
				if (a = b)
					if (a += b)
						if (a -= b)
							if (a &&= b)
								if (yield a)
									if (a, b);
}

// Should not add parenthesis
async function foo() {
	if (a)
		if (await a)
			if (a.b)
				if (a && b);
}

// Don't case parenthesis in outer test
if (((a || b))) if (((c || d)));

// Semicolon
if (a)
	if (b) foo()
	;[].forEach(bar)

if (a) {
	if (b) foo()
}
;[].forEach(bar)

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

if (condition)
	if (anotherCondition) // Comment
		doSomething();

// Semicolon
if (a) {
	if (b) foo()
}
[].forEach(bar)
