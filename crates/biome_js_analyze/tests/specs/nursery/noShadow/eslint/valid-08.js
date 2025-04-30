/* should not generate diagnostics */

// This is caught by `noRedeclare`, so we don't need to flag it again as a shadow.
{ var a; } var a;
