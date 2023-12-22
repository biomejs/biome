import type A from "" with { type: "json" };
import type * as B from "" with { type: "json" };
import type { C } from "" with { type: "json" };
import { type D } from "" with { type: "json" };
import { E, type F } from "" with { type: "json" };

export type { A } from "" with { type: "json" };
export type * as B from "" with { type: "json" };
export { E, type F } from "" with { type: "json" };