// should generate diagnostics

// Basic indexOf patterns - not equal to -1
"foo".indexOf("o") !== -1;
"foo".indexOf(bar) >= 0;

// Basic indexOf patterns - equal to -1 (negated)
["a", "b", "c"].indexOf("a") === -1;
["a", "b", "c"].indexOf(bar) < 0;

// Greater than or equal to 0
"hello".indexOf("e") >= 0;

// Less than 0 (negated)
"hello".indexOf("x") < 0;

// Not equal to -1
"test".indexOf("t") != -1;

// Equal to -1 (negated)
"test".indexOf("x") == -1;
