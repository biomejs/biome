/* should generate diagnostics */

(a: string) => -a;
(a: {}) => -a;
(a: number[]) => -a;
-'hello';
(a: { x: number }) => -a;
(a: unknown) => -a;
(a: void) => -a;
(a: boolean) => -a;
(a: null) => -a;
(a: undefined) => -a;
(a: symbol) => -a;
(a: number | string) => -a;
(a: string & { __brand: "id" }) => -a;
(a: () => number) => -a;
-true;
-null;
-undefined;
-[];
-{};
-/regex/;
-(function () {});
class Foo {}
declare const foo: Foo;
-foo;
const text = "1";
-text;
<T extends string>(t: T) => -t;
-(-"1");
(a: number | undefined) => -a;
(a: { x?: number }) => -a.x;
(a: string[]) => -a[0];
declare const boxed: Number;
-boxed;
