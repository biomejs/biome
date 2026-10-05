/* should not generate diagnostics */

export class Example {
	constructor(private something: string) {}

	example() {
		const { something } = this;
		return something;
	}
}
