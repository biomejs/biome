// The chain is expanded when its last group fits on its own line at an
// indent width of 8, even if an earlier group doesn't fit.
f(a.b("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").c({ k: 1 }));

// The object would also break in the expanded chain, so the chain stays on one
// line and only the object breaks.
z.string().min(2).max(200).regex(/[a-z]/gm, { message: "Only English alphabet symbols and hyphen allowed" });

// The head and the leading groups are narrower than one indentation level, so
// the chain is expanded and its last call puts each argument on its own
// line.
foo(a.b().c(iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii, { k: 1 }));

// The last group fits on its own line, so the chain is expanded; the width of
// the head's lines doesn't affect this choice.
new Foo([[["ssssssssssssssssssssssssssssssssssssssssssssssss"]]]).bbbbbb().c({ alpha: 1, beta: 2 });

// A leading group whose line doesn't fit in the expanded chain could fit on
// the first line of the one-line chain, so the chain is expanded and both
// objects break.
f(a.bbbb({ k: "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk" }).c({ alpha: "aaaaaaaaaaaaaaaaaaaa", beta: "bbbbbbbbbbbbbbbbbbbbbbbbb", gamma: "ccccccccc" }));
