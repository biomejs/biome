/* should generate diagnostics */
!foo === bar;
/* ASI unsafe: < at start of expression after newline (JSX element or TS type assertion) */
foo
!<Component /> === bar;
/* ASI unsafe: function/class/object at line start — wrapping would create call continuation */
foo
!function(){} === bar;
foo
!class{} === bar;
foo
!{} === bar;
