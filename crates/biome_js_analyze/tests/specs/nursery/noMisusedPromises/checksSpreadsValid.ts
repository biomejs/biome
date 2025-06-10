/* should not generate diagnostics */

const getData = () => fetch('/');

console.log({ foo: 42, ...(await getData()) });

const awaitData = async () => {
  await fetch('/');
};

console.log({ foo: 42, ...(await awaitData()) });
