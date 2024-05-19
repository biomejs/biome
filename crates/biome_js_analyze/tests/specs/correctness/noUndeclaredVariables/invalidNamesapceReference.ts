import * as Ns1 from "";
export type T1 = Ns1; // This doesn't reference the import namespace `Ns1`

import type * as Ns2 from "";
export type T2 = Ns2;  // This doesn't reference the import namespace `Ns1`