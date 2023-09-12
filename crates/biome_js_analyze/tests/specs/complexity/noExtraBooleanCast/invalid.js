if (Boolean(foo)) {
}

if (!!Boolean(foo)) {
}

if (!Boolean(foo)) {
}

while (!!foo) {}

let x = 1;
do {
	1 + 1;
} while (Boolean(x));

for (; !!foo; ) {}

new Boolean(!!x);

!!!x;

!Boolean(x);