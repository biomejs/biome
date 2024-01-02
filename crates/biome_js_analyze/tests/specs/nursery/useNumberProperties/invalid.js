const foo = { [NaN]: 1 };

const foo = { [NaN]() {} };

foo[NaN] = 1;

class A {
	[NaN]() {}
}

foo = { [NaN]: 1 };

const foo = Infinity;

if (Object.is(foo, Infinity)) {
}

const foo = bar[Infinity];

const foo = { Infinity };

const foo = { Infinity: Infinity };

const foo = { [Infinity]: -Infinity };

const foo = { [-Infinity]: Infinity };

const foo = { Infinity: -Infinity };

const { foo = Infinity } = {};

const { foo = -Infinity } = {};

const foo = Infinity.toString();

const foo = -Infinity.toString();

const foo = (-Infinity).toString();

const foo = +Infinity;

const foo = +-Infinity;

const foo = -Infinity;

const foo = -(-Infinity);

const foo = 1 - Infinity;

const foo = 1 - -Infinity;

const isPositiveZero = (value) => value === 0 && 1 / value === Infinity;

const isNegativeZero = (value) => value === 0 && 1 / value === -Infinity;

const { a = NaN } = {};

const { [NaN]: a = NaN } = {};

const [a = NaN] = [];

function foo({ a = NaN }) {}

function foo({ [NaN]: a = NaN }) {}

function foo([a = NaN]) {}

function foo() {
	return -Infinity;
}

globalThis.parseFloat(foo);

// Biome doesn't suport `global` for global namespacing
// global.parseFloat(foo);

window.parseFloat(foo);

// Biome doesn't suport `self` for global namespacing
// self.parseFloat(foo);

globalThis.NaN - globalThis.Infinity;
