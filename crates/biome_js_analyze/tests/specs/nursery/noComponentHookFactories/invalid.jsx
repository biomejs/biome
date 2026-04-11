/* should generate diagnostics */

// Factory function with return function Component() pattern
function createComponent(defaultValue) {
  return function Component() {
    return <div>{defaultValue}</div>;
  };
}

// Factory with const arrow component
function createButton(color) {
  const Button = () => {
    return <button style={{ color }}></button>;
  };
  return Button;
}

// Arrow factory with return function Button() pattern
const createColoredButton = (color) => {
  return function Button({ children }) {
    return <button style={{ backgroundColor: color }}>{children}</button>;
  };
};

// Arrow factory with const arrow component
const createArrowButton = (color) => {
  const Button = ({ children }) => {
    return <button style={{ backgroundColor: color }}>{children}</button>;
  };
  return Button;
};

// Component defined inside component (function declaration)
function Parent() {
  function Child() {
    return <div />;
  }
  return <Child />;
}

// Component defined inside component (arrow)
function ParentArrow() {
  const Child = () => {
    return <div />;
  };
  return <Child />;
}

// Hook factory with return function useHook() pattern
function createCustomHook(endpoint) {
  return function useData() {
    const [data, setData] = useState(null);
    useEffect(() => {
      fetch(endpoint).then(r => r.json()).then(setData);
    }, []);
    return data;
  };
}

// Arrow hook factory with const arrow hook
const createArrowHook = (endpoint) => {
  const useData = () => {
    const [data, setData] = useState(null);
    return data;
  };
  return useData;
};

// Hook defined inside a component
function MyComponent() {
  function useLocalState() {
    return useState(0);
  }
  const [count, setCount] = useLocalState();
  return <div>{count}</div>;
}

// Hook defined inside another hook
function useOuter() {
  function useInner() {
    return useState(0);
  }
  return useInner();
}

// Factory creating multiple components
function createComponents(theme) {
  function Header() {
    return <header style={{ color: theme.primary }} />;
  }
  function Footer() {
    return <footer style={{ color: theme.secondary }} />;
  }
  return { Header, Footer };
}

// Factory returning both a component and a hook
function createFeature(config) {
  function useFeatureData() {
    return useState(config.defaultValue);
  }
  function FeatureComponent() {
    const [data] = useFeatureData();
    return <div>{data}</div>;
  }
  return { useFeatureData, FeatureComponent };
}

// Deeply nested component
function outer() {
  function inner() {
    function DeeplyNested() {
      return <div />;
    }
    return DeeplyNested;
  }
  return inner;
}

// makeButton factory pattern (return function X() {})
function makeButton(color) {
  return function Button({ children }) {
    return (
      <button style={{ backgroundColor: color }}>
        {children}
      </button>
    );
  };
}

// Hook factory with closure over config (return function useHook() {})
function createUseAuth(config) {
  return function useAuth() {
    const [user, setUser] = useState(null);
    useEffect(() => {
      config.authProvider.onAuthStateChanged(setUser);
    }, []);
    return user;
  };
}

// IIFE-like factory pattern
const MyIIFEComponent = (function() {
  return function Inner() {
    return <div />;
  };
})();

// Generic factory (lowercase param, NOT an HOC)
function withTheme(theme) {
  function ThemedComponent({ children }) {
    return <div style={{ color: theme }}>{children}</div>;
  }
  return ThemedComponent;
}

// Hook defined inside factory function (function declaration)
function makeHook(key) {
  function useMyHook() {
    return useState(key);
  }
  return useMyHook;
}

// Arrow hook inside factory function
function createHook(initialValue) {
  const useCounter = () => {
    return useState(initialValue);
  };
  return useCounter;
}

// Component nested inside a regular utility function
function setup() {
  function Tooltip() {
    return <span>tooltip</span>;
  }
  return Tooltip;
}
