// argument without type
export var arrowFn = (arg): string => `test ${arg}`;

// argument with any type
export var arrowFn = (arg: any): string => `test ${arg}`;

var foo = arr.map((i) => i * i);
new Promise((resolve) => resolve(1));

// js binding argument
new Promise(resolve => resolve(1));


class Test {
	constructor(foo) {}
	get prop(): number {
		return 1;
	}
	set prop(foo) {}
	method(foo): void {
		return;
	}
	arrow = (foo): string => "arrow";
}

var obj = {
	method(foo): string {
		return "test";
	},
	set prop(foo) {}
};

