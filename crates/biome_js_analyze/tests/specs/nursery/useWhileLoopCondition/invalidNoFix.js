/* should generate diagnostics */
while (/* keep */ true) {
	if (!hasMore()) {
		break;
	}
}

for (;;) {
	if (
		// Check before processing.
		!hasMore()
	) {
		break;
	}
}

for /* keep */ (;;) {
	if (!hasMore()) {
		break;
	}
}

for (; /* keep */ true;) {
	if (!hasMore()) {
		break;
	}
}

do {
	if (!hasMore()) {
		break;
	}
} /* keep */ while (true);

do /* keep */ {
	if (!hasMore()) {
		break;
	}
} while (true);

while (true) {
	if (!hasMore()) {
		// Preserve this comment.
		break;
	}
}

while (true) {
	// Preserve this comment.
	if (!hasMore()) {
		break;
	}

	processNext();
}

while (true) {
	if (!hasMore()) break; // Preserve this comment.
	processNext();
}

while (true) {
	if (!hasMore()) {
		break;
	} // Preserve this comment.
	processNext();
}

while (true) { // Preserve this comment.
	if (!hasMore()) {
		break;
	}
	processNext();
}
