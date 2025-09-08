// Static properties
class TestIncorrectlyModifiableStatic {
	private static incorrectlyModifiableStatic = 7;
}

// Private static fields
class TestIncorrectlyModifiableStaticPrivate {
	static #incorrectlyModifiableStatic = 7;
}

// Static arrow functions
class TestIncorrectlyModifiableStaticArrow {
	private static incorrectlyModifiableStaticArrow = () => 7;
}

//  Private static arrow functions
class TestIncorrectlyModifiableStaticArrowPrivate {
	static #incorrectlyModifiableStaticArrow = () => 7;
}

// Nested classes with same property names
class TestIncorrectlyModifiableInline {
	private incorrectlyModifiableInline = 7;
	public createConfusingChildClass() {
		return class {
			private incorrectlyModifiableInline = 7;
		};
	}
}

// Nested classes with private fields
class TestIncorrectlyModifiableInlinePrivate {
	#incorrectlyModifiableInline = 7;
	public createConfusingChildClass() {
		return class {
			#incorrectlyModifiableInline = 7;
		};
	}
}

// Constructor reassignment
class TestIncorrectlyModifiableDelayed {
	private incorrectlyModifiableDelayed = 7;
	public constructor() {
		this.incorrectlyModifiableDelayed = 7;
	}
}

// Constructor reassignment with private field
class TestIncorrectlyModifiableDelayedPrivate {
	#incorrectlyModifiableDelayed = 7;
	public constructor() {
		this.#incorrectlyModifiableDelayed = 7;
	}
}

// Example 11: Subtraction operation
class TestIncorrectlyModifiablePostMinus {
	private incorrectlyModifiablePostMinus = 7;
	public mutate() {
		this.incorrectlyModifiablePostMinus - 1;
	}
}

// Example 12: Subtraction operation with private field
class TestIncorrectlyModifiablePostMinusPrivate {
	#incorrectlyModifiablePostMinus = 7;
	public mutate() {
		this.#incorrectlyModifiablePostMinus - 1;
	}
}

// Addition operation
class TestIncorrectlyModifiablePostPlus {
	private incorrectlyModifiablePostPlus = 7;
	public mutate() {
		this.incorrectlyModifiablePostPlus + 1;
	}
}

// Addition operation with private field
class TestIncorrectlyModifiablePostPlusPrivate {
	#incorrectlyModifiablePostPlus = 7;
	public mutate() {
		this.#incorrectlyModifiablePostPlus + 1;
	}
}

// Negation operation
class TestIncorrectlyModifiablePreMinus {
	private incorrectlyModifiablePreMinus = 7;
	public mutate() {
		-this.incorrectlyModifiablePreMinus;
	}
}

// Negation operation with private field
class TestIncorrectlyModifiablePreMinusPrivate {
	#incorrectlyModifiablePreMinus = 7;
	public mutate() {
		-this.#incorrectlyModifiablePreMinus;
	}
}

// Unary plus operation
class TestIncorrectlyModifiablePrePlus {
	private incorrectlyModifiablePrePlus = 7;
	public mutate() {
		+this.incorrectlyModifiablePrePlus;
	}
}

// Unary plus operation with private field
class TestIncorrectlyModifiablePrePlusPrivate {
	#incorrectlyModifiablePrePlus = 7;
	public mutate() {
		+this.#incorrectlyModifiablePrePlus;
	}
}

// Property with same name in different class
class TestOverlappingClassVariable {
	private overlappingClassVariable = 7;
	public workWithSimilarClass(other: SimilarClass) {
		other.overlappingClassVariable = 7;
	}
}
class SimilarClass {
	public overlappingClassVariable = 7;
}

// Parameter property
class TestIncorrectlyModifiableParameter {
	public constructor(private incorrectlyModifiableParameter = 7) {}
}

// Parameter property with other parameters
class TestIncorrectlyModifiableParameterWithOthers {
	public constructor(
		public ignore: boolean,
		private incorrectlyModifiableParameter = 7,
	) {}
}

