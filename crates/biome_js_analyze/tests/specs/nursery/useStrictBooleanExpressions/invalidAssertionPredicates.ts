// should generate diagnostics
[1].filter((value): asserts value is number => {});
[1].some(function (value): asserts value {});
function assertNumber(value: unknown): asserts value is number {}
[1].filter(assertNumber);
