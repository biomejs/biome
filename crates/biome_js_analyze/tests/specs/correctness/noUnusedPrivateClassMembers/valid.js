/* should not generate diagnostics */

class UsedMember {
	#usedMember = 42;

	method() {
		return this.#usedMember;
	}
}

class UsedMember2 {
	#usedMethod() {
		return 42;
	}

	anotherMethod() {
		return this.#usedMethod();
	}
}

class UsedMember3 {
	get #usedAccessor() {}
	set #usedAccessor(value) {}

	method() {
		this.#usedAccessor = 42;
	}
}

class UsedMember4 {
	publicMember = 42;
}

class UsedMember5 {
	#usedMember = 42;
	anotherMember = this.#usedMember;
}

class UsedMember6 {
	#usedMember = 42;
	foo() {
		this.#usedMember = this.#usedMember;
	}
}

class UsedMember7 {
	#usedMember;

	foo() {
		bar(this.#usedMember += 1);
	}
}

class UsedMember8 {
	#usedMember = 42;
	method() {
		return someGlobalMethod(this.#usedMember);
	}
}

class UsedMember9 {
	#usedInOuterClass;

	foo() {
		return class {};
	}

	bar() {
		return this.#usedInOuterClass;
	}
}

class UsedMember10 {
	#usedInForInLoop;
	method() {
		for (const bar in this.#usedInForInLoop) {

		}
	}
}

class UsedMember11 {
	#usedInForOfLoop;
	method() {
		for (const bar of this.#usedInForOfLoop) {

		}
	}
}

class UsedMember12 {
	#usedInAssignmentPattern;
	method() {
		[bar = 1] = this.#usedInAssignmentPattern;
	}
}

class UsedMember13 {
	#usedInArrayPattern;
	method() {
		[bar] = this.#usedInArrayPattern;
	}
}

class UsedMember14 {
	#usedInAssignmentPattern;
	method() {
		[bar] = this.#usedInAssignmentPattern;
	}
}

class UsedMember15 {
	#usedInObjectAssignment;

	method() {
		({ [this.#usedInObjectAssignment]: a } = foo);
	}
}

class UsedMember16 {
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

class UsedMember17 {
	set #accessorUsedInMemberAccess(value) {}

	method(a) {
		[this.#accessorUsedInMemberAccess] = a;
	}
}

class UsedMember18 {
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

// issue #6933
class UsedPreUpdateExpr {
  #val = 0;
  method() {
    return ++this.#val;
  }
}

// issue #6933
class UsedPostUpdateExpr {
  #val = 0;
  method() {
    return this.#val++;
  }
}

class C2 {
	#usedOnlyInIncrement;

	foo() {
		this.#usedOnlyInIncrement++;
	}
}
