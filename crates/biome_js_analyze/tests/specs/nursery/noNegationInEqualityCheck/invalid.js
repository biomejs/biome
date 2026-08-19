/* should generate diagnostics */
!foo === bar;
(!foo) === bar;
(!foo) !== bar;
((!foo)) === bar;
(((!foo))) === bar;
/* should preserve trailing comments */
!/* keep */foo === bar;
!/* keep */ foo === bar;
(/* keep */ !foo) === bar;
(/* keep */ !foo) !== bar;
((/* keep */ !foo)) === bar;
(/* a */ /* b */ !foo) === bar;
(/* c1 */!foo/* c2 */) === bar;
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
/* ASI unsafe: block comment with newline in previous token's trailing trivia */
foo /* multi
line */!(x) === bar;
/* ASI unsafe: block comment with newline before ! exposes restricted char */
/*
*/!/regex/.test(value) === bar;
/* Issue 1: ASI guard misses generated parens — function/class/object
with newline should skip fix (wrapping adds '(' which is an ASI hazard) */
foo
!function(){} === bar;
foo
!class{} === bar;
foo
!{} === bar;
/* Issue 2: line comment newlines preserved — without \n the comment
would comment out the replacement operator */
(!foo // close
) === bar;
/* Issue 3: leading comments on argument preserved — parser may attach
trivia between ! and arg as the argument's leading trivia */
!/* leading on arg */foo === bar;
