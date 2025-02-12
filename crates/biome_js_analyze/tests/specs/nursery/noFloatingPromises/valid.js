async function returnsPromise() {
  return 'value';
}

await returnsPromise();
void returnsPromise();
return returnsPromise();

returnsPromise().then(
  () => { },
  () => { },
);

returnsPromise().catch(() => { });