/* should not generate diagnostics */
const bar = case1!.bar;

function case2(bar: number | undefined) {
  const bar1: number = bar!;
}

function case3(bar?: { n: number }) {
  return bar?.n;
}

checksCounter?.case4!.trim();

function case5(key: string | null) {
  const obj = {};
  return obj?.[key!];
}

function issue3419(value: string | null): string {
  return (value!);
}

// Test case for issue #7927: compound assignment with non-null assertions on both sides
const arr: number[] = [1, 2, 3];
arr[0]! ^= arr[1]!;
