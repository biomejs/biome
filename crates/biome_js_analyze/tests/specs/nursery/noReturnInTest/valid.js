/* should not generate diagnostics */
it("noop", function () {});
test("noop", () => {});
test("expression body", () => expect(1).toBe(1));
test("no function");

test("no return", () => {
	expect(1).toBe(1);
});

it("no return", function () {
	expect(1).toBe(1);
});

it("await", async () => {
	await new Promise((resolve) => setTimeout(resolve, 100));
	expect(1).toBe(1);
});

it("function declaration", declaredTest);
function declaredTest() {
	expect(1).toBe(1);
}

test("return inside another statement", () => {
	if (isWindows) {
		return;
	}
	for (const value of values) {
		if (!value) {
			return expect(value).toBe(0);
		}
	}
	expect(1).toBe(1);
});

test("return inside a nested function", () => {
	const doubled = [1, 2].map((value) => {
		return value * 2;
	});
	function helper() {
		return 1;
	}
	expect(doubled).toEqual([2, 4]);
});

// Functions that are not passed to a test.
function helper() {
	return 1;
}
const arrowHelper = () => {
	return 1;
};
test("calls helpers", () => {
	expect(helper()).toBe(1);
	expect(arrowHelper()).toBe(1);
	expect(helper).toBeDefined();
});
[1, 2].forEach(function () {
	return 1;
});
const object = {
	test() {
		return 1;
	},
};

// Calls that are not tests.
describe("suite", () => {
	return;
});
test.describe("suite", () => {
	return;
});
describe.each([1, 2])("suite %s", () => {
	return;
});
describe.each``("suite", () => {
	return;
});
describe.shuffle("suite", () => {
	return;
});
describe.shuffle.each([1, 2])("suite %s", () => {
	return;
});
test.describe.parallel("suite", () => {
	return;
});
test.describe.serial.only("suite", () => {
	return;
});
beforeEach(() => {
	return setup();
});
run("not a test", () => {
	return 1;
});
object.test("not a test", () => {
	return 1;
});
/test/.test("not a test", () => {
	return 1;
});
it.each(() => {
	return [1, 2];
})("table from a function", () => {});

// A step hands the returned value back to its caller.
test("steps", async () => {
	const user = await test.step("log in", async () => {
		return login();
	});
	await test.step.skip("log out", async () => {
		return logout();
	});
	expect(user).toBeDefined();
});

// A property returns whether it holds.
test.prop([fc.nat(), fc.nat()])("addition is commutative", (a, b) => {
	return a + b === b + a;
});
it.prop([fc.string()])("trimming twice changes nothing", (text) => {
	return text.trim().trim() === text.trim();
});

// The first argument decides whether the tests are skipped.
test.skip(({ browserName }) => {
	return browserName === "webkit";
}, "not supported in WebKit");
test.skip(shouldSkip, "not supported");
function shouldSkip() {
	return true;
}

// Only functions passed directly to the test are checked.
test("function in an options object", { retry: () => { return 2; } }, () => {});
test("wrapped function", wrap(() => {
	return 1;
}));
