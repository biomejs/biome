// should not generate diagnostics
// Each condition below stays necessary: the written value can be `null`, or
// `undefined` when a method without a return type annotation reaches its
// end, a variable or a field may be read before it is assigned, and a
// compound assignment gives `n` the type `number`.

interface Timer {
	cancel(): void;
}

declare function next(): Timer | null;
declare function make(): Timer;
declare const flag: boolean;

function reassigned(timer: Timer | null) {
	if (timer !== null) {
		timer = next();
		timer?.cancel();
	}
}

function maybeAssigned() {
	let timer: Timer | null;
	if (flag) {
		timer = make();
	}
	timer?.cancel();
}

function counted() {
	let n: 0 | 1 = 0;
	n++;
	if (n === 2) {
		return n;
	}
}

class Late {
	timer: Timer | null;

	constructor() {
		if (this.timer !== null) {
			this.timer?.cancel();
		}
		this.timer = next();
	}
}

class Cache {
	lookup(key: string) {
		if (key) {
			return make();
		}
	}
}

function looked(cache: Cache) {
	const timer: Timer | undefined = cache.lookup("a");
	timer?.cancel();
}
