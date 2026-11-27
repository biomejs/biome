/* should not generate diagnostics */
import type A from "";
import type * as B from "";
import type { C } from "";
import { type D } from "";
import { E, type F } from "" with { type: "json" };
import Def, { type F } from "" with { type: "json" };

export type { A } from "";
export type * as B from "";
export { E, type F } from "";
export { E, type F } from "" with { type: "json" };