/* should generate diagnostics */

(() => {
	const value = getValue();
	run(value);
})();

(function () {
	const value = getValue();
	run(value);
})();

(function () {
	const value = getValue();
	run(value);
}());

((() => {
	run();
}))();

(() => { eval?.(code); })();

(() => { (0, run)(code); })();

if (condition) {
	(() => {
		const value = getValue();
		run(value);
	})();
}

if (condition) (() => { run(); })();
else (function () { stop(); })();

// Leading comment.
(() => {
	// Keep this comment.
	run();
})(); // Trailing comment.

(() => {
	this.run();
})();

(() => {
	const getArguments = () => arguments;
	getArguments();
})();

(() => {
	function nested() {
		return value;
	}

	run(nested);
})();

(() => {
	function nested() {
		this.run();
	}

	run(nested);
})();

(function () {
	function nested() {
		var count = arguments.length;
		return eval(code) + count + this.value + new.target;
	}

	run(nested);
})();

(function () {
	const object = {
		method() {
			return this.value;
		},
		get value() {
			return this.other;
		},
		set value(next) {
			this.other = next;
		},
	};
	class Example {
		constructor(value = arguments) {
			this.value = value;
		}
		method() {
			return this.value;
		}
	}
	run(object, Example);
})();

(() => {
	const callback = () => {
		var count = 1;
		return count;
	};
	run(callback);
})();

(function () {
	const callback = () => {
		var count = 1;
		return count;
	};
	run(callback);
})();

(() => {
	function foo() {}
	foo();
})();

(() => {
	let total = 0;
	for (const item of items) {
		total += item;
	}
	loop: for (;;) {
		break loop;
	}
	run(total);
})();

other: {
	(() => {
		loop: for (;;) {
			break loop;
		}
	})();
}

loop: for (;;) {
	run(() => {
		(() => {
			loop: for (;;) {
				break loop;
			}
		})();
	});
}

(() => {})();

(/* comment */ () => {
	run();
})();

(() => {
	run();
} /* comment */)();

(() => {
	run();
})() /* comment */;

(() => /* comment */ {
	run();
})();
