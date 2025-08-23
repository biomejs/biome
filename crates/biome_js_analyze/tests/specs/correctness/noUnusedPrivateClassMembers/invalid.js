class Bioo {
	#unusedProperty = 5;

	#unusedMethod() {

	};
}

class OnlyWrite {
	#usedOnlyInWrite = 5;

	method() {
			this.#usedOnlyInWrite = 212;
	}
}

class SelfUpdate {
	#usedOnlyToUpdateItself = 5;

	method() {
			this.#usedOnlyToUpdateItself++;
	}
}

class Accessor {
	get #unusedAccessor() {}
	set #unusedAccessor(value) {}
}

class First {
	#unusedMemberInFirstClass = 5;
}

class Foo {
	#usedOnlyInWrite = 5;
	method() {
			this.#usedOnlyInWrite = 42;
	}
}

// a is not alias to this
class UsedMember18 {
	#usedInInnerClass;

	method(a) {
		return class {
			foo = a.#usedInInnerClass;
		}
	}
}
