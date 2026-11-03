/* should generate diagnostics */
value.trim().startsWith("-");
value.trim().endsWith("-");
value.trim().startsWith();
value.trim().endsWith();
" foo ".trim().startsWith("f");
" foo ".trim().endsWith("o");
value.trim().startsWith(" foo");
value.trim().endsWith("foo ");
value.trim().startsWith("");
value.trim().startsWith(`foo`);
value.trim().endsWith(``);
value.trim().startsWith("foo\\n");
value.trim().endsWith("\\tfoo");
value.trim().startsWith("foo" + "bar");
value.trim().startsWith(" " + "foo");
value.trim().startsWith("foo " + "x" + "");
value.trim().endsWith("" + "foo " + 1);
value.trim().startsWith(undefined);
value.trim().endsWith(void 0);
value.trim().startsWith(123);
value.trim().endsWith(false);
value.trim().startsWith(null);
value.trim().endsWith(1n);
value.trim().startsWith(1 + 2);
value.trim().endsWith(1 + " foo");
(value).trim().startsWith("-");
(value.trim()).startsWith("-");
object.value.trim().startsWith("-");
getValue().trim().endsWith("-");
{ const prefix = "foo"; value.trim().startsWith(prefix); }
{ const suffix = `foo`; value.trim().endsWith(suffix); }
{ const search = undefined; value.trim().startsWith(search); }
{ const head = " foo"; const prefix = head + "bar"; value.trim().startsWith(prefix); }
{ const text = "foo"; text.trim().startsWith("f"); }
if (value.trim().startsWith("-")) {}
value
	// comment
	.trim/* comment */()
	.startsWith("-");
