class TSDoubleUnusedPrivateConstructor {
	constructor(private unusedProperty = 3, private anotherUnusedProperty = 4) {
		// This constructor has two unused private properties

	}
}

class TSPartiallyUsedPrivateConstructor {
  constructor(private param: number) {
    foo(param)
  }
}