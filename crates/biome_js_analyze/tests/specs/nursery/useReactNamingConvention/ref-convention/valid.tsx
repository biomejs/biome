/* should not generate diagnostics */
import { useRef } from "react";

const ref = useRef(null);
const inputRef = useRef(null);
obj.nested.myRef = useRef(null);

// The result is immediately dereferenced instead of being stored.
const value = useRef(null).current;
const wrappedRef = (useRef(null));
const wrapped2Ref = useRef(null) as Ref;
const wrapped3Ref = useRef(null) satisfies Ref;
