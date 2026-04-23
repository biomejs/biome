const promise = Promise.resolve('value');

if (promise) {
  // Do something
}

const val = promise ? 123 : 456;

[1, 2, 3].filter(() => promise);

while (promise) {
  // Do something
}

const a = (): boolean | Promise<boolean> => {
  return Promise.resolve(true);
}

if (a()) {
  // Do something
}

while (a()) {
  // Do something
}

declare const pickPromiseFn: Pick<{fn: () => Promise<boolean>, other: number}, "fn">;
if (pickPromiseFn.fn()) {
  // Do something
}
while (pickPromiseFn.fn()) {
  // Do something
}
const val2 = pickPromiseFn.fn() ? 1 : 2;
[1, 2, 3].filter(() => pickPromiseFn.fn());

declare const omitPromiseFn: Omit<{fn: () => Promise<boolean>, other: number}, "other">;
if (omitPromiseFn.fn()) {
  // Do something
}
const val3 = omitPromiseFn.fn() ? 1 : 2;

declare const reqCache: Required<{p?: Promise<string>}>;
if (reqCache.p) console.log("cached");
const v = reqCache.p ? "yes" : "no";

declare const roCache: Readonly<{p: Promise<string>}>;
if (roCache.p) console.log("cached");
