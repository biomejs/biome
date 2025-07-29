/* should not generate diagnostics */

function ignore() {
}

const ignore = function () {
};
const ignore = () => {
};

const container = {member: true};
container.member;

const container = {member: 1};
+container.member;

const container = {member: 1};
++container.member;

const container = {member: 1};
container.member++;

const container = {member: 1};
-container.member;

const container = {member: 1};
--container.member;

const container = {member: 1};
container.member--;

class TestEmpty {
}

class TestReadonlyStatic {
	private static readonly correctlyReadonlyStatic = 7;
}

class TestReadonlyStatic {
	static readonly #correctlyReadonlyStatic = 7;
}

class TestModifiableStatic {
	private static correctlyModifiableStatic = 7;

	public constructor() {
		TestModifiableStatic.correctlyModifiableStatic += 1;
	}
}

class TestModifiableStatic {
	static #correctlyModifiableStatic = 7;

	public constructor() {
		TestModifiableStatic.#correctlyModifiableStatic += 1;
	}
}

class TestModifiableByParameterProperty {
	private static readonly correctlyModifiableByParameterProperty = 7;

	public constructor(
		public correctlyModifiablePublicParameter: number = (() => {
			return (TestModifiableStatic.correctlyModifiableByParameterProperty += 1);
		})(),
	) {
	}
}

class TestModifiableByParameterProperty {
	static readonly #correctlyModifiableByParameterProperty = 7;

	public constructor(
		public correctlyModifiablePublicParameter: number = (() => {
			return (TestModifiableStatic.#correctlyModifiableByParameterProperty += 1);
		})(),
	) {
	}
}

class TestModifiableWithinConstructor {
	private correctlyModifiableWithinConstructor = 7;

	public constructor() {
		(() => {
			this.correctlyModifiableWithinConstructor += 1;
		})();
	}
}

class TestModifiableWithinConstructor {
	#correctlyModifiableWithinConstructor = 7;

	public constructor() {
		(() => {
			this.#correctlyModifiableWithinConstructor += 1;
		})();
	}
}

class TestModifiableWithinConstructorArrowFunction {
	private correctlyModifiableWithinConstructorArrowFunction = 7;

	public constructor() {
		(() => {
			this.correctlyModifiableWithinConstructorArrowFunction += 1;
		})();
	}
}

class TestModifiableWithinConstructorArrowFunction {
	#correctlyModifiableWithinConstructorArrowFunction = 7;

	public constructor() {
		(() => {
			this.#correctlyModifiableWithinConstructorArrowFunction += 1;
		})();
	}
}

class TestModifiableWithinConstructorInSetAccessor {
	#correctlyModifiableWithinConstructorInSetAccessor = 7;

	public constructor() {
		const confusingObject = {
			set accessor(value: number) {
				this.#correctlyModifiableWithinConstructorInSetAccessor += value;
			},
		};
	}
}

class TestReadonlyInline {
	private readonly correctlyReadonlyInline = 7;
}

class TestReadonlyInline {
	readonly #correctlyReadonlyInline = 7;
}

class TestReadonlyDelayed {
	private readonly correctlyReadonlyDelayed = 7;

	public constructor() {
		this.correctlyReadonlyDelayed += 1;
	}
}

class TestReadonlyDelayed {
	readonly #correctlyReadonlyDelayed = 7;

	public constructor() {
		this.#correctlyReadonlyDelayed += 1;
	}
}

class TestModifiableInline {
	private correctlyModifiableInline = 7;

	public mutate() {
		this.correctlyModifiableInline += 1;

		return class {
			private correctlyModifiableInline = 7;

			mutate() {
				this.correctlyModifiableInline += 1;
			}
		};
	}
}

class TestModifiableInline {
	#correctlyModifiableInline = 7;

	public mutate() {
		this.#correctlyModifiableInline += 1;

		return class {
			#correctlyModifiableInline = 7;

			mutate() {
				this.#correctlyModifiableInline += 1;
			}
		};
	}
}

class TestModifiableDelayed {
	private correctlyModifiableDelayed = 7;

	public mutate() {
		this.correctlyModifiableDelayed += 1;
	}
}

class TestModifiableDelayed {
	#correctlyModifiableDelayed = 7;

	public mutate() {
		this.#correctlyModifiableDelayed += 1;
	}
}

class TestModifiablePostDecremented {
	private correctlyModifiablePostDecremented = 7;

