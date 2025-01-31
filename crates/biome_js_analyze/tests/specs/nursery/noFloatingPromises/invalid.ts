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


const promise = new Promise((resolve) => resolve('value'));
promise.then(() => { }).finally(() => { });

Promise.resolve('value').then(() => { })
Promise.all([p1, p2, p3])


const promiseWithParentheses = (new Promise((resolve, reject) => resolve('value')));
promiseWithParentheses;
(returnsPromise());


const promiseWithGlobalIdentifier = new window.Promise((resolve, reject) => resolve('value'));
promiseWithGlobalIdentifier.then(() => { }).finally(() => { });
globalThis.Promise.reject('value').finally();
