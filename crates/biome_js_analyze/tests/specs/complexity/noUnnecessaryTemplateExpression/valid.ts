/* should not generate diagnostics */

// Test data setup
declare const name: string;
declare const obj: any;
declare const firstName: string;
declare const lastName: string;
declare const color: string;
declare const animal: string;
declare const obstacle: string;
declare const content: string;
declare const a: number;
declare const b: number;
declare const c: number;
declare const d: number;
declare const isActive: boolean;
declare const value: any;
declare const array: string[];
declare const str: string;
declare const items: any[];
declare const MyEnum: { Value: string };
declare const first: string;
declare const last: string;
declare const flag: boolean;
declare const newValue: any;
declare let count: number;
declare const css: any;
declare const gql: any;
declare const html: any;
declare function getValue(): string;
declare function fn<T>(): T;
declare function fetchData(): Promise<any>;

// Valid template literals with meaningful interpolations
const greeting = `Hello, ${name}!`;
const math = `1 + 1 = ${1 + 1}`;
const object = `Object: ${JSON.stringify(obj)}`;
const multiple = `${firstName} ${lastName}`;
const sentence = `The ${color} ${animal} jumped over the ${obstacle}.`;

// Template literals with newlines
const multiline = `line 1
line 2`;
const withNewlineChar = `First\nSecond`;

// Template literals with special characters
const withQuotes = `It's "nice"`;
const withBothQuotes = `He said "It's great"`;
const withBacktick = `Contains a \` backtick`;

// Tagged template literals
const styled = css`color: red;`;
const query = gql`query { user }`;
const htmlTemplate = html`<div>${content}</div>`;

// Multiple non-string interpolations
const calculation = `Result: ${a + b} and ${c * d}`;
const mixed = `String: ${'literal'} Number: ${42} Expression: ${getValue()}`;
const comparison = `Is equal: ${a === b}`;

// Empty template literal (should be kept as is)
const empty = ``;

// Template with only whitespace
const whitespace = `   `;
const tab = `	`;

// Complex expressions
const conditional = `Status: ${isActive ? 'Active' : 'Inactive'}`;
const nested = `Outer ${`inner ${value}`}`;
const logical = `Value: ${value || 'default'}`;

// Function calls and member expressions
const methodCall = `Result: ${array.join(', ')}`;
const chainedMethod = `Length: ${str.trim().length}`;
const arrayAccess = `Item: ${items[0]}`;

// TypeScript specific valid cases
const enumValue = `Type: ${MyEnum.Value}`;
const genericCall = `Result: ${fn<string>()}`;
const typeGuard = `Is string: ${typeof value === 'string'}`;

// Binary expressions
const addition = `Sum: ${a + b}`;
const concatenation = `Full: ${first + ' ' + last}`;

// Unary expressions
const negation = `Negative: ${-value}`;
const not = `Inverted: ${!flag}`;

// Assignment expressions
const assignment = `Updated: ${value = newValue}`;
const compound = `Incremented: ${count += 1}`;

// Array and object expressions
const arrayLiteral = `Array: ${[1, 2, 3]}`;
const objectLiteral = `Object: ${{ key: 'value' }}`;

// Regular expressions
const regex = `Pattern: ${/test/g}`;

// New expressions
const instance = `Instance: ${new Date()}`;

// Await expressions
const asyncResult = `Data: ${await fetchData()}`;

// Yield expressions
function* generator() {
    const yielded = `Yielded: ${yield value}`;
}