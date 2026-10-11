// should generate diagnostics
import { describe } from "vitest";
import * as hooks from "vitest";

describe("as assertion", () => {
    const beforeEach = hooks.beforeEach as typeof hooks.beforeEach;
    beforeEach(() => {});
    beforeEach(() => {});
});

describe("satisfies assertion", () => {
    const afterEach = hooks.afterEach satisfies typeof hooks.afterEach;
    afterEach(() => {});
    afterEach(() => {});
});

describe("non-null assertion", () => {
    const beforeAll = hooks.beforeAll!;
    beforeAll(() => {});
    beforeAll(() => {});
});

describe("type assertion", () => {
    const afterAll = <typeof hooks.afterAll>hooks.afterAll;
    afterAll(() => {});
    afterAll(() => {});
});

describe("asserted namespace", () => {
    const beforeEach = (hooks as typeof hooks).beforeEach!;
    beforeEach(() => {});
    beforeEach(() => {});
});
