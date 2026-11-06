/* should generate diagnostics */
const arguments = [];
function outerConst() { arguments.map(() => {}); }
function inDefaultValue(value = arguments.map(() => {})) {}
function arrowReadsOuterFunction() { return () => arguments.map(() => {}); }
function varWithoutValue() { var arguments; arguments.map(() => {}); }
function varWithoutValueComputed() { var arguments; arguments["map"](() => {}); }
function varWithoutValueInArrow() { var arguments; return () => arguments.map(() => {}); }
function varWithoutValueInBlock() { { var arguments; } arguments.map(() => {}); }
function varWithoutValueTwice() { var arguments; var arguments; arguments.map(() => {}); }
function varWithoutValueReassignedAfterCall() { var arguments; arguments.map(() => {}); arguments = []; }
function wrapper() { function arguments() { arguments.map(() => {}); } }
const namedExpression = function arguments() { arguments.map(() => {}); };
