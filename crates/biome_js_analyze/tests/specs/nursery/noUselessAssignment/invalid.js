/* should generate diagnostics */
let topLevel = "used";
console.log(topLevel);
topLevel = "unused";

function overwrittenAtEnd() {
	let v = "used";
	console.log(v);
	v = "unused";
}

function overwrittenBeforeReturn() {
	let v = "used";
	if (condition) {
		v = "unused";
		return;
	}
	console.log(v);
}

function overwrittenInElse() {
	let v = "used";
	if (condition) {
		console.log(v);
	} else {
		v = "unused";
	}
}

const functionExpression = function () {
	let v = "used";
	console.log(v);
	v = "unused";
};

const arrowFunction = () => {
	let v = "used";
	console.log(v);
	v = "unused";
};

class StaticBlock {
	static {
		let v = "used";
		console.log(v);
		v = "unused";
	}
}

function overwrittenInitializer() {
	let v = "unused";
	if (condition) {
		v = "used";
		console.log(v);
		return;
	}
}

function consecutiveWrites() {
	let v = "used";
	console.log(v);
	v = "unused";
	v = "used";
	console.log(v);
}

function overwrittenInBothBranches() {
	let v = "unused";
	if (condition) {
		if (condition2) {
			v = "used-2";
		} else {
			v = "used-3";
		}
	} else {
		v = "used-4";
	}
	console.log(v);
}

function overwrittenAfterBranch() {
	let v;
	if (condition) {
		v = "unused";
	}
	if (condition2) {
		v = "used-1";
	} else {
		v = "used-2";
	}
	console.log(v);
}

function update() {
	let a = 42;
	console.log(a);
	a++;
	--a;
}

function compoundAssignment() {
	let a = 42;
	console.log(a);
	a += 1;
}

function logicalAssignment() {
	let a = getValue();
	console.log(a);
	a ??= "default";
}

function destructuringAssignment() {
	let a = "used", b = "used", c = "used", d = "used";
	console.log(a, b, c, d);
	({ a, arr: [b, c, , ...d] } = fn());
	console.log(c);
}

function destructuringAssignmentWithDefault() {
	let a = "used", b = "used", c = "used";
	console.log(a, b, c);
	({ a = "unused", foo: b, ...c } = fn());
}

function shadowed() {
	let v = "used";
	if (condition) {
		let v = "used";
		console.log(v);
		v = "unused";
	}
	console.log(v);
	v = "unused";
}

function readAfterReturn() {
	let v = "used";
	console.log(v);
	v = "unused";
	return;
	console.log(v);
}

function readAfterThrow() {
	let v = "used";
	console.log(v);
	v = "unused";
	throw new Error();
	console.log(v);
}

function readAfterContinue() {
	let v = "used";
	console.log(v);
	for (let i = 0; i < 10; i++) {
		v = "unused";
		continue;
		console.log(v);
	}
}

function readAfterBreak() {
	let v = "used";
	console.log(v);
	for (let i = 0; i < 10; i++) {
		if (condition) {
			v = "unused";
			break;
		}
		console.log(v);
	}
}

function overwrittenInTryAndCatch() {
	let message = "unused";
	try {
		const result = call();
		message = result.message;
	} catch (e) {
		message = "used";
	}
	console.log(message);
}

function readInTryOnly() {
	let message = "unused";
	try {
		message = "used";
		console.log(message);
	} catch (e) {}
}

function overwrittenInFinally() {
	let done = false;
	try {
		work();
	} finally {
		done = true;
		console.log(done);
	}
}

function readAfterFinallyReturn() {
	let outcome = "unknown";
	try {
		bar();
	} catch (err) {
		outcome = "exception";
	} finally {
		return;
		console.log(outcome);
	}
}

function* generator() {
	let done = false;
	yield 1;
	done = true;
	console.log(done);
}

function selfUpdate() {
	var x = 1;
	x = x + 1;
	x = 5;
	f(x);
}

function writeInSwitch() {
	let v = "unused";
	switch (kind) {
		case 1:
			v = "one";
			break;
		default:
			v = "other";
	}
	console.log(v);
}

function writeInWhile() {
	let v = 0;
	while (condition) {
		v = next();
		v = other();
		console.log(v);
	}
}

function conditionalExpressionWrite() {
	let v = "used";
	console.log(v);
	condition ? (v = "unused") : (v = "unused too");
}
