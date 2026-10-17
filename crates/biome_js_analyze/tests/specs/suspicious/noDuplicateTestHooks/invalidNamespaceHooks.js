// should generate diagnostics
import { describe } from "vitest";
import * as hooks from "vitest";

describe("namespace import", () => {
    const { afterEach } = hooks;
    afterEach(() => {});
    afterEach(() => {});
});

describe("global namespace", () => {
    const after = globalThis.after;
    after(() => {});
    after(() => {});
});

describe("aliased namespace import", () => {
    const alias = hooks;
    const beforeAll = alias.beforeAll;
    beforeAll(() => {});
    beforeAll(() => {});
});
