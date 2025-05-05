/* should not generate diagnostics */
import { type T1, type T2 } from "mod";
export type { T1, T2 };

import V1, { type T3 } from "mod";
export type { T3 };
export { V1 };
