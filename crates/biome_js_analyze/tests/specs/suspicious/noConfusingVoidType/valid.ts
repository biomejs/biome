// ref: https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/tests/rules/no-invalid-void-type.test.ts
function func(): void {}
type NormalType = () => void;
let normalArrow = (): void => {};
let ughThisThing = void 0;
function takeThing(thing: undefined) {}
takeThing(void 0);
let voidPromise: Promise<void> = new Promise<void>(() => {});
let voidMap: Map<string, void> = new Map<string, void>();

function returnsVoidPromiseDirectly(): Promise<void> {
  return Promise.resolve();
}

async function returnsVoidPromiseAsync(): Promise<void> {}
type UnionType = string | number;
type GenericVoid = Generic<void>;
type Generic<T> = [T];
type voidPromiseUnion = void | Promise<void>;
type promiseNeverUnion = Promise<void> | never;
const arrowGeneric1 = <T = void,>(arg: T) => {};
declare function functionDeclaration1<T = void>(arg: T): void;

type Allowed<T> = [T];
type Banned<T> = [T];
type AllowedVoid = Allowed<void>;
type AllowedVoid = Ex.Mx.Tx<void>;
type voidPromiseUnion = void | Promise<void>;
type promiseVoidUnion = Promise<void> | void;

async function foo(bar: () => void | Promise<void>) {
  await bar();
}
type promiseNeverUnion = Promise<void> | never;
type voidPromiseNeverUnion = void | Promise<void> | never;

class Test {
  public static helper(this: void) {}
  method(this: void) {}
}

functionGeneric<void>(undefined);

type Conditional<T> = T extends void ? Record<string, never> : T
