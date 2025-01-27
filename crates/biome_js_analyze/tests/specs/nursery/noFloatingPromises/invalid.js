async function returnsPromise() {
  return 'value';
}
returnsPromise();
returnsPromise().then(() => { }).finally(() => { });