	public mutate() {
		this.correctlyModifiablePostDecremented -= 1;
	}
}

class TestModifiablePostDecremented {
	#correctlyModifiablePostDecremented = 7;

	public mutate() {
		this.#correctlyModifiablePostDecremented -= 1;
	}
}

class TestyModifiablePostIncremented {
	private correctlyModifiablePostIncremented = 7;

	public mutate() {
		this.correctlyModifiablePostIncremented += 1;
	}
}

class TestyModifiablePostIncremented {
	#correctlyModifiablePostIncremented = 7;

	public mutate() {
		this.#correctlyModifiablePostIncremented += 1;
	}
}

class TestModifiablePreDecremented {
	private correctlyModifiablePreDecremented = 7;

	public mutate() {
		--this.correctlyModifiablePreDecremented;
	}
}

class TestModifiablePreDecremented {
	#correctlyModifiablePreDecremented = 7;

	public mutate() {
		--this.#correctlyModifiablePreDecremented;
	}
}

class TestModifiablePreIncremented {
	private correctlyModifiablePreIncremented = 7;

	public mutate() {
		++this.correctlyModifiablePreIncremented;
	}
}

class TestModifiablePreIncremented {
	#correctlyModifiablePreIncremented = 7;

	public mutate() {
		++this.#correctlyModifiablePreIncremented;
	}
}

class TestProtectedModifiable {
	protected protectedModifiable = 7;
}

class TestPublicModifiable {
	public publicModifiable = 7;
}

class TestReadonlyParameter {
	public constructor(private readonly correctlyReadonlyParameter = 7) {
	}
}

class TestCorrectlyModifiableParameter {
	public constructor(private correctlyModifiableParameter = 7) {
	}

	public mutate() {
		this.correctlyModifiableParameter = 1;
	}
}

class TestComputedParameter {
	public mutate() {
		this['computed'] = 1;
	}
}

class TestComputedParameter {
	private ['computed-ignored-by-rule'] = 1;
}

class Foo {
	private value: number = 0;

	bar(newValue: { value: number }) {
		({value: this.value} = newValue);
		return this.value;
	}
}

class Foo {
	#value: number = 0;

