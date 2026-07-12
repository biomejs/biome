/* should generate diagnostics */
!foo === bar;
(!foo) === bar;
(!foo) !== bar;
((!foo)) === bar;
(((!foo))) === bar;
/* should preserve trailing comments */
!/* keep */foo === bar;
/* comment before bang should be preserved */
/* c1 */!foo === bar;
!foo /* c2 */ === bar;
/* should signal but fix should be skipped (ASI unsafe) */
foo
!/regex/.test(value) === bar;
