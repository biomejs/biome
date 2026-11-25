/* should generate diagnostics */
Promise.resolve().catch((err: Error) => {
	throw err;
});

Promise.resolve()["catch"]((x: any) => "return");

Promise.resolve().catch((e, ...rest: []) => {
	throw e;
});

Promise.resolve().catch(
	(err: string | ((number | unknown) & { b: () => void })) => {
		throw err;
	},
);

Promise.resolve().catch(
	(
		...err: [
			unknown[],
			string | ((number | unknown) & { b: () => void }),
			string,
		]
	) => {
		throw err;
	},
);

Promise.resolve().catch(function (err: string) {
	throw err;
});

Promise.resolve().catch(function (err /* awkward spot for comment */) {
	throw err;
});

Promise.resolve().catch(function namedCallback(err: string) {
	throw err;
});

Promise.resolve().catch(err => {
	throw err;
});

Promise.resolve().then(
	() => {},
	error => {},
);

Promise.resolve().catch((err?) => {
	throw err;
});

Promise.resolve().catch((err?: string) => {
	throw err;
});

Promise.resolve().catch(err/* with comment */=> {
	throw err;
});

Promise.resolve().catch((err = 2) => {
	throw err;
});

Promise.resolve().catch((err: any /* comment 1 */ = /* comment 2 */ 2) => {
	throw err;
});

Promise.resolve().catch((...args) => {
	throw args[0];
});

Promise.reject(new Error("I will reject!")).catch(([err]: [unknown]) => {
	console.log(err);
});

Promise.resolve(" a string ").catch(
	(a: any, b: () => any, c: (x: string & number) => void) => {},
);

Promise.resolve("object destructuring").catch(({}) => {});

Promise.resolve("object destructuring").catch(function ({ gotcha }) {
	return null;
});

Promise.reject().catch((...x: any) => {});

Promise.resolve().catch((...[args]: [string]) => {
	console.log(args);
});

Promise.resolve().catch((...{ find }: [string]) => {
	console.log(find);
});

Promise.resolve().catch(function (this: Window, err) {
	throw err;
});

Promise.resolve().catch((...args: string[]) => {});

Promise.resolve()?.catch((err) => {});

async function fromAsyncFunction() {}
fromAsyncFunction().catch((err) => {});

function fromParameter(promise: Promise<string>) {
	promise.then(
		(value) => value,
		(err: Error) => {},
	);
}

declare const declaredPromise: Promise<number>;
declaredPromise.catch((err) => {});

declare const thenable: {
	then(onFulfilled: (value: string) => void, onRejected: (reason: any) => void): void;
};
thenable.then(
	(value) => {},
	(reason) => {},
);

declare const assertedPromise: Promise<number> | undefined;
assertedPromise!.catch((err) => {});

const holder = { promise: Promise.resolve(1) };
holder.promise!.catch((err) => {});
