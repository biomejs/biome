/* should not generate diagnostics */

class UsedMember {
	#usedMember = 42;

	method() {
			return this.#usedMember;
	}
}

class UsedMember {
	#usedMethod() {
			return 42;
	}

	anotherMethod() {
			return this.#usedMethod();
	}
}


class UsedMember {
	get #usedAccessor() {}
	set #usedAccessor(value) {}

	method() {
			this.#usedAccessor = 42;
	}
}

class UsedMember {
	publicMember = 42;
}

class UsedMember {
	#usedMember = 42;
	anotherMember = this.#usedMember;
}

class UsedMember {
	#usedMember = 42;
	foo() {
		this.#usedMember = this.#usedMember;
	}
}

class UsedMember {
	#usedMember;

	foo() {
			bar(this.#usedMember += 1);
	}
}

class UsedMember {
	#usedMember = 42;
	method() {
			return someGlobalMethod(this.#usedMember);
	}
}

class UsedMember {
	#usedInOuterClass;

	foo() {
			return class {};
	}

	bar() {
			return this.#usedInOuterClass;
	}
}


class UsedMember {
	#usedInForInLoop;
	method() {
			for (const bar in this.#usedInForInLoop) {

			}
	}
}

class UsedMember {
	#usedInForOfLoop;
	method() {
			for (const bar of this.#usedInForOfLoop) {

			}
	}
}

class UsedMember {
	#usedInAssignmentPattern;
	method() {
			[bar = 1] = this.#usedInAssignmentPattern;
	}
}

class UsedMember {
	#usedInArrayPattern;
	method() {
			[bar] = this.#usedInArrayPattern;
	}
}

class UsedMember {
	#usedInAssignmentPattern;
	method() {
			[bar] = this.#usedInAssignmentPattern;
	}
}

class UsedMember {
	#usedInObjectAssignment;

	method() {
			({ [this.#usedInObjectAssignment]: a } = foo);
	}
}

class UsedMember {
	set #accessorWithSetterFirst(value) {
			doSomething(value);
	}
	get #accessorWithSetterFirst() {
			return something();
	}
	method() {
			this.#accessorWithSetterFirst += 1;
	}
}

class UsedMember {
	set #accessorUsedInMemberAccess(value) {}

	method(a) {
			[this.#accessorUsedInMemberAccess] = a;
	}
}

class UsedMember {
	get #accessorWithGetterFirst() {
			return something();
	}
	set #accessorWithGetterFirst(value) {
			doSomething(value);
	}
	method() {
			this.#accessorWithGetterFirst += 1;
	}
}

class UsedMember {
	#usedInInnerClass;

	method(a) {
			return class {
					foo = a.#usedInInnerClass;
			}
	}
}

class Foo {
	#usedMethod() {
			return 42;
	}
	anotherMethod() {
			return this.#usedMethod();
	}
}

class C {
	set #x(value) {
			doSomething(value);
	}

	foo() {
			this.#x = 1;
	}
}
