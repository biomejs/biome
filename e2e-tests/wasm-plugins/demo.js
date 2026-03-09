// E2E test for the booleanNaming WASM plugin rule.
//
// The rule checks that boolean variables follow a naming convention:
// names must match ^(is|was|has|should|can|will|did|does)[A-Z].

// --- Cases that SHOULD trigger booleanNaming ---

// Boolean literals without proper prefix
const active = true;
const enabled = false;

// Comparison operators without proper prefix
const equal = a === b;
const notEqual = a !== b;
const looseEqual = a == b;
const looseNotEqual = a != b;
const smaller = x < y;
const bigger = x > y;
const smallerOrEqual = x <= y;
const biggerOrEqual = x >= y;

// Negation without proper prefix
const negated = !value;

// instanceof / in without proper prefix
const found = "key" in obj;
const typeCheck = x instanceof Array;

// --- Cases that should NOT trigger booleanNaming ---

// Properly prefixed boolean literals
const isActive = true;
const hasPermission = false;
const wasDeleted = true;
const shouldUpdate = false;
const canEdit = true;
const willReload = false;
const didChange = true;
const doesExist = false;

// Properly prefixed comparison expressions
const isEqual = a === b;
const isSmaller = x < y;
const hasMatch = x instanceof RegExp;
const isFound = "key" in obj;
const isNegated = !value;

// Non-boolean initializers (should not trigger even without prefix)
const count = 42;
const name = "hello";
const items = [1, 2, 3];
const user = { name: "test" };
const result = getValue();
