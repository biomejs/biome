/* should generate diagnostics */
// The fix keeps these comments.
x = ((1 /* keep */ + 2) - 4) > 0;
(5 - (1 /* keep */ + 2)) === 0;
(left[/* keep */ key] - right) > 0;
0 < (left - getRight(/* keep */));
/* before */ a - b > 0 /* after */;

// The fix would delete these comments, so there is no fix.
a - /* comment */ b > 0;
a /* comment */ - b > 0;
a - b /* comment */ > 0;
a - b > /* comment */ 0;
a - b > (/* comment */ 0);
(/* comment */ a - b) > 0;
0 /* comment */ < a - b;
