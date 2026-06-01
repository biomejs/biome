// Render the auto-generated Rust file `tailwind_preset_v4.rs`.
//
// Codegen scope is intentionally narrow — only the long phf maps,
// sets, and arrays are emitted. Structural types (`NamedValueType`,
// `CssDataType`, `ThemeNamespace`, `NamedBranch`, `ArbitraryBranch`,
// `Negative`, `UtilityEntry`, `FunctionalEntry`) live in the hand-written
// sibling `tailwind_preset_v4_types.rs` and are imported here.

import type {
	ArbitraryBranch,
	ExtractedUtilities,
	FunctionalUtility,
	NamedBranch,
} from "./extract-utilities.js";
import {
	THEME_NAMESPACES,
	type ThemeNamespacePrefix,
} from "./theme-namespaces.js";

const HEADER = `//! AUTO-GENERATED. DO NOT EDIT MANUALLY.
//! Run \`pnpm execute:v4\` from \`packages/tailwindcss-config-analyzer\`.
//!
//! Structural types live in the sibling \`tailwind_preset_v4_types\`.
//!
//! Source references (Tailwind v4):
//! - property-order:  https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/property-order.ts
//! - utilities:       https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/utilities.ts
//! - default theme:   https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/theme.css
//! - infer-data-type: https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/utils/infer-data-type.ts

use phf::{phf_map, phf_set};

use super::tailwind_preset_v4_types::{
    ArbitraryBranch, CssDataType, FunctionalEntry, NamedBranch, NamedValueType, Negative::*,
    ThemeNamespace, UtilityEntry,
};
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

function renderPropertyIndex(props: string[]): string {
	const items = props
		.map((p, i) => `    ${rustString(p)} => ${i}u16,`)
		.join("\n");
	return `pub static PROPERTY_INDEX: phf::Map<&'static str, u16> = phf_map! {
${items}
};
`;
}

function renderStaticUtilities(
	utils: ExtractedUtilities,
	propIdx: Map<string, number>,
	propCount: number,
): string {
	const lines = utils.static.map((u) => {
		const idx = propIdx.get(u.sort_property) ?? propCount;
		const negReg =
			u.negative_registration_idx === null
				? "None"
				: `Some(${u.negative_registration_idx})`;
		return `    ${rustString(u.name)} => UtilityEntry { property_idx: ${idx}, property_count: ${u.property_count}, registration_idx: ${u.registration_idx}, negative_registration_idx: ${negReg} },`;
	});
	return `pub static STATIC_UTILITIES: phf::Map<&'static str, UtilityEntry> = phf_map! {
