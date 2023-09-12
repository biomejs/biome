let a = {
	...spread,

	foo() {
	},

	*foo() {
	},

	async *foo(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb, cccccccccccccccccccccccccccccc) {
	},

	[fooooooooooooooooooooooooooooooooooooooooooooooooo()]: () => {
	},

	[foo()]: {

	},

	...spread,
}

const x = {apple: "banana"};

const y = {
	apple: "banana",
};

({a, b, c} = {a: 'apple', b: 'banana', c: 'coconut'});

({
	a, b, c} = {a: 'apple', b: 'banana', c: 'coconut'});
