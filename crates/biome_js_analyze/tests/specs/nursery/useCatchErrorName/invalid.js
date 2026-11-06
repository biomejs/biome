/* should generate diagnostics */
try {
} catch (err) {
	console.log(err);
}

obj.catch((err) => err);

obj?.catch((err) => err);

obj.then(undefined, (err) => err);

obj.catch(function (err) {
	console.log(err);
});

obj.then(undefined, function (err) {
	console.log(err);
});

obj.catch(err => ({ err }));

(obj.catch)((err) => {});

obj.catch(function (_) {
	console.log(_);
});

try {
} catch (_) {
	console.log(_);
}

// Only the outer `_` is used.
try {
} catch (_) {
	console.log(_);
	try {
	} catch (_) {}
}

// Both `_` are used.
try {
} catch (_) {
	console.log(_);
	try {
	} catch (_) {
		console.log(_);
	}
}

// Nested catch clauses are both reported.
try {
} catch (anyName) {
	try {
	} catch (anyOtherName) {}
}

foo.then(() => {
	try {} catch (e) {}
}).catch((err) => err);

// The arrow function parameter `error` is visible inside the catch clause.
const handleError1 = (error) => {
	try {
		doSomething();
	} catch (foo) {
		console.log(foo);
	}
};

const handleError2 = (error) => {
	const error_ = new Error("foo bar");
	obj.catch((foo) => {});
};

// Declarations in the block clash with the caught error.
try {} catch (e) {
	const error = 1;
}

try {} catch (e) {
	let error;
}

try {} catch (e) {
	class error {}
}

try {} catch (e) {
	function error() {}
}

obj.catch((e) => {
	const error = 1;
});

try {
} catch (e) {
	const error = new Error(e);
	throw error;
}

// A shadowing variable in a nested block.
try {
} catch (e) {
	{
		const error = 1;
		console.log(e);
	}
}

// `error` is an undeclared variable here.
try {
} catch (e) {
	console.log(error, e);
}

obj.then(
	function error(err) {},
	function error(err) {},
);

// Only the first parameter is checked.
obj.catch(function (other, err) {});

// A parenthesized handler is still the handler.
obj.catch(((err) => {}));

obj.catch(function (e) {
	const error = 1;
});

obj.then(undefined, (e) => {
	const error = 1;
});

// `var` declarations in a nested block belong to the whole function.
obj.catch(function (e) {
	if (e) {
		var error = 1;
	}
});
