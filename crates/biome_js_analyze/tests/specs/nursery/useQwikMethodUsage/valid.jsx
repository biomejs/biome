// should not generate diagnostics 
export const Counter = component$(() => {
  const count = useSignal(0);
});

export const useCounter = () => {
  const count = useSignal(0);
  return count;
};
