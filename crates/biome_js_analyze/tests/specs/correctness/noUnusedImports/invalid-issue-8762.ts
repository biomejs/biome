/* should generate diagnostics */

// A `@module` comment only counts in the file header, before the first line
// of code. Both comments below come after code, so neither counts.

import type { Foo } from "./foo";
import type { Bar } from "./bar";

/**
 * Documents {@link Foo}.
 * @module
 */
console.log("side effect");

/**
 * Documents {@link Bar}.
 * @module
 */
