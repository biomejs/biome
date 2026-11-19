/* should generate diagnostics */

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

// `any` and `unknown`: no fix
declare const anyValue: any;
const anyResult = anyValue || 'default';

declare const unknownValue: unknown;
const unknownResult = unknownValue || 'default';

// Function return
function getValue(x: string | null): string {
  return x || 'default';
}

// Arrow function
const getDefault = (x: number | undefined) => x || 42;

// Inferred callback parameter
declare function consume<T>(callback: (value: T) => void): void;

consume<string | null>((value) => {
	value || "default";
});

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

// Chained `||`
declare const x: string | null;
declare const y: string | null;
const k = x || y || 'default';

// Number union with `null`: no fix
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

// Optional property in Pick
declare const pickOptional: Pick<{a?: string, b: number}, "a">;
const vPickOpt = pickOptional.a || "default";

// Nullable property in Pick
declare const pickNullable: Pick<{a: string | null}, "a">;
const vPickNull = pickNullable.a || "default";

// Nullable property in Omit
declare const omitNullable: Omit<{x: number, y: string | null}, "x">;
const vOmitNull = omitNullable.y || "default";

// Nested nullable object in Pick
declare const nestPick: Pick<{cfg: {name: string} | null}, "cfg">;
const nestV = nestPick.cfg || {name: "d"};

// Optional property in Omit
declare const omitOpt: Omit<{a: string, b?: number}, "a">;
const omitV = omitOpt.b || 0;

// Partial type
declare const partialObj: Partial<{name: string}>;
const partialOr = partialObj.name || 'default';

type P = Partial<{a: number}>;
declare const p: P;
const v = p.a || 0;

// Partial<T> with nullable base member
declare const partNull: Partial<{a: string | null}>;
const vPartNull = partNull.a || "default";

// Required with nullable optional
declare const reqNull: Required<{a?: string | null}>;
const vReqNull = reqNull.a || "default";

// Readonly with optional property
declare const roOpt: Readonly<{a?: string}>;
const vRoOpt = roOpt.a || "default";

// `||` on destructured parameters without a default
type FormValues = {
	value: string;
};

type GenericProps<T extends FormValues> = {
	values: T;
	inert?: boolean;
};

type PlainProps = {
	inert?: boolean;
};

export function genericWithoutDefault<T extends FormValues>({
	inert,
}: GenericProps<T>) {
	return inert || undefined;
}

export function plainWithoutDefault({ inert }: PlainProps) {
	return inert || undefined;
}

// `||` on a destructured parameter whose default keeps `null` in the type
export function nullableWithDefault({
	value = "fallback",
}: {
	value?: string | null;
}) {
	return value || undefined;
}

// `||` on an array element without a default
declare const list: (number | undefined)[];
const [head] = list;
export const arrayResult = head || undefined;

// `||` after an assertion function declared without a type annotation, which TypeScript ignores (TS2775)
const assertDefined = (value: unknown): asserts value => {
	if (!value) {
		throw new Error();
	}
};

function withDefault(value: string | undefined) {
	assertDefined(value);
	return value || "default";
}

// Types that a `0`, `''`, or `false` value can have: no fix
declare const numberObject: Number | null;
const primitive1 = numberObject || 1;
declare const stringObject: String | null;
const primitive2 = stringObject || 'x';
declare const objectInstance: Object | null;
const primitive3 = objectInstance || {};
declare const emptyObject: {} | null;
const primitive4 = emptyObject || {};
declare const withToString: { toString(): string } | null;
const primitive5 = withToString || { toString: () => '' };
declare const withLength: { length: number } | null;
const primitive6 = withLength || { length: 0 };
interface WithLength {
	length: number;
}
declare const interfaceWithLength: WithLength | null;
const primitive7 = interfaceWithLength || { length: 0 };
export function unconstrainedGeneric<T>(value: T | null, fallback: T) {
	return value || fallback;
}
export function emptyObjectGeneric<T extends {} | null>(value: T, fallback: NonNullable<T>) {
	return value || fallback;
}

// Types that no `0`, `''`, or `false` value can have: safe fix
class Box {
	content = '';
}
declare const box: Box | null;
const object1 = box || new Box();
declare const strings: string[] | null;
const object2 = strings || [];
declare const callback: (() => void) | null;
const object3 = callback || (() => {});
declare const named: { a: string } | null;
const object4 = named || { a: '' };
interface Named {
	a: string;
}
declare const interfaceNamed: Named | null;
const object5 = interfaceNamed || { a: '' };
export function objectGeneric<T extends object>(value: T | null, fallback: T) {
	return value || fallback;
}

// An object expression type can be reassigned a falsy primitive that fits it: no fix
let lengthObject = Math.random() > 0.5 ? { length: 0 } : null;
const object6 = lengthObject || { length: 1 };

// An object expression type that no falsy primitive fits: safe fix
let plainObject = Math.random() > 0.5 ? { a: 0 } : null;
const object7 = plainObject || { a: 1 };

// A branded string and an iterable: no fix
declare const branded: (string & { __brand: 'id' }) | null;
const primitive8 = branded || 'x';
declare const iterable: Iterable<string> | null;
const primitive9 = iterable || [];

// Inherited members are not read: no fix
interface Tagged extends String {
	tag?: string;
}
declare const tagged: Tagged | undefined;
const primitive10 = tagged || 'x';

class Derived extends Box {}
declare const derived: Derived | null;
const primitive11 = derived || new Derived();

// A class constructor is always truthy: safe fix
declare const BoxConstructor: typeof Box | undefined;
const object8 = BoxConstructor || Box;

// Members that the generated `String` type lacks, such as `padStart`: reported with a fix
interface Padded {
	padStart(length: number): string;
}
declare const padded: Padded | undefined;
const object9 = padded || { padStart: (length: number) => '' };
