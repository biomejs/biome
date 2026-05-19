// Sample values used by the codegen probe matrix.

import type { ThemeNamespaceVariant } from "./theme-namespaces.js";
import type { CssDataType, NamedValueType } from "./value-types.js";

// One CSS-valid value per theme namespace, injected into a probe
// `@theme` block so each namespace exposes a UNIQUE token. The exact
// resolved value is irrelevant — only its CSS validity matters.
export const NAMESPACE_PROBE_VALUE: Record<ThemeNamespaceVariant, string> = {
	Color: "#000",
	Spacing: "1rem",
	Text: "1rem",
	TextShadow: "0 0 1px #000",
	Font: "Arial",
	FontWeight: "400",
	Leading: "1.5",
	Tracking: "0.025em",
	Breakpoint: "40rem",
	Container: "20rem",
	Radius: "0.25rem",
	Shadow: "0 0 1px #000",
	InsetShadow: "inset 0 0 1px #000",
	DropShadow: "0 0 1px #000",
	Blur: "4px",
	Perspective: "500px",
	Aspect: "16/9",
	Ease: "linear",
	Animate: "spin 1s linear infinite",
	BackgroundImage: "linear-gradient(#000, #fff)",
};

// Token used for the per-namespace probe class. Lowercase + variant
// suffix; collision-free against any real Tailwind theme key.
export function probeToken(variant: ThemeNamespaceVariant): string {
	return `zzprobe${variant.toLowerCase()}`;
}

// Bare value samples used for the named-path predicate probe (no
// brackets). Tailwind v4 only accepts these three predicate kinds in
// named form; other CSS value types must be written as arbitrary
// values (`[length:1rem]` etc.).
export const NAMED_PREDICATE_PROBES: { type: NamedValueType; value: string }[] = [
	{ type: "Number", value: "7" },
	{ type: "Percentage", value: "25%" },
	{ type: "Ratio", value: "1/2" },
];

// Per-CssDataType samples used for the arbitrary-path probe. The
// explicit `[<dataType-marker>:<value>]` syntax forces dispatch
// through the matching compileFn branch in Tailwind. Samples avoid
// whitespace because Tailwind's underscore→space substitution would
// muddy the probe.
export const ARBITRARY_PROBES: {
	type: CssDataType;
	marker: string;
	value: string;
}[] = [
	{ type: "Color", marker: "color", value: "#000" },
	{ type: "Length", marker: "length", value: "10px" },
	{ type: "Percentage", marker: "percentage", value: "25%" },
	{ type: "Number", marker: "number", value: "7" },
	{ type: "Integer", marker: "integer", value: "100" },
	{ type: "Ratio", marker: "ratio", value: "16/9" },
	{ type: "Angle", marker: "angle", value: "45deg" },
	{ type: "Url", marker: "url", value: "url(a.png)" },
	{ type: "Position", marker: "position", value: "top" },
	{ type: "BgSize", marker: "bg-size", value: "cover" },
	{ type: "LineWidth", marker: "line-width", value: "thin" },
	{ type: "Image", marker: "image", value: "linear-gradient(red,blue)" },
	// `FamilyName` / `GenericName` removed: their probe is always deduped
	// against the `Arbitrary` fallback for the `font-` utility (both route to
	// `font-family`). See `value-types.ts` for the rationale.
	{ type: "AbsoluteSize", marker: "absolute-size", value: "small" },
	{ type: "RelativeSize", marker: "relative-size", value: "larger" },
	{ type: "Vector", marker: "vector", value: "1_0_0" },
];

// Nonsense arbitrary value — passes the parser, fails every type
// predicate. Used as the type-blind baseline; if it compiles, the
// utility has an `Arbitrary` fallback path.
export const NONSENSE_PROBE = "abcxyz";
