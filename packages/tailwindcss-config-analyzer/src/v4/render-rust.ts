// Render the auto-generated Rust file `tailwind_preset_v4.rs`.

import type { Branch, ExtractedUtilities } from "./extract-utilities.js";
import {
	THEME_NAMESPACES,
	type ThemeNamespacePrefix,
} from "./theme-namespaces.js";
import { VALUE_TYPES } from "./value-types.js";

const HEADER = `//! AUTO-GENERATED. DO NOT EDIT MANUALLY.
//! Run \`pnpm execute:v4\` from \`packages/tailwindcss-config-analyzer\`.
//!
//! Source references (Tailwind v4):
//! - property-order:  https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/property-order.ts
//! - utilities:       https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/utilities.ts
//! - default theme:   https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/theme.css
//! - infer-data-type: https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/utils/infer-data-type.ts

// Some preset items are intentionally unused while the v4 sort algorithm is
// still being implemented incrementally:
// - \`PROPERTY_ORDER\` is consumed only when classifying arbitrary CSS
//   (\`[mask:none]\`), which is a TODO.
// - \`Branch::ArbitraryTyped\` and \`Branch::Arbitrary\` payload fields fire only for
//   bracketed arbitrary values (\`p-[10px]\`), which is a TODO.
#![expect(dead_code, reason = "intentionally unused while sort algorithm is being implemented; see TODO comment above")]

use phf::{phf_map, phf_set};

use super::predicates;

use Branch::*;
`;

const STRUCT_DEFS = `// Sort metadata for a single utility class.
#[derive(Copy, Clone)]
pub struct UtilityEntry {
    pub property_idx: u16,
    pub property_count: u8,
    pub registration_idx: u16,
}

// One dispatch branch inside a functional utility's compileFn.
//
// - Named:           named-path theme-namespace lookup
//                    (\`text-lg\` ↔ \`--text-lg\`).
// - NamedKeyword:    named-path hardcoded keyword set baked into the
//                    compileFn (\`origin-top\`, \`accent-current\`).
//                    First field is an index into \`KEYWORD_POOL\`.
// - NamedTyped:      named-path predicate match for bare value patterns
//                    (\`p-4\` Number, \`from-25%\` Percentage, \`w-1/2\` Ratio).
// - ArbitraryTyped:  arbitrary-path predicate match used for utilities
//                    whose property differs by CSS value type
//                    (\`from-[#fff]\` → \`--tw-gradient-from\`,
//                    \`from-[10px]\` → \`--tw-gradient-from-position\`).
// - Arbitrary:       arbitrary-path fallback used when the utility emits
//                    the same property regardless of value type
//                    (\`p-[10px]\`, \`p-[#fff]\` → \`padding\`).
//                    Resolved after every \`ArbitraryTyped\` branch.
//
// Keyword sets are interned in \`KEYWORD_POOL\` and referenced by index
// so that \`Branch\` stays small (the largest variant payload is now
// three u16s + a u8 instead of a fat slice pointer).
#[derive(Copy, Clone)]
pub enum Branch {
    Named(ThemeNamespace, u16, u8),
    NamedKeyword(u16, u16, u8),
    NamedTyped(ValueType, u16, u8),
    ArbitraryTyped(ValueType, u16, u8),
    Arbitrary(u16, u8),
}

#[derive(Copy, Clone)]
pub struct FunctionalEntry {
    pub registration_idx: u16,
    pub branches: &'static [Branch],
}
`;

function rustString(s: string): string {
	return `"${s.replace(/\\/g, "\\\\").replace(/"/g, '\\"')}"`;
}

function camelToSnake(s: string): string {
	return s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
}

function camelToScreamingSnake(s: string): string {
	return camelToSnake(s).toUpperCase();
}

function renderValueTypeEnum(): string {
	const variants = VALUE_TYPES.map((v) => `    ${v},`).join("\n");
	const matches = VALUE_TYPES.map(
		(v) =>
			`            Self::${v} => predicates::is_${camelToSnake(v)}(value),`,
	).join("\n");
	return `// CSS value types (from infer-data-type.ts).
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ValueType {
${variants}
}

impl ValueType {
    pub fn matches(self, value: &str) -> bool {
        match self {
${matches}
        }
    }
}
`;
}

function renderThemeNamespaceEnum(): string {
	const variants = THEME_NAMESPACES.map((n) => `    ${n.variant},`).join("\n");
	const keysArms = THEME_NAMESPACES.map(
		(n) =>
			`            Self::${n.variant} => &THEME_KEYS_${camelToScreamingSnake(n.variant)},`,
	).join("\n");
	return `// Theme namespaces (from default theme.css).
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ThemeNamespace {
${variants}
}

impl ThemeNamespace {
    pub fn keys(self) -> &'static phf::Set<&'static str> {
        match self {
${keysArms}
        }
    }
}
`;
}

function renderPropertyOrder(props: string[]): string {
	const items = props.map((p) => `    ${rustString(p)},`).join("\n");
	return `// CSS property sort order.
pub static PROPERTY_ORDER: [&str; ${props.length}] = [
${items}
];
`;
}

