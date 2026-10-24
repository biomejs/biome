/** {@link Foo} This text mentions @module but has no `@module` tag. */

/* should generate diagnostics */

// `@module` only counts as a tag at the start of a comment line.

import type { Foo } from "./foo";
