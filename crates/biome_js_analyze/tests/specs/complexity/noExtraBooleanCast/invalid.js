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

// Test case for issue #7225 - should preserve parentheses
const b0 = false;
const b1 = false;
const boolean = !Boolean(b0 && b1);

// Test case for issue #11289 - should preserve parentheses
const result = Boolean(foo ? bar : baz) ? "X" : "Y";
