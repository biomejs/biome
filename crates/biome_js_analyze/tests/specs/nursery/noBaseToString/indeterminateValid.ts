// should not generate diagnostics
type Recursive = Recursive;
declare const value: Recursive;

String(value);
