type Args<F> = F extends (...args: infer A) => void ? A : never;
