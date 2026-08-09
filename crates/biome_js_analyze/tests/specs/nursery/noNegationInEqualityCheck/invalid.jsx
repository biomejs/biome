/* should generate diagnostics */
!foo === bar;
/* ASI unsafe: < at start of expression after newline (JSX element or TS type assertion) */
foo
!<Component /> === bar;
