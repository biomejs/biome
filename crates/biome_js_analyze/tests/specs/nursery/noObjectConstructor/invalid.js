/* should generate diagnostics */
const a = Object();
const b = new Object();
const c = new Object;
const d = Object?.();
const e = (Object)();
const f = new (Object)();
const g = () => Object();
const h = () => new Object().toString();
const i = Object() instanceof Object;
(new Object() instanceof Object);
foo(Object(), new Object);
function inner() {
	return Object();
}
const j = Object( );
const k = /* leading */ Object() /* trailing */;
const l = Object(/* inner */);
const m = new /* inner */ Object();
