/* should not generate diagnostics */

// Types are erased before the code runs.
let callback: jest.Mock;
type Globals = typeof jest;
type Expectation = ReturnType<typeof expect>;

declare const xit: (name: string) => void;
xit("is declared in the file");
