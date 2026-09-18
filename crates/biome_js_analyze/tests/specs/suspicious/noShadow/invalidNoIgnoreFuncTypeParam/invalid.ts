/* should generate diagnostics */

// With ignoreFunctionTypeParameterNameValueShadow: false, these should be flagged
const test = 1;
type Fn = (test: string) => typeof test;

const item = "hello";
function process(callback: (item: string) => void) {}
