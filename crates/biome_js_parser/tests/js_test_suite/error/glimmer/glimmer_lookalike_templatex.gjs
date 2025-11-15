// Test that <templatex> (identifier after "template") is properly rejected
// This tests lexer backtracking - should restore position and report clear error
const x = <templatex>content</templatex>;
