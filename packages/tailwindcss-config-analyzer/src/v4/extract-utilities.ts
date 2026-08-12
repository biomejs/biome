// Extract utility metadata from a Tailwind v4 design system.
//
// Strategy:
//   1. Build a custom CSS that imports tailwindcss and adds an `@theme`
//      block injecting a UNIQUE probe token into every theme namespace
//      (e.g. `--color-zzcolorprobe: red`). Unique probes prevent
//      cross-namespace token collisions like `sm` existing in both
//      `--text-*` and `--breakpoint-*`.
//   2. Named-path probes:
//        a. (basename × namespace): `<basename>-<namespace-token>` →
//           emit `NamedBranch::Theme` if compiles.
//        b. (basename × Number/Percentage/Ratio sample): `<basename>-7`,
//           `<basename>-25%`, `<basename>-1/2` → emit `NamedBranch::Typed`.
//   3. Arbitrary-path probes:
//        a. Nonsense `<basename>-[abcxyz]` — establishes the utility's
//           type-blind fallback property, if any.
//        b. (basename × every CssDataType, with explicit dataType marker)
//           `<basename>-[<marker>:<sample>]` — emit `ArbitraryBranch::Typed`
//           only when the resulting (property, count) differs from the nonsense
//           fallback. Equal results are absorbed by the `ArbitraryBranch::Fallback`
//           entry.
//        c. If the utility produced a nonsense result, emit one
//           `ArbitraryBranch::Fallback` entry carrying that fallback property.
//   4. Iterate `getClassList()` for hardcoded keywords on functional
//      utilities. Classes not in `keys('static')`, not numeric, not
//      theme-keyed, and not a probe token are keyword variants baked
//      into the compileFn (e.g. `origin-top`, `accent-current`). Group
//      by (basename, prop, count) and emit `NamedBranch::Keyword` branches.
//   5. Static utilities (`keys('static')`) get a flat per-name table.

import { __unstable__loadDesignSystem } from "tailwindcss";
import { makeLoadStylesheet } from "./css-helpers.js";
import { extractThemeKeys } from "./extract-theme-keys.js";
import {
	ARBITRARY_PROBES,
	NAMED_PREDICATE_PROBES,
	NAMESPACE_PROBE_VALUE,
	NONSENSE_PROBE,
	probeToken,
} from "./probe-samples.js";
import {
	THEME_NAMESPACES,
	type ThemeNamespaceVariant,
} from "./theme-namespaces.js";
import type {
	CssDataType,
	ModifierKind,
	NamedValueType,
} from "./value-types.js";

// Tailwind's own per-candidate sort data: the deduplicated, ascending
// property-order indices the compiled declarations touch, and the total
// declaration count. Taken verbatim from `compileAstNodes(...).propertySort`
// so the preset carries the exact numbers Tailwind sorts with (including
// `--tw-sort` hints and nested at-rule declarations).
export type PropertySort = {
	order: number[];
	count: number;
};

export type StaticUtility = {
	name: string;
	registration_idx: number;
	sort: PropertySort;
	negative_registration_idx: number | null;
};

export type NamedBranch =
	| {
			kind: "Theme";
			namespace: ThemeNamespaceVariant;
			modifier: ModifierKind;
			sort: PropertySort;
	  }
	| {
			kind: "Keyword";
			keywords: string[];
			modifier: ModifierKind;
			sort: PropertySort;
	  }
	| {
			kind: "Typed";
			value_type: NamedValueType;
			modifier: ModifierKind;
			sort: PropertySort;
	  };

export type ArbitraryBranch =
	| {
			kind: "Typed";
			value_type: CssDataType;
			modifier: ModifierKind;
			sort: PropertySort;
	  }
	| {
			kind: "Fallback";
			modifier: ModifierKind;
			sort: PropertySort;
	  };

export type FunctionalBranches = {
	namedBranches: NamedBranch[];
	arbitraryBranches: ArbitraryBranch[];
};

export type FunctionalUtility = {
	basename: string;
	registration_idx: number;
	namedBranches: NamedBranch[];
	arbitraryBranches: ArbitraryBranch[];
	// The placement of the bare basename when the utility compiles
	// without a value (`border`, `ring`, `shadow` have defaults; `w`
	// does not).
	bare: PropertySort | null;
	// The placements of the bare basename with a modifier, probed with a
	// numeric modifier and a bare-word modifier separately because the
	// compiled shape can differ: `shadow/50` sets an extra opacity
	// declaration that `shadow/x` does not, and `@container/sidebar`
	// names the container.
	bareOpacity: PropertySort | null;
	bareName: PropertySort | null;
	negative: Negative | null;
};

