// no jsdoc is still fair game
function f(bar: number): string;
function f(baz: string): string;
function f(whatever: any): any {}

// 1 overload has JSDocs, but 2 do not (and are mergeable)
/** jsdoc 1 */
function foo(bar: number): string;
function foo(baz: string): string;
function foo(qux: boolean): string;
function foo(whatever: any): any {}

// TODO: Decide on whether to allow or deny this example (TSEslint passes, though of dubious merit)
// /** baba is you */
// export function foo(bar: string): void	
// export function foo(bar: number): void
// export function foo(bar: any): void {} 

// jsdoc on implementation

function bar(a: number): string;
function bar(a: string): string;
function bar(a: boolean): string;
/** jsdoc 1 */
function bar(a: any): any {}

// not jsdoc comments
interface frotz {
	// a b c d e f g...
	(a: string, b: number, c: number): string;
	/*************************** HIJKLMNOPQRS */
	(a: number, b: number, c: string): string;
	/* t u v w x y z */
	(a: number, b: string, c: string): string;
}

// same jsdocs
class banana {
	/** bake a cake */
	public bake(cakeType: string): void;
	/** bake a cake */
	public bake(flourAmt: number): void;
	/** bake a cake */
	public bake(ingredients: object): void;
	public bake(whatever: any): void {};
}

// several identical jsdocs (all get checked)
/** jsdoc 1 */
/** jsdoc 2 */
/** jsdoc 3 */
declare function f10(this: string): void;
/** jsdoc 1 */
/** jsdoc 2 */
/** jsdoc 3 */
declare function f10(this: number): void;
