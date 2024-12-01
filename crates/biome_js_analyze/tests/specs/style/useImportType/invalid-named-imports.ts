import { A, B, C, D, E } from "";
type AA = A;
type BB = typeof B;
export { type C };
export { D };
const EE = E;

import { X, Y } from "";
type XX = X;
const YY = Y;

import { type H, type I, type J } from "";
export type { H, I, J };

import type { type M, N, type O } from "";

// multiline
import {
    U,
    V,
    // leading comment
    W,
} from "";
export { U, type V, type W };
