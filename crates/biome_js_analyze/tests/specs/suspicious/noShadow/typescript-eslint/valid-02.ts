/* should not generate diagnostics */
type Args = 1;
function foo<T extends (Args: any) => void>(arg: T) {}
