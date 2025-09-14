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

// issue #6994
class UsedAssignmentExpr {
  #val = 0;
  method() {
    return this.#val = 1
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

class AppSelfAdd {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest += this.#persistenceRequest;
	}
}

class AppSelfSubtract {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest -= this.#persistenceRequest;
	}
}

class AppSelfMultiply {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest *= this.#persistenceRequest;
	}
}

class AppSelfDivide {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest /= this.#persistenceRequest;
	}
}

class AppSelfExponent {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest **= this.#persistenceRequest;
	}
}

class AppSelfAnd {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest &&= this.#persistenceRequest;
	}
}

class AppSelfOr {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest ||= this.#persistenceRequest;
	}
}

class AppSelfNullish {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest ??= this.#persistenceRequest;
	}
}

class AppAddAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest += 1;
	}
}

class AppSubtractAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest -= 1;
	}
}

class AppMultiplyAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest *= 2;
	}
}

class AppDivideAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest /= 2;
	}
}

class AppExponentAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest **= 2;
	}
}

class AppModuloAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest %= 2;
	}
}

class AppAndAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest &= 1;
	}
}

class AppOrAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest |= 1;
	}
}

class AppXorAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest ^= 1;
	}
}

class AppLeftShiftAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest <<= 1;
	}
}

class AppRightShiftAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest >>= 1;
	}
}

class AppUnsignedRightShiftAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest >>>= 1;
	}
}

class AppAndLogicalAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest &&= 1;
	}
}

class AppOrLogicalAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest ||= 1;
	}
}

class AppNullishAssignment {
	#persistenceRequest = 0;
	saveData() {
		this.#persistenceRequest ??= 1;
	}
}
