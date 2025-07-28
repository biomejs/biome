// Test data setup
declare const text: string;
declare const someVariable: any;
declare const value: any;
declare const obj: any;
declare function getValue(): string;
declare function fn(): any;

// Single interpolation
const wrappedText = `${text}`;
const wrappedNumber = `${42}`;
const wrappedExpression = `${getValue()}`;
const wrappedVar = `${someVariable}`;

// String literal interpolations that can be combined
const ab = `${'a'}${'b'}`;
const combined = `${'Hello '}${'World'}`;
const withNumbers = `${'1 + 1 = '}${2}`;
const multipleStrings = `${'foo'}${'bar'}${'baz'}`;

// Template literals without interpolations
const plain = `simple string`;
const withSpaces = `text with spaces`;
const noInterpolation = `just a plain template literal`;

// Mixed string literals
const mixed1 = `${'start'} middle ${'end'}`;
const mixed2 = `prefix ${'string literal'} suffix`;
const mixed3 = `${'only'}${'strings'}${'here'}`;

// Complex combinations
const multiString = `${'a'}${'b'}${'c'}${'d'}`;
const withEmpty = `${''} nonempty`;
const emptyStart = `${''}text`;
const emptyEnd = `text${''}`;

// Should handle quotes properly
const singleQuotes = `${'text with "quotes"'}`;
const doubleQuotes = `${"text with 'quotes'"}`;

// TypeScript specific cases
const typeAssertion = `${value as string}`;
const nonNullAssertion = `${value!}`;
const optionalChain = `${obj?.prop}`;

// Function and method calls
const functionResult = `${fn()}`;
const methodResult = `${obj.method()}`;

// Member expressions
const property = `${obj.prop}`;
const computed = `${obj['key']}`;

// Literals
const numberLiteral = `${123}`;
const booleanLiteral = `${true}`;
const nullLiteral = `${null}`;
const undefinedLiteral = `${undefined}`;