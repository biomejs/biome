createEffect(() => {
  console.log(signal());
});

createEffect((prev) => {
  console.log(signal());
  return prev + 1;
}, 0);

createEffect((prev) => {
  console.log(signal());
  return (prev || 0) + 1;
});

createEffect((prev) => {
  console.log(signal());
  return prev ? prev + 1 : 1;
}, undefined);

const value = createMemo(() => computeExpensiveValue(a(), b()));

const sum = createMemo((prev) => input() + prev, 0);

const args = [() => { console.log(signal()); }, [signal()]];
createEffect(...args);
