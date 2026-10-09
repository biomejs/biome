/**
 * Documents {@link Foo}.
 * @module
 */

/* should generate diagnostics */

import type { Foo } from "./foo";
import type { Bar } from "./bar";

/**
 * A second `@module` comment is not the module comment: {@link Bar} is
 * still unused.
 * @module
 */
console.log("side effect");