export type Negative =
	| { kind: "SameBranches"; registration_idx: number }
	| ({
			kind: "Distinct";
			registration_idx: number;
	  } & FunctionalBranches);

export type ExtractedUtilities = {
	static: StaticUtility[];
	functional: FunctionalUtility[];
};

type DesignSystem = Awaited<ReturnType<typeof __unstable__loadDesignSystem>>;

// Mirror of the comparator `compileCandidates` sorts compiled nodes
// with: walk the shared prefix of the ascending order lists, first
// differing index decides, an exhausted list counts as Infinity (so the
// longer list wins a shared prefix), then declaration count descending.
function comparePropertySort(a: PropertySort, b: PropertySort): number {
	let i = 0;
	while (i < a.order.length && i < b.order.length && a.order[i] === b.order[i])
		i++;
	return (
		(a.order[i] ?? Number.POSITIVE_INFINITY) -
			(b.order[i] ?? Number.POSITIVE_INFINITY) || b.count - a.count
	);
}

// Tailwind's own sort data for a class: every parse of the candidate is
// compiled, and the node that would sort first supplies the
// (order, count) pair — matching how `getClassOrder` positions a
// candidate by its first node in the sorted sheet. Returns null when
// nothing compiles.
function propertySortOf(
	ds: DesignSystem,
	className: string,
): PropertySort | null {
	let best: PropertySort | null = null;
	for (const candidate of ds.parseCandidate(className)) {
		for (const { propertySort } of ds.compileAstNodes(candidate)) {
			if (best === null || comparePropertySort(propertySort, best) < 0) {
				best = propertySort;
			}
		}
	}
	return best;
}

function sortKeyOf(sort: PropertySort): string {
	return `${sort.order.join(".")}|${sort.count}`;
}

// The `/modifier` a branch accepts, probed from a representative candidate
// that matches the branch (`bg-<colortoken>`, `text-<texttoken>`, `w-7`).
// A numeric modifier compiling at all means the branch takes one; a
// font-size utility additionally accepts a leading keyword (`/loose`),
// which distinguishes line-height from opacity.
function modifierKindOf(ds: DesignSystem, base: string): ModifierKind {
	if (!propertySortOf(ds, `${base}/50`)) return "None";
	return propertySortOf(ds, `${base}/loose`) ? "LineHeight" : "Opacity";
}

export async function extractUtilities(): Promise<ExtractedUtilities> {
	const loadStylesheet = makeLoadStylesheet();

	const themeLines: string[] = [];
	for (const { variant, cssPrefix } of THEME_NAMESPACES) {
		const token = probeToken(variant);
		const value = NAMESPACE_PROBE_VALUE[variant];
		themeLines.push(`  ${cssPrefix}${token}: ${value};`);
	}
	const css = `@import "tailwindcss";\n@theme {\n${themeLines.join("\n")}\n}`;
	const ds = await __unstable__loadDesignSystem(css, {
		base: process.cwd(),
		loadStylesheet,
	});

	const staticKeys = ds.utilities.keys("static");
	const functionalKeys = ds.utilities.keys("functional");
	const staticKeySet = new Set(staticKeys);

	const themeKeys = await extractThemeKeys();
	const allThemeKeys = new Set<string>();
	for (const ks of themeKeys.values()) for (const k of ks) allThemeKeys.add(k);
	const probeTokens = new Set(
		THEME_NAMESPACES.map(({ variant }) => probeToken(variant)),
	);

	const staticUtilities = extractStatic(ds, staticKeys);
	const branchesByBasename = extractFunctionalBranches(ds, functionalKeys);
	addKeywordBranches(ds, {
		branchesByBasename,
		staticKeySet,
		allThemeKeys,
		probeTokens,
	});

	type RawNegative = { registration_idx: number } & FunctionalBranches;
	const positives = new Map<string, FunctionalUtility>();
	const negatives = new Map<string, RawNegative>();
	for (let i = 0; i < functionalKeys.length; i++) {
		const key = functionalKeys[i];
		const branches = dedupeFunctionalBranches(
			branchesByBasename.get(key) ?? emptyFunctionalBranches(),
		);
		if (key.startsWith("-")) {
			negatives.set(key.slice(1), { registration_idx: i, ...branches });
		} else {
			positives.set(key, {
				basename: key,
				registration_idx: i,
				...branches,
				bare: propertySortOf(ds, key),
				bareOpacity: propertySortOf(ds, `${key}/50`),
				bareName: propertySortOf(ds, `${key}/x`),
				negative: null,
			});
		}
	}
	for (const [basename, neg] of negatives) {
		const positive = positives.get(basename);
		if (!positive) {
			throw new Error(
				`Negative basename '-${basename}' has no positive counterpart`,
			);
		}
		positive.negative = sameBranches(positive, neg)
			? { kind: "SameBranches", registration_idx: neg.registration_idx }
			: {
					kind: "Distinct",
					registration_idx: neg.registration_idx,
					namedBranches: neg.namedBranches,
					arbitraryBranches: neg.arbitraryBranches,
				};
	}
	// Preserve the original Tailwind registration order of positive entries.
	const functionalUtilities = [...positives.values()].sort(
		(a, b) => a.registration_idx - b.registration_idx,
	);

	return { static: staticUtilities, functional: functionalUtilities };
}

