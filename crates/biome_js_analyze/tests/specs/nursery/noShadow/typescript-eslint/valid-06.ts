/* should not generate diagnostics */
const arg = 0;

interface Test {
  (arg: string): typeof arg;
}
