// CSS value-type catalog — single source of truth.
//
// Mirrors the predicates in
// https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/utils/infer-data-type.ts
//
// Used by the codegen script to:
//   - emit `pub enum CssDataType { ... }` into the generated Rust file
//   - tag arbitrary-value sample inputs during utility probing
//
// When Tailwind adds or removes a predicate, edit this list and rerun the
// codegen + the corresponding Rust predicate body in `predicates.rs`.

// `FamilyName` and `GenericName` are part of Tailwind v4's `inferDataType`
// catalog, but they are intentionally omitted here. Their only consumer is the
// `font-` utility, whose arbitrary path routes both types to the same property
// (`font-family`) as its type-blind fallback. The codegen's branch-dedupe step
// always collapses the typed arbitrary probe into the existing fallback, so
// carrying the variants would only produce dead enum members and never-fired
// predicates. Add them back if a future utility is found that disambiguates
// property by these data types.
export const VALUE_TYPES = [
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

export type CssDataType = (typeof VALUE_TYPES)[number];
