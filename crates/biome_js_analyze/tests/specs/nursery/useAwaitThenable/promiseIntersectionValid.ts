/* should not generate diagnostics */

function intersectionPromise(): Promise<number> & { cancel: () => void } {
    const promise = Promise.resolve(1) as Promise<number> & {
        cancel: () => void;
    };
    promise.cancel = () => {};
    return promise;
}

export async function probe(): Promise<number> {
    return await intersectionPromise();
}
