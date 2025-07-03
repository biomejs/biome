/* should not generate diagnostics */

const promise = Promise.resolve('value');

// Always `await` the Promise in a conditional
if (await promise) {
  // Do something
}

const val = (await promise) ? 123 : 456;

const returnVal = await promise;
[1, 2, 3].filter(() => returnVal);

while (await promise) {
  // Do something
}

const maybePromise = 1 == 2 ? promise : undefined;
