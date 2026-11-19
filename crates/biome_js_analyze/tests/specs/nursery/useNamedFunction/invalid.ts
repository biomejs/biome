/* should generate diagnostics */
const typed = function<T>(this: Window, value: T): T { return value; };
const asserted = function() {} as () => void;
