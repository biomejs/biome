/* should generate diagnostics */
function returnsFromBranch(bar) {
	if (bar.baz) {
		return returnsFromBranch(bar.baz);
	}

	return bar;
}

const bar = function namedExpression(value) {
	return namedExpression(value.next);
};

function multipleArguments(first, second) {
	return multipleArguments(first.next, second.next);
}

function noArguments() {
	return noArguments();
}

async function asyncFunction(value) {
	if (value) {
		return asyncFunction(value.next);
	}

	return value;
}

function parenthesized(value) {
	return (parenthesized)((value.next));
}

export default function exportedDefault(value) {
	return exportedDefault(value.next);
}

function usingInSiblingBlock(value) {
	{
		using resource = acquire(value);
	}
	return usingInSiblingBlock(value.next);
}

function usingAfterReturn(value) {
	if (value.next) {
		return usingAfterReturn(value.next);
	}
	using resource = acquire(value);
	return resource;
}

function regularForLoop(value) {
	for (let i = 0; i < value.length; i++) {
		return regularForLoop(value[i]);
	}
}
