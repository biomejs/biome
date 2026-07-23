/* should generate diagnostics */

interface Example {
  methodFunc(arg: string): number;
}

type Generic<T, U> = {
  methodFunc(arg: T): U;
}

type Union =
  | {
    foo(bar: number): number;
  }
  | 4;

type Intersection =
{
  qux(quux: number): "quuux";
} & { foo: string };

interface Overloaded {
  moo(): void;
  moo(x: number): number;
  moo(x: string): boolean;
}
