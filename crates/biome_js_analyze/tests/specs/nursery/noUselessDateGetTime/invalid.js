/* should generate diagnostics */
new Date(date.getTime());
new Date(date.getTime(),);
new Date(new Date(date.getTime()).getTime());
new Date((0, date).getTime());
new Date((date.getTime()));
new Date((date.getTime)());
new Date(foo.bar.getTime());
new Date(getDate().getTime());
new globalThis.Date(date.getTime());
new window.Date(date.getTime());
const cloned = new Date(originalDate.getTime());
new Date(date.getTime(/* comment */));
new Date(date./* comment */getTime());
new Date(date /* comment */.getTime());
new Date(date.getTime /* comment */());
new Date(date.getTime() /* comment */);
new Date(date // comment
	.getTime());
new Date(getDate(/* comment */).getTime());
new Date((date /* comment */).getTime());
new Date(
	date.getTime()
);
