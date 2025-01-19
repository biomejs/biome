async function returnsPromise(): Promise<string> {
  return 'value';
}
returnsPromise();
returnsPromise().then(() => { }).finally(() => { });
