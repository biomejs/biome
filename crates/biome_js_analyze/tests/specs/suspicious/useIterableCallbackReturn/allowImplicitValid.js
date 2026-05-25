/* should not generate diagnostics */

// issue #9445: .filter(Boolean) pattern
;[1, 2, 3].map((it) => {
	if (it % 2 === 0) {
		return it;
	}
	if (it > 2) {
		return it ** 2;
	}
	return;
}).filter(Boolean);

// empty return is accepted with allowImplicit
[1, 2, 3].map((x) => {
	if (x > 2) return x;
	return;
});

// all paths return values
[1, 2, 3].map((x) => {
	if (x > 2) return x;
	return 0;
});

// implicit return (arrow expression)
[1, 2, 3].map((x) => x * 2);

// only empty returns (all paths covered)
[1, 2, 3].forEach((x) => {
	if (x > 2) return;
	console.log(x);
});

// .filter(Boolean) pattern with objects
const items = [{ name: "a", value: 1 }, { name: "", value: 0 }];
const names = items.map((item) => {
	if (!item.name) return;
	return item.name;
}).filter(Boolean);

[].filter(() => {
	return;
});

[].find(() => {
	if (true) return;
	return 1;
});

Array.from([], () => {
	return;
});
