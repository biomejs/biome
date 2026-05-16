// should not generate diagnostics
// Valid in record mode
type T1 = Record<string, number>;
type T2 = Record<string, Record<string, string>>;
