// Invalid: variable declared in if block but used outside
function doIf() {
	if (true) {
		var build = true;
	}

	console.log(build);
}

// Invalid: variable declared in if/else blocks
function doIfElse() {
	if (true) {
		var build = true;
	} else {
		var build = false;
	}
}

// Invalid: variable declared in try block but used in catch
function doTryCatch() {
	try {
		var build = 1;
	} catch (e) {
		var f = build;
	}
}

// Invalid: variable declared in for loop but used outside
function doFor() {
	for (var x = 1; x < 10; x++) {
		var y = f(x);
	}
	console.log(y);
}

// Invalid: variable declared in while loop but used outside
function doWhile() {
	while (true) {
		var z = 1;
		break;
	}
	console.log(z);
}

// Invalid: variable declared in do-while loop but used outside
function doDoWhile() {
	do {
		var a = 1;
	} while (false);
	console.log(a);
}

// Invalid: variable declared in switch case but used outside
function doSwitch() {
	switch (x) {
		case 1:
			var caseVar = "test";
			break;
	}
	console.log(caseVar);
}

// Invalid: variable declared in with statement but used outside
function doWith() {
	with (obj) {
		var withVar = "test";
	}
	console.log(withVar);
}

// Invalid: nested blocks
function doNested() {
	if (true) {
		if (true) {
			var nested = "value";
		}
		console.log(nested); // used outside inner block
	}
}

// Invalid: variable in try block used in finally
function doTryFinally() {
	try {
		var tryVar = 1;
	} finally {
		console.log(tryVar);
	}
}

// Invalid: for-in loop variable used outside
function doForIn() {
	for (var key in obj) {
		var loopVar = obj[key];
	}
	console.log(loopVar);
}

// Invalid: for-of loop variable used outside
function doForOf() {
	for (var item of items) {
		var itemVar = item.value;
	}
	console.log(itemVar);
}

// Invalid: class static block
class C {
	static {
		if (something) {
			var build = true;
		}
		build = false;
	}
}
