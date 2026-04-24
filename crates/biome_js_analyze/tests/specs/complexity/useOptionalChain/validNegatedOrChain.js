/* should not generate diagnostics */

// Different identifiers - not the same chain
!foo || !bar;
!a || !b;

// Mixed negation - not all operands negated
!a || a.b;
a || !a.b;

// Wrong operator - && instead of ||
!a && !a.b;

// Mixed operators
!a || !a.b && !a.b.c;
