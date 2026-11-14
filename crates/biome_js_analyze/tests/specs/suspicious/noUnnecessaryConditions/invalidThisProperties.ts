// should generate diagnostics
// A property of `this` narrows like a property of a variable in a method, in
// an arrow function in a field initializer, and in an object literal method:
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
