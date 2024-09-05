function foo(someString) {
	return /[a-Z]*/.test(someString)
}

function foo(someString) {
	const r = /[a-Z]*/;
	return r.test(someString)
}

const foo = (someString) => {
	return /[a-Z]*/.test(someString)
}

class Foo {
	constructor() {
		this.regex = /[a-Z]*/;
	}
}

class Foo {
	regex = /[a-Z]*/;
}

class Foo {
	get regex() {
		return /[a-Z]*/;
	}
}

class Foo {
	set apply(s) {
		this.value = /[a-Z]*/.test(s);
	}
}

const foo = {
	regex() {
		return /[a-Z]*/;
	}
}

const foo = {
	get regex() {
		return /[a-Z]*/;
	}
}

const foo = {
	set apply(s) {
		this.value = /[a-Z]*/.test(s);
	}
}
