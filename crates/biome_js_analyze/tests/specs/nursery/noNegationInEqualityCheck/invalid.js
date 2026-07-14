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
/* ASI unsafe: [ at start of expression after newline */
foo
![1,2,3] === bar;
/* ASI unsafe: template literal after newline */
foo
!`template` === bar;
/* ASI unsafe: unary + after newline */
foo
!+x === bar;
/* ASI unsafe: unary - after newline */
foo
!-x === bar;
/* ASI unsafe: ( at start of expression after newline */
foo
!(x) === bar;
/* Bug 1: function/class/object at expression start should be wrapped in parens */
!function(){} === bar;
!class{} === bar;
!{} === bar;
/* ASI unsafe: block comment with newline before ! exposes restricted char */
/*
*/!/regex/.test(value) === bar;
