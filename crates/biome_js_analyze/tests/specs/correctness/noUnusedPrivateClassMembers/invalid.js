class Bioo {
	#unusedProperty = 5;

	#unusedMethod() {

	};
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

// a is not alias to this
class UsedMember18 {
	#usedInInnerClass;

	method(a) {
		return class {
			foo = a.#usedInInnerClass;
		}
	}
}
class OnlyWrite {
	#usedOnlyInWrite = 5;

	method() {
		this.#usedOnlyInWrite = 212;
	}
}

// issue #6994
class UsedAssignmentExpr {
	#val = 0;
	method() {
		return this.#val = 1
	}
}

