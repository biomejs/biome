let a = {
	...spread,

	foo() {
	},

	*foo() {
	},
	...spread,
}

const x = { apple: "banna"};

const y = {
	apple: "banana",
};

({a, b, c} = {a: 'apple', b: 'banana', c: 'coconut'});

({
	a, b, c
} = {a: 'apple', b: 'banana', c: 'coconut' });
