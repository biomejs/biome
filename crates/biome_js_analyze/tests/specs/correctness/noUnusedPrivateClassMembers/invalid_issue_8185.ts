/* should generate diagnostics */

// https://github.com/biomejs/biome/issues/8185
export class Foo {
	#name = "foo";

	constructor() {
		this.#init();
	}

	#init() {
		this.#name = "fooed";
	}
}

class MultipleWrites {
	#count = 0;

	reset() {
		this.#count = 0;
	}

	set(value: number) {
		this.#count = value;
	}
}

class UpdateExpressions {
	#counter = 0;

	increment() {
		this.#counter++;
	}

	decrement() {
		--this.#counter;
	}
}

class NotInStatementList {
	#flag = false;

	toggle(condition: boolean) {
		if (condition) this.#flag = true;
		while (condition) this.#flag = false;
	}
}

class SideEffect {
	#value: number | undefined;

	refresh() {
		this.#value = compute();
	}
}

class TsPrivate {
	private name = "foo";

	constructor() {
		this.init();
	}

	private init() {
		this.name = "fooed";
	}
}

class ParameterProperty {
	constructor(private counter = 0) {}

	reset() {
		this.counter = 0;
	}
}

class NestedInMember {
	#callback = () => {
		this.#callback = () => {};
	};
}

class StaticMember {
	static #instances = 0;

	static register() {
		StaticMember.#instances = 1;
	}
}

class KeepsReads {
	#unused = 1;
	#used = 2;

	update() {
		this.#unused = 3;
		this.#used = 4;
	}

	get value() {
		return this.#used;
	}
}

class OtherReceiver {
	private name = "";

	apply(element: HTMLElement) {
		element.name = "x";
	}
}

class ParameterOtherReceiver {
	constructor(private id: number) {}

	apply(node: { id: number }) {
		node.id = 5;
	}
}

class TsStatic {
	private static instance: TsStatic | undefined;

	static create() {
		TsStatic.instance = new TsStatic();
	}
}

class NestedWrites {
	#timer: ReturnType<typeof setTimeout> | null = null;

	schedule(callback: () => void) {
		this.#timer = setTimeout(() => {
			this.#timer = null;
			callback();
		}, 100);
	}
}
