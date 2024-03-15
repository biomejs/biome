test("something", (done) => {
    done();
});
test("something", (done) => {
    done();
});
test("something", (finished) => {
    finished();
});
test("something", (done) => {
    done();
});
test("something", (done) => done());
test("something", (done) => done());
test("something", function (done) {
    done();
});
test("something", function (done) {
    done();
});
test("something", async (done) => {
    done();
});
test("something", async (done) => done());
test("something", async function (done) {
    done();
});
test("something", (done) => {
    done();
});
beforeAll((done) => {
    done();
});
beforeAll((finished) => {
    finished();
});
beforeEach((done) => {
    done();
});
afterAll((done) => done());
afterEach((done) => done());
beforeAll(function (done) {
    done();
});
afterEach(function (done) {
    done();
});
beforeAll(async (done) => {
    done();
});
beforeAll(async (done) => done());
beforeAll(async function (done) {
    done();
});
beforeEach((done) => {
    done();
});
beforeEach((done) => {
    done();
});
it.each``("something", ({ a, b }, done) => {});
test.each``("something", ({ a, b }, done) => {});
