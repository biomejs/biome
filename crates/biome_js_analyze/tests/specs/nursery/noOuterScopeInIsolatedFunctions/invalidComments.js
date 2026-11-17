/* should generate diagnostics */
const foo = "hi";

/** @isolated */
function declaration() {
	return foo.slice();
}

/** @isolated */
const arrow = () => foo.slice();

/* @isolated */
const blockComment = () => foo.slice();

// @isolated
const lineComment = () => foo.slice();

/**
 * @isolated
 */
const multilineJsdoc = () => foo.slice();

// @isolated - runs in a worker
const withExplanation = () => foo.slice();

// @isolated -- runs in a worker
const withDoubleDashExplanation = () => foo.slice();

// @ISOLATED
const uppercase = () => foo.slice();

const inline = /* @isolated */ () => foo.slice();

// @isolated
export const exported = () => foo.slice();

// @isolated
export function exportedDeclaration() {
	return foo.slice();
}

// @isolated
export default () => foo.slice();

const object = {
	/** @isolated */
	method() {
		return foo.slice();
	},
	/** @isolated */
	property: () => foo.slice(),
	/** @isolated */
	functionProperty: function () {
		return foo.slice();
	},
	/** @isolated */
	usesThis() {
		return this.foo;
	},
	/** @isolated */
	arrowUsesThis: () => this.foo,
	/** @isolated */
	get getter() {
		return foo;
	},
	/** @isolated */
	set setter(value) {
		foo.slice(value);
	},
};

class Example extends Base {
	/** @isolated */
	constructor() {
		super();
		console.log(foo);
	}

	/** @isolated */
	get getter() {
		return foo;
	}

	/** @isolated */
	set setter(value) {
		foo.slice(value);
	}

	/** @isolated */
	method() {
		return foo.slice();
	}

	/** @isolated */
	usesThis() {
		return this.foo;
	}

	/** @isolated */
	usesSuper() {
		return super.foo;
	}

	/** @isolated */
	nestedArrow() {
		const getValue = () => this.foo + super.foo;
		return getValue;
	}
}

wrapper(/* @isolated */ () => foo.slice());
