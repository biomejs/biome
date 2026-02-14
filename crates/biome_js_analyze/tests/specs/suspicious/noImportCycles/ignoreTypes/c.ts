/* should not generate diagnostics */

// This is not a type-only import, but a.ts imports c.ts as type-only.
// It does not make an import cycle after the compiler erased type-only imports.
import { baz } from "./a.ts";

export type Baz = {};
