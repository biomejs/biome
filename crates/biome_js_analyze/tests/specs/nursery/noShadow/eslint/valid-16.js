/* should not generate diagnostics */
class C { static { { var x; } { var x; /* redeclaration */ } } }
