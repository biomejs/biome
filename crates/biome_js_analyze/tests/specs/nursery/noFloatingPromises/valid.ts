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

class ValidTestClassParent {
  async returnsPromiseFromParent(): Promise<string> {
    return 'value';
  }
}
class ValidTestClass extends ValidTestClassParent {
  returnsPromiseFunctionProperty: () => Promise<void>
  returnsPromiseProperty: Promise<void>
  constructor() {
    super()
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

  async someMethod4() {
    this.returnsPromiseFromParent().then(() => { }).catch(() => {}).finally(() => { });
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
async function testClassMethodCalls(): Promise<string> {
  await validTestClass.returnsPromiseMethod();
  validTestClass.returnsPromiseMethod().catch(() => { });
  validTestClass.returnsPromiseFunctionProperty().then(() => { }, () => { }).finally(() => { });
  validTestClass.returnsPromiseProperty.catch(() => { });
  return validTestClass.returnsPromiseArrowFunction();
}

const validTestClassInitializedExpression = class ValidTestClass extends ValidTestClassParent {
  returnsPromiseFunctionProperty: () => Promise<void>
  returnsPromiseProperty: Promise<void>
  constructor() {
    super();
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

  async someMethod4() {
    this.returnsPromiseFromParent().then(() => { }).catch(() => {}).finally(() => { });
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

const validTestClassExpression = new validTestClassInitializedExpression();
async function testClassExpressionMethodCalls(): Promise<string> {
  await validTestClassExpression.returnsPromiseMethod();
  validTestClassExpression.returnsPromiseMethod().catch(() => { });
  validTestClassExpression.returnsPromiseFunctionProperty().then(() => { }, () => { }).finally(() => { });
  validTestClassExpression.returnsPromiseProperty.catch(() => { });
  return validTestClassExpression.returnsPromiseArrowFunction();
}

const UnnamedValidClassExpression = class extends ValidTestClassParent {
  returnsPromiseFunctionProperty: () => Promise<void>
  returnsPromiseProperty: Promise<void>
  constructor() {
    super();
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

  async someMethod4() {
    this.returnsPromiseFromParent().then(() => { }).catch(() => {}).finally(() => { });
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

const unnamedValidClassExpression = new UnnamedValidClassExpression();
async function testUnnamedClassExpressionMethodCalls(): Promise<string> {
  await unnamedValidClassExpression.returnsPromiseMethod();
  unnamedValidClassExpression.returnsPromiseMethod().catch(() => { });
  unnamedValidClassExpression.returnsPromiseFunctionProperty().then(() => { }, () => { }).finally(() => { });
  unnamedValidClassExpression.returnsPromiseProperty.catch(() => { });
  return unnamedValidClassExpression.returnsPromiseArrowFunction();
}