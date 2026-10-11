// should generate diagnostics
const { describe, beforeEach } = require("node:test");

describe("duplicate CommonJS hook", () => {
    beforeEach(() => {});
    beforeEach(() => {});
});

describe("duplicate member import", () => {
    const afterEach = require("node:test").afterEach;
    afterEach(() => {});
    afterEach(() => {});
});

describe("duplicate computed import", () => {
    const afterAll = require("vitest")["afterAll"];
    afterAll(() => {});
    afterAll(() => {});
});
