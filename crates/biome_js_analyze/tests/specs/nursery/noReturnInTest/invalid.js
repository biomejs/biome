/* should generate diagnostics */
test("arrow function", () => {
	return expect(1).toBe(1);
});

it("function expression", function () {
	return expect(1).toBe(1);
});

it.skip("skipped test", function () {
	return expect(1).toBe(1);
});

test.concurrent.only("chained modifiers", () => {
	return expect(1).toBe(1);
});

xit("disabled alias", () => {
	return expect(1).toBe(1);
});

fit("focused alias", () => {
	return expect(1).toBe(1);
});

Deno.test("deno test", () => {
	return expect(1).toBe(1);
});

it.each``("tagged table", function () {
	return expect(1).toBe(1);
});

it.each()("table", function () {
	return expect(1).toBe(1);
});

it.only.each``("focused tagged table", function () {
	return expect(1).toBe(1);
});

it.only.each()("focused table", function () {
	return expect(1).toBe(1);
});

test("async function", async () => {
	return page.goto("https://example.com");
});

test("promise chain", () => {
	return fetchUser().then((user) => expect(user.name).toBe("Ada"));
});

test("return without a value", () => {
	expect(1).toBe(1);
	return;
});

test("return before the end", () => {
	return;
	expect(1).toBe(1);
});

test("options before the function", { retry: 2 }, () => {
	return expect(1).toBe(1);
});

test("timeout after the function", () => {
	return expect(1).toBe(1);
}, 1000);

test("parenthesized function", (() => {
	return expect(1).toBe(1);
}));

it("function declaration", declaredTest);
function declaredTest() {
	return expect(1).toBe(1);
}

it("first use of a shared function", sharedTest);
it("second use of a shared function", sharedTest);
function sharedTest() {
	return expect(1).toBe(1);
}

const arrowTest = () => {
	return expect(1).toBe(1);
};
test("arrow function in a variable", arrowTest);

const functionTest = function () {
	return expect(1).toBe(1);
};
test("function expression in a variable", (functionTest));

describe("suite", () => {
	it("nested test", () => {
		return expect(1).toBe(1);
	});
});
