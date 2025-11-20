function f1(a: number): void;
function f1(b: string): void;
function f1(a: number | string): void {}

function f2(x: number): void;
function f2(x: any): void;
function f2(x: any): any {
    return x;
}

function f3(x: number): void;
function f3(x: string): void;
function f3(x: any): any {
    return x;
}

function opt(xs?: number[]): void;
function opt(xs: number[], y: string): void;
function opt(...args: any[]) {}

interface I1 {
    a0(): void;
    a0(x: string): string;
    a0(x: number): void;
}

interface I2 {
    a1(): void;
    a1(x: number): void;
}

// Exported functions.
export function f4(a: number): void;
export function f4(a: string): void;
export function f4(a: unknown): void {
    return;
}

// Exported default functions.
export default function f5(a: number): void;
export default function f5(a: string): void;
export default function f5(a: unknown): void {
    return;
}

// The second signature is different by single required parameter.
interface I3 {
    a3(): void;
    a3(x: number, y?: number, ...z: number[]): void;
}

// The difference is the rest parameter.
interface I4 {
    b(): void;
    b(...x: number[]): void;
}

// Both parameters are optional.
interface I5 {
    c2(x?: number): void;
    c2(x?: string): void;
}

interface I6 {
    d(x: number): void;
    d(x: string): void;
}

// Support call signatures in types.
type T1 = {
    (): void;
    (x: number): void;
};

// Support call signatures in interfaces.
interface I7 {
    (): void;
    (x: number): void;
}

// Supports private methods in classes.
declare class Example {
    #privateMethod(a: number): void;
    #privateMethod(a: number, b?: string): void;
}

// Supports class constructors.
declare class C {
    constructor();
    constructor(x: number);
}

// Supports unions.
interface I8 {
    f(x: number);
    f(x: string | boolean);
}

// Supports tuples.
interface I9 {
    f(x: number);
    f(x: [string, boolean]);
}

// Supports generics.
interface Generic<T> {
    y(x: T[]): void;
    y(x: T): void;
}

// Merges signatures when type parameters are the same.
function f6<T extends number>(x: T[]): void;
function f6<T extends number>(x: T): void;
function f6(x: unknown): void {}

// Supports abstract methods.
abstract class Foo {
    public abstract f(x: number): void;
    public abstract f(x: string): void;
}

// Supports literal names.
interface I10 {
    'f'(x: string): void;
    'f'(x: number): void;
}

// Supports constructor signatures.
interface Foo {
    new (x: string): Foo;
    new (x: number): Foo;
}

// Supports computed property names.
interface IFoo {
    ['name'](x: string): void;
    ['name'](x: number): void;
}

declare module 'foo' {
    export default function (foo: number): string[];
    export default function (foo: number, bar?: string): string[];
}

declare function f7(x: string): void;
declare function f7(x: number): void;
declare function f7(x: boolean): void;

// Transfers JsDoc comments to signatures.
/** JsDoc 1 */
declare function f8(x: string): void;
/** JsDoc 2 */
declare function f8(x: number): void;

// Transfers JsDoc comments to signatures even if signatures are not adjacent.
/** JsDoc 1 */
declare function f9(x: string): void;
declare function f9(x: boolean): boolean;
/** JsDoc 2 */
declare function f9(x: number): void;

// Merges "this" params.
declare function f10(this: string): void;
declare function f10(this: number): void;
