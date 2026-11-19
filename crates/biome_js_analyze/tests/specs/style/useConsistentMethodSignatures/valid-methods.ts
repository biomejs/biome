/* should not generate diagnostics */

interface Methodish {
  propFunc(arg: string): number;
}

type flubber = {
  genericProp<T, U>(arg: T): U;
}

type bar = () => void;

interface OK {
  notAFunc: number;
}

declare class Baz {
  methodFunc: (arg: string) => number;
}
