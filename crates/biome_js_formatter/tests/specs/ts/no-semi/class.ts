class A {
  bar: A;
  [baz]

  // none of the semicolons above this comment can be omitted.
  // none of the semicolons below this comment are necessary.

  bar: A;
  private [baz]
}

const computed = "test";

class A {
	test: string

	private [computed]
	protected [computed]
	public [computed]
	static [computed]

	readonly [computed]

	[key: string]: string;

	async [computed]() {}
}

declare module test {
	class Declaration {
		prop: string;
		[computed];

		constructor()

		[computed]

		get getter()
		[computed]

		set setter(a: string)
		set setter(a: string, )
		[computed]

		method()

		[computed]

		[key: string]: string
		[computed]
	}
}
