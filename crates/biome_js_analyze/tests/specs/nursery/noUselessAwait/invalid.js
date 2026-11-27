/* should generate diagnostics */
async function values() {
	await [];
	await [Promise.resolve()];
	await (() => {});
	await (() => Promise.resolve());
	await (a === b);
	await (a instanceof Promise);
	await ("a" in b);
	await (a > b);
	await class {};
	await class extends Promise {};
	await function () {};
	await function name() {};
	await function () { return Promise.resolve() };
	await 0;
	await "";
	await true;
	await null;
	await 1n;
	await /regex/;
	await `${Promise.resolve()}`;
	await !Promise.resolve();
	await void Promise.resolve();
	await typeof foo;
	await +Promise.resolve();
	await ~1;
	await ++foo;
	await foo--;
	await (Promise.resolve(), 1);
	await await foo;
	+await +1;
	-await --bar;
	const a = +await ++b;
}
