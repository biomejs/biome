// should generate diagnostics
import * as hooks from "vitest";

describe("hoisted function and var", () => {
    var beforeEach = hooks.beforeEach;
    function beforeEach() {}
    beforeEach(() => {});
    beforeEach(() => {});
});

describe("parameter and var", () => {
    (function (beforeEach) {
        beforeEach(() => {});
        beforeEach(() => {});
        var beforeEach = () => {};
    })(hooks.beforeEach);
});

describe("destructured parameter and var", () => {
    (function ({ afterEach }) {
        afterEach(() => {});
        afterEach(() => {});
        var afterEach = () => {};
    })(hooks);
});

describe("parameter and function declaration", function (beforeEach) {
    function beforeEach() {}
    beforeEach();
    beforeEach();
});
