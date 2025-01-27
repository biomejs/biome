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


const returnsPromiseAssignedArrowFunction = async (): Promise<string> => {
  return 'value';
};

returnsPromiseAssignedArrowFunction();

const returnsPromiseAssignedFunction = async function (): Promise<string> {
  return 'value'
}

async function returnsPromiseAssignedFunctionInAsyncFunction(): Promise<void> {
  returnsPromiseAssignedFunction().then(() => { })
}

const returnsPromiseAssignedArrowFunctionAnnotatedType: () => Promise<string> = () => {
  return Promise.resolve('value');
};

returnsPromiseAssignedArrowFunctionAnnotatedType();