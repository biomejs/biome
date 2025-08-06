class Example1 {
	#prop0: number = 42;
	private prop1: number = 42;
	protected prop2: string;
	public prop3: string;
}

class Example2 {
	constructor(
		private prop1: number,
		public prop2: string,
		protected prop3: string,
	) {
	}
}

class Example3 {
	#prop0: number = 42;
	private prop1: number = 42;
	protected prop2: number;
	public prop3: number;

	constructor(prop0: number, prop1: number, prop2: number, prop3: number, private prop4: number, public prop5: number, protected prop6: number) {
		this.#prop0 = prop0;
		this.prop1 = prop1;
		this.prop2 = prop2;
		this.prop3 = prop3;
	}
}

// with some getters/ reads do not affect readonly
class Example4 {
	#prop0: number = 42;
	private prop1: number = 42;
	protected prop2: number;
	public prop3: number;

	constructor(prop0: number, prop1: number, prop2: number, prop3: number) {
		this.#prop0 = prop0;
		this.prop1 = prop1;
		this.prop2 = prop2;
		this.prop3 = prop3;
	}

	getProp0(): number {
		return this.#prop0;
	}

	getProp1(): number {
		return this.prop1;
	}

	getProp2(): number {
		return this.prop2;
	}

	getProp3(): number {
		return this.prop3;
	}
}
