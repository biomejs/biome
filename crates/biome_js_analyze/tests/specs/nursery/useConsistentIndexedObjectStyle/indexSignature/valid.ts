// should not generate diagnostics
// Valid in index-signature mode
type T1 = { [key: string]: number };
type T2 = { [key: string]: { [key: string]: string } };
