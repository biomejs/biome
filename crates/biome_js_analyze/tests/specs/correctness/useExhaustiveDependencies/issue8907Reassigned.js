import { useEffect, useState, useRef } from "react";

// Issue #8907: If a let binding is reassigned, it's no longer stable
export const Component3 = () => {
  let [a, setA] = useState(0);
  setA = () => {}; // reassigned!

  useEffect(() => {
    setA(1); // should now require setA in deps
  }, []);
};

// Same for useRef
export const Component4 = () => {
  let b = useRef("");
  b = { current: "other" }; // reassigned!

  useEffect(() => {
    console.log(b.current); // should now require b in deps
  }, []);
};
