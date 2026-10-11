/* should generate diagnostics */

// Only the first comment of a file can be its module comment. Every `@module`
// comment below comes after another comment, so none of them count.

/**
 * Documents {@link Foo}.
 * @module
 */

import type { Foo } from "./foo";
import type { Bar } from "./bar";
import type { Baz } from "./baz";

/**
 * Documents {@link Bar}.
 * @module
 */
console.log("side effect");

/**
 * Documents {@link Baz}.
 * @module
 */
