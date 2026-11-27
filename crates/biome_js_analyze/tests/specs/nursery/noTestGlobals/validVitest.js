/* should not generate diagnostics */
import { describe, expect, it, vi } from "vitest";

describe("sum", () => {
	it("adds two numbers", () => {
		expect(vi.fn()).not.toHaveBeenCalled();
	});
});

// Vitest doesn't provide these Jest global variables.
jest.useFakeTimers();
fdescribe("focused", () => {
	fit("runs", () => {});
});
xdescribe("skipped", () => {
	xit("is skipped", () => {});
	xtest("is skipped", () => {});
});
