// CSS value-type catalog — single source of truth.
//
// Mirrors the predicates in
// https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/utils/infer-data-type.ts
//
// Used by the codegen script to:
//   - emit `pub enum ValueType { ... }` into the generated Rust file
//   - tag arbitrary-value sample inputs during utility probing
//
// When Tailwind adds or removes a predicate, edit this list and rerun the
// codegen + the corresponding Rust predicate body in `predicates.rs`.

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
	"FamilyName",
	"GenericName",
	"AbsoluteSize",
	"RelativeSize",
	"Vector",
] as const;

export type ValueType = (typeof VALUE_TYPES)[number];
