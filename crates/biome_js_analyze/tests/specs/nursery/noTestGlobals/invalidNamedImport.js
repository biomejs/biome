/* should generate diagnostics */
import { describe } from "@jest/globals";
import { sum } from "./sum";

describe("sum", () => {
	it("adds two numbers", () => {
		expect(sum(1, 2)).toBe(3);
	});
});
