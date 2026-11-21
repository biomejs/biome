/* should not generate diagnostics */

// The target of a logical assignment can be wrapped.
let parenthesized = null;
(parenthesized) ??= init();

let nonNull: number | null = null;
nonNull! ||= init();

let asserted: unknown = null;
(asserted as number) &&= init();

let satisfied: number | null = null;
(satisfied satisfies number | null) ??= init();