function extractStatic(
	ds: DesignSystem,
	staticKeys: string[],
): StaticUtility[] {
	type Raw = {
		name: string;
		registration_idx: number;
		sort: PropertySort;
	};
	const positives = new Map<string, Raw>();
	const negativeRegByName = new Map<string, number>();
	for (let i = 0; i < staticKeys.length; i++) {
		const name = staticKeys[i];
		const sort = propertySortOf(ds, name);
		if (!sort) continue;
		if (name.startsWith("-")) {
			negativeRegByName.set(name.slice(1), i);
		} else {
			positives.set(name, { name, registration_idx: i, sort });
		}
	}
	const out: StaticUtility[] = [];
	for (const p of positives.values()) {
		out.push({
			name: p.name,
			registration_idx: p.registration_idx,
			sort: p.sort,
			negative_registration_idx: negativeRegByName.get(p.name) ?? null,
		});
	}
	// Preserve original Tailwind registration order.
	out.sort((a, b) => a.registration_idx - b.registration_idx);
	for (const [name] of negativeRegByName) {
		if (!positives.has(name)) {
			throw new Error(
				`Negative static utility '-${name}' has no positive counterpart`,
			);
		}
	}
	return out;
}

function emptyFunctionalBranches(): FunctionalBranches {
	return {
		namedBranches: [],
		arbitraryBranches: [],
	};
}

function extractFunctionalBranches(
	ds: DesignSystem,
	functionalKeys: string[],
): Map<string, FunctionalBranches> {
	const branchesByBasename = new Map<string, FunctionalBranches>();
	for (const basename of functionalKeys) {
		const branches = emptyFunctionalBranches();

		for (const { variant } of THEME_NAMESPACES) {
			const probe = `${basename}-${probeToken(variant)}`;
			const sort = propertySortOf(ds, probe);
			if (!sort) continue;
			branches.namedBranches.push({
				kind: "Theme",
				namespace: variant,
				modifier: modifierKindOf(ds, probe),
				sort,
			});
		}

		for (const p of NAMED_PREDICATE_PROBES) {
			const sort = propertySortOf(ds, `${basename}-${p.value}`);
			if (!sort) continue;
			// A bare Number/Percentage/Ratio value is never a color or a
			// font-size, so a typed branch never carries an opacity or
			// line-height modifier. (Probing would also misread `w-7/50` as
			// the fraction `7/50` on a ratio-capable utility.)
			branches.namedBranches.push({
				kind: "Typed",
				value_type: p.type,
				modifier: "None",
				sort,
			});
		}

		const nonsenseProbe = `${basename}-[${NONSENSE_PROBE}]`;
		const nonsense = propertySortOf(ds, nonsenseProbe);
		if (nonsense) {
			branches.arbitraryBranches.push({
				kind: "Fallback",
				modifier: modifierKindOf(ds, nonsenseProbe),
				sort: nonsense,
			});
		}
		for (const p of ARBITRARY_PROBES) {
			const probe = `${basename}-[${p.marker}:${p.value}]`;
			const sort = propertySortOf(ds, probe);
			if (!sort) continue;
			if (nonsense && sortKeyOf(sort) === sortKeyOf(nonsense)) {
				continue;
			}
			branches.arbitraryBranches.push({
				kind: "Typed",
				value_type: p.type,
				modifier: modifierKindOf(ds, probe),
				sort,
			});
		}

		branchesByBasename.set(basename, branches);
	}
	return branchesByBasename;
}

