/* should generate diagnostics */
const a = null;
let b = null;
var c = 1, d = null, e = 2;
let f = (null);
let g = /* comment */ null /* trailing */;
let { h } = null;
foo(null);
foo(bar, null);
new HttpResponse(null);
foo([null]);
foo(bar ?? null);
foo(...[null]);
x = null;
if (foo == null) {}
if (foo != null) {}
if (null == foo) {}
if (null != foo) {}
function returnsNull() {
	return null;
}
const arrow = () => null;
const object = { key: null };
delete null;
delete ((null));
delete [null][0];
new Object.create(null);
new foo.insertBefore(bar, null);
create(null);
insertBefore(bar, null);
Object["create"](null);
foo["insertBefore"](bar, null);
Object[create](null);
foo[insertBefore](bar, null);
Object.notCreate(null);
foo.notInsertBefore(foo, null);
NotObject.create(null);
lib.Object.create(null);
Object.create(...[null]);
Object.create(null, bar, extraArgument);
Object.create(null, ...rest);
Object?.create(null);
Object.create?.(null);
foo.insertBefore(null);
foo.insertBefore(foo, null, bar);
foo.insertBefore(...[foo], null);
foo.insertBefore(null, bar);
foo.insertBefore?.(bar, null);
Object.create(bar, null);
useRef?.(null);
useRef(null, extra);
React.useRef?.(null);
React?.useRef(null);
React.useRef(null, extra);
