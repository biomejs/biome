/* should not generate diagnostics */
function statementBetween() { if (test) { return true; } doSomething(); return false; }

function nonBooleanReturns() { if (test) { return a; } return b; }

function multipleStatements() { if (test) { return true; doSomething(); } return false; }

function emptyReturn() { if (test) { return; } return false; }

function conditionalTest() { if (a ? b : c) { return true; } return false; }

function parenthesizedConditionalTest() { if ((a ? b : c)) { return true; } return false; }

function sameValues() { if (test) { return true; } else { return true; } }

function sameValuesFlat() { if (test) { return false; } return false; }

function earlyReturns() { if (a) { return false; } if (b) { return false; } return true; }

function threeEarlyReturns() { if (a) { return false; } if (b) { return false; } if (c) { return false; } return true; }

function earlyReturnsTrue() { if (a) { return true; } if (b) { return true; } return false; }

function earlyReturnsStatements() { if (a) return false; if (b) return false; return true; }

function stringLiterals() { if (test) { return "true"; } return "false"; }

function elseIfChain() { if (a) { return true; } else if (b) { return false; } }

function nothingAfter() { if (test) { return true; } }

function notAReturn() { if (test) { return true; } throw new Error(); }

function loopAfter() { if (test) { return true; } while (true) { return false; } }

function sequenceReturn() { if (test) { return (log(), true); } return false; }

function sequenceAlternate() { if (test) { return true; } return (cleanup(), false); }
