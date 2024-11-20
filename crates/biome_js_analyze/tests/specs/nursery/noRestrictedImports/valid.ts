declare module "node:fs" { }

import 'bare-allowed';
import 'default-forbidden';
import 'namespace-forbidden';
import 'member-forbidden';

import * as n1 from 'namespace-allowed';
import * as n2 from 'default-forbidden';
import * as n3 from 'member-forbidden';

import type d0 from 'default-and-member-allowed';
import type d1 from 'member-forbidden';
import type d2 from 'namespace-forbidden';

import { type default as d3, type allowed1, type allowed2 as a2 } from 'default-and-member-allowed';
import { type default as d4, type allowed2, type allowed3 as a3 } from 'member-forbidden';
import { type default as d5, type allowed3, type allowed4 as a4 } from 'namespace-forbidden';

import type { default as d6, allowed5, allowed6 as a6 } from 'default-and-member-allowed';
import type { default as d7, allowed6, allowed7 as a7 } from 'member-forbidden';
import type { default as d8, allowed7, allowed8 as a8 } from 'namespace-forbidden';

export * as n1 from 'namespace-allowed';
export * as n2 from 'default-forbidden';
export * as n3 from 'member-forbidden';

export { type allowed1, type allowed2 as a2 } from 'default-and-member-allowed';
export { type allowed2, type allowed3 as a3 } from 'member-forbidden';
export { type allowed3, type allowed4 as a4 } from 'namespace-forbidden';

export type { default as d6, allowed5, allowed6 as a6 } from 'default-and-member-allowed';
export type { default as d7, allowed6, allowed7 as a7 } from 'member-forbidden';
export type { default as d8, allowed7, allowed8 as a8 } from 'namespace-forbidden';
