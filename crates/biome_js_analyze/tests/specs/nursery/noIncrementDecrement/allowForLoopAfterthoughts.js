// Invalid
let foo = 0;
foo++;

let baz = 0;
++baz;

for (let i = 0; i < 10; j = i++) {
	doSomething(i, j);
}

for (let i = 10; i--;) {
	doSomething(i);
}

for (let i = 0; i < 10;) i++;

// Valid
for (let i = 0; i < 10; i++) {
	doSomething(i);
}
