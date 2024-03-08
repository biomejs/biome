import { type T1, V1 } from "./mod.ts";
export { T1, V1 };

import type { T2, T3 } from "./mod.ts";
export { T2, T3 };

import type T4 from "./mod.ts";
export { T4 };

import type * as ns from "./mod.ts";
export { ns };

interface Interface {}
type TypeAlias = {}
enum Enum {}
function func() {}
class Class {}
export { Interface, TypeAlias, Enum, func as f, Class };

export /*0*/ { /*1*/ type /*2*/ func /*3*/, /*4*/ type Class as C /*5*/ } /*6*/;

export {}
