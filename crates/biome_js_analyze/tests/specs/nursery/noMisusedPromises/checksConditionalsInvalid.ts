const promise = Promise.resolve('value');

if (promise) {
  // Do something
}

const val = promise ? 123 : 456;

[1, 2, 3].filter(() => promise);

while (promise) {
  // Do something
}
