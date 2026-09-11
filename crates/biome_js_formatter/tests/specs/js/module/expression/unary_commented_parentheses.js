!(// leading
a || b);

!((
  // leading
  a || b
));

!(/* leading */ a && b);
!(a || b /* trailing */);
!(a || b // trailing
);
!(// leading
a + b * c);
!(// leading
a ? b : c);
!(// leading
a = b);
!(// leading
a, b);
+(// leading
a || b);
-(// leading
a || b);
~(// leading
a || b);
typeof (// leading
a || b);
void (// leading
a || b);
delete (// leading
a || b);
!(// leading
a || (b && c));
!(// leading
!(// nested
a || b));
!(// leading
a);
!(a || b);
// biome-ignore format: preserve expression
!(a   ||   b);

!(
  (
    // blah 1
    foo
    // blah 2
    || bar
    || baz
    // blah 3
    || qux
  )
);
