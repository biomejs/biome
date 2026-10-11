// should not generate diagnostics

// biome-ignore lint/nursery/noArgumentsCallerOrCallee: Legacy recursion.
arguments.callee();
// biome-ignore lint/nursery/noArgumentsCallerOrCallee: Legacy caller access.
arguments.caller;
