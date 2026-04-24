/* should generate diagnostics */
// https://github.com/biomejs/biome/issues/10039
// The rule should also flag unreassigned private members in non-declaration
// class forms: anonymous and named class expressions, and anonymous export
// default classes. Named export default classes (`export default class Foo`)
// live in `invalid_classExportDefaultNamed.ts` because a module can only
// contain a single `export default`.

// Anonymous class expression
const AnonClass = class {
	#anonPrivate = 123;
	constructor() {
		console.log(this.#anonPrivate);
	}
};

// Named class expression
const NamedExpr = class Named {
	private namedProp = 456;
	constructor() {
		console.log(this.namedProp);
	}
};

// Anonymous export default class
export default class {
	#defaultPrivate = 789;
	constructor() {
		console.log(this.#defaultPrivate);
	}
}
