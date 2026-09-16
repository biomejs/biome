// should generate diagnostics
const untyped = function(): void {};
const arrow = (): void => {};
const assertion = (() => {}) as () => void;

type Handler = () => void;
const typed: Handler = () => {}, untypedSibling = () => {};
export const exportedTyped: Handler = function() {}, exportedUntyped = function() {};
