// should generate diagnostics
import { describe, beforeEach } from "vitest";
import { beforeEach as hooksBeforeEach } from "vitest";

describe("duplicate imported hook", () => {
    beforeEach(() => {});
    {
        const beforeEach = () => {};
        beforeEach();
        beforeEach();
    }
    beforeEach(() => {});
});

describe("assigned hook", () => {
    let beforeEach;
    beforeEach = hooksBeforeEach;
    beforeEach(() => {});
    beforeEach(() => {});
});

describe("reassigned function", () => {
    let beforeEach = () => {};
    beforeEach = hooksBeforeEach;
    beforeEach(() => {});
    beforeEach(() => {});
});
