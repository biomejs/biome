/* should generate diagnostics */
while (true) {
	if (!hasMore()) {
		break;
	}

	processNext();
}

while (true) {
	if (!hasMore()) break;
	processNext();
}

while (true) {
	if (done) {
		break;
	}

	processNext();
}

while (true) {
	if (done || cancelled) {
		break;
	}

	processNext();
}

while ((true)) {
	if ((!(a && b))) {
		break;
	}
	processNext();
}

while (true) {
	if (!!value) {
		break;
	}
	processNext();
}

while (true) {
	if (a === b) {
		break;
	}
	processNext();
}

while (true) {
	if (() => {}) {
		break;
	}
}

while (true) { if (done) break; processNext(); }

for (;;) {
	if (!hasMore()) {
		break;
	}

	processNext();
}

for (; true;) {
	if (!hasMore()) break;
	processNext();
}

for (;;) {
	if (!hasMore()) {
		break;
	}

	if (skip) {
		continue;
	}

	processNext();
}

do {
	if (!hasMore()) {
		break;
	}

	processNext();
} while (true);

do {
	if (!hasMore()) {
		break;
	}

	if (skip) {
		continue;
	}

	processNext();
} while (true);

do {
	if (!hasMore()) break;
	processNext();
} while (true)

do {
	if (done || cancelled) {
		break;
	}

	processNext();
} while (true);

// `break` statements of nested loops, `switch` statements, and functions don't stop the loop.
while (true) {
	if (!hasMore()) {
		break;
	}

	for (const item of items) {
		if (done(item)) {
			break;
		}
	}
}

while (true) {
	if (!hasMore()) {
		break;
	}

	switch (value) {
		case 1:
			break;
	}
}

while (true) {
	if (!hasMore()) {
		break;
	}

	function stop() {
		while (done) {
			break;
		}
	}
}

while (true) {
	if (!hasMore()) {
		break;
	}

	const stop = () => {
		for (;;) {
			break;
		}
	};
}

outer: for (const item of items) {
	while (true) {
		if (!hasMore()) {
			break;
		}
		if (skip) {
			break outer;
		}
	}
}

// A `var` declared in the body is the same variable outside of it.
while (true) {
	if (done) {
		break;
	}

	var done = true;
	work();
}

// Variables declared inside the condition itself don't matter.
while (true) {
	if (((value) => value)(input)) {
		break;
	}
	work();
}

while (true) {
	if (!hasMore()) {
		break;
	}
}

for (;;) {
	if (!hasMore()) {
		break;
	}
}

for (; true;) {
	if (!hasMore()) {
		break;
	}
}

do {
	if (!hasMore()) {
		break;
	}
} while (true);

async function* generator() {
	while (true) {
		if (!(await hasMore())) {
			break;
		}
		yield next();
	}
}

// Comments before the loop, after the loop, and in the rest of the body are kept.
// Before the loop.
while (true) {
	if (!hasMore()) {
		break;
	}

	// Process the remaining work.
	processNext();
} // After the loop.

do {
	if (!hasMore()) {
		break;
	}
	processNext(); // Trailing comment.
} while (true); // Preserve this comment.

while (true) {
	if (!hasMore()) {
		break;
	}
	// Only a comment is left.
}
