123 == NaN;
123 === NaN;
NaN === "abc";
NaN == "abc";
123 != NaN;
123 !== NaN;
NaN !== "abc";
NaN != "abc";
NaN < "abc";
"abc" < NaN;
NaN > "abc";
"abc" > NaN;
NaN <= "abc";
"abc" <= NaN;
NaN >= "abc";
"abc" >= NaN;
123 == Number.NaN;
123 === Number.NaN;
Number.NaN === "abc";
Number.NaN == "abc";
123 != Number.NaN;
123 !== Number.NaN;
Number.NaN !== "abc";
Number.NaN != "abc";
Number.NaN < "abc";
"abc" < Number.NaN;
Number.NaN > "abc";
"abc" > Number.NaN;
Number.NaN <= "abc";
"abc" <= Number.NaN;
Number.NaN >= "abc";
"abc" >= Number.NaN;
x === Number?.NaN;
x === Number['NaN'];

123 == globalThis.NaN;
123 == window.NaN;
123 == globalThis.Number.NaN;

// switch-case
switch(NaN) { case foo: break; }
switch(NaN) {}
switch(foo) { case NaN: break; }
switch(NaN) { default: break; }
switch(NaN) { case foo: break; default: break; }
switch(foo) { case NaN: }
switch(foo) { case (NaN): break; }
switch(foo) { case bar: break; case NaN: break; default: break; }
switch(foo) { case bar: case NaN: default: break; }
switch(foo) { case bar: break; case NaN: break; case baz: break; case NaN: break; }
switch(NaN) { case NaN: break; }
switch(foo) { case Number.NaN: break; }
switch(Number.NaN) { case foo: break; }
switch(Number.NaN) {}
switch(Number.NaN) { default: break; }
switch(Number.NaN) { case foo: break; default: break; }
switch(foo) { case Number.NaN: }
switch(foo) { case (Number.NaN): break; }
switch(foo) { case bar: break; case Number.NaN: break; default: break; }
switch(foo) { case bar: case Number.NaN: default: break; }
switch(foo) { case bar: break; case NaN: break; case baz: break; case Number.NaN: break; }
switch(Number.NaN) { case Number.NaN: break; }
