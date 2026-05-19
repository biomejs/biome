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
import { makeLoadStylesheet, parseDeclarations } from "./css-helpers.js";
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
import type { CssDataType } from "./value-types.js";

export type StaticUtility = {
	name: string;
	registration_idx: number;
	sort_property: string;
	property_count: number;
	negative_registration_idx: number | null;
};

export type NamedBranch =
	| {
			kind: "Theme";
			namespace: ThemeNamespaceVariant;
			sort_property: string;
			property_count: number;
	  }
	| {
			kind: "Keyword";
			keywords: string[];
			sort_property: string;
			property_count: number;
	  }
	| {
			kind: "Typed";
			value_type: CssDataType;
			sort_property: string;
			property_count: number;
	  };

export type ArbitraryBranch =
	| {
			kind: "Typed";
			value_type: CssDataType;
			sort_property: string;
			property_count: number;
	  }
	| {
			kind: "Fallback";
			sort_property: string;
			property_count: number;
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
	ds: Awaited<ReturnType<typeof __unstable__loadDesignSystem>>,
	staticKeys: string[],
): StaticUtility[] {
	const staticCss = ds.candidatesToCss(staticKeys);
	type Raw = {
		name: string;
		registration_idx: number;
		sort_property: string;
		property_count: number;
	};
	const positives = new Map<string, Raw>();
	const negativeRegByName = new Map<string, number>();
	for (let i = 0; i < staticKeys.length; i++) {
		const css = staticCss[i];
		if (!css) continue;
		const { sort_property, property_count } = parseDeclarations(css);
		if (!sort_property) continue;
		const name = staticKeys[i];
		if (name.startsWith("-")) {
			negativeRegByName.set(name.slice(1), i);
		} else {
			positives.set(name, {
				name,
				registration_idx: i,
				sort_property,
				property_count,
			});
		}
	}
	const out: StaticUtility[] = [];
	for (const p of positives.values()) {
		out.push({
			name: p.name,
			registration_idx: p.registration_idx,
			sort_property: p.sort_property,
			property_count: p.property_count,
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

type ProbeSlot =
	| { basename: string; kind: "ns"; variant: ThemeNamespaceVariant }
	| { basename: string; kind: "named-typed"; type: CssDataType }
	| { basename: string; kind: "nonsense" }
	| { basename: string; kind: "arbitrary-typed"; type: CssDataType };

function emptyFunctionalBranches(): FunctionalBranches {
	return {
		namedBranches: [],
		arbitraryBranches: [],
	};
}

function extractFunctionalBranches(
	ds: Awaited<ReturnType<typeof __unstable__loadDesignSystem>>,
	functionalKeys: string[],
): Map<string, FunctionalBranches> {
	const probeClasses: string[] = [];
	const probeMeta: ProbeSlot[] = [];
	for (const basename of functionalKeys) {
		for (const { variant } of THEME_NAMESPACES) {
			probeClasses.push(`${basename}-${probeToken(variant)}`);
			probeMeta.push({ basename, kind: "ns", variant });
		}
		for (const p of NAMED_PREDICATE_PROBES) {
			probeClasses.push(`${basename}-${p.value}`);
			probeMeta.push({ basename, kind: "named-typed", type: p.type });
		}
		probeClasses.push(`${basename}-[${NONSENSE_PROBE}]`);
		probeMeta.push({ basename, kind: "nonsense" });
		for (const p of ARBITRARY_PROBES) {
			probeClasses.push(`${basename}-[${p.marker}:${p.value}]`);
			probeMeta.push({ basename, kind: "arbitrary-typed", type: p.type });
		}
	}
	const probeCss = ds.candidatesToCss(probeClasses);

	type ProbeResult = { sort_property: string; property_count: number } | null;
	const results = new Map<string, Map<string, ProbeResult>>();
	for (let i = 0; i < probeMeta.length; i++) {
		const meta = probeMeta[i];
		const css = probeCss[i];
		const map = results.get(meta.basename) ?? new Map<string, ProbeResult>();
		const slotKey =
			meta.kind === "ns"
				? `ns:${meta.variant}`
				: meta.kind === "named-typed"
					? `nt:${meta.type}`
					: meta.kind === "nonsense"
						? "nonsense"
						: `at:${meta.type}`;
		if (!css) {
			map.set(slotKey, null);
		} else {
			const { sort_property, property_count } = parseDeclarations(css);
			map.set(
				slotKey,
				sort_property ? { sort_property, property_count } : null,
			);
		}
		results.set(meta.basename, map);
	}

	const branchesByBasename = new Map<string, FunctionalBranches>();
	for (const basename of functionalKeys) {
		const branches = emptyFunctionalBranches();
		const map = results.get(basename) ?? new Map<string, ProbeResult>();

		for (const { variant } of THEME_NAMESPACES) {
			const r = map.get(`ns:${variant}`);
			if (!r) continue;
			branches.namedBranches.push({
				kind: "Theme",
				namespace: variant,
				sort_property: r.sort_property,
				property_count: r.property_count,
			});
		}

		for (const p of NAMED_PREDICATE_PROBES) {
			const r = map.get(`nt:${p.type}`);
			if (!r) continue;
			branches.namedBranches.push({
				kind: "Typed",
				value_type: p.type,
				sort_property: r.sort_property,
				property_count: r.property_count,
			});
		}

		const nonsense = map.get("nonsense") ?? null;
		if (nonsense) {
			branches.arbitraryBranches.push({
				kind: "Fallback",
				sort_property: nonsense.sort_property,
				property_count: nonsense.property_count,
			});
		}
		for (const p of ARBITRARY_PROBES) {
			const r = map.get(`at:${p.type}`);
			if (!r) continue;
			if (
				nonsense &&
				r.sort_property === nonsense.sort_property &&
				r.property_count === nonsense.property_count
			) {
				continue;
			}
			branches.arbitraryBranches.push({
				kind: "Typed",
				value_type: p.type,
				sort_property: r.sort_property,
				property_count: r.property_count,
			});
		}

		branchesByBasename.set(basename, branches);
	}
	return branchesByBasename;
}

function addKeywordBranches(
	ds: Awaited<ReturnType<typeof __unstable__loadDesignSystem>>,
	ctx: {
		branchesByBasename: Map<string, FunctionalBranches>;
		staticKeySet: Set<string>;
		allThemeKeys: Set<string>;
		probeTokens: Set<string>;
	},
): void {
	type KeywordGroup = {
		basename: string;
		sort_property: string;
		property_count: number;
		keywords: Set<string>;
	};
	const groups = new Map<string, KeywordGroup>();
	const classList = ds.getClassList().map(([n]) => n);
	const classListCss = ds.candidatesToCss(classList);
	for (let i = 0; i < classList.length; i++) {
		const cls = classList[i];
		if (ctx.staticKeySet.has(cls)) continue;
		const cands = ds.parseCandidate(cls);
		const cand = cands.find((c) => c.kind === "functional");
		if (!cand || cand.kind !== "functional") continue;
		if (!cand.value || cand.value.kind !== "named") continue;
		const value = cand.value.value;
		if (/[\d.]/.test(value)) continue;
		if (ctx.allThemeKeys.has(value)) continue;
		if (ctx.probeTokens.has(value)) continue;
		const css = classListCss[i];
		if (!css) continue;
		const { sort_property, property_count } = parseDeclarations(css);
		if (!sort_property) continue;
		const key = `${cand.root}|${sort_property}|${property_count}`;
		let group = groups.get(key);
		if (!group) {
			group = {
				basename: cand.root,
				sort_property,
				property_count,
				keywords: new Set(),
			};
			groups.set(key, group);
		}
		group.keywords.add(value);
	}
	for (const group of groups.values()) {
		const branches =
			ctx.branchesByBasename.get(group.basename) ?? emptyFunctionalBranches();
		branches.namedBranches.push({
			kind: "Keyword",
			keywords: [...group.keywords].sort(),
			sort_property: group.sort_property,
			property_count: group.property_count,
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
		sameBranchList(
			a.arbitraryBranches,
			b.arbitraryBranches,
			arbitraryBranchKey,
		)
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
			return `N|${b.namespace}|${b.sort_property}|${b.property_count}`;
		case "Keyword":
			return `K|${b.keywords.join(",")}|${b.sort_property}|${b.property_count}`;
		case "Typed":
			return `NT|${b.value_type}|${b.sort_property}|${b.property_count}`;
	}
}

function arbitraryBranchKey(b: ArbitraryBranch): string {
	switch (b.kind) {
		case "Typed":
			return `AT|${b.value_type}|${b.sort_property}|${b.property_count}`;
		case "Fallback":
			return `A|${b.sort_property}|${b.property_count}`;
	}
}

function dedupeFunctionalBranches(branches: FunctionalBranches): FunctionalBranches {
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
