/* should not generate diagnostics */

export class BadExample {
	constructor(private something: string) {}

	example() {
		const { something } = this
		return something
	}
}
