// Tests around grouping layouts and simple arguments

// Cases where the second argument is too complex to group.
foo(() => {
    foo
  },
  Math.floor(a + b),
);
foo(() => {
    foo
  },
  Math.floor(a, b),
);
foo(() => {
    foo
  },
  Math.floor(/123456/),
);
foo(() => {
    foo
  },
    a[(1,2)]
);
foo( () => {
		foo;
	},
	arr[Math.floor(1 + 2)],
);
foo( () => {
		foo;
	},
	a || b + 1,
);
foo( () => {
		foo;
	},
	a + b ?? 1,
);


// Cases where the second argument is simple enough to group.
foo(() => {
    foo
  },
  []
);
foo(() => {
    foo
  },
  activities[1]
);
foo(() => {
    foo
  },
  Math.floor(/1234/),
);
foo(() => {
    foo
  },
  a + b,
);
foo(() => {
  foo;
}, a || b);
foo(() => {
    foo
  },
  ++b,
);
foo(() => {
    foo
  },
  +!-+b,
);
foo(() => {
    foo
  },
  bar.baz.long,
);
foo(() => {
    foo;
}, arr[Math.floor(1)]);
foo(() => {
    foo;
}, [Math.floor(1 + 2)]);