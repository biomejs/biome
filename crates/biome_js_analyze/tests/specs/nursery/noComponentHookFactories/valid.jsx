/* should not generate diagnostics */

// Component defined at module level
function Component({ defaultValue }) {
  return <div>{defaultValue}</div>;
}

// Custom hook at module level
function useData(endpoint) {
  const [data, setData] = useState(null);
  useEffect(() => {
    fetch(endpoint).then(r => r.json()).then(setData);
  }, []);
  return data;
}

// Module-level arrow component
const MyComponent = () => {
  return <div />;
};

// Module-level arrow hook
const useMyHook = () => {
  const [state, setState] = useState(null);
  return [state, setState];
};

// Normal event handlers inside components are fine (lowercase name)
function ParentComponent(props) {
  function onClick(event) {
    props.onClick(event.target.value);
  }
  const onKeyPress = () => null;
  return (
    <div>
      <button onClick={onClick} onKeyPress={onKeyPress} />
    </div>
  );
}

// Helper functions inside components are fine (lowercase name)
function ComponentWithHelper({ items }) {
  function getTotal() {
    return items.reduce((sum, item) => sum + item.price, 0);
  }
  return <div>{getTotal()}</div>;
}

// Regular function returning non-component function (lowercase name)
function createHandler(defaultValue) {
  return function handler() {
    return defaultValue;
  };
}

// Regular nested functions are fine
function outer() {
  function inner() {
    return 42;
  }
  return inner();
}

// HOC pattern — PascalCase parameter signals it accepts a component
function withAuth(WrappedComponent) {
  function AuthenticatedComponent(props) {
    return <WrappedComponent {...props} />;
  }
  return AuthenticatedComponent;
}

// HOC arrow pattern — PascalCase parameter
const withLoading = (BaseComponent) => {
  return function WithLoadingComponent({ isLoading, ...props }) {
    if (isLoading) return <div>Loading...</div>;
    return <BaseComponent {...props} />;
  };
};

// HOC with Component param name
function withThemeHOC(Component) {
  const ThemedComponent = (props) => {
    return <Component theme="dark" {...props} />;
  };
  return ThemedComponent;
}

// Component mock inside vi.mock
vi.mock(import('../path/to/go'), () => ({
  MyComponent: ({propX}) => <div data-x={propX} />,
}));

// Component mock inside vi.mock with importOriginal
vi.mock(import('../path/to/go'), async (importOriginal) => {
  const original = await importOriginal();
  return {
    ...original,
    MyComponent: ({propX}) => <div data-x={propX} />,
  };
});

// Component mock inside jest.mock
jest.mock('../path/to/go', () => ({
  MyComponent: ({propX}) => <div data-x={propX} />,
}));

// Export default component at module level
export default function App() {
  return <div />;
}
