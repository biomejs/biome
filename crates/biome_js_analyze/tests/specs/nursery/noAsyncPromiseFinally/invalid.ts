/* should generate diagnostics */
type Callback = () => void;
promise.finally((async () => {}) as Callback);

const castCleanup = (async () => {}) as Callback;
promise.finally(castCleanup);

promise.finally((async () => {})!);
