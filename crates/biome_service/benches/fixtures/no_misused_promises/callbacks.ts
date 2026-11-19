const values = [1, 2, 3];

values.forEach(async (value) => {
	await Promise.resolve(value);
});

declare function consume(
	kind: "async",
	callback: () => Promise<void>,
): void;
declare function consume(kind: "sync", callback: () => void): void;

consume("sync", async () => {});
consume("async", async () => {});

declare function consumeRest(...callbacks: Array<() => void>): void;
consumeRest(async () => {});

declare const Job: { new (callback: () => void): object };
new Job(async () => {});

type Prefix = [number, string];
declare function consumeTuple(...args: [...Prefix, () => void]): void;
const prefix: Prefix = [1, "value"];
consumeTuple(...prefix, async () => {});

values.filter(async (value) => value > 0);
