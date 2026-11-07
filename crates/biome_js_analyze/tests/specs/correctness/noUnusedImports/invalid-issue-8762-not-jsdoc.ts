/* {@link Foo} @module */

/* should generate diagnostics */

// The first comment has a `@module` tag, but it is a block comment, not JSDoc.

import type { Foo } from "./foo";
