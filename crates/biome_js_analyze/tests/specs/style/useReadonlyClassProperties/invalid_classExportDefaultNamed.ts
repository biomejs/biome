/* should generate diagnostics */
// https://github.com/biomejs/biome/issues/10039
// Named export default class. Kept in a separate file because a module
// can only contain a single `export default` statement.

export default class NamedDefault {
	#namedDefaultPrivate = 42;
	constructor() {
		console.log(this.#namedDefaultPrivate);
	}
}
