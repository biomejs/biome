// Tailwind v4 value-type catalogs.
//
// Mirrors the predicates in
// https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/utils/infer-data-type.ts
//
// Used by the codegen script to tag named-path and arbitrary-path probe
// samples, and to emit the corresponding hand-written Rust enum variants.
//
// When Tailwind adds or removes an arbitrary-value predicate, edit
// `CSS_DATA_TYPES` and rerun codegen + the corresponding Rust predicate body
// in `value_match.rs`. Named-path typed values are limited to parser-level
// Number / Percentage / Ratio categories.

// `FamilyName` and `GenericName` are part of Tailwind v4's `inferDataType`
// catalog, but they are intentionally omitted here. Their only consumer is the
// `font-` utility, whose arbitrary path routes both types to the same property
// (`font-family`) as its type-blind fallback. The codegen's branch-dedupe step
// always collapses the typed arbitrary probe into the existing fallback, so
// carrying the variants would only produce dead enum members and never-fired
// predicates. Add them back if a future utility is found that disambiguates
// property by these data types.
export const NAMED_VALUE_TYPES = ["Number", "Percentage", "Ratio"] as const;

export type NamedValueType = (typeof NAMED_VALUE_TYPES)[number];

export const CSS_DATA_TYPES = [
	"Color",
	"Length",
	"Percentage",
	"Number",
	"Integer",
	"Ratio",
	"Angle",
	"Url",
	"Position",
	"BgSize",
	"LineWidth",
	"Image",
	"AbsoluteSize",
	"RelativeSize",
	"Vector",
] as const;

export type CssDataType = (typeof CSS_DATA_TYPES)[number];
