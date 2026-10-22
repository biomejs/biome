/* should not generate diagnostics */
let v;
const o = {};

// The setter doesn't return a value
Object.defineProperty(o, "noReturn", {
	get: () => v,
	set(nv) {
		v = nv;
	},
});

// Early return without a value
Object.defineProperty(o, "earlyReturn", {
	set: (nv) => {
		if (!nv) {
			return;
		}
		v = nv;
	},
});

// Returns inside nested functions belong to those functions
Object.defineProperty(o, "nestedFunctions", {
	set(nv) {
		v = [nv].map((x) => x * 2);
		function helper() {
			return nv;
		}
		helper();
	},
});

// Not a property descriptor
const plainObject = {
	set: (nv) => nv,
};
defineProperty(o, "otherCallee", { set: (nv) => nv });
Foo.defineProperty(o, "otherObject", { set: (nv) => nv });
Object.assign(o, { set: (nv) => nv });
Object.defineProperty(o, { set: (nv) => nv });
Object.create({ set: (nv) => nv });
Object.defineProperties(o, { set: (nv) => nv });
Reflect.defineProperties(o, { prop: { set: (nv) => nv } });

// The key isn't statically `set`
const key = "set";
Object.defineProperty(o, "dynamicKey", {
	[key]: (nv) => nv,
});

// The setter is defined elsewhere
const setter = (nv) => nv;
Object.defineProperty(o, "reference", { set: setter });

// `Object` and `Reflect` are local variables, not the globals
function shadowed(Object, Reflect) {
	Object.defineProperty(o, "shadowedObject", {
		set: (nv) => nv,
	});
	Reflect.defineProperty(o, "shadowedReflect", {
		set: (nv) => nv,
	});
}
