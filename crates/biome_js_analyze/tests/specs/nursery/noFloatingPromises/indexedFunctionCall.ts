const handlers: Array<() => Promise<void>> = [() => Promise.resolve()];

handlers[0]();
