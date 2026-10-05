// Extract built-in Tailwind v4 variant metadata used for sorting.

import { __unstable__loadDesignSystem } from "tailwindcss";
import { makeLoadStylesheet } from "./css-helpers.js";

export type VariantKind = "Static" | "Functional" | "Compound";

export type VariantCompare =
	| "Default"
	| "BreakpointAsc"
	| "BreakpointDesc"
	| "ContainerAsc"
	| "ContainerDesc";

export type ExtractedVariant = {
	name: string;
	kind: VariantKind;
	order: number;
	compare: VariantCompare;
	compounds: number;
	compounds_with: number;
};

export type ThemeValue = {
	name: string;
	value: string;
};

export type ExtractedVariants = {
	variants: ExtractedVariant[];
	breakpoints: ThemeValue[];
	containers: ThemeValue[];
};

type RawVariant = {
	kind: "static" | "functional" | "compound";
	order: number;
	compounds: number;
	compoundsWith: number;
};

type DesignSystemWithVariants = {
	variants: {
		variants: Map<string, RawVariant>;
	};
	theme: {
		namespace(name: string): Iterable<[string, string]>;
	};
};

export async function extractVariants(): Promise<ExtractedVariants> {
	const ds = (await __unstable__loadDesignSystem(`@import "tailwindcss";`, {
		base: process.cwd(),
		loadStylesheet: makeLoadStylesheet(),
	})) as unknown as DesignSystemWithVariants;

	const breakpoints = themeValues(ds, "--breakpoint");
	const containers = themeValues(ds, "--container");
	const breakpointNames = new Set(breakpoints.map(({ name }) => name));

	const variants = [...ds.variants.variants.entries()]
		.map(([name, variant]) => ({
			name,
			kind: variantKind(variant.kind),
			order: variant.order,
			compare: compareKind(name, breakpointNames),
			compounds: variant.compounds,
			compounds_with: variant.compoundsWith,
		}))
		.sort(
			(a, b) =>
				a.order - b.order || (a.name < b.name ? -1 : a.name > b.name ? 1 : 0),
		);

	return { variants, breakpoints, containers };
}

function themeValues(
	ds: DesignSystemWithVariants,
	namespace: string,
): ThemeValue[] {
	return [...ds.theme.namespace(namespace)]
		.map(([name, value]) => ({ name, value }))
		.sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
}

function variantKind(kind: RawVariant["kind"]): VariantKind {
	switch (kind) {
		case "static":
			return "Static";
		case "functional":
			return "Functional";
		case "compound":
			return "Compound";
	}
}

function compareKind(
	name: string,
	breakpointNames: Set<string>,
): VariantCompare {
	if (name === "max") {
		return "BreakpointDesc";
	}
	if (name === "min" || breakpointNames.has(name)) {
		return "BreakpointAsc";
	}
	if (name === "@max") {
		return "ContainerDesc";
	}
	if (name === "@" || name === "@min") {
		return "ContainerAsc";
	}
	return "Default";
}
