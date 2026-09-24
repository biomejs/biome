/* should generate diagnostics */
Promise.reject("failure" as string);
Promise.reject("failure" satisfies string);
Promise.reject(<string>"failure");
Promise.reject(undefined!);
new Promise<void>((resolve, reject: (reason: unknown) => void) => reject(5));
new Promise<number>(function(this: void, resolve, reject) {
    reject(5);
});
