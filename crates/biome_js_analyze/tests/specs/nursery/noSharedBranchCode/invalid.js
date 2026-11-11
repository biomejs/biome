/* should generate diagnostics */
function startShared(condition) {
	if (condition) {
		console.log("hello");
		doA();
	} else {
		console.log("hello");
		doB();
	}
}

function endShared(condition) {
	if (condition) {
		doA();
		cleanup();
	} else {
		doB();
		cleanup();
	}
}

function startSharedAssignment(condition) {
	let foo;
	if (condition) {
		console.log("start");
		foo = 13;
	} else {
		console.log("start");
		foo = 42;
	}
}

function startAndEndShared(x) {
	if (x) {
		console.log("before");
		doX();
		console.log("after");
	} else {
		console.log("before");
		doY();
		console.log("after");
	}
}

function multipleStatements(flag) {
	if (flag) {
		initialize();
		prepare();
		processData();
		finalize();
		report();
	} else {
		initialize();
		prepare();
		processOtherData();
		finalize();
		report();
	}
}

function updates(test) {
	if (test) {
		a++;
		b++;
	} else {
		a++;
		c++;
	}
}

function elseIfChain(flag, otherFlag) {
	if (flag) {
		console.log("start");
		doA();
	} else if (otherFlag) {
		console.log("start");
		doB();
	} else {
		console.log("start");
		doC();
	}
}

function elseIfChainWithReturn(x) {
	if (x === 1) {
		setup();
		return 1;
	} else if (x === 2) {
		setup();
		return 2;
	} else {
		setup();
		return 3;
	}
}

function differentQuotesAndComments(condition) {
	if (condition) {
		// log it
		log('value');
		doA();
	} else {
		log("value");
		doB();
	}
}

function sharedDeclarationMovesTogether(condition) {
	if (condition) {
		doA();
		const message = "done";
		console.log(message);
	} else {
		doB();
		const message = "done";
		console.log(message);
	}
}

function outerVariable(condition) {
	const value = 1;
	if (condition) {
		doA();
		console.log(value);
	} else {
		doB();
		console.log(value);
	}
}

function varIsNotBlockScoped(condition) {
	if (condition) {
		var value = 1;
		console.log(value);
	} else {
		var value = 2;
		console.log(value);
	}
}

function nestedIf(a, b) {
	if (a) {
		if (b) {
			start();
			doA();
		} else {
			start();
			doB();
		}
	} else {
		doC();
	}
}

function assignsConditionVariableAtEnd(x) {
	if (x > 0) {
		a();
		x = 0;
	} else {
		b();
		x = 0;
	}
}

function assignsUnrelatedProperty(state, other) {
	if (other.ready) {
		state.count = 0;
		a();
	} else {
		state.count = 0;
		b();
	}
}
