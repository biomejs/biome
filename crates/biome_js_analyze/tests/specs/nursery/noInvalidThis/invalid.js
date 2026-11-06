/* should generate diagnostics */

// Top level of a module.
this.a = 0;
console.log(this);
z((x) => console.log(x, this));
() => {
	this.a = 0;
};

// Functions.
function foo() {
	this.a = 0;
}
export function exported() {
	this.a = 0;
}
export default function () {
	this.a = 0;
}
(function () {
	this.a = 0;
})();
const func = function () {
	this.a = 0;
};
func2 = function () {
	this.a = 0;
};
const Named = function named() {
	this.a = 0;
};
function bar() {
	z((x) => console.log(x, this));
}
function nested() {
	function inner() {
		this.a = 0;
	}
}
function params(a = this.a) {}

// Functions returned from methods.
const obj1 = {
	foo() {
		function inner() {
			this.a = 0;
		}
	},
	bar: function () {
		return function () {
			this.a = 0;
		};
	},
};
obj.foo = function () {
	return function () {
		this.a = 0;
	};
};
class A {
	foo() {
		return function () {
			this.a = 0;
		};
	}
	static {
		function foo() {
			this.a = 0;
		}
		(function () {
			this.a = 0;
		})();
	}
}

// Computed keys and heritage use the surrounding `this`.
class B {
	[this.a] = 0;
	[this.b]() {}
	static {}
	[this.c];
}
const obj2 = {
	[this.a]: 0,
	[this.b]() {},
};
class C extends this.Base {}

// Functions passed somewhere without a `this` value.
foo(function () {
	this.a = 0;
});
new Foo(function () {
	this.a = 0;
});
const callee = (function () {
	this.a = 0;
}).bar(obj);
const boundNull = function () {
	this.a = 0;
}.bind(null);
(function () {
	this.a = 0;
}).call(undefined);
(function () {
	this.a = 0;
}).apply(void 0);
(function () {
	this.a = 0;
}).call();
list.forEach(function () {
	this.a = 0;
});
list.forEach(function () {
	this.a = 0;
}, null);
Array.from(list, function () {
	this.a = 0;
});
Reflect.apply(function () {
	this.a = 0;
}, null, []);
// Arrow functions returned from a function inherit its `this`.
obj.foo = (function () {
	return () => {
		this.a = 0;
	};
})();
// The top level of a module.
obj.foo = (() => () => {
	this.a = 0;
})();
const notIife = (function () {
	return function () {
		this.a = 0;
	};
});
const notArrowIife = () => function () {
	this.a = 0;
};
const wrapped = foo(() => function () {
	this.a = 0;
})();
[func3 = function () {
	this.a = 0;
}] = list;
function defaultParameter(func = function () {
	this.a = 0;
}) {}
