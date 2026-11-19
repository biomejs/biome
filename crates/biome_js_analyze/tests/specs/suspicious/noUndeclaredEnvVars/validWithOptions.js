/* should not generate diagnostics with custom allowed vars */

// MY_CUSTOM_VAR is now allowed via options (exact match)
const myVar = process.env.MY_CUSTOM_VAR;

// Pattern match for ^CUSTOM_.*$ should work
const customFoo = process.env.CUSTOM_FOO;
const customBar = process.env.CUSTOM_BAR;

// Still allowed by default
const nodeEnv = process.env.NODE_ENV;
