async function returnsPromise(): Promise<string> {
  return 'value';
}
returnsPromise();
returnsPromise().then(() => { }).finally(() => { });


function returnsPromiseWithoutAsync(): Promise<string> {
  return Promise.resolve("value")
}


returnsPromiseWithoutAsync()