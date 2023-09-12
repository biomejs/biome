import { useEffect } from "react";

function MyComponent() {
  let a = 1;
  useEffect(() => {}, [a]);
}

// multiple extra dependencies

function MyComponent2() {
  let a = 1, b = 1;
  useEffect(() => {}, [a, b]);
}

// extra const

function MyComponent2() {
  const a = 1;
  useEffect(() => {}, [a]);
}

// dependency more deep than capture
// Note: This can be a valid case, but there is
// no way for the lint rule to know

function MyComponent1() {
  let someObj = getObj();
  useEffect(() => {
      console.log(someObj)
  }, [someObj.id]);
}
