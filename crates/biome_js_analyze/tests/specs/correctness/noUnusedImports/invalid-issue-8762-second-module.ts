/**
 * Documents {@link Foo}.
 * @module
 */

/**
 * A second `@module` comment in the header is not the module comment:
 * {@link Bar} is still unused.
 * @module
 */

/* should generate diagnostics */

import type { Foo } from "./foo";
import type { Bar } from "./bar";
