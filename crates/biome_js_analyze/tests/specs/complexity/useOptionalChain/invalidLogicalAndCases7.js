// should generate diagnostics

// simple prefix
bar && foo && foo.length;

// prefix with longer chain
bar && foo && foo.bar && foo.bar.baz;

// multiple unrelated prefixes
a && b && foo && foo.bar;

// prefix with member access in chain
bar && foo.bar && foo.bar.baz;

// prefix is a call expression
something() && foo && foo.bar;

// prefix with a chain that has a jump
bar && foo && foo.bar.baz;

// prefix with chained calls
bar && foo && foo.bar && foo.bar();

// longer prefix chain
a && b && c && foo && foo.bar;

// the original issue example
bar && foo && foo.length;
