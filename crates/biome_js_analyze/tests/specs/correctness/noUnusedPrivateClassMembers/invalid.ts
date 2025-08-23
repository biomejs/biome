class TsBioo {
	private unusedProperty = 5;

	private unusedMethod() {

	};
}

class TSUnusedPrivateConstructor {
	constructor(private nusedProperty = 3){

	}
}

class TsAccessor {
	private get unusedAccessor() { }
	private set unusedAccessor(value) { }
}

// github.com/biomejs/biome/issues/6165
class TsBioo2 {
	private unusedProperty = 5;
	private unusedMethod() {}

	private usedProperty = 4;
	public test() {
		return this.usedProperty;
	}
}

class TSDoubleUnusedPrivateConstructor {
	constructor(private unusedOne: number, #unusedTwo: unknown) {
		// This constructor has two unused private properties
	}
}

class TSPartiallyUsedPrivateConstructor {
	constructor(private param: number) {
		foo(param)
	}
}

