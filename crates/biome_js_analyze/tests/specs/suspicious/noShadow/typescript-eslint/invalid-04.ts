
// In this example T gets shadowed by the type parameter of the function
// but the `arg` bindings do NOT shadow each other.
type T = string;
function foo<T extends (arg: any) => void>(arg: T) {}
