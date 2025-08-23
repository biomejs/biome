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
