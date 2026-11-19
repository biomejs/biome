// should generate diagnostics
// A property of `this` narrows like a property of a variable in a method, in
// an arrow function in a field initializer, in an object literal method, and
// in a function or method whose `this` parameter gives `this` its type:
// after the test, `this.timer` and `this.#pending` cannot be nullish at the
// optional chains, as in TypeScript.

interface Timer {
	cancel(): void;
}

class Scheduler {
	timer: Timer | null = null;
	#pending: Timer | undefined;

	stop() {
		if (this.timer) {
			this.timer?.cancel();
		}
	}

	flush() {
		if (this.#pending === undefined) {
			return;
		}
		this.#pending?.cancel();
	}

	onIdle = () => {
		if (this.timer !== null) {
			this.timer?.cancel();
		}
	};
}

const scheduler = {
	timer: null as Timer | null,
	stop() {
		if (this.timer) {
			this.timer?.cancel();
		}
	},
};

function stopTimer(this: { timer: Timer | null }) {
	if (this.timer) {
		this.timer?.cancel();
	}
}

class Poller {
	timer: Timer | null = null;

	stop(this: Poller) {
		if (this.timer) {
			this.timer?.cancel();
		}
	}
}
