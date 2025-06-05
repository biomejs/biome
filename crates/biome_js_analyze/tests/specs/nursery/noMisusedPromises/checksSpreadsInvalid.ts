const getData = () => fetch('/');

// FIXME: Not yet detected
console.log({ foo: 42, ...getData() });

const awaitData = async () => {
  await fetch('/');
};

console.log({ foo: 42, ...awaitData() });
