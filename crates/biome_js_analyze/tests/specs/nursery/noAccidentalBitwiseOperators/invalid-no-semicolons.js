/* should generate diagnostics */

// No fix is offered, because the added parentheses would continue the previous line
foo()
obj & obj.a | x
foo()
a | {} && b

// A fix is offered at the start of the file or after a semicolon
foo();
obj & obj.a | x
