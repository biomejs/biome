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


const promise = new Promise((resolve) => resolve('value'));
promise.then(() => { }, () => { })

Promise.resolve('value').then(() => { }, () => { })
Promise.all([p1, p2, p3]).catch(() => { })

const Promise = { resolve(): {} };
Promise.resolve()
async function bindingPromiseInsideFunction() {
  Promise.resolve()
}


class ValidTestClass {
  returnsPromiseFunctionProperty: () => Promise<void>
  returnsPromiseProperty: Promise<void>
  constructor() {
    this.returnsPromiseFunctionProperty = () => Promise.resolve();
    this.returnsPromiseProperty = new Promise((resolve, reject) => { })
  }

  async returnsPromiseMethod(): Promise<string> {
    return 'value';
  }
  async someMethod() {
    this.returnsPromiseMethod().catch(() => { });
  }

  returnsString(): string {
    return 'value';
  }
  async someMethod2() {
    this.returnsString();
  }

  async someMethod3() {
    this.returnsPromiseProperty.then(() => { }, () => { });
  }

  returnsPromiseFunction = async function (): Promise<string> {
    return 'value';
  }
  returnsPromiseArrowFunction = async (): Promise<string> => {
    return 'value';
  }

  async someMetho3() {
    await this.returnsPromiseFunction().then(() => { });
    this.returnsPromiseArrowFunction().catch(() => { });
  }
}

const validTestClass = new ValidTestClass();
async function testClassExpression(): Promise<string> {
  await validTestClass.returnsPromiseMethod();
  validTestClass.returnsPromiseMethod().catch(() => { });
  validTestClass.returnsPromiseFunctionProperty().then(() => { }, () => { }).finally(() => { });
  validTestClass.returnsPromiseProperty.catch(() => { });
  return validTestClass.returnsPromiseArrowFunction();
}