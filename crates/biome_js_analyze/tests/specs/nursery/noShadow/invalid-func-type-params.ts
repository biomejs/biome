// When ignoreFunctionTypeParameterNameValueShadow is false,
// parameter names in function type annotations should be flagged.

function fn1(options: unknown, callback: (options: unknown) => void) {
}

const test = 1;
type Callback = (test: string) => void;
