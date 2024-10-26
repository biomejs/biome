// SCRIPT
let // NO ASI
x = 1;
for await (var x of []) let // ASI
x = 1;
