class UsedMember {
	get #usedAccessor() {}
	set #usedAccessor(value) {}

	method() {
		// no return statement so no meaningful read
		this.#usedAccessor = 42;
	}
}

class UsedMember {
	#usedInInnerClass;

	method(a) {
		return class {
			// not really used, a is not reference to this scope
			foo = a.#usedInInnerClass;
		}
	}
}

class UsedMember {
	set #accessorUsedInMemberAccess(value) {} // <- unused

	method(a) {
		// there is no getter, so this is not a read at all
		[this.#accessorUsedInMemberAccess] = a;
	}
}

class C {
	set #x(value) {
		doSomething(value);
	}

	foo() {
    // no return statement so not a meaningful read.
		this.#x = 1;
	}
}
