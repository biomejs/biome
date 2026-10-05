declare function consume<T>(callback: (value: T) => Promise<void>): void;

consume<string>(async (value) => {
	await value;
});
