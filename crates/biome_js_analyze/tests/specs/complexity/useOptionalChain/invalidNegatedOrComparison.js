/* should generate diagnostics */

// Each of the eight binary operators that yield the same boolean as the
// optional-chain form when `foo` is nullish (because the left operand of
// the comparison becomes `undefined`).
!foo || foo.bar !== "x";
!foo || foo.bar === "x";
!foo || foo.bar != null;
!foo || foo.bar == null;
!foo || foo.bar > 0;
!foo || foo.bar >= 0;
!foo || foo.bar < 0;
!foo || foo.bar <= 0;

// Longer chains: every `!` on the left contributes a `?.` to the head of
// the comparison.
!foo || foo.bar.baz !== "x";
!foo || !foo.bar || foo.bar.baz !== "x";
!foo || !foo.bar || !foo.bar.baz || foo.bar.baz.buzz > 0;

// Computed-member access and call expressions should also be covered.
!foo || foo[bar] !== "x";
!foo || foo.bar() !== undefined;

// Prefix preserved: leading operands that don't match the chain are kept on
// the left of `||`.
bar || (!foo || foo.bar !== "x");
