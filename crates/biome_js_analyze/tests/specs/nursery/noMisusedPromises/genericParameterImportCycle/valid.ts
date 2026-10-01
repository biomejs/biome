/* should not generate diagnostics */

import type { Entity } from "./entity";

declare function load(): Entity<string>;

if (load()) {
}
