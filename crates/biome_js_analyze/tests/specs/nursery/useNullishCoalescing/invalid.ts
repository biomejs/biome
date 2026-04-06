// should generate diagnostics

// Nullish literals
const a: null = null;
const b = a || 'default';

const c: undefined = undefined;
const d = c || 'fallback';

// Union with null
declare const maybeString: string | null;
const e = maybeString || 'default';

// Union with undefined
declare const maybeNumber: number | undefined;
const f = maybeNumber || 0;

// Union with null and undefined
declare const maybeValue: string | null | undefined;
const g = maybeValue || 'fallback';

// Function return
function getValue(x: string | null): string {
  return x || 'default';
}

// Arrow function
const getDefault = (x: number | undefined) => x || 42;

// Object property
declare const obj: { prop: string | null };
const h = obj.prop || 'default';

// Array element
declare const arr: (number | null)[];
const i = arr[0] || 0;

// Nested in parentheses
declare const val: string | null;
const j = (val || 'default').toUpperCase();

// Assignment
let result: string;
declare const source: string | null;
result = source || 'fallback';

// Chained ||
declare const x: string | null;
declare const y: string | null;
const k = x || y || 'default';

// Mixed nullish + falsy types (no safe fix)
declare const mixedValue: number | null;
const unsafeFix = mixedValue || 100;

// Optional property in type alias
type TypeWithOptional = { a?: string; b: number };
declare const objWithOptional: TypeWithOptional;
const optionalFromType = objWithOptional.a || 'default';

// Optional property in interface
interface InterfaceWithOptional {
  a?: string;
  b: number;
}
declare const objFromInterface: InterfaceWithOptional;
const optionalFromInterface = objFromInterface.a || 'default';

// Template literal interpolation
declare const tpl: string | null;
const t1 = `prefix ${tpl || 'default'} suffix`;

// Template literal with member access
declare const tplObj: { name: string | undefined };
const t2 = `Hello, ${tplObj.name || 'stranger'}!`;

// Template literal with chained ||
declare const tplA: string | null;
declare const tplB: string | null;
const t3 = `value: ${tplA || tplB || 'none'}`;

// Template literal with || in nested expression
declare const tplNum: number | undefined;
const t4 = `count: ${(tplNum || 0) + 1}`;

// Tagged template literal
declare function tag(strings: TemplateStringsArray, ...values: unknown[]): string;
declare const tplTag: string | null;
const t5 = tag`tagged ${tplTag || 'default'} end`;
