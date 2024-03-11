test("something", () => {});
test("something", async () => {});
test("something", function () {});
test.each``("something", ({ a, b }) => {});
test.each()("something", ({ a, b }) => {});
it.each()("something", ({ a, b }) => {});
it.each([])("something", (a, b) => {});
it.each``("something", ({ a, b }) => {});
it.each([])("something", (a, b) => {
    a();
    b();
});
it.each``("something", ({ a, b }) => {
    a();
    b();
});
test("something", async function () {});
test("something", someArg);
beforeEach(() => {});
beforeAll(async () => {});
afterAll(() => {});
afterAll(async function () {});
afterAll(async function () {}, 5);
