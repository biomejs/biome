let foo = 0;
foo++;

let bar = 42;
bar--;

let baz = 0;
++baz;

let quz = 42;
--quz;

for (let i = 0; i < 10; i++) {
	doSomething(i);
}

for (let i = 0; i < 10;) {
	doSomething(i);

	i++;
}

for (i = 0; i < l; foo, i++) {
	doSomething(i);
}
