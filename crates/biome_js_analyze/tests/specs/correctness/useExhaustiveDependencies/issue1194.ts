import { useMemo } from 'react';

enum Foo {
  A = 1,
}

import Foo2 = Foo;

const useBad = () => {
  useMemo(() => {
    2 === Foo2.A;
  }, []);
};