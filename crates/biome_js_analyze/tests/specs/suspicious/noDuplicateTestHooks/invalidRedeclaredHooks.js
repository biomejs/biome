// should generate diagnostics
import * as hooks from "vitest";

describe("redeclared hook", () => {
    var beforeEach = hooks.beforeEach;
    beforeEach(() => {});
    beforeEach(() => {});
    var beforeEach = () => {};
});

describe("escaped redeclaration", () => {
    var afterEach = hooks.afterEach;
    afterEach(() => {});
    afterEach(() => {});
    var aft\u0065rEach = () => {};
});
