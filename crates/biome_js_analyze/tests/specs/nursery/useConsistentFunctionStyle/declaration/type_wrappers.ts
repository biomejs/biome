// should generate diagnostics
const asArrow = (() => {}) as () => void;
const satisfiesArrow = (() => {}) satisfies () => void;
const assertedArrow = <() => void>(() => {});
const nonNullArrow = (() => {})!;
const asFunction = (function() {}) as () => void;
const nestedWrappers = (((() => {}) as () => void) satisfies () => void)!;
const instantiated = (function<T>(value: T) { return value; })<string>;