// Inline lambda with option
class TestCorrectlyNonInlineLambdas {
	private incorrectlyInlineLambda = () => 7;
}

// Property in a higher-order class function
function ClassWithName<TBase extends new (...args: any[]) => {}>(Base: TBase) {
	return class extends Base {
		private _name: string;
	};
}

// Private field in a higher-order class function
function ClassWithNamePrivate<TBase extends new (...args: any[]) => {}>(Base: TBase) {
	return class extends Base {
		#name: string;
	};
}

// Object property access
class Test {
	private testObj = { prop: '' };
	public test(): void {
		this.testObj.prop = '';
	}
}

// Basic property initialization in constructor
enum Foo { Bar, Bazz }
const foo = Foo.Bar;
class Test1 {
	private prop = foo;
	constructor() {
		this.prop = foo;
	}
}

// Property with no constructor reassignment
class Test2 {
	private prop = foo;
}

// Using declared constant
enum Foo2 { Bar, Bazz }
declare const foo2: Foo2;
class Test3 {
	private prop = foo2;
}

// Property in nested scope with shadowing
enum Foo3 { Bar, Bazz }
const bar = Foo3.Bar;
function wrapper() {
	const Foo = 10;
	class Test4 {
		private prop = bar;
		constructor() {
			this.prop = bar;
		}
	}
}

// Property with type shadowing
function wrapper2() {
	type Foo = 10;
	class Test5 {
		private prop = bar;
		constructor() {
			this.prop = bar;
		}
	}
}

// Using IIFE for enum
const Bar = (function () {
	enum Foo4 { Bar, Bazz }
	return Foo4;
})();
const bar2 = Bar.Bar;
class Test6 {
	private prop = bar2;
	constructor() {
		this.prop = bar2;
	}
}

// Object property
class Test7 {
	private prop = { foo: 'bar' };
}

// Object property with constructor reassignment
class Test8 {
	private prop = { foo: 'bar' };
	constructor() {
		this.prop = { foo: 'bazz' };
	}
}

// Array property
class Test9 {
	private prop = [1, 2, 'three'];
}

// Array property with constructor reassignment
class Test10 {
	private prop = [1, 2, 'three'];
	constructor() {
		this.prop = [1, 2, 'four'];
	}
}

// Property used in method
class X {
	private _isValid = true;
	getIsValid = () => this._isValid;
	constructor(data?: {}) {
		if (!data) {
			this._isValid = false;
		}
	}
}

// Property with type annotation
class Test12 {
	private prop: string = 'hello';
}

// Property with union type
class Test13 {
	private prop: string | number = 'hello';
}

// Property with type but no initial value
class Test14 {
	private prop: string;
	constructor() {
		this.prop = 'hello';
	}
}

// Example 40: Property with no type annotation
class Test15 {
	private prop;
	constructor() {
		this.prop = 'hello';
	}
}

// Conditional assignment in constructor
class Test16 {
	private prop;
	constructor(x: boolean) {
		if (x) {
			this.prop = 'hello';
		} else {
			this.prop = 10;
		}
	}
}

// Null property
class Test18 {
	private prop = null;
}

// Null property with constructor reassignment
class Test19 {
	private prop = null;
	constructor() {
		this.prop = null;
	}
}

// Property with type assertion
class Test20 {
	private prop = 'hello' as string;
}

// Property with Promise
class Test21 {
	private prop = Promise.resolve('hello');
}

// this refers to the inner class instance
class TestChildClassExpressionModifiable {
	private childClassExpressionModifiable = 7;
	public createConfusingChildClass() {
		return class {
			private childClassExpressionModifiable = 7;
			mutate() {
				this.childClassExpressionModifiable += 1;
			}
		};
	}
}

export class Test {
	private field: number;

	constructor() {
		this.field ??= 1;
	}
}
