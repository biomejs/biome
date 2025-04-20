createEffect(() => {
  console.log(signal());
}, [signal()]);

createEffect(() => {
  console.log(signal());
}, [signal]);

const deps = [signal];
createEffect(() => {
  console.log(signal());
}, deps);

const value = createMemo(() => computeExpensiveValue(a(), b()), [a(), b()]);

const value = createMemo(() => computeExpensiveValue(a(), b()), [a, b]);

const value = createMemo(() => computeExpensiveValue(a(), b()), [a, b()]);

const deps = [a, b];
const value = createMemo(() => computeExpensiveValue(a(), b()), deps);

const deps = [a, b];
const memoFn = () => computeExpensiveValue(a(), b());
const value = createMemo(memoFn, deps);