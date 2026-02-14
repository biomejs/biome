/* should not generate diagnostics */
import type { Foo } from "./b.ts";
import type { Baz } from "./c.ts";

export type Bar = {};

export const baz: Baz = {};
