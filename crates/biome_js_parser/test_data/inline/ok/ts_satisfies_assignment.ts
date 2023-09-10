let a: any;
type B<A> = { a: A };
(a satisfies string) = "string";
((a satisfies any) satisfies string) = null;
({ b: a satisfies string } = { b: "test" });
([ a satisfies string ] = [ "test" ]);
for (a satisfies string in []) {}
(a satisfies B<string>) = { a: "test" };
