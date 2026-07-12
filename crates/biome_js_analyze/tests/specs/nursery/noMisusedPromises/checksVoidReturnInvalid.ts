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
