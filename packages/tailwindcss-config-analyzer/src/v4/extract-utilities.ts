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
//           emit `Named` if compiles.
//        b. (basename × Number/Percentage/Ratio sample): `<basename>-7`,
//           `<basename>-25%`, `<basename>-1/2` → emit `NamedTyped`.
//   3. Arbitrary-path probes:
//        a. Nonsense `<basename>-[abcxyz]` — establishes the utility's
//           type-blind fallback property, if any.
//        b. (basename × every ValueType, with explicit dataType marker)
//           `<basename>-[<marker>:<sample>]` — emit `Typed` only when
//           the resulting (property, count) differs from the nonsense
//           fallback. Equal results are absorbed by the `Arbitrary`
//           entry.
//        c. If the utility produced a nonsense result, emit one
//           `Arbitrary` entry carrying that fallback property.
//   4. Iterate `getClassList()` for hardcoded keywords on functional
//      utilities. Classes not in `keys('static')`, not numeric, not
//      theme-keyed, and not a probe token are keyword variants baked
//      into the compileFn (e.g. `origin-top`, `accent-current`). Group
//      by (basename, prop, count) and emit `NamedKeyword` branches.
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
import type { ValueType } from "./value-types.js";

export type StaticUtility = {
	name: string;
	registration_idx: number;
	sort_property: string;
	property_count: number;
};

export type Branch =
	| {
			kind: "Named";
			namespace: ThemeNamespaceVariant;
			sort_property: string;
			property_count: number;
	  }
	| {
			kind: "NamedKeyword";
			keywords: string[];
			sort_property: string;
			property_count: number;
	  }
	| {
			kind: "NamedTyped";
			value_type: ValueType;
			sort_property: string;
			property_count: number;
	  }
	| {
			kind: "Typed";
			value_type: ValueType;
			sort_property: string;
			property_count: number;
	  }
	| {
			kind: "Arbitrary";
			sort_property: string;
			property_count: number;
	  };

export type FunctionalUtility = {
	basename: string;
	registration_idx: number;
	branches: Branch[];
};

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

	const functionalUtilities: FunctionalUtility[] = [];
	for (let i = 0; i < functionalKeys.length; i++) {
		const basename = functionalKeys[i];
		const branches = dedupeBranches(branchesByBasename.get(basename) ?? []);
		functionalUtilities.push({
			basename,
			registration_idx: i,
			branches,
		});
	}

	return { static: staticUtilities, functional: functionalUtilities };
}

function extractStatic(
	ds: Awaited<ReturnType<typeof __unstable__loadDesignSystem>>,
	staticKeys: string[],
): StaticUtility[] {
	const staticCss = ds.candidatesToCss(staticKeys);
	const out: StaticUtility[] = [];
	for (let i = 0; i < staticKeys.length; i++) {
		const css = staticCss[i];
		if (!css) continue;
		const { sort_property, property_count } = parseDeclarations(css);
		if (!sort_property) continue;
		out.push({
			name: staticKeys[i],
			registration_idx: i,
			sort_property,
			property_count,
		});
	}
	return out;
}

type ProbeSlot =
	| { basename: string; kind: "ns"; variant: ThemeNamespaceVariant }
	| { basename: string; kind: "named-typed"; type: ValueType }
	| { basename: string; kind: "nonsense" }
	| { basename: string; kind: "typed"; type: ValueType };

function extractFunctionalBranches(
	ds: Awaited<ReturnType<typeof __unstable__loadDesignSystem>>,
	functionalKeys: string[],
): Map<string, Branch[]> {
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
			probeMeta.push({ basename, kind: "typed", type: p.type });
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
						: `t:${meta.type}`;
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

	const branchesByBasename = new Map<string, Branch[]>();
	for (const basename of functionalKeys) {
		const branches: Branch[] = [];
		const map = results.get(basename) ?? new Map<string, ProbeResult>();

		for (const { variant } of THEME_NAMESPACES) {
			const r = map.get(`ns:${variant}`);
			if (!r) continue;
			branches.push({
				kind: "Named",
				namespace: variant,
				sort_property: r.sort_property,
				property_count: r.property_count,
			});
		}

		for (const p of NAMED_PREDICATE_PROBES) {
			const r = map.get(`nt:${p.type}`);
			if (!r) continue;
			branches.push({
				kind: "NamedTyped",
				value_type: p.type,
				sort_property: r.sort_property,
				property_count: r.property_count,
			});
		}

		const nonsense = map.get("nonsense") ?? null;
		if (nonsense) {
			branches.push({
				kind: "Arbitrary",
				sort_property: nonsense.sort_property,
				property_count: nonsense.property_count,
			});
		}
		for (const p of ARBITRARY_PROBES) {
			const r = map.get(`t:${p.type}`);
			if (!r) continue;
			if (
				nonsense &&
				r.sort_property === nonsense.sort_property &&
				r.property_count === nonsense.property_count
			) {
				continue;
			}
			branches.push({
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
		branchesByBasename: Map<string, Branch[]>;
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
		const list = ctx.branchesByBasename.get(group.basename) ?? [];
		list.push({
			kind: "NamedKeyword",
			keywords: [...group.keywords].sort(),
			sort_property: group.sort_property,
			property_count: group.property_count,
		});
		ctx.branchesByBasename.set(group.basename, list);
	}
}

function dedupeBranches(branches: Branch[]): Branch[] {
	const seen = new Set<string>();
	const out: Branch[] = [];
	for (const b of branches) {
		let key: string;
		switch (b.kind) {
			case "Named":
				key = `N|${b.namespace}|${b.sort_property}|${b.property_count}`;
				break;
			case "NamedKeyword":
				key = `K|${b.keywords.join(",")}|${b.sort_property}|${b.property_count}`;
				break;
			case "NamedTyped":
				key = `NT|${b.value_type}|${b.sort_property}|${b.property_count}`;
				break;
			case "Typed":
				key = `T|${b.value_type}|${b.sort_property}|${b.property_count}`;
				break;
			case "Arbitrary":
				key = `A|${b.sort_property}|${b.property_count}`;
				break;
		}
		if (seen.has(key)) continue;
		seen.add(key);
		out.push(b);
	}
	return out;
}
