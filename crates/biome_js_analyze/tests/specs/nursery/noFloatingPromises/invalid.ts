async function returnsPromise(): Promise<string> {
  return 'value';
}
returnsPromise();
returnsPromise().then(() => { }).finally(() => { });

async function returnsPromiseInAsyncFunction(): Promise<void> {
  returnsPromise();
}

const returnsPromiseInAsyncArrowFunction = async (): Promise<void> => {
  returnsPromise().then(() => { }).finally(() => { });
}

class Test {
  async returnsPromiseInAsyncClassMethod(): Promise<void> {
    returnsPromise();
  }
}


function returnsPromiseWithoutAsync(): Promise<string> {
  return Promise.resolve("value")
}


returnsPromiseWithoutAsync()