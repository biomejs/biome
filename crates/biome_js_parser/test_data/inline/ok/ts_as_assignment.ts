let a: any;
type B<A> = { a: A };
(a as string) = "string";
((a as any) as string) = null;
({ b: a as string } = { b: "test" });
([ a as string ] = [ "test" ]);
for (a as string in []) {}
(a as B<string>) = { a: "test" };
(<number> a) += 1
