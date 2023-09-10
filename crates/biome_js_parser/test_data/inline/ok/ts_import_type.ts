type A = typeof import("test");
type B = import("test");
type C = typeof import("test").a.b.c.d.e.f;
type D = import("test")<string>;
type E = import("test").C<string>;
