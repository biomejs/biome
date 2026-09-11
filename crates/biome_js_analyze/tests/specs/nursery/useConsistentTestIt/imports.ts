import { fc, test } from "@fast-check/vitest";

test.prop([fc.string()])("length is non-negative", (s) => s.length >= 0);
