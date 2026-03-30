// Basic template expression
const basic = `${b}`;

// Template with prefix text
const withPrefix = `prefix ${b}`;

// Template with suffix text
const withSuffix = `${b} suffix`;

// Template with prefix and suffix
const withBoth = `prefix ${b} suffix`;

// Multiple expressions
const multiple1 = `${b} and ${c}`;
const multiple2 = `${b}${c}${d}`;
const multiple3 = `${b} ${c} ${d}`;

// Call expressions (no args)
const call1 = `${foo()}`;
const call2 = `${foo.bar()}`;
const call3 = `${foo.bar.baz()}`;

// Call expressions (with args)
const callWithArgs1 = `${foo(a)}`;
const callWithArgs2 = `${foo(a, b)}`;

// New expressions
const newExpr1 = `${new Foo()}`;
const newExpr2 = `${new Foo(a)}`;
const newExpr3 = `${new Foo(a, b)}`;

// Await expressions
const awaitExpr1 = `${await foo()}`;
const awaitExpr2 = `${await foo.bar()}`;

// Yield expressions
function* foo() {
	return `${yield b}`;
}
function* bar() {
	return `${yield* b}`;
}

// Array expressions
const arr1 = `${ []}`;
const arr2 = `${[1, 2, 3]}`;
const arr3 = `${[a, b, c]}`;

// Object expressions
const obj1 = `${{}}`;
const obj2 = `${{ a: 1 }}`;
const obj3 = `${{ a: 1, b: 2 }}`;

// Unary expressions
const unary1 = `${!b}`;
const unary2 = `${-b}`;
const unary3 = `${+b}`;
const unary4 = `${~b}`;
const unary5 = `${typeof b}`;
const unary6 = `${void b}`;

// Update expressions
const update1 = `${++b}`;
const update2 = `${--b}`;
const update3 = `${b++}`;
const update4 = `${b--}`;

// Static member expressions
const member1 = `${foo.bar}`;
const member2 = `${foo.bar.baz}`;

// Computed member expressions
const computed1 = `${foo[bar]}`;
const computed2 = `${foo[0]}`;
const computed3 = `${foo["a"]}`;

// Assignment expressions (parenthesized by formatter)
const assign1 = `${b = c}`;
const assign2 = `${b += c}`;
const assign3 = `${b -= c}`;

// Binary expressions
const binary1 = `${b + c}`;
const binary2 = `${b - c}`;
const binary3 = `${b * c}`;
const binary4 = `${b / c}`;
const binary5 = `${b % c}`;
const binary6 = `${b ** c}`;

// Logical expressions
const logical1 = `${b && c}`;
const logical2 = `${b || c}`;
const logical3 = `${b ?? c}`;

// Conditional expressions
const cond = `${b ? c : d}`;

// Sequence expressions
const seq1 = `${(b, c)}`;
const seq2 = `${(b, c, d)}`;

// Arrow function expressions
const arrow1 = `${() => b}`;
const arrow2 = `${(a) => b}`;
const arrow3 = `${(a, b) => c}`;

// Function expressions (body expands)
const func1 = `${function() { return b; }}`;
const func2 = `${function foo() { return b; }}`;

// Class expressions
const cls1 = `${class {}}`;
const cls2 = `${class Foo {}}`;

// Tagged templates inside templates
const tagged1 = `${foo`bar`}`;
const tagged2 = `${foo`${b}`}`;

// Nested template literals
const nested1 = `${`${b}`}`;
const nested2 = `outer ${`inner ${b}`} outer`;
const nested3 = `${`${`${b}`}`}`;

// Template in various contexts
function templateInFunc() {
	return `${b}`;
}
const templateInArrow = () => `${b}`;
const templateInObj = { a: `${b}` };
const templateInArr = [`${b}`];
const templateConcat = `${b}` + `${c}`;

// Tagged templates
foo`${b}`;
foo`prefix ${b}`;
foo`${b} suffix`;
foo`${b}${c}`;
foo.bar`${b}`;

// =====================
// Boundary tests (template elements don't break, they exceed line width)
// =====================

// Boundary: 78 chars without spaces, 80 with spaces (fits exactly)
const boundaryFit = `${aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa}`;

// Boundary: 79 chars without spaces, 81 with spaces (exceeds by 1)
const boundaryExceed = `${aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa}`;

// Template literals with newlines in chunks
const withNewlines = `first line
${b}
last line`;

// Template literals in class
class Foo {
	a = `${b}`;
	b() {
		return `${c}`;
	}
	static c = `${d}`;
}

// This and super expressions
const thisExpr1 = `${this}`;
const thisExpr2 = `${this.foo}`;
const thisExpr3 = `${this.foo()}`;
class Baz extends Bar {
	a() {
		return `${super.b}`;
	}
	b() {
		return `${super.b()}`;
	}
}

// Import meta and optional chaining
const importMeta = `${import.meta.url}`;
const optChain1 = `${foo?.bar}`;
const optChain2 = `${foo?.bar()}`;
const optChain3 = `${foo?.[bar]}`;
