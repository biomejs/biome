export class Sample {
	private member;
	#prop;

	constructor() {
		this.#prop = 0;
		this.member = 0;
	}

	method(name) {
		return this[name];
	}
}

export class SampleAddRemove {
	private add: () => void;
	private append: () => void; // <- unused

	constructor(private remove: () => void) {
		this.add = () => {
		};
		this.remove = () => {
		};
	}

	on(action: "add" | "remove"): void {
		this[action]();
	}
}

// will only make a match on the string literals and ignore anything else
type YesNo = "yes" | "no" | { ignored: number };

export class SampleYesNo {
	private yes: () => void;
	private no: () => void;
	private dontKnow: () => void; // <- unused

	on(action: YesNo): void {
		this[action]();
	}
}

export class SampleYesNoString {
	private yes: () => void;
	private no: () => void;
	private dontKnow: () => void;

	on(action: string): void {
		this[action]();
	}
}

export class SampleYesNoAny {
	private yes: () => void;
	private no: () => void;
	private dontKnow: () => void;

	on(action: any): void {
		this[action]();
	}
}

export class SampleYesNoUnknown {
	private yes: () => void;
	private no: () => void;
	private dontKnow: () => void;

	on(action: unknown): void {
		this[action]();
	}
}

