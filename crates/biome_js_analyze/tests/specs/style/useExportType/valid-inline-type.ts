/* should not generate diagnostics */
import { type T1, type T2 } from "mod";
export { type T1, type T2 };

import V1, { type T3 } from "mod";
export { type T3, V1 };

class V2 {}
export { type T4, V2 } from "mod";

export { type T5, type T6 } from "mod";

// Edge cases
export type {};
export type {} from "mod";
