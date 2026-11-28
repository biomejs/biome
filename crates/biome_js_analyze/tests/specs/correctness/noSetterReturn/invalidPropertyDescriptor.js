/* should generate diagnostics */
let v;
const o = {};

Object.defineProperty(o, "arrowExpressionBody", {
	get: () => v,
	set: (nv) => (v = nv),
});

Object.defineProperty(o, "arrowBlockBody", {
	set: (nv) => {
		return nv;
	},
});

Object.defineProperty(o, "functionExpression", {
	set: function (nv) {
		return nv;
	},
});

Object.defineProperty(o, "method", {
	set(nv) {
		if (nv) {
			return nv;
		}
	},
});

Reflect.defineProperty(o, "reflect", {
	set(nv) {
		return nv;
	},
});

Object.defineProperties(o, {
	defineProperties: {
		set(nv) {
			return nv;
		},
	},
});

Object.create(null, {
	create: {
		set: (nv) => nv,
	},
});

Object.defineProperty(o, "stringKey", {
	"set"(nv) {
		return nv;
	},
});

Object.defineProperty(o, "computedKey", {
	["set"]: (nv) => nv,
});

Object.defineProperty(o, "templateKey", {
	[`set`]: (nv) => nv,
});

Object.defineProperty(o, "parentheses", ({
	set: ((nv) => nv),
}));

Object?.defineProperty(o, "optionalChain", {
	set(nv) {
		return nv;
	},
});

Object["defineProperty"](o, "computedMethod", {
	set(nv) {
		return nv;
	},
});

globalThis.Object.defineProperty(o, "globalThis", {
	set(nv) {
		return nv;
	},
});
