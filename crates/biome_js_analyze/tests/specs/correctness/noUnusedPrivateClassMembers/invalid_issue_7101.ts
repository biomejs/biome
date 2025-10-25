class TSDoubleUnusedPrivateConstructor {
	constructor(
		usedProperty = 3,
		private unusedProperty: number,
		private anotherUnusedProperty = 4
	) {
		// This constructor has two unused private properties

	}
}

class TSPartiallyUsedPrivateConstructor {
  constructor(private param: number) {
		// this is not read or write as far as class members are concerned.
    foo(param)
  }
}
