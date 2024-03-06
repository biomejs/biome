import { A, B, C, D, E } from "";
type AA = A;
type BB = typeof B;
export { type C };
export { D };
const EE = E;

import { X, Y } from "";
type XX = X;
const YY = Y;

//import { type U, V } from "";
//type VV = V;

import { type X, type Y, type Z } from "";
export type { X, Y, Z };
