/* should generate diagnostics */

type Hook = () => [number, (value: number) => void];

// Hook defined with a type assertion around the function
function createAssertedHook(key: number) {
  const useAsserted = (() => {
    return useState(key);
  }) as Hook;
  return useAsserted;
}

// Hook defined with a satisfies operator around the function
function createSatisfyingHook(key: number) {
  const useSatisfying = (() => {
    return useState(key);
  }) satisfies Hook;
  return useSatisfying;
}
