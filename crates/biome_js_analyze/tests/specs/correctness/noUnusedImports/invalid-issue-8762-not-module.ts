/** {@link Foo} @modules */

/* should generate diagnostics */

// The first comment is JSDoc, but `@modules` is not the `@module` tag.

import type { Foo } from "./foo";
