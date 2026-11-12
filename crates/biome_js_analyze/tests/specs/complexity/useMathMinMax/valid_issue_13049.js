/* should not generate diagnostics */
const strings = ["foo", "bar"].map((a) => a);
const largest = strings.reduce((a, b) => (a > b ? a : b), "");

const largestNoInit = ["foo", "bar"].map((a) => a).reduce((a, b) => (a > b ? a : b));
