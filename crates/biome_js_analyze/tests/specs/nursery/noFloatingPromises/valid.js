/* should not generate diagnostics */
async function returnsPromise() {
  return 'value';
}

await returnsPromise();
void returnsPromise();

function otherFunction() {
  return returnsPromise();
}

returnsPromise().then(
  () => { },
  () => { },
);

returnsPromise().catch(() => { });