${lines.join("\n")}
};
`;
}

// Build the deduplicated keyword pool from every NamedBranch::Keyword branch
// across all utilities. Pool entries are unique sets, addressed by
// stable index used in the generated NamedBranch::Keyword variant.
function collectKeywordPool(utils: ExtractedUtilities): {
	pool: string[][];
	idxOf: Map<string, number>;
} {
	const pool: string[][] = [];
	const idxOf = new Map<string, number>();
	for (const u of utils.functional) {
		for (const b of u.namedBranches) {
			if (b.kind !== "Keyword") continue;
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

function renderNamedBranchList(
	indent: string,
	branches: NamedBranch[],
	propIdx: Map<string, number>,
	propCount: number,
	keywordIdx: Map<string, number>,
): string {
	return branches
		.map(
			(b) =>
				`${indent}${formatNamedBranch(b, propIdx, propCount, keywordIdx)},`,
		)
		.join("\n");
}

function renderArbitraryBranchList(
	indent: string,
	branches: ArbitraryBranch[],
	propIdx: Map<string, number>,
	propCount: number,
): string {
	return branches
		.map((b) => `${indent}${formatArbitraryBranch(b, propIdx, propCount)},`)
		.join("\n");
}

function renderNegative(
	u: FunctionalUtility,
	propIdx: Map<string, number>,
	propCount: number,
	keywordIdx: Map<string, number>,
): string {
	if (u.negative === null) {
		return "        negative: None,";
	}
	switch (u.negative.kind) {
		case "SameBranches":
			return `        negative: Some(SameBranches { registration_idx: ${u.negative.registration_idx} }),`;
		case "Distinct": {
			const namedItems = renderNamedBranchList(
				"                ",
				u.negative.namedBranches,
				propIdx,
				propCount,
				keywordIdx,
			);
			const arbitraryItems = renderArbitraryBranchList(
				"                ",
				u.negative.arbitraryBranches,
				propIdx,
				propCount,
			);
			return `        negative: Some(Distinct {
            registration_idx: ${u.negative.registration_idx},
${renderBranchSlice("            ", "named_branches", namedItems)}
${renderBranchSlice("            ", "arbitrary_branches", arbitraryItems)}
        }),`;
		}
	}
}

function renderFunctionalUtilities(
	utils: ExtractedUtilities,
	propIdx: Map<string, number>,
	propCount: number,
	keywordIdx: Map<string, number>,
): string {
	const populated = utils.functional.filter(
		(u) =>
			u.namedBranches.length > 0 ||
			u.arbitraryBranches.length > 0 ||
			u.negative !== null,
	);
	const entries = populated.map((u) => {
		const namedItems = renderNamedBranchList(
			"            ",
			u.namedBranches,
			propIdx,
			propCount,
			keywordIdx,
		);
		const arbitraryItems = renderArbitraryBranchList(
			"            ",
			u.arbitraryBranches,
			propIdx,
			propCount,
		);
		const negative = renderNegative(u, propIdx, propCount, keywordIdx);
		return `    ${rustString(u.basename)} => FunctionalEntry {
        registration_idx: ${u.registration_idx},
${renderBranchSlice("        ", "named_branches", namedItems)}
${renderBranchSlice("        ", "arbitrary_branches", arbitraryItems)}
${negative}
    },`;
	});
	return `pub static FUNCTIONAL_UTILITIES: phf::Map<&'static str, FunctionalEntry> = phf_map! {
${entries.join("\n")}
};
`;
}

function renderBranchSlice(
	indent: string,
	field: string,
	items: string,
): string {
	if (items.length === 0) {
		return `${indent}${field}: &[],`;
	}
	return `${indent}${field}: &[
${items}
${indent}],`;
}

function formatNamedBranch(
	b: NamedBranch,
	propIdx: Map<string, number>,
	propCount: number,
	keywordIdx: Map<string, number>,
): string {
	const idx = propIdx.get(b.sort_property) ?? propCount;
	switch (b.kind) {
		case "Theme":
			return `NamedBranch::Theme(ThemeNamespace::${b.namespace}, ${idx}, ${b.property_count})`;
		case "Keyword": {
			const key = b.keywords.join("\0");
			const pool = keywordIdx.get(key);
			if (pool === undefined) {
				throw new Error(
					`keyword pool missing entry for: ${b.keywords.join(",")}`,
				);
			}
			return `NamedBranch::Keyword(${pool}, ${idx}, ${b.property_count})`;
		}
		case "Typed":
			return `NamedBranch::Typed(NamedValueType::${b.value_type}, ${idx}, ${b.property_count})`;
	}
}

function formatArbitraryBranch(
	b: ArbitraryBranch,
	propIdx: Map<string, number>,
	propCount: number,
): string {
	const idx = propIdx.get(b.sort_property) ?? propCount;
	switch (b.kind) {
		case "Typed":
			return `ArbitraryBranch::Typed(CssDataType::${b.value_type}, ${idx}, ${b.property_count})`;
		case "Fallback":
			return `ArbitraryBranch::Fallback(${idx}, ${b.property_count})`;
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
			`pub(super) static THEME_KEYS_${camelToScreamingSnake(ns.variant)}: phf::Set<&'static str> = phf_set! {\n${items}${items ? "\n" : ""}};\n`,
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
		renderPropertyIndex(input.propertyOrder),
		renderKeywordPool(keywordPool),
		renderStaticUtilities(input.utilities, propIdx, propCount),
		renderFunctionalUtilities(input.utilities, propIdx, propCount, keywordIdx),
		renderThemeKeys(input.themeKeys),
	].join("\n");
}
