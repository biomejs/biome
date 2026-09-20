/* should not generate diagnostics */
Promise.reject(error as Error);
Promise.reject(error satisfies Error);
Promise.reject(<Error>error);
Promise.reject(error!);
new Promise<void>((resolve, reject: (reason: unknown) => void) => reject(new Error()));
new Promise<number>(function(this: void, resolve, reject) {
    resolve(5);
    reject(new Error());
});
