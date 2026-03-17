// should not generate diagnostics

export class Foo<O extends Record<string, any>> {
	name: string;
	options: O;

	constructor(name: string, opts?: 0);
	constructor(options: O)
	constructor(nameOrOptions: string | O, opts?: O) {
		this.name = typeof nameOrOptions === 'string' ? nameOrOptions : 'unknown';
		this.options = opts ?? {} as O;
	}
}
