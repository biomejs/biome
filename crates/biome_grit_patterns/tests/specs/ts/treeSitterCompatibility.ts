let legacyVar = 1;
"string";
42;
const legacyObj = { prop: "value" };
legacyObj.prop;
[1, 2, 3];
legacyVar = 2;
legacyVar + 1;
true ? "yes" : "no";
() => {};
function func() {}
func();
if (true) {}
for (let i = 0; i < 1; i++) {}
while (false) {}
function anotherFunc() { return; }
legacyVar;

// Native Biome AST Patterns (PascalCase)
`template literal`;
const nativeArrow = () => {};
const nativeCall = () => {}; nativeCall();
1 + 1;
true ? 1 : 2;
