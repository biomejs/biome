// =====================
// Class setters
// =====================

// Basic setter
class Foo1 {
	set a(b) {}
}

// Setter with assignment in body
class Foo2 {
	set a(b) {
		this._a = b;
	}
}

// Static setter
class Foo3 {
	static set a(b) {}
}

// Private setter
class Foo4 {
	set #a(b) {}
}

// Static private setter
class Foo5 {
	static set #a(b) {}
}

// Setter with getter pair
class Foo6 {
	get a() {
		return this._a;
	}
	set a(b) {
		this._a = b;
	}
}

// Multiple setters
class Foo7 {
	set a(b) {}
	set c(d) {}
}

// Setter with computed property name
class Foo8 {
	set ["a"](b) {}
}

// Setter with computed property name from variable
const foo = "foo";
class Foo9 {
	set [foo](b) {}
}

// Setter in class expression
const Foo = class {
	set a(b) {}
};

// Setter in class with extends
class Foo10 extends Bar {
	set a(b) {}
}

// Setter with decorators
class Foo11 {
	@a
	set b(c) {}
}

// Setter with multiple decorators
class Foo12 {
	@a
	@b
	set c(d) {}
}

// Setter with complex body
class Foo13 {
	set a(b) {
		if (b > 0) {
			this._a = b;
		} else {
			throw new Error("Invalid value");
		}
	}
}

// Setter calling super
class Foo14 extends Bar {
	set a(b) {
		super.a = b;
	}
}

// Setter with numeric property name
class Foo15 {
	set 0(a) {}
}

// Setter with string property name
class Foo16 {
	set "foo"(a) {}
}

// Anonymous class with setter
new (class {
	set a(b) {}
})();

// Setter in nested class
class Foo17 {
	bar() {
		return class {
			set a(b) {}
		};
	}
}

// Setter with trailing comma
class Foo18 {
	set a(b,) {}
}

// Boundary test (fits): 78+2=80 chars with delimiter spacing
class Foo19 {
	set a({ a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, uv }) {}
}

// Boundary test (breaks): 79+2=81 chars with delimiter spacing
class Foo20 {
	set a({ a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, uvw }) {}
}

// =====================
// Object setters
// =====================

// Basic setter in object literal
const foo1 = {
	set a(b) {},
};

// Setter with assignment in body
const foo2 = {
	set a(b) {
		this._a = b;
	},
};

// Setter with getter pair
const foo3 = {
	get a() {
		return this._a;
	},
	set a(b) {
		this._a = b;
	},
};

// Multiple setters
const foo4 = {
	set a(b) {},
	set c(d) {},
};

// Setter with numeric property name
const foo5 = {
	set 0(a) {},
};

// Setter with string property name
const foo6 = {
	set 'foo'(a) {},
};

// Setter in nested object
const foo7 = {
	bar: {
		set a(b) {},
	},
};

// Setter in object returned from function
function bar() {
	return {
		set a(b) {},
	};
}

// Setter with computed property name
const foo8 = {
	set ['bar'](a) {},
};

// Setter with computed property name from variable
const baz = 'foo';
const foo9 = {
	set [baz](b) {},
};

// Setter with complex body
const foo10 = {
	set a(b) {
		if (b > 0) {
			this._a = b;
		} else {
			throw new Error('Invalid value');
		}
	},
};

// Anonymous object in expression context
console.log({
	set a(b) {},
});

// Setter with trailing comma in object literal
const foo11 = {
	set a(b,) {},
};

// Boundary test (fits): 78+2=80 chars with delimiter spacing
const foo12 = {
	set a({ a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u }) {},
};

// Boundary test (breaks): 79+2=81 chars with delimiter spacing
const foo13 = {
	set a({ a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, uv }) {},
};