function renderStaticUtilities(
	utils: ExtractedUtilities,
	propIdx: Map<string, number>,
	propCount: number,
): string {
	const lines = utils.static.map((u) => {
		const idx = propIdx.get(u.sort_property) ?? propCount;
		return `    ${rustString(u.name)} => UtilityEntry { property_idx: ${idx}, property_count: ${u.property_count}, registration_idx: ${u.registration_idx} },`;
	});
	return `pub static STATIC_UTILITIES: phf::Map<&'static str, UtilityEntry> = phf_map! {
${lines.join("\n")}
};
`;
}

// Build the deduplicated keyword pool from every NamedKeyword branch
// across all utilities. Pool entries are unique sets, addressed by
// stable index used in the generated Branch::NamedKeyword variant.
function collectKeywordPool(utils: ExtractedUtilities): {
	pool: string[][];
	idxOf: Map<string, number>;
} {
	const pool: string[][] = [];
	const idxOf = new Map<string, number>();
	for (const u of utils.functional) {
		for (const b of u.branches) {
			if (b.kind !== "NamedKeyword") continue;
			const key = b.keywords.join("\0");
			if (idxOf.has(key)) continue;
			idxOf.set(key, pool.length);
			pool.push(b.keywords);
		}
	}
	return { pool, idxOf };
}

function renderKeywordPool(pool: string[][]): string {
	if (pool.length === 0) {
		return "pub(super) static KEYWORD_POOL: &[&[&str]] = &[];\n";
	}
	const items = pool.map((kws) => {
		const inner = kws.map(rustString).join(", ");
		return `    &[${inner}],`;
	});
	return `pub(super) static KEYWORD_POOL: &[&[&str]] = &[
${items.join("\n")}
];
`;
}

function renderFunctionalUtilities(
	utils: ExtractedUtilities,
	propIdx: Map<string, number>,
	propCount: number,
	keywordIdx: Map<string, number>,
): string {
	const populated = utils.functional.filter((u) => u.branches.length > 0);
	const entries = populated.map((u) => {
		const items = u.branches
			.map(
				(b) =>
					`            ${formatBranch(b, propIdx, propCount, keywordIdx)},`,
			)
			.join("\n");
		return `    ${rustString(u.basename)} => FunctionalEntry {
        registration_idx: ${u.registration_idx},
        branches: &[
${items}
        ],
    },`;
	});
	return `pub static FUNCTIONAL_UTILITIES: phf::Map<&'static str, FunctionalEntry> = phf_map! {
${entries.join("\n")}
};
`;
}

function formatBranch(
	b: Branch,
	propIdx: Map<string, number>,
	propCount: number,
	keywordIdx: Map<string, number>,
): string {
	const idx = propIdx.get(b.sort_property) ?? propCount;
	switch (b.kind) {
		case "Named":
			return `Named(ThemeNamespace::${b.namespace}, ${idx}, ${b.property_count})`;
		case "NamedKeyword": {
			const key = b.keywords.join("\0");
			const pool = keywordIdx.get(key);
			if (pool === undefined) {
				throw new Error(
					`keyword pool missing entry for: ${b.keywords.join(",")}`,
				);
			}
			return `NamedKeyword(${pool}, ${idx}, ${b.property_count})`;
		}
		case "NamedTyped":
			return `NamedTyped(ValueType::${b.value_type}, ${idx}, ${b.property_count})`;
		case "ArbitraryTyped":
			return `ArbitraryTyped(ValueType::${b.value_type}, ${idx}, ${b.property_count})`;
		case "Arbitrary":
			return `Arbitrary(${idx}, ${b.property_count})`;
	}
}

function renderThemeKeys(keys: Map<ThemeNamespacePrefix, Set<string>>): string {
	const blocks: string[] = [];
	for (const ns of THEME_NAMESPACES) {
		const set = keys.get(ns.cssPrefix);
		const items = set
			? [...set]
					.sort()
					.map((k) => `    ${rustString(k)},`)
					.join("\n")
			: "";
		blocks.push(
			`static THEME_KEYS_${camelToScreamingSnake(ns.variant)}: phf::Set<&'static str> = phf_set! {\n${items}${items ? "\n" : ""}};\n`,
		);
	}
	return blocks.join("");
}

export function renderRust(input: {
	propertyOrder: string[];
	themeKeys: Map<ThemeNamespacePrefix, Set<string>>;
	utilities: ExtractedUtilities;
}): string {
	// One Map<property, idx> built once and threaded through emitters,
	// instead of repeated linear `Array.indexOf` lookups per branch.
	const propIdx = new Map(input.propertyOrder.map((p, i) => [p, i] as const));
	const propCount = input.propertyOrder.length;
	const { pool: keywordPool, idxOf: keywordIdx } = collectKeywordPool(
		input.utilities,
	);

	return [
		HEADER,
		renderValueTypeEnum(),
		renderThemeNamespaceEnum(),
		STRUCT_DEFS,
		renderPropertyOrder(input.propertyOrder),
		renderKeywordPool(keywordPool),
		renderStaticUtilities(input.utilities, propIdx, propCount),
		renderFunctionalUtilities(input.utilities, propIdx, propCount, keywordIdx),
		renderThemeKeys(input.themeKeys),
	].join("\n");
}
