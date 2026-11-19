/* should not generate diagnostics */

// Class members.
class A {
	a = this;
	static b = this;
	c = console.log(this);
	d = z((x) => console.log(x, this));
	e = function () {
		this.a = 0;
	};
	#f = function () {
		this.a = 0;
	};
	g = () => this;
	accessor h = this.a;
	constructor(a = this.a) {
		this.a = 0;
	}
	method() {
		this.a = 0;
		z((x) => console.log(x, this));
	}
	static staticMethod() {
		this.a = 0;
	}
	get getter() {
		return this.a;
	}
	set setter(value) {
		this.a = value;
	}
	static {
		this.a = 0;
		() => {
			this.a = 0;
		};
		class D {
			[this.a];
		}
	}
}

// Object members.
const obj1 = {
	foo() {
		this.a = 0;
	},
	bar: function () {
		this.a = 0;
	},
	get baz() {
		return this.a;
	},
	set baz(value) {
		this.a = value;
	},
	qux: foo || function () {
		this.a = 0;
	},
	quux: hasNative ? foo : function () {
		this.a = 0;
	},
	corge: (function () {
		return function () {
			this.a = 0;
		};
	})(),
};
Object.defineProperty(obj, "foo", {
	value: function () {
		this.a = 0;
	},
});

// Functions assigned to object properties.
obj.foo = function () {
	this.a = 0;
};
obj["foo"] = function () {
	this.a = 0;
};
obj.foo = foo || function () {
	this.a = 0;
};
obj.foo = foo ? bar : function () {
	this.a = 0;
};
obj.foo = (function () {
	return function () {
		this.a = 0;
	};
})();
obj.foo = (() => function () {
	this.a = 0;
})();
obj.foo = (function () {
	return function () {
		this.a = 0;
	};
})?.();
obj.method &&= function () {
	this.a = 0;
};
obj.method ||= function () {
	this.a = 0;
};
obj.method ??= function () {
	this.a = 0;
};
[obj.method = function () {
	this.a = 0;
}] = list;

// Constructors.
function Foo() {
	this.a = 0;
	z((x) => console.log(x, this));
}
export function ExportedFoo() {
	this.a = 0;
}
const Bar = function () {
	this.a = 0;
};
const Baz = function Baz() {
	this.a = 0;
};
Qux = function () {
	this.a = 0;
};
function Φ() {
	this.a = 0;
}
const 𐐀 = function () {
	this.a = 0;
};
function withDefault(Ctor = function () {
	this.a = 0;
}) {}
[Ctor = function () {
	this.a = 0;
}] = list;

// Functions called with a `this` value.
const bound = function () {
	this.a = 0;
}.bind(obj);
(function () {
	this.a = 0;
}).call(obj);
(function () {
	this.a = 0;
}).apply(obj, []);
(function () {
	this.a = 0;
})["call"](obj);
const optionalBound = function () {
	this.a = 0;
}?.bind(obj);
const parenthesizedBound = (function () {
	this.a = 0;
}?.bind)(obj);
const optionalCall = function () {
	this.a = 0;
}.bind?.(obj);
Reflect.apply(function () {
	this.a = 0;
}, obj, []);
Array.from(list, function () {
	this.a = 0;
}, obj);
Int8Array.from(list, function () {
	this.a = 0;
}, obj);
Array.fromAsync(list, function () {
	this.a = 0;
}, obj);
Array?.from(list, function () {
	this.a = 0;
}, obj);
(Array?.from)(list, function () {
	this.a = 0;
}, obj);
list.every(function () { this.a = 0; }, obj);
list.filter(function () { this.a = 0; }, obj);
list.find(function () { this.a = 0; }, obj);
list.findIndex(function () { this.a = 0; }, obj);
list.findLast(function () { this.a = 0; }, obj);
list.findLastIndex(function () { this.a = 0; }, obj);
list.flatMap(function () { this.a = 0; }, obj);
list.forEach(function () { this.a = 0; }, obj);
list.map(function () { this.a = 0; }, obj);
list.some(function () { this.a = 0; }, obj);
list?.every(function () { this.a = 0; }, obj);
(list?.every)(function () { this.a = 0; }, obj);
