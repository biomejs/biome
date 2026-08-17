// should generate diagnostics
// expr || false and expr && true produce diagnostics with unsafe fixes
// because the non-literal operand may not be boolean (e.g. boolean | undefined).
var x: boolean | undefined;
var y: boolean;
y = x || false;
y = x && true;
