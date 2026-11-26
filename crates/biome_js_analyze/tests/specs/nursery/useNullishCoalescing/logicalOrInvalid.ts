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

// `||` on a parameter whose default keeps `null` in the type
export function nullableParameterWithDefault(value: string | null | undefined = "fallback") {
	return value || "default";
}

// `||` on a parameter whose default may itself be `undefined`
declare const maybeFallback: string | undefined;
export function undefinedDefault(value: string | undefined = maybeFallback) {
	return value || "default";
}

// `||` on a defaulted parameter that is reassigned
export function reassignedDefault(value: string | undefined = "fallback") {
	value = undefined;
	return value || "default";
}

// `||` on a parameter whose `any` default keeps `null` in the type
declare const anyFallback: any;
export function anyDefault(value: string | null | undefined = anyFallback) {
	return value || "default";
}
