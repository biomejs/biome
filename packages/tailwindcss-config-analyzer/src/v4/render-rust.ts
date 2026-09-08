// Render the auto-generated Rust file `tailwind_preset_v4.rs`.
//
// Codegen scope is intentionally narrow — only the long phf maps,
// sets, and arrays are emitted. Structural types (`NamedValueType`,
// `CssDataType`, `ThemeNamespace`, `NamedBranch`, `ArbitraryBranch`,
// `Negative`, `UtilityEntry`, `FunctionalEntry`, `VariantKind`,
// `VariantCompare`, `VariantEntry`) live in the hand-written sibling
// `tailwind_preset_v4_types.rs` and are imported here.

import type {
	ArbitraryBranch,
	ExtractedUtilities,
	FunctionalUtility,
	NamedBranch,
	PropertySort,
} from "./extract-utilities.js";
import type {
	ExtractedVariant,
	ExtractedVariants,
	ThemeValue,
} from "./extract-variants.js";
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
//! - variants:        https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/variants.ts
//! - default theme:   https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/theme.css
//! - infer-data-type: https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/utils/infer-data-type.ts

use phf::{phf_map, phf_set};

use super::tailwind_preset_v4_types::{
    ArbitraryBranch, CssDataType, FunctionalEntry, ModifierKind, NamedBranch, NamedValueType,
    Negative::*, ThemeNamespace, UtilityEntry, VariantCompare, VariantEntry, VariantKind,
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

// Build the deduplicated signature pool from every placement's
// property-order list (statics, branches, and negative branches). Pool
// entries are unique ascending index lists, addressed by the stable
// index stored in each placement.
function collectSignaturePool(utils: ExtractedUtilities): {
	pool: number[][];
	idxOf: (sort: PropertySort) => number;
} {
	const pool: number[][] = [];
	const byKey = new Map<string, number>();
	const intern = (sort: PropertySort): number => {
		const key = sort.order.join(".");
		let idx = byKey.get(key);
		if (idx === undefined) {
			idx = pool.length;
			byKey.set(key, idx);
			pool.push(sort.order);
		}
		return idx;
	};
	const internBranches = (
		named: NamedBranch[],
		arbitrary: ArbitraryBranch[],
	) => {
		for (const b of named) intern(b.sort);
		for (const b of arbitrary) intern(b.sort);
	};
	for (const u of utils.static) intern(u.sort);
	for (const u of utils.functional) {
		internBranches(u.namedBranches, u.arbitraryBranches);
		if (u.bare) intern(u.bare);
		if (u.bareOpacity) intern(u.bareOpacity);
		if (u.bareName) intern(u.bareName);
		if (u.negative?.kind === "Distinct") {
			internBranches(u.negative.namedBranches, u.negative.arbitraryBranches);
		}
	}
	return { pool, idxOf: intern };
}

function checked(
	sort: PropertySort,
	idx: number,
): { sig: number; count: number } {
	if (idx > 0xffff) throw new Error(`signature pool index ${idx} exceeds u16`);
	if (sort.count > 0xff)
		throw new Error(`declaration count ${sort.count} exceeds u8`);
	return { sig: idx, count: sort.count };
}

function renderSignaturePool(pool: number[][]): string {
	const items = pool.map((order) => {
		for (const i of order) {
			if (i > 0xffff) throw new Error(`property-order index ${i} exceeds u16`);
		}
		return `    &[${order.join(", ")}],`;
	});
	return `pub(super) static SIGNATURE_POOL: &[&[u16]] = &[
${items.join("\n")}
];
`;
}

function renderStaticUtilities(
	utils: ExtractedUtilities,
	sigIdx: (sort: PropertySort) => number,
): string {
	const lines = utils.static.map((u) => {
		const { sig, count } = checked(u.sort, sigIdx(u.sort));
		const hasNegative = u.negative_registration_idx !== null;
		return `    ${rustString(u.name)} => UtilityEntry { sig: ${sig}, count: ${count}, has_negative: ${hasNegative} },`;
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
	sigIdx: (sort: PropertySort) => number,
	keywordIdx: Map<string, number>,
): string {
	return branches
		.map((b) => `${indent}${formatNamedBranch(b, sigIdx, keywordIdx)},`)
		.join("\n");
}

function renderArbitraryBranchList(
	indent: string,
	branches: ArbitraryBranch[],
	sigIdx: (sort: PropertySort) => number,
): string {
	return branches
		.map((b) => `${indent}${formatArbitraryBranch(b, sigIdx)},`)
		.join("\n");
}

function renderNegative(
	u: FunctionalUtility,
	sigIdx: (sort: PropertySort) => number,
	keywordIdx: Map<string, number>,
): string {
	if (u.negative === null) {
		return "        negative: None,";
	}
	switch (u.negative.kind) {
		case "SameBranches":
			return "        negative: Some(SameBranches),";
		case "Distinct": {
			const namedItems = renderNamedBranchList(
				"                ",
				u.negative.namedBranches,
				sigIdx,
				keywordIdx,
			);
			const arbitraryItems = renderArbitraryBranchList(
				"                ",
				u.negative.arbitraryBranches,
				sigIdx,
			);
			return `        negative: Some(Distinct {
${renderBranchSlice("            ", "named_branches", namedItems)}
${renderBranchSlice("            ", "arbitrary_branches", arbitraryItems)}
        }),`;
		}
	}
}

function renderFunctionalUtilities(
	utils: ExtractedUtilities,
	sigIdx: (sort: PropertySort) => number,
	keywordIdx: Map<string, number>,
): string {
	const populated = utils.functional.filter(
		(u) =>
			u.namedBranches.length > 0 ||
			u.arbitraryBranches.length > 0 ||
			u.bare !== null ||
			u.bareOpacity !== null ||
			u.bareName !== null ||
			u.negative !== null,
	);
	const entries = populated.map((u) => {
		const namedItems = renderNamedBranchList(
			"            ",
			u.namedBranches,
			sigIdx,
			keywordIdx,
		);
		const arbitraryItems = renderArbitraryBranchList(
			"            ",
			u.arbitraryBranches,
			sigIdx,
		);
		const renderPlacement = (sort: PropertySort | null) => {
			if (!sort) return "None";
			const { sig, count } = checked(sort, sigIdx(sort));
			return `Some((${sig}, ${count}))`;
		};
		const negative = renderNegative(u, sigIdx, keywordIdx);
		return `    ${rustString(u.basename)} => FunctionalEntry {
${renderBranchSlice("        ", "named_branches", namedItems)}
${renderBranchSlice("        ", "arbitrary_branches", arbitraryItems)}
        bare: ${renderPlacement(u.bare)},
        bare_opacity: ${renderPlacement(u.bareOpacity)},
        bare_name: ${renderPlacement(u.bareName)},
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
	sigIdx: (sort: PropertySort) => number,
	keywordIdx: Map<string, number>,
): string {
	const { sig, count } = checked(b.sort, sigIdx(b.sort));
	const m = `ModifierKind::${b.modifier}`;
	switch (b.kind) {
		case "Theme":
			return `NamedBranch::Theme(ThemeNamespace::${b.namespace}, ${m}, ${sig}, ${count})`;
		case "Keyword": {
			const key = b.keywords.join("\0");
			const pool = keywordIdx.get(key);
			if (pool === undefined) {
				throw new Error(
					`keyword pool missing entry for: ${b.keywords.join(",")}`,
				);
			}
			return `NamedBranch::Keyword(${pool}, ${m}, ${sig}, ${count})`;
		}
		case "Typed":
			return `NamedBranch::Typed(NamedValueType::${b.value_type}, ${m}, ${sig}, ${count})`;
	}
}

function formatArbitraryBranch(
	b: ArbitraryBranch,
	sigIdx: (sort: PropertySort) => number,
): string {
	const { sig, count } = checked(b.sort, sigIdx(b.sort));
	const m = `ModifierKind::${b.modifier}`;
	switch (b.kind) {
		case "Typed":
			return `ArbitraryBranch::Typed(CssDataType::${b.value_type}, ${m}, ${sig}, ${count})`;
		case "Fallback":
			return `ArbitraryBranch::Fallback(${m}, ${sig}, ${count})`;
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

function renderVariants(variants: ExtractedVariant[]): string {
	const lines = variants.map(
		(v) =>
			`    ${rustString(v.name)} => VariantEntry { kind: VariantKind::${v.kind}, order: ${v.order}, compare: VariantCompare::${v.compare}, compounds: ${v.compounds}, compounds_with: ${v.compounds_with} },`,
	);
	return `pub(super) static VARIANTS: phf::Map<&'static str, VariantEntry> = phf_map! {
${lines.join("\n")}
};
`;
}

function renderThemeValueMap(mapName: string, values: ThemeValue[]): string {
	const lines = values.map(
		({ name, value }) => `    ${rustString(name)} => ${rustString(value)},`,
	);
	return `pub(super) static ${mapName}: phf::Map<&'static str, &'static str> = phf_map! {
${lines.join("\n")}
};
`;
}

export function renderRust(input: {
	propertyOrder: string[];
	themeKeys: Map<ThemeNamespacePrefix, Set<string>>;
	utilities: ExtractedUtilities;
	variants: ExtractedVariants;
}): string {
	const { pool: keywordPool, idxOf: keywordIdx } = collectKeywordPool(
		input.utilities,
	);
	const { pool: signaturePool, idxOf: sigIdx } = collectSignaturePool(
		input.utilities,
	);

	return [
		HEADER,
		renderPropertyIndex(input.propertyOrder),
		renderKeywordPool(keywordPool),
		renderSignaturePool(signaturePool),
		renderStaticUtilities(input.utilities, sigIdx),
		renderFunctionalUtilities(input.utilities, sigIdx, keywordIdx),
		renderVariants(input.variants.variants),
		renderThemeValueMap("BREAKPOINT_VALUES", input.variants.breakpoints),
		renderThemeValueMap("CONTAINER_VALUES", input.variants.containers),
		renderThemeKeys(input.themeKeys),
	].join("\n");
}
