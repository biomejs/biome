// should not generate diagnostics
// Each optional chain on `this.timer` stays necessary. A write to `this.timer`
// in the method itself gives it the type of the written value, which
// TypeScript also types as `Timer | null` in `restart`. A write through
// `super` can set it through a setter of the parent class, so Biome also
// stops the narrowing in `replace`, where TypeScript gives `Timer`. In a
// constructor, a field without an initializer may be read before it is
// assigned, so TypeScript also keeps its declared type there (TS2565).

interface Timer {
	cancel(): void;
}

declare function next(): Timer | null;

class Scheduler {
	timer: Timer | null;

	constructor() {
		if (this.timer !== null) {
			this.timer?.cancel();
		}
		this.timer = next();
	}

	restart() {
		if (this.timer !== null) {
			this.timer = next();
			this.timer?.cancel();
		}
	}
}

class Base {
	get timer(): Timer | null {
		return null;
	}

	set timer(value: Timer | null) {}
}

class Child extends Base {
	replace() {
		if (this.timer !== null) {
			super.timer = next();
			this.timer?.cancel();
		}
	}
}
