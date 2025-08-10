/* should not generate diagnostics */

// Valid: variable declared at function scope
function doIf() {
	var build;

	if (true) {
		build = true;
	}

	console.log(build);
}

// Valid: variable declared at function scope and used in if/else
function doIfElse() {
	var build;

	if (true) {
		build = true;
	} else {
		build = false;
	}
}

// Valid: variables declared at function scope
function doTryCatch() {
	var build;
	var f;

	try {
		build = 1;
	} catch (e) {
		f = build;
	}
}

// Valid: variable used within the same loop block
function doFor() {
	for (var x = 1; x < 10; x++) {
		var y = f(x);
		console.log(y);
	}
}

// Valid: let and const declarations (block-scoped by nature)
function doModern() {
	if (true) {
		let build = true;
		const config = {};
		console.log(build, config);
	}
}

// Valid: variable declared and used in same block
function doSameBlock() {
	if (true) {
		var x = 1;
		console.log(x);
	}
}

// Valid: function declarations are hoisted
function testFunction() {
	foo(); // This is valid

	function foo() {
		return "test";
	}
}

// Valid: var at function scope used in nested blocks
function doNested() {
	var nested = "value";

	if (true) {
		if (true) {
			console.log(nested); // OK - declared at function scope
		}
	}
}

// Valid: class static block with proper scoping
class C {
	static {
		var build = false;
		if (something) {
			build = true;
		}
	}
}

// Valid: while loop with proper scoping
function doWhile() {
	var z;
	while (true) {
		z = 1;
		break;
	}
	console.log(z);
}

// Valid: switch statement with proper scoping
function doSwitch() {
	var caseVar;
	switch (x) {
		case 1:
			caseVar = "test";
			break;
	}
	console.log(caseVar);
}

// Valid: for-in with function scope variable
function doForIn() {
	var loopVar;
	for (var key in obj) {
		loopVar = obj[key];
	}
	console.log(loopVar);
}
