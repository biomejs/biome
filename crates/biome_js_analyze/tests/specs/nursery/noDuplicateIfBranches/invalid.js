/* should generate diagnostics */
if (isAdmin) {
	showDashboard();
	loadStats();
} else {
	showDashboard();
	loadStats();
}

if (foo) {
	bar();
} else if (baz) {
	bar();
}

if (foo) {
	bar();
} else if (baz) {
	bar();
} else {
	bar();
}

if (foo) {
	bar();
} else if (baz) {
	qux();
} else {
	qux();
}

function returns() {
	if (foo)
		return bar();
	else
		return bar();
}

if (foo)
	bar();
else {
	bar();
}

if (foo)
	bar();
else
	bar()

if (foo) {
	// Comment does not make the branch different.
	bar();
} else {
	bar();
}

if (foo) {
	bar(
		baz
	);
} else {
	bar(baz);
}

function semicolons() {
	if (foo) {
		return bar();
	} else {
		return bar()
	}
}

function doWhile() {
	if (foo) {
		do {
			bar();
		} while (baz);
	} else {
		do {
			bar();
		} while (baz)
	}
}

// Empty statements are ignored.
if (foo) {
	bar();
	;
} else {
	bar();
}

if (foo) {
	const value = 'unicorn';
} else {
	const value = "unicorn";
}

if (foo) {
	callback = () => {
		return 42;
	};
} else {
	callback = () => {
		return 42
	};
}

function nested() {
	if (foo) {
		if (bar) {
			baz();
		} else {
			baz();
		}
	}
}

if (foo) {
	for (let i = 0; i < n; i++) step();
} else {
	for (let i = 0; i < n; i++) step()
}
