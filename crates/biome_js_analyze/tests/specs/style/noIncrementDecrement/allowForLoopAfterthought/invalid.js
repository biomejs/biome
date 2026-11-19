let foo = 0;
foo++;

let bar = 0;
++bar;

for (let i = 0; i < 10; j = i++) {
	doSomething(i, j);
}

for (let i = 10; i--;) {
	doSomething(i);
}

for (let i = 0; i < 10;) i++;

for (i = 0; i < l; i++) { v++; }

for (i++;;);

for (;--i;);

for (;;) ++i;

for (;; i = j++);

for (;; i++, f(--j));

for (;; foo + (i++, bar));