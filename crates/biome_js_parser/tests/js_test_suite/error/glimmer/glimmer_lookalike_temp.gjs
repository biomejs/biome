// Test that <temp> (not "template") is properly rejected
// This tests lexer backtracking - should restore position and report clear error
const x = <temp>content</temp>;
