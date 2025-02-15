async function returnsPromise(): Promise<string> {
	return "value";
}
returnsPromise();
returnsPromise()
	.then(() => {})
	.finally(() => {});

async function returnsPromiseInAsyncFunction(): Promise<void> {
	returnsPromise();
}

const returnsPromiseInAsyncArrowFunction = async (): Promise<void> => {
	returnsPromise()
		.then(() => {})
		.finally(() => {});
};

class Test {
	async returnsPromiseInAsyncClassMethod(): Promise<void> {
		returnsPromise();
	}
}

function returnsPromiseWithoutAsync(): Promise<string> {
	return Promise.resolve("value");
}

returnsPromiseWithoutAsync();

const returnsPromiseAssignedArrowFunction = async (): Promise<string> => {
	return "value";
};

returnsPromiseAssignedArrowFunction();

const returnsPromiseAssignedFunction = async function (): Promise<string> {
	return "value";
};

async function returnsPromiseAssignedFunctionInAsyncFunction(): Promise<void> {
	returnsPromiseAssignedFunction().then(() => {});
}

const returnsPromiseAssignedArrowFunctionAnnotatedType: () => Promise<string> =
	() => {
		return Promise.resolve("value");
	};

returnsPromiseAssignedArrowFunctionAnnotatedType();

const promise = new Promise((resolve) => resolve("value"));
promise.then(() => {}).finally(() => {});

Promise.resolve("value").then(() => {});
Promise.all([p1, p2, p3]);

const promiseWithParentheses = new Promise((resolve, reject) =>
	resolve("value")
);
promiseWithParentheses;
returnsPromise();

const promiseWithGlobalIdentifier = new window.Promise((resolve, reject) =>
	resolve("value")
);
promiseWithGlobalIdentifier.then(() => {}).finally(() => {});
globalThis.Promise.reject("value").finally();

class InvalidTestClassParent {
	async returnsPromiseFromParent(): Promise<string> {
		return "value";
	}
}
class InvalidTestClass extends InvalidTestClassParent {
	returnsPromiseFunctionProperty: () => Promise<void>;
	returnsPromiseProperty: Promise<void>;
	constructor() {
		super();
		this.returnsPromiseFunctionProperty = () => Promise.resolve();
		this.returnsPromiseProperty = new Promise((resolve, reject) => {});
	}

	async returnsPromiseMethod(): Promise<string> {
		return "value";
	}
	async someMethod() {
		this.returnsPromiseMethod();
	}

	async someMethod2() {
		this.returnsPromiseFromParent()
			.then(() => {})
			.finally(() => {});
	}

	async someMethod3() {
		this.returnsPromiseFunctionProperty();
	}

	async someMethod4() {
		this.returnsPromiseProperty.then(() => {}).finally(() => {});
	}

	async #returnsPromisePrivateMethod(): Promise<string> {
		return "value";
	}
	async someMethod5() {
		this.#returnsPromisePrivateMethod()
			.then(() => {})
			.finally(() => {});
	}

	returnsPromiseFunction = async function (): Promise<string> {
		return "value";
	};
	returnsPromiseArrowFunction = async (): Promise<string> => {
		return "value";
	};

	async someMetho3() {
		this.returnsPromiseFunction()
			.then(() => {})
			.finally(() => {});
		this.returnsPromiseArrowFunction();
	}
}

const invalidTestClass = new InvalidTestClass();
invalidTestClass
	.returnsPromiseMethod()
	.then(() => {})
	.finally(() => {});
invalidTestClass.returnsPromiseFunctionProperty();
invalidTestClass.returnsPromiseProperty;
invalidTestClass.returnsPromiseProperty.then(() => {}).finally(() => {});

const InvalidTestClassInitializedExpression = class InvalidTestClass extends InvalidTestClassParent {
	returnsPromiseFunctionProperty: () => Promise<void>;
	returnsPromiseProperty: Promise<void>;
	constructor() {
		super();
		this.returnsPromiseFunctionProperty = () => Promise.resolve();
		this.returnsPromiseProperty = new Promise((resolve, reject) => {});
	}

	async returnsPromiseMethod(): Promise<string> {
		return "value";
	}
	async someMethod() {
		this.returnsPromiseMethod();
	}

	async someMethod2() {
		this.returnsPromiseFromParent()
			.then(() => {})
			.finally(() => {});
	}

	async someMethod3() {
		this.returnsPromiseFunctionProperty();
	}

	async someMethod4() {
		this.returnsPromiseProperty.then(() => {}).finally(() => {});
	}

	async #returnsPromisePrivateMethod(): Promise<string> {
		return "value";
	}
	async someMethod5() {
		this.#returnsPromisePrivateMethod()
			.then(() => {})
			.finally(() => {});
	}

	returnsPromiseFunction = async function (): Promise<string> {
		return "value";
	};
	returnsPromiseArrowFunction = async (): Promise<string> => {
		return "value";
	};

	async someMetho3() {
		this.returnsPromiseFunction()
			.then(() => {})
			.finally(() => {});
		this.returnsPromiseArrowFunction();
	}
};

const invalidTestClassExpression = new InvalidTestClassInitializedExpression();
invalidTestClassExpression
	.returnsPromiseMethod()
	.then(() => {})
	.finally(() => {});
invalidTestClassExpression.returnsPromiseFunctionProperty();
invalidTestClassExpression.returnsPromiseProperty;
invalidTestClassExpression.returnsPromiseProperty
	.then(() => {})
	.finally(() => {});

const InvalidTestUnnamedClassInitializedExpression = class extends InvalidTestClassParent {
	returnsPromiseFunctionProperty: () => Promise<void>;
	returnsPromiseProperty: Promise<void>;
	constructor() {
		super();
		this.returnsPromiseFunctionProperty = () => Promise.resolve();
		this.returnsPromiseProperty = new Promise((resolve, reject) => {});
	}

	async returnsPromiseMethod(): Promise<string> {
		return "value";
	}
	async someMethod() {
		this.returnsPromiseMethod();
	}

	async someMethod2() {
		this.returnsPromiseFromParent()
			.then(() => {})
			.finally(() => {});
	}

	async someMethod3() {
		this.returnsPromiseFunctionProperty();
	}

	async someMethod4() {
		this.returnsPromiseProperty.then(() => {}).finally(() => {});
	}

	async #returnsPromisePrivateMethod(): Promise<string> {
		return "value";
	}
	async someMethod5() {
		this.#returnsPromisePrivateMethod()
			.then(() => {})
			.finally(() => {});
	}

	returnsPromiseFunction = async function (): Promise<string> {
		return "value";
	};
	returnsPromiseArrowFunction = async (): Promise<string> => {
		return "value";
	};

	async someMetho3() {
		this.returnsPromiseFunction()
			.then(() => {})
			.finally(() => {});
		this.returnsPromiseArrowFunction();
	}
};

const invalidTestUnnamedClassInitializedExpression =
	new InvalidTestUnnamedClassInitializedExpression();
invalidTestUnnamedClassInitializedExpression
	.returnsPromiseMethod()
	.then(() => {})
	.finally(() => {});
invalidTestUnnamedClassInitializedExpression.returnsPromiseFunctionProperty();
invalidTestUnnamedClassInitializedExpression.returnsPromiseProperty;
invalidTestUnnamedClassInitializedExpression.returnsPromiseProperty
	.then(() => {})
	.finally(() => {});
