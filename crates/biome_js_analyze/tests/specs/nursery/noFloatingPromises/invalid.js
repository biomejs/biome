async function returnsPromise() {
  return 'value';
}
returnsPromise();
returnsPromise().then(() => { }).finally(() => { });

async function issue7999() {
  // ✅
  returnsPromise();
}
