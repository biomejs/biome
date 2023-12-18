// Tests around grouping layouts and simple arguments

// Cases where the second argument is too complex to group.
foo(() => {
    foo
  },
  [] as string[][][],
);
foo(() => {
    foo
  },
  {} as Foo<Bar, Baz>,
);
foo(() => {
    foo
  },
  bar as {},
);


// Cases where the second argument is simple enough to group.
foo(() => {
    foo
  },
  [] as never[]
);
foo(() => {
    foo
  },
  bar as boolean
);
foo(() => {
    foo
  },
  [] as object[][]
);

foo(() => {
    foo
  },
  [] as Foo<number>[][]
);
foo(() => {
    foo
  },
  bar as MyCustomType[],
);