	bar(newValue: { value: number }) {
		({value: this.#value} = newValue);
		return this.#value;
	}
}

function ClassWithName<TBase extends new (...args: any[]) => {}>(Base: TBase) {
	return class extends Base {
		private _name: string;

		public test(value: string) {
			this._name = value;
		}
	};
}

function ClassWithName<TBase extends new (...args: any[]) => {}>(Base: TBase) {
	return class extends Base {
		#name: string;

		public test(value: string) {
			this.#name = value;
		}
	};
}

class Foo {
	private value: Record<string, number> = {};

	bar(newValue: Record<string, number>) {
		({...this.value} = newValue);
		return this.value;
	}
}

class Foo {
	#value: Record<string, number> = {};

	bar(newValue: Record<string, number>) {
		({...this.#value} = newValue);
		return this.#value;
	}
}

class Foo {
	#value: Record<string, number> = {};

	bar(newValue: Record<string, number>) {
		({...this.#value} = newValue);
		let value = 3;
		({value, ...this.#value} = newValue);
		return this.#value;
	}
}

class Foo {
	private value: number[] = [];

	bar(newValue: number[]) {
		[...this.value] = newValue;
		return this.value;
	}
}

class Foo {
	#value: number[] = [];

	bar(newValue: number[]) {
		[...this.#value] = newValue;
		return this.#value;
	}
}

class Foo {
	private value: number = 0;

	bar(newValue: number[]) {
		[this.value] = newValue;
		return this.value;
	}
}

class Foo {
	#value: number = 0;

	bar(newValue: number[]) {
		[this.#value] = newValue;
		return this.#value;
	}
}

class Test {
	private testObj = {
		prop: '',
	};

	public test(): void {
		this.testObj = '';
	}
}

class Test {
	#testObj = {
		prop: '',
	};

	public test(): void {
		this.#testObj = '';
	}
}

class TestObject {
	public prop: number;
}

class Test {
	private testObj = new TestObject();

	public test(): void {
		this.testObj = new TestObject();
	}
}

class TestObject {
	public prop: number;
}

class Test {
	#testObj = new TestObject();

	public test(): void {
		this.#testObj = new TestObject();
	}
}

class TestStaticPrivateAccessor {
	private static accessor staticAcc = 1;
}

class TestStaticPrivateFieldAccessor {
	static accessor #staticAcc = 1;
}

class TestPrivateAccessor {
	private accessor acc = 3;
}

class TestPrivateFieldAccessor {
	accessor #acc = 3;
}

class TestModifiableWithinConstructorInFunctionExpression {
	private correctlyModifiableWithinConstructorInFunctionExpression = 7;

	public constructor() {
		const self = this;

		(() => {
			self.correctlyModifiableWithinConstructorInFunctionExpression += 1;
		})();
	}
}

class TestModifiableWithinConstructorInFunctionExpression {
	#correctlyModifiableWithinConstructorInFunctionExpression = 7;

	public constructor() {
		const self = this;

		(() => {
			self.#correctlyModifiableWithinConstructorInFunctionExpression += 1;
		})();
	}
}


class TestModifiableWithinConstructorInMethodDeclaration {
	private correctlyModifiableWithinConstructorInMethodDeclaration = 7;

	public constructor() {
		const self = this;

		const confusingObject = {
			methodDeclaration() {
				self.correctlyModifiableWithinConstructorInMethodDeclaration = 7;
			},
		};
	}
}

class TestModifiableWithinConstructorInSetAccessor {
	private correctlyModifiableWithinConstructorInSetAccessor = 7;

	public constructor() {
		const self = this;

		const confusingObject = {
			set accessor(value: number) {
				self.correctlyModifiableWithinConstructorInSetAccessor += value;
			},
		};
	}
}

class TestModifiableWithinConstructorInGetAccessor {
	private correctlyModifiableWithinConstructorInGetAccessor = 7;

	public constructor() {
		const self = this;

		const confusingObject = {
			get accessor() {
				return (self.correctlyModifiableWithinConstructorInGetAccessor += 1);
			},
		};
	}
}

class PriceIfElse {
	#price: string;

	@Input()
	set some(value: string | number) {
		if (value === undefined || value === null || value === 'undefined' || value === 'null' || Number.isNaN(value)) {
			this.#price = '';
		} else {
			this.#price = '' + value;
		}
	}
}

class PriceElse {
	#price: string;

	set some(value: string | number) {
		if (value === undefined) {
			consle.info("ignore undefined value");
		} else {
			this.#price = '' + value;
		}
	}
}

class PriceTernaryAllBranches {
	#price: string;

	set some(value: string | number) {
		value !== undefined ? this.#price = value : this.#price = 2;
	}
}

class PriceTernaryMatchBranch {
	#price: string;

	set some(value: string | number) {
		value !== undefined ? this.#price = value : console.info("ignore undefined value");
	}
}

class PriceTernaryNonMatchBranch {
	#price: string;

	set some(value: string | number) {
		value !== undefined ? console.info("ignore non undefined value") : this.#price = value;
	}
}

class GetterWithMutationValue {
	#value: string;

	get value() {
		if (!this.#value) {
			this.#value = "defaultValue";
		}

		return this.#value;
	}
}

class ArrowFunctionWithMutation {
	private bar: string | null = null;

	readonly action = () => {
		this.bar = "init";
	};
}

class Counter {
	private counter: number

	count() {
		console.log(this.counter++);
	}
}

class Counter2 {
	private counter: number

	count() {
		const counterString = `${this.counter++}`
	}
}

class Counter3 {
	private counter: number

	async count() {
		this.counter = 1
		const counterString = `${this.counter++}`
	}
}

class Counter4 {
	private counter: number

	async count() {
		await console.log(this.counter++)
		const counterString = await `${this.counter++}`
	}
}

export class Test {
	private field: number;

	someMethod() {
		this.field ??= 1;
	}
}

export class Test {
	private field: number;

	someMethod() {
		this.field &&= 1;
	}
}

export class Test {
	private field: number;

	someMethod() {
		this.field ||= 1;
	}
}

export class ToastService {
	activeToasts: Array<{ id: number; message: string; type: string; autoClose: boolean }> = [];
	private _toastId = 0;

	show(message: string, type: string, autoClose: boolean): void {
		const id = this._toastId++;
		this.activeToasts.push({id, message, type, autoClose});
	}
}
