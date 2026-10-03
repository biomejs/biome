/* should not generate diagnostics */
let topLevel = "used";
console.log(topLevel);
topLevel = "used-2";
console.log(topLevel);

function readAfterWrite() {
	let v = "used";
	console.log(v);
	v = "used-2";
	console.log(v);
}

function readInBothPaths() {
	let v = "used";
	if (condition) {
		v = "used-2";
		console.log(v);
		return;
	}
	console.log(v);
}

function writeInOneBranch() {
	let v = "used";
	if (condition) {
		//
	} else {
		v = "used-2";
	}
	console.log(v);
}

function readInNextIteration() {
	let v = "used";
	for (let i = 0; i < 10; i++) {
		console.log(v);
		v = "used in next iteration";
	}
}

function readInNextWhileIteration() {
	let v = "used";
	while (condition) {
		console.log(v);
		v = "used in next iteration";
	}
}

function readInNextDoWhileIteration() {
	let v = "used";
	do {
		console.log(v);
		v = "used in next iteration";
	} while (condition);
}

function readInNextForOfIteration() {
	let previous = null;
	for (const item of items) {
		console.log(previous, item);
		previous = item;
	}
}

function readAfterContinue() {
	let v = "used";
	console.log(v);
	for (let i = 0; i < 10; i++) {
		if (condition) {
			v = "maybe used";
			continue;
		}
		console.log(v);
	}
}

function readByLabeledContinue() {
	let v = "used";
	outer: for (const a of as) {
		for (const b of bs) {
			v = b;
			continue outer;
		}
		console.log(v, a);
	}
}

function updates() {
	let i = 0;
	i++;
	i++;
	console.log(i);
}

function updateThenRead() {
	let a = 42;
	console.log(a);
	a--;
	console.log(a);
}

function selfReferencingAssignment() {
	let a = 42;
	console.log(a);
	a = 10;
	a = a + 1;
	console.log(a);
}

function destructuringAssignment() {
	let a = "used", b = "used", c = "used", d = "used";
	console.log(a, b, c, d);
	({ a, arr: [b, c, ...d] } = fn());
	console.log(a, b, c, d);
}

function memberAssignment() {
	let a = {};
	console.log(a);
	a.b = "unused like, but maybe used in setter";
	a.c++;
}

function memberTargetReadBeforeValue() {
	let i = 0;
	console.log(i);
	i = 1;
	arr[i] = (i = 2);
	console.log(i);
}

function writtenByClosure() {
	let v = "used";
	console.log(v);
	function bar() {
		v = "used in outer scope";
	}
	bar();
	console.log(v);
}

function readByClosure() {
	let v = "used";
	console.log(v);
	setTimeout(() => console.log(v), 1);
	v = "used in other scope";
}

function readByClassMember() {
	let v = "used";
	console.log(v);
	v = "used later";
	return class {
		value = v;
	};
}

function readByObjectMethod() {
	let v = "used";
	console.log(v);
	v = "used later";
	return {
		get value() {
			return v;
		},
	};
}

function neverRead() {
	let v = "unused";
	v = "unused-2";
	doSomething();
}

function unknownVariable() {
	v = "used";
	console.log(v);
	v = "unused";
}

function unreachableWrite() {
	const x = 1;
	console.log(x);
	return;
	x = "Foo";
}

function unreachableBlock() {
	return;
	const x = 1;
	if (y) {
		bar(x);
	}
}

function readByLogicalExpression() {
	let v = "used";
	condition && (v = "maybe");
	console.log(v);
}

function readByConditionalExpression() {
	let v = "used";
	condition ? (v = "maybe") : null;
	console.log(v);
}

function readByOptionalCall() {
	let v = "used";
	object?.method((v = "maybe"));
	console.log(v);
}

function readByLogicalAssignment() {
	let v = getValue();
	v ||= "default";
	console.log(v);
}

function readByDestructuringDefault() {
	let fallback = "used";
	const { value = (fallback = "maybe") } = getObject();
	console.log(value, fallback);
}

function readByDestructuring() {
	const obj = { a: 5 };
	const { a, b = a } = obj;
	console.log(b);
	const arr = [6];
	const [c, d = c] = arr;
	console.log(d);
}

function readByDestructuringDefaultAssignment() {
	const obj = { a: 1 };
	let { a, b = (a = 2) } = obj;
	console.log(a, b);
}

function readByIteratedExpression() {
	let items = getItems();
	for (const item of items) {
		console.log(item);
	}
	for (const key in items) {
		console.log(key);
	}
}

function writeInForOfHead() {
	let item = null;
	for (item of items) {
	}
	console.log(item);
}

function readInCatch() {
	let message = "init";
	try {
		const result = call();
		message = result.message;
	} catch (e) {
		// ignore
	}
	console.log(message);
}

function readAfterNestedTry() {
	let v = "init";
	try {
		v = callA();
		try {
			v = callB();
		} catch (e) {
			// ignore
		}
	} catch (e) {
		// ignore
	}
	console.log(v);
}

function writeInFinally() {
	let a;
	try {
		foo();
	} finally {
		a = 5;
	}
	console.log(a);
}

function* readInFinallyOfGenerator() {
	let done = false;
	try {
		yield 1;
		done = true;
	} catch {
		done = true;
	} finally {
		if (!done) {
			console.log("done is false");
		}
	}
}

function* readInCatchOfGenerator() {
	let done = false;
	try {
		yield 1;
	} catch {
		console.log(done);
	}
}

function readInFinally() {
	let outcome = "unknown";
	try {
		helper1();
		outcome = "success";
	} catch (err) {
		helper2();
		outcome = "exception";
	} finally {
		console.log(outcome);
	}
}

function readInFinallyAfterBreak() {
	let v = "init";
	for (const item of items) {
		try {
			v = item;
			break;
		} catch {
			// ignore
		} finally {
			console.log(v);
		}
	}
}

function readInCatchAfterThrowingCall() {
	let bar;
	try {
		bar = 2;
		unsafeFn();
		return { error: undefined };
	} catch {
		return { bar };
	}
}

function readAfterTryWithThrowingCall() {
	let bar;
	try {
		bar = 2;
		unsafeFn();
		bar = 4;
	} catch {
		// handle error
	}
	return bar;
}

function readInSwitchFallthrough() {
	let v = "init";
	switch (kind) {
		case 1:
			v = "one";
		case 2:
			console.log(v);
	}
}

function usingDeclaration() {
	using resource = getResource();
	console.log(resource);
}

function updatedByClosure(n, func) {
	n = toInteger(n);
	return function () {
		if (--n < 1) {
			return func.apply(this, arguments);
		}
	};
}

function compoundAssignedByClosure() {
	let total = 0;
	console.log(total);
	total = 10;
	return () => (total += 1);
}

// Variables used by nested functions are ignored, even when they only write them.
function writtenByClosureOnly() {
	let v = "used";
	console.log(v);
	setTimeout(() => (v = 42), 1);
	v = "unused";
}

// Accesses inside the same statement are not ordered, so a read in the statement keeps its writes.
function nestedUpdate() {
	var x = 1;
	x = x++;
	f(x);
}

function nestedAssignment() {
	let x;
	x = (x = 1) + 1;
	f(x);
}

// The graph doesn't evaluate destructuring patterns of declarations, so their variables are ignored.
function destructuringDefaultWrite() {
	const obj = { a: 1 };
	let { a, b = (a = 2) } = obj;
	a = 3;
	console.log(a, b);
}
