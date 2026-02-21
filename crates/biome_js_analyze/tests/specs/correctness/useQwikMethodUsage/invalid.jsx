import { useSignal, useTask$, useStore, useComputed$, useResource$, useWatch$ } from "@builder.io/qwik";
import { useSignal as useQwikSignal } from "qwik";

// 1. useSignal in regular arrow function
export const Counter = () => {
  const count = useSignal(0);
};

// 2. useTask$ in regular function
export function MyComponent() {
  useTask$(() => {
    console.log("invalid task");
  });
}

// 3. useStore in module scope
const globalStore = useStore({ count: 0 });

// 4. useComputed$ in class method
class MyClass {
  method() {
    const computed = useComputed$(() => {
      return 42;
    });
  }
}

// 5. useResource$ in object method
const myObject = {
  method: function() {
    const resource = useResource$(async () => {
      return await fetch("/api/data");
    });
  }
};

// 6. useWatch$ in nested function
export const ComponentWithNested = () => {
  function nested() {
    useWatch$(({ track }) => {
      track(() => console.log("watching"));
    });
  }
  return null;
};

// 7. Multiple hooks in invalid context
const InvalidMultipleHooks = () => {
  const signal = useSignal(0);
  const store = useStore({ value: 0 });
  useTask$(() => {
    console.log(signal.value, store.value);
  });
};

// 8. Hook in event handler
export const ButtonComponent = () => {
  const handleClick = () => {
    const signal = useSignal(0);
  };
  return <button onClick={handleClick}>Click</button>;
};

// 9. Hook in async function
async function fetchData() {
  const signal = useSignal(null);
  const data = await fetch("/api");
  signal.value = data;
}

// 10. Hook in generator function
function* generator() {
  const signal = useSignal(0);
  yield signal.value;
}

// 11. Hook in IIFE
(function() {
  const signal = useSignal(0);
  console.log(signal.value);
})();

// 12. Hook in conditional
export const ConditionalComponent = () => {
  if (true) {
    const signal = useSignal(0);
  }
  return null;
};

// 13. Hook in loop
export const LoopComponent = () => {
  for (let i = 0; i < 5; i++) {
    const signal = useSignal(i);
  }
  return null;
};

// 14. Hook in try-catch
export const TryCatchComponent = () => {
  try {
    const signal = useSignal(0);
  } catch (e) {
    console.error(e);
  }
  return null;
};

// 15. Hook in callback
[1, 2, 3].map(() => {
  const signal = useSignal(0);
  return signal.value;
});

// 16. Hook in promise chain
Promise.resolve().then(() => {
  const signal = useSignal(0);
  return signal.value;
});

// 17. Custom hook in invalid context (should not start with use)
const myCustomHook = () => {
  const signal = useSignal(0);
  return signal;
};

// 18. Hook in exported default function
export default function() {
  const signal = useSignal(0);
  return signal;
}

// 19. Hook after early return
export const EarlyReturnComponent = () => {
  if (true) {
    return null;
  }
  const signal = useSignal(0);
  return signal;
};

// 20. Hook in switch case
export const SwitchComponent = () => {
  switch (true) {
    case true: {
      const signal = useSignal(0);
      break;
    }
  }
  return null;
};

// 21. Edge case: Function merely named component$ (not called by component$)
function component$() {
  const signal = useSignal(0);
  return signal;
}

// 22. Edge case: Arrow function merely named component$
{
  const component$ = () => {
    const signal = useSignal(0);
    return signal;
  };
}

// 23. Edge case: Import with alias from "qwik"
const AliasedComponent = () => {
  const signal = useQwikSignal(0);
  return signal;
};

// 24. Edge case: Hook in function expression named component$
const myFunc = function component$() {
  const signal = useSignal(0);
  return signal;
};

// 25. Edge case: import with single quotes
// This test case requires a separate file or inline simulation since we can't have multiple imports of the same module

// 26. Edge case: Import as namespace
// import * as qwik from "qwik"; qwik.useSignal()
// This test case would require separate file to avoid conflicts