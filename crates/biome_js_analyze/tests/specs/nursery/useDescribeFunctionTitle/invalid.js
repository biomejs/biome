/* should generate diagnostics */
import { myFunction } from "./myFunction";
import { parse as parseJson } from "./parse";
import defaultFunction from "./defaultFunction";

describe("myFunction", () => {});

describe(myFunction.name, () => {});

describe('myFunction', function () {});

describe("parseJson", () => {});

describe("defaultFunction", () => {});

describe.only("myFunction", () => {});

describe.skip(myFunction.name, () => {});

describe.concurrent("myFunction", () => {});

suite("myFunction", () => {});

describe(/* before */ "myFunction" /* after */, () => {});

if (process.env.CI) {
	describe("myFunction", () => {});
}

describe("outer", () => {
	describe("myFunction", () => {});
});

describe.shuffle("myFunction", () => {});

describe.only.concurrent("myFunction", () => {});

xdescribe("myFunction", () => {});

fdescribe("myFunction", () => {});

describe.each([[1]])("myFunction", (value) => {});

describe.for([[1]])(myFunction.name, ([value]) => {});

describe.skipIf(process.env.CI)("myFunction", () => {});

describe.runIf(process.env.CI)("myFunction", () => {});

describe.each`
	value
	${1}
`("myFunction", ({ value }) => {});

describe.only.each([[1]])("myFunction", (value) => {});

describe.skip.each`
	value
	${1}
`("myFunction", ({ value }) => {});
