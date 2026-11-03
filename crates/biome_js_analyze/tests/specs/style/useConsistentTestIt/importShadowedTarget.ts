import { test } from "vitest";
test("first", () => {});
for (const it of cases) {
	test(it.name, () => {});
}
