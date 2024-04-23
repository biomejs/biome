import React from "react";
import {
	useEffect,
	useCallback,
	useMemo,
	useLayoutEffect,
	useInsertionEffect,
	useImperativeHandle,
	useState,
	useReducer,
	useTransition,
} from "react";
import { useRef } from "unknown/hooks"

function MyComponent1() {
    let a = 1;
    const b = a + 1;
    useEffect(() => {
      console.log(a, b);
    }, []);
}

// interaction with other react hooks

function MyComponent2() {
  const [name, setName] = useState(0);
  const [state, dispatch] = useReducer();
  const memoizedCallback = useCallback();
  const memoizedValue = useMemo();
  const deferredValue = useDeferredValue(value);
  const [isPending, startTransition] = useTransition();
  useEffect(() => {
      console.log(name);
      setName(1);

      console.log(state);
      dispatch(1);

      console.log(memoizedCallback);
      console.log(memoizedValue);
      console.log(deferredValue);

      console.log(isPending);
      startTransition();
  }, []);
}

// all hooks with dependencies

function MyComponent3() {
  let a = 1;
  useEffect(() => console.log(a), []);
  useCallback(() => console.log(a), []);
  useMemo(() => console.log(a), []);
  useImperativeHandle(ref, () => console.log(a), []);
  useLayoutEffect(() => console.log(a), []);
  useInsertionEffect(() => console.log(a), []);
}

// inner closures

function MyComponent4() {
  let a = 1;
  useEffect(() => {
      return () => console.log(a)
  }, []);
}

// same variable multiple times

function MyComponent5() {
  let a = 1;
  useEffect(() => {
    console.log(a);
    return () => console.log(a);
  }, []);
}

// Capturing an object property

function MyComponent6() {
  let someObj = getObj();
  useEffect(() => {
      console.log(someObj.name)
  }, []);
}

const MyComponent7 = React.memo(function ({ a }) {
  useEffect(() => {
      console.log(a);
  }, []);
});

const MyComponent8 = React.memo(({ a }) => {
  useEffect(() => {
      console.log(a);
  }, []);
});

// exported functions
export function MyComponent9() {
  let a = 1;
  useEffect(() => {
      console.log(a);
  }, []);
}

export default function MyComponent10() {
  let a = 1;
  useEffect(() => {
      console.log(a);
  }, []);
}

// named function
function MyComponent11() {
  let a = 1;
  useEffect(function inner() {
      console.log(a);
  }, []);
}

function MyComponent12() {
  let a = 1;
  useEffect(async function inner() {
      console.log(a);
  }, []);
}

// React.useXXX case
function MyComponent13() {
  let a = 1;
  React.useEffect(() => {
      console.log(a);
  }, []);
}

// imports from other libraries
function MyComponent14() {
	const ref = useRef();
	useEffect(() => {
			console.log(ref.current);
	}, []);
}

// local overrides
function MyComponent15() {
	const useRef = () => {
		return { current: 1 }
	}
	const ref = useRef();
	useEffect(() => {
			console.log(ref.current);
	}, []);
}

// https://github.com/biomejs/biome/issues/2362
function MissingFunctionDeclaration() {
  const [value, setValue] = useState("")

  function func() {
    setValue("some")
  }

  useEffect(() => {
    func()
  }, [])

  return <>{value}</>
}
