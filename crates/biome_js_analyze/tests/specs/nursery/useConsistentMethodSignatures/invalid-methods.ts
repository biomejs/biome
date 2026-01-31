/* should generate diagnostics */

interface PropIface {
  propFunc: (arg: string) => number;
}

type GenericProp<T, U> = {
  p: (arg: T) => U;
};

type PropUnion =
  | {
      foo: (bar: number) => number;
    }
  | 4;

type bad = {
  qux: (quux: number) => "quuux";
} & { foo: string };

interface OverloadedProps {
  moo: (() => void) & ((x: number) => number) & ((x: string) => boolean);
}
