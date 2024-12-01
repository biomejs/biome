import { type T1, V1 } from "./mod.ts";
export { T1, V1 };

import type { T2, T3 } from "./mod.ts";
export { T2, T3 };

import type T4 from "./mod.ts";
export { T4 };

// multiline
import { type T5, type T6, V2 } from "./mod.ts";
export {
    // leading comment
    T5,
    T6,
    V2,
};

import type * as ns from "./mod.ts";
export { ns };

interface Interface {}
type TypeAlias = {}
enum Enum {}
function func() {}
class Class {}
export { Interface, TypeAlias, Enum, func as f, Class };

export /*0*/ { /*1*/ type /*2*/ func /*3*/, /*4*/ type Class as C /*5*/ } /*6*/;

import { type T7, type T8 } from "./mod.ts";
export {
  /*1*/
  type T7,
  /*2*/
  type T8,
};

import type * as Ns from ""
export { Ns }

import { type T9, type T10 } from "./mod.ts";
export { type T9, type T10 };

export { type T11, type T12 } from "./mod.ts";
