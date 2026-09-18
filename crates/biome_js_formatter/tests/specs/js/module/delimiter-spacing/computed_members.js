// =====================
// Computed member expressions - number literals
// =====================

// Integer literal
foo[0];

// Decimal number
foo[1.5];

// Multi-digit integer
foo[100];

// Optional chaining with number literal
foo?.[0];

// Nested computed member with number literals
foo[0][1];
foo[0][1][2];

// Number literal followed by property access
foo[0].bar;
foo[0].bar.baz;

// Property access followed by number literal
foo.bar[0];
foo.bar.baz[0];

// Number literal on function call result
foo()[0];
foo(a)[0];

// Number literal on method call result
foo.bar()[0];

// Number literal on array literal
[1, 2, 3][0];

// Number literal in assignment
foo[0] = a;

// In variable declaration
const a = foo[0];

// Very long number (boundary test - always adds spaces, no line break)
foo[999999999999999999999999999999999999999999999999999999999999999999999999];

// Contrast tests: These do NOT get spaces from number literal path
// Negative number (unary expression, not number literal)
foo[-1];

// Variable
foo[a];

// String literal
foo["a"];

// =====================
// Computed member expressions - dynamic expressions
// =====================

// Variable property access
foo[a];
foo[bar];

// Property access chain
foo[a][b];
foo[a][b][c];

// Mixed with dot notation
foo.bar[a];
foo[a].bar;

// Function call as property
foo[bar()];
foo[bar(a)];

// Arithmetic expressions
foo[a + 1];
foo[a - b];
foo[a * b];

// Unary expressions
foo[-a];
foo[typeof a];

// Ternary expressions
foo[a ? b : c];

// Logical expressions
foo[a && b];
foo[a || b];
foo[a ?? b];

// Template literal as property
foo[`${a}`];

// Object property as index
foo[bar.a];
foo[bar.a.b];

// Parenthesized expression
foo[(a)];
foo[(a + b)];

// Optional chaining with dynamic expressions
foo?.[a];
foo?.[bar()];

// In assignment
foo[a] = b;
foo[bar()] = baz;

// Boundary test - 79 chars, adding spaces makes 81, causing line break
foo[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa];

// =====================
// Computed member names
// =====================

// Computed property with string literal
const foo1 = {
	["a"]: 1,
};

// Computed property with variable
const foo2 = {
	[a]: 1,
};

// Computed property with expression
const foo3 = {
	[a + b]: 1,
};

// Computed property with function call
const foo4 = {
	[foo()]: 1,
};

// Computed property with template literal
const foo5 = {
	[`a`]: 1,
};

// Multiple computed properties
const foo6 = {
	[a]: 1,
	[b]: 2,
	[c]: 3,
};

// Mixed computed and regular properties
const foo7 = {
	a: 1,
	[b]: 2,
	c: 3,
	[d]: 4,
};

// Computed method name
const foo8 = {
	[a]() {},
};

// Computed async method
const foo9 = {
	async [a]() {},
};

// Computed generator method
const foo10 = {
	*[a]() {},
};

// Computed getter
const foo11 = {
	get [a]() {
		return b;
	},
};

// Computed setter
const foo12 = {
	set [a](b) {
		c = b;
	},
};

// Computed property in class
class Foo1 {
	[a] = 1;
}

// Computed method in class
class Foo2 {
	[a]() {}
}

// Computed static property in class
class Foo3 {
	static [a] = 1;
}

// Computed static method in class
class Foo4 {
	static [a]() {}
}

// Computed getter in class
class Foo5 {
	get [a]() {
		return b;
	}
}

// Computed setter in class
class Foo6 {
	set [a](b) {
		c = b;
	}
}

// Computed property with Symbol
const foo13 = {
	[Symbol.iterator]() {},
};

// Computed property in object destructuring
const { [a]: b } = foo;

// Long computed property name
const foo14 = {
	[loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong]: 1,
};
