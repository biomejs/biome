// An assignment narrows the variable or property that it writes, as in
// TypeScript, so each condition below can't change.

interface Timer {
	cancel(): void;
}

declare function make(): Timer;

function assigned() {
	let timer: Timer | null = null;
	timer = make();
	timer?.cancel();
}

function initializer() {
	const timer: Timer | null = make();
	timer?.cancel();
}

function coalesced(timer: Timer | undefined) {
	timer ??= make();
	timer?.cancel();
}

class Scheduler {
	timer: Timer | null = null;

	constructor() {
		if (this.timer !== null) {
			this.timer?.cancel();
		}
	}

	restart() {
		this.timer = make();
		this.timer?.cancel();
	}
}

function enabled() {
	let ready: boolean | undefined = true;
	if (ready) {
	}
}
