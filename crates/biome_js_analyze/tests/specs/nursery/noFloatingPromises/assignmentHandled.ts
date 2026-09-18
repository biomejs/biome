/* should not generate diagnostics */
async function returnsPromise(): Promise<string> {
  return 'value';
}

let foo: Promise<string> | undefined;
foo = returnsPromise();

let bar;
bar = [1, 2, 3].map(async (x) => x + 1);
