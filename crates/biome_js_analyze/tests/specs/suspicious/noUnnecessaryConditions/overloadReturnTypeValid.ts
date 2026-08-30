/* should not generate diagnostics */

declare function q(fn: () => 1): string;
declare function q(fn: () => 0): string | undefined;

export const a1 = q(() => 0)?.length;

declare function r(fn: () => 0): string | undefined;
declare function r(fn: () => 1): string;

export const a2 = r(() => 0)?.length;
