/* should not generate diagnostics */
async function returnsPromise(): Promise<string> {
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