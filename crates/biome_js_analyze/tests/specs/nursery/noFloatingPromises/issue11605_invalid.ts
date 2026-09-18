interface Service {
	go(): Promise<void>;
}
interface Context {
	service: Service;
}
declare const context: Context;

declare function test<T>(callback: (args: T) => Promise<void>): void;

// Explicit type arguments on the call instantiate `T`.
test<Context>(async (args) => {
	const service = args.service;
	service.go();
});

test<Context>(async (args) => {
	args.service.go();
});

// Single unparenthesised parameter.
test<Context>(async args => {
	args.service.go();
});

// Destructured parameter.
test<Context>(async ({ service }) => {
	service.go();
});

// Non-generic callback parameter.
declare function run(callback: (args: Context) => Promise<void>): void;
run(async (args) => {
	args.service.go();
});

// Function expression callback, and a parenthesised callback.
run(async function (args) {
	args.service.go();
});
run((async (args) => {
	args.service.go();
}));

// Second argument, with a preceding argument selecting the overload.
declare function schedule(kind: "sync", callback: (args: number) => void): void;
declare function schedule(kind: "async", callback: (args: Context) => Promise<void>): void;
schedule("async", async (args) => {
	args.service.go();
});

// Callbacks with several parameters, typed by position.
declare function each(callback: (item: Context, index: number) => Promise<void>): void;
each(async (item, index) => {
	item.service.go();
});
declare function reversed(callback: (index: number, item: Context) => Promise<void>): void;
reversed(async (index, item) => {
	item.service.go();
});

// A `this` parameter does not occupy an argument position.
declare function bound(callback: (this: Window, item: Context) => Promise<void>): void;
bound(async function (item) {
	item.service.go();
});

// Callback passed to a constructor.
declare class Job {
	constructor(callback: (args: Context) => Promise<void>);
}
new Job(async (args) => {
	args.service.go();
});

// Explicit type arguments on a plain call.
declare function make<T>(): T;
make<Service>().go();
