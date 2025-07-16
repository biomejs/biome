useVisibleTask$(() => {
  console.log('Component is visible');
});

useVisibleTask$(({ cleanup }) => {
  const subscription = someObservable.subscribe();
  cleanup(() => subscription.unsubscribe());
});

useVisibleTask$(() => {
  document.title = 'New Title';
}, { eagerness: 'visible' });

const MyComponent = component$(() => {
  useVisibleTask$(() => {
    console.log('Component mounted');
  });
  return <div>Hello</div>;
});

export const App = component$(() => {
  useVisibleTask$(({ track }) => {
    track(() => state.value);
    console.log('State changed');
  });
  return <div>App</div>;
});

function setupComponent() {
  useVisibleTask$(() => {
    // Setup logic
  });
}

const setup = () => {
  useVisibleTask$(() => {
    // Setup in arrow function
  });
};

class MyClass {
  constructor() {
    useVisibleTask$(() => {
      // Setup in constructor
    });
  }
}

useVisibleTask$(() => {
  console.log('First task');
});

useVisibleTask$(() => {
  console.log('Second task');
});

useVisibleTask$(({ track, cleanup }) => {
  const interval = setInterval(() => {
    track(() => count.value);
    console.log('Count:', count.value);
  }, 1000);
  cleanup(() => clearInterval(interval));
}); 