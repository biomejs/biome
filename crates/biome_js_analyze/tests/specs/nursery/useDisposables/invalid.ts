/* should generate diagnostics */
const disposable = {
  [Symbol.dispose]() {
    // do something
  }
};

const asyncDisposable = {
  async [Symbol.asyncDispose]() {
    // do something
  }
};

function createDisposable(): Disposable {
  return {
    [Symbol.dispose]() {
      // do something
    },
  };
}

const createdDisposable = createDisposable();

function createAsyncDisposable(): AsyncDisposable {
  return {
    async [Symbol.asyncDispose](): Promise<void> {
      // do something
    },
  };
}

const createdAsyncDisposable = createAsyncDisposable();

class DisposableClass implements Disposable {
	[Symbol.dispose](): void {
		// do something
	}
}

const disposableInstance = new DisposableClass();

class AsyncDisposableClass implements AsyncDisposable {
	async [Symbol.asyncDispose](): Promise<void> {
		// do something
	}
}

const asyncDisposableInstance = new AsyncDisposableClass();
