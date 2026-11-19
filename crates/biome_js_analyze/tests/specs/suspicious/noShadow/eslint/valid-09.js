/* should not generate diagnostics */

// The first `let` does not leak outside the block, and so does not shadow the second `let`.
{ let a; } let a;
