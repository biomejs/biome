/* should not generate diagnostics */

import { RuleSlug } from "./ruleSlug";

export const label = (slug: RuleSlug): string => `rule ${slug}`;

export const created = (text: string): string => `rule ${RuleSlug.create(text)}`;
