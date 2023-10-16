/* should not generate diagnostics */

import React from "react";
import { useEffect } from "react";
import doSomething from 'a';

// No captures
function MyComponent1() {
    useEffect(() => {
    });
}

// All needed captures in the dependency list
function MyComponent2() {
    let a = 1;
    const b = 1;
    const c = a + 1;
    useEffect(() => {
        console.log(a, b, c);
    }, [a, c]);
}

// capturing declarations
function doSomething() { }
class A {}

function MyComponent3() {
    useEffect(() => {
        doSomething();
        console.log(new A ());
    }, []);
}

// interaction with other react hooks
function MyComponent4() {
    const [name, setName] = useState(0);
    const ref = useRef();
    const theme = useContext();
    const [state, dispatch] = useReducer();
    const memoizedCallback = useCallback();
    const memoizedValue = useMemo();
    const [isPending, startTransition] = useTransition();
    const id = useId();
    const externalStore = useSyncExternalStore();
    useEffect(() => {
        console.log(name);
        setName(1);

        console.log(ref);

        console.log(theme);

        console.log(state);
        dispatch(1)

        memoizedCallback();
        console.log(memoizedValue);

        console.log(isPending);
        startTransition();

        console.log(id);

        console.log(externalStore);
    }, [name, state, memoizedCallback, memoizedValue, isPending]);
}

// all hooks with dependencies
function MyComponent5() {
    let a = 1;
    useEffect(() => console.log(a), [a]);
    useCallback(() => console.log(a), [a]);
    useMemo(() => console.log(a), [a]);
    useImperativeHandle(ref, () => console.log(a), [a]);
    useLayoutEffect(() => console.log(a), [a]);
    useInsertionEffect(() => console.log(a), [a]);
}

// inner closures
function MyComponent5() {
    let a = 1;
    useEffect(() => {
        let b = 2;
        return () => console.log(a, b)
    }, [a]);
}

// from import
function MyComponent6() {
    useEffect(() => {
        doSomething();
    });
}

// Capturing an object property
function MyComponent7() {
    let someObj = getObj();
    useEffect(() => {
        console.log(someObj.name);
        console.log(someObj.age)
    }, [someObj.name, someObj.age]);
}

// Specified dependency cover captures
function MyComponent8({ a }) {
    useEffect(() => {
      console.log(a.b);
    }, [a]);
}

// Capturing const outside of component
// https://github.com/rome/tools/issues/3727
const outside = f();
function MyComponent9() {
    useEffect(() => {
      console.log(outside);
    });
}

// Memoized Components
const MyComponent10 = React.memo(function () {
    useEffect(() => {
        console.log(outside);
    });
});

const MyComponent11 = React.memo(() => {
    useEffect(() => {
        console.log(outside);
    });
});

// exported functions
export function MyComponent12() {
    let a = 1;
    useEffect(() => {
        console.log(a);
    }, [a]);
}

export default function MyComponent13() {
    let a = 1;
    useEffect(() => {
        console.log(a);
    }, [a]);
}

// named function
function MyComponent14() {
    let a = 1;
    useEffect(function inner() {
        console.log(a);
    }, [a]);
}

function MyComponent15() {
    let a = 1;
    useEffect(async function inner() {
        console.log(a);
    }, [a]);
}

// React.useXXX case
function MyComponent16() {
    let a = 1;
    React.useEffect(() => {
        console.log(a);
    }, [a]);
}
