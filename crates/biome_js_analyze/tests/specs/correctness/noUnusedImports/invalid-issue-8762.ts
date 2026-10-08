/* should generate diagnostics */

// Without a `@module` tag, a JSDoc comment that isn't attached to a
// declaration doesn't count as a usage.

/** {@link Foo} */

import type { Foo } from "./foo";
import type { Bar } from "./bar";
import type { Baz } from "./baz";

/** {@link Bar} */
console.log("side effect");

/** {@link Baz} @modules */
