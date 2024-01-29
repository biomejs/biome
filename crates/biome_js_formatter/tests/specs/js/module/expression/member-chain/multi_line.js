// These should all collapse
foo
    .bar

    .baz;
foo
    .bar

    .baz

    .bar()
foo

    .bar()

    .baz;
foo

    .bar[1]

    .baz();
foo

    .bar().baz

    .bar();
foo.bar

    .baz()
    .bar

    .baz();
foo

[1]

    .bar();
foo

    .bar

[1]

    .bar();

// These should all preserve empty lines in some way
foo.bar()

    .baz();
foo.bar()

    .baz.bar();
foo

    .bar()

    .baz();

foo

    .bar.what().foo
    .what()

    .baz();
foo.bar

    .baz()

    .bar
    .baz()

    .foo();

