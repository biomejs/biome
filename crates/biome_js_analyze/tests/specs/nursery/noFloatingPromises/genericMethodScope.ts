// should generate diagnostics

declare const factory: { create<T>(): T };
factory.create<Promise<void>>();

interface Test<Args> {
	(body: (args: Args) => void): void;
	extend<T>(): Test<Args & T>;
}

declare const base: Test<{}>;
const test = base.extend<{
	fixture: () => Promise<void>;
	service: { run(): Promise<void> };
}>();
test(({ fixture, service }) => {
	fixture();
	service.run();
});
