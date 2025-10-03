// should not generate diagnostics
import { component$, useSignal } from "@builder.io/qwik";
import { component$ as MyComponent } from "qwik";

export const Counter = component$(() => {
  const count = useSignal(0);
  return <div>{count.value}</div>;
});

export const useCounter = () => {
  const count = useSignal(0);
  return count;
};

// Aliased component$ should work correctly
export const AliasedCounter = MyComponent(() => {
  const count = useSignal(0);
  return <div>{count.value}</div>;
});
