/* should not generate diagnostics */
function loop(bar) {
	while (bar.baz) {
		bar = bar.baz;
	}

	return bar;
}

function forLoop(bar) {
	for (; bar.baz; bar = bar.baz) {}

	return bar;
}

function insideCallback(bar) {
	if (Array.isArray(bar)) {
		return bar.map(baz => insideCallback(baz));
	}

	return bar;
}

function binaryExpression(value) {
	return 1 + binaryExpression(value);
}

function conditional(value) {
	return value ? conditional(value.next) : value;
}

async function awaited(value) {
	return await awaited(value);
}

function insideNestedFunction(value) {
	function bar() {
		return insideNestedFunction(value);
	}

	return bar();
}

function shadowedByVariable(value) {
	const shadowedByVariable = () => value;
	return shadowedByVariable();
}

function shadowedByParameter(shadowedByParameter) {
	return shadowedByParameter();
}

function optionalCall(value) {
	return optionalCall?.(value);
}

function memberCall(value) {
	return object.memberCall(value);
}

const arrow = value => {
	return arrow(value);
};

const anonymous = function (value) {
	return anonymous(value);
};

function* generator(value) {
	return generator(value);
}

const object = {
	method(value) {
		return method(value);
	},
	get getter() {
		return getter();
	},
};

class Foo {
	method(value) {
		return method(value);
	}
}

function tryFinally(value) {
	try {
		return tryFinally(value.next);
	} finally {
		cleanup(value);
	}
}

function tryCatch(value) {
	try {
		return tryCatch(value.next);
	} catch (error) {
		if (value.retry) {
			throw error;
		}

		return value;
	}
}

function insideCatch(value) {
	try {
		return value.next;
	} catch {
		return insideCatch(value.next);
	}
}

function usingDeclaration(value) {
	using resource = acquire(value);
	return usingDeclaration(value.next);
}

async function awaitUsingDeclaration(value) {
	await using resource = acquire(value);
	return awaitUsingDeclaration(value.next);
}

function usingInOuterBlock(value) {
	using resource = acquire(value);
	if (value.next) {
		return usingInOuterBlock(value.next);
	}
}

function usingInForHeader(value) {
	for (using resource = acquire(value); value.next;) {
		return usingInForHeader(value.next);
	}
}

async function awaitUsingInForHeader(value) {
	for (await using resource = acquire(value); value.next;) {
		return awaitUsingInForHeader(value.next);
	}
}

function usingInForOfHeader(value) {
	for (using resource of resources) {
		return usingInForOfHeader(value.next);
	}
}

async function awaitUsingInForOfHeader(value) {
	for await (await using resource of resources) {
		return awaitUsingInForOfHeader(value.next);
	}
}
