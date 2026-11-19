/* should not generate diagnostics */
globalThis.window?.BigInt(1);
globalThis.window?.["BigInt"](1);
globalThis?.window.BigInt(1);
(globalThis.window?.BigInt)(1);
(globalThis.window?.["BigInt"])(1);
(globalThis?.window).BigInt(1);
(globalThis.window?.globalThis).BigInt(1);
globalThis.window?.BigInt?.(1);
