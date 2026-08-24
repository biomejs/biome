/* should not generate diagnostics */
import { fc, test } from "@fast-check/vitest";
import { expect } from "vitest";

test.prop([fc.string()])("round-trips", (s) => {
	expect(s).toBe(s);
});

test.concurrent.prop([fc.string()])("round-trips", (s) => {
	expect(s).toBe(s);
});
