import * as A from "";
export { A }
const Inner = A.Inner;

// With Import attributes
import * as B from "" with { type: "json" };
type BB = B;

// No reference
import * as C from "";

import type * as D from "";
export type { D };
