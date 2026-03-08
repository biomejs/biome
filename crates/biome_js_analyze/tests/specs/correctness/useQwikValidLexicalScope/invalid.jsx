// should emit diagnostics
import { component$, useSignal, useTask$, useStore, useComputed$, useResource$, $, useWatch$ } from "@builder.io/qwik";

// 1. minimal example and concise-body arrow function without braces
const f = () => "bar";

// 2. $ usage in module scope
const moduleHandler = $(() => {
  console.log("invalid module scope");
});

// 3. $ usage in regular function
function regularFunction() {
  const handler = $(() => {
    console.log("invalid regular function");
  });
}

// 4. $ usage in arrow function
const arrowFunction = () => {
  const handler = $(() => {
    console.log("invalid arrow function");
  });
};

// 5. $ usage in class method
class MyClass {
  method() {
    const handler = $(() => {
      console.log("invalid class method");
    });
  }
}

// 6. $ usage in object method
const myObject = {
  method: function() {
    const handler = $(() => {
      console.log("invalid object method");
    });
  }
};

// 7. $ usage in nested regular function inside component$
export const Component = component$(() => {
  function nestedRegular() {
    const handler = $(() => {
      console.log("invalid nested function");
    });
  }
  return <div />;
});

// 8. $ usage in event handler not using $
export const ComponentWithEvent = component$(() => {
  const handleClick = () => {
    const handler = $(() => {
      console.log("invalid event handler");
    });
  };
  return <button onClick={handleClick}>Click</button>;
});

// 9. $ usage in conditional block without proper context
export const ConditionalComponent = component$(() => {
  if (true) {
    const regularFunction = () => {
      const handler = $(() => {
        console.log("invalid conditional");
      });
    };
  }
  return <div />;
});

// 10. $ usage in loop without proper context
export const LoopComponent = component$(() => {
  for (let i = 0; i < 5; i++) {
    const regularFunction = () => {
      const handler = $(() => {
        console.log("invalid loop");
      });
    };
  }
  return <div />;
});

// 11. $ usage in try-catch without proper context
export const TryCatchComponent = component$(() => {
  try {
    const regularFunction = () => {
      const handler = $(() => {
        console.log("invalid try-catch");
      });
    };
  } catch (e) {
    console.error(e);
  }
  return <div />;
});

// 12. $ usage in async function
async function asyncFunction() {
  const handler = $(() => {
    console.log("invalid async function");
  });
}

// 13. $ usage in generator function
function* generatorFunction() {
  const handler = $(() => {
    console.log("invalid generator function");
  });
  yield 1;
}

// 14. $ usage in exported regular function
export function exportedFunction() {
  const handler = $(() => {
    console.log("invalid exported function");
  });
}

// 15. $ usage in default export function
export default function() {
  const handler = $(() => {
    console.log("invalid default export");
  });
}

// 16. Multiple $ usages in same invalid context
const multipleInvalid = () => {
  const handler1 = $(() => console.log("first"));
  const handler2 = $(() => console.log("second"));
  const handler3 = $(() => console.log("third"));
};

// 17. $ usage in IIFE
(function() {
  const handler = $(() => {
    console.log("invalid IIFE");
  });
})();

// 18. $ usage in arrow function property
const objWithArrow = {
  method: () => {
    const handler = $(() => {
      console.log("invalid arrow property");
    });
  }
};

// 19. $ usage in callback function
[1, 2, 3].forEach(function(item) {
  const handler = $(() => {
    console.log("invalid callback");
  });
});

// 20. $ usage in promise then
Promise.resolve().then(() => {
  const handler = $(() => {
    console.log("invalid promise then");
  });
});