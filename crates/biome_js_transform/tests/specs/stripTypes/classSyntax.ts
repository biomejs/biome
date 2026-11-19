abstract class Foo<T> extends Base<T> implements Iface {
	private readonly a: string = 'a';
	b?: number;
	c!: string;
	declare d: number;
	abstract e(): void;
	[key: string]: unknown;
	override f?(this: Foo<T>, a?: T): void {}
	constructor() {
		super();
		this.c = 'c';
	}
}

abstract class Nesting {
	abstract g(): void;
	h() {
		abstract class Inner {
			abstract i: number;
		}
		return Inner;
	}
}
