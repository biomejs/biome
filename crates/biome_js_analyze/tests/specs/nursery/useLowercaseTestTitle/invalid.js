/* should generate diagnostics */
describe("Foo", () => {});
describe('Foo', () => {});
describe(`Foo`, () => {});
it("Foo MM mm", function () {});
test(`Foo MM mm`, function () {});
test("SFC Compile", () => {});
xit("Foo", () => {});
fit("Foo", () => {});
xdescribe("Foo", () => {});
fdescribe("Foo", () => {});
it.only("Foo", () => {});
test.skip("Foo", () => {});
test.todo("Foo");
describe.concurrent("Foo", () => {});
suite("Foo", () => {});
test.describe("Foo", () => {});
test.describe.serial("Foo", () => {});
it.each([1, 2])("Foo %i", (n) => {});
describe.each([1, 2])("Foo %i", (n) => {});
test.each`
	a
	${1}
`("Foo $a", () => {});
test.skipIf(isCI)("Foo", () => {});
it.runIf(isCI)("Foo", () => {});
describe.skipIf(isCI)("Foo", () => {});
bench("Foo", () => {});
bench.skip("Foo", () => {});
test("Élan", () => {});
test("ǅemal", () => {});
test(/* comment */ "Foo" /* comment */, () => {});
describe("Outer", () => {
	it("Inner", () => {});
});
