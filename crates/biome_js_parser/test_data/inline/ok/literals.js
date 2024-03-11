5
true
false
5n
"foo"
'bar'
null
0, 0.0, 0n, 0e00
"test\
new-line";
/^[يفمئامئ‍ئاسۆند]/i; //regex with unicode
/[\p{Control}--[\t\n]]/v;
/\’/; // regex with escaped non-ascii chars (issue #1941)