function addKeywordBranches(
	ds: DesignSystem,
	ctx: {
		branchesByBasename: Map<string, FunctionalBranches>;
		staticKeySet: Set<string>;
		allThemeKeys: Set<string>;
		probeTokens: Set<string>;
	},
): void {
	type KeywordGroup = {
		basename: string;
		sort: PropertySort;
		keywords: Set<string>;
	};
	const groups = new Map<string, KeywordGroup>();
	const classList = ds.getClassList().map(([n]) => n);
	for (const cls of classList) {
		if (ctx.staticKeySet.has(cls)) continue;
		const cands = ds.parseCandidate(cls);
		const cand = cands.find((c) => c.kind === "functional");
		if (cand?.kind !== "functional") continue;
		if (cand.value?.kind !== "named") continue;
		const value = cand.value.value;
		if (/[\d.]/.test(value)) continue;
		if (ctx.allThemeKeys.has(value)) continue;
		if (ctx.probeTokens.has(value)) continue;
		const sort = propertySortOf(ds, cls);
		if (!sort) continue;
		const key = `${cand.root}|${sortKeyOf(sort)}`;
		let group = groups.get(key);
		if (!group) {
			group = { basename: cand.root, sort, keywords: new Set() };
			groups.set(key, group);
		}
		group.keywords.add(value);
	}
	for (const group of groups.values()) {
		const branches =
			ctx.branchesByBasename.get(group.basename) ?? emptyFunctionalBranches();
		const keywords = [...group.keywords].sort();
		branches.namedBranches.push({
			kind: "Keyword",
			keywords,
			// Color keywords (`bg-current`, `border-transparent`) take an
			// opacity modifier; probe a representative keyword to find out.
			modifier: modifierKindOf(ds, `${group.basename}-${keywords[0]}`),
			sort: group.sort,
		});
		ctx.branchesByBasename.set(group.basename, branches);
	}
}

// Branch resolve precedence — most specific match first. Stable sort
// keeps relative order within the same kind (e.g. multiple arbitrary `Typed`
// entries stay in CssDataType-catalog order from the probe matrix).
const NAMED_BRANCH_KIND_ORDER: Record<NamedBranch["kind"], number> = {
	Keyword: 0,
	Theme: 1,
	Typed: 2,
};

const ARBITRARY_BRANCH_KIND_ORDER: Record<ArbitraryBranch["kind"], number> = {
	Typed: 0,
	Fallback: 1,
};

function sameBranches(a: FunctionalBranches, b: FunctionalBranches): boolean {
	return (
		sameBranchList(a.namedBranches, b.namedBranches, namedBranchKey) &&
		sameBranchList(a.arbitraryBranches, b.arbitraryBranches, arbitraryBranchKey)
	);
}

function sameBranchList<T>(
	a: T[],
	b: T[],
	key: (branch: T) => string,
): boolean {
	if (a.length !== b.length) return false;
	for (let i = 0; i < a.length; i++) {
		if (key(a[i]) !== key(b[i])) return false;
	}
	return true;
}

function namedBranchKey(b: NamedBranch): string {
	switch (b.kind) {
		case "Theme":
			return `N|${b.namespace}|${b.modifier}|${sortKeyOf(b.sort)}`;
		case "Keyword":
			return `K|${b.keywords.join(",")}|${b.modifier}|${sortKeyOf(b.sort)}`;
		case "Typed":
			return `NT|${b.value_type}|${b.modifier}|${sortKeyOf(b.sort)}`;
	}
}

function arbitraryBranchKey(b: ArbitraryBranch): string {
	switch (b.kind) {
		case "Typed":
			return `AT|${b.value_type}|${b.modifier}|${sortKeyOf(b.sort)}`;
		case "Fallback":
			return `A|${b.modifier}|${sortKeyOf(b.sort)}`;
	}
}

function dedupeFunctionalBranches(
	branches: FunctionalBranches,
): FunctionalBranches {
	return {
		namedBranches: dedupeBranchList(
			branches.namedBranches,
			namedBranchKey,
			NAMED_BRANCH_KIND_ORDER,
		),
		arbitraryBranches: dedupeBranchList(
			branches.arbitraryBranches,
			arbitraryBranchKey,
			ARBITRARY_BRANCH_KIND_ORDER,
		),
	};
}

function dedupeBranchList<K extends string, T extends { kind: K }>(
	branches: T[],
	key: (branch: T) => string,
	kindOrder: Record<K, number>,
): T[] {
	const seen = new Set<string>();
	const out: T[] = [];
	for (const b of branches) {
		const branchKey = key(b);
		if (seen.has(branchKey)) continue;
		seen.add(branchKey);
		out.push(b);
	}
	out.sort((a, b) => kindOrder[a.kind] - kindOrder[b.kind]);
	return out;
}
