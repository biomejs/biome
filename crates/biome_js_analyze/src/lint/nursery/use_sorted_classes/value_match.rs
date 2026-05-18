//! AST predicates for Tailwind v4 ArbitraryTyped value classification.
//!
//! Each ValueType predicate receives a `CssGenericComponentValueList` and
//! returns whether the parsed arbitrary value satisfies that type. The caller
//! walks utility branches in preset order, mirroring Tailwind's
//! `infer-data-type.ts` priority model without collapsing the value to one
//! global type first.

use biome_tailwind_syntax::CssGenericComponentValueList;

use super::tailwind_preset_v4_types::ValueType;

#[expect(dead_code, reason = "wired into sort_v4 in a follow-up task")]
pub fn value_matches_type(_list: &CssGenericComponentValueList, _vt: ValueType) -> bool {
    // TODO: Implement per-ValueType predicates in follow-up tasks.
    false
}
