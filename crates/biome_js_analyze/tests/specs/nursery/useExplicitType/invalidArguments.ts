// argument without type
export var arrowFn = (arg): string => `test ${arg}`;

// argument with any type
export var arrowFn = (arg: any): string => `test ${arg}`;

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

