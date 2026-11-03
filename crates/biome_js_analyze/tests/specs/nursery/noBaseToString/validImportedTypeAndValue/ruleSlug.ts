/* should not generate diagnostics */

export type RuleSlug = string;

const create = (text: string): RuleSlug => text;

export const RuleSlug = { create } as const;
