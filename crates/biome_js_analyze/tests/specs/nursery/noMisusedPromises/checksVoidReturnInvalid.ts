[1, 2, 3].forEach(async value => {
  await fetch(`/${value}`);
});

new Promise<void>(async (resolve, reject) => {
  await fetch('/');
  resolve();
});

declare function consume(kind: "async", callback: () => Promise<void>): void;
declare function consume(kind: "sync", callback: () => void): void;
consume("sync", async () => {});

declare function consumeRest(...callbacks: Array<() => void>): void;
consumeRest(async () => {});

class ConstructorConsumer {
  constructor(callback: () => void);
  constructor(callback: () => Promise<void>, marker: number);
  constructor(_callback: (() => void) | (() => Promise<void>), _marker?: number) {}
}
new ConstructorConsumer(async () => {});

declare function consumeAfterSpread(prefix: number, callback: () => void): void;
const prefixes: number[] = [];
consumeAfterSpread(...prefixes, async () => {});
