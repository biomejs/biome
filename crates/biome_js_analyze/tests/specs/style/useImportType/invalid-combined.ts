// Leading comment
import/*1*/A/*2*/
// Comma comment
,/*3*/{ B }/*4*/from/*5*/""/*6*/; // Trailing comment
// Comment
export type { A, B };

import C, { D, E, F } from "";
export { type C, type D, type E, F };

import G, { type H, I } from "";
export type { G, H, I };

// Leading comment
import /*1*/M/*2*/
// Comma comment
,/*3*/*/*4*/as/*5*/N/*6*/from/*7*/""/*8*/;/*9*/
// Comment
export type { M, N };

import O, * as P from "";
export { type O, P };

import Q, * as R from "";
export { Q, type R };

import T, { U } from "";
export { T, type U };
