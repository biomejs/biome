/* should not generate diagnostics */

/* for the `paths` option */
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

/* for the `patterns` option */
import '../../sideeffect-allowed';
import * as alias1 from "namespace-import-allowed";
export * from "namespace-import-allowed";
import { export1, export2 as alias2, "string-name" as alias3, default as defaultExport } from "named-import-allowed";
export { export1, export2 as alias2, "string-name" as alias3, default as defaultExport } from "named-import-allowed";
import defaultExport from "default-import-allowed";

import('d-sideeffect-allowed');
await import('d-sideeffect-allowed');
const alias1 = await import('namespace-import-allowed');
import('namespace-import-allowed').then(alias1 => { });
import('namespace-import-allowed').then((alias1) => { });
import('namespace-import-allowed').then(function(alias1) { });
myFunction(...args, import("namespace-import-allowed"), ...args)
const { export1, export2: alias2, "string-name": alias3, default: defaultExport } = await import("named-import-allowed");
import('named-import-allowed').then(({ export1, export2: alias2, "string-name": alias3, default: defaultExport }) => { });
import('named-import-allowed').then(function({ export1, export2: alias2, "string-name": alias3, default: defaultExport }) { });
const { default: defaultExport } = await import('default-import-allowed')
import('default-import-allowed').then(({ default: defaultExport }) => { });
import('default-import-allowed').then(function({ default: defaultExport }) { });
const defaultExport = require('default-import-allowed');

import { type export1, type export2 as alias2, type "string-name" as alias3, type default as defaultExport } from "named-import-allowed";
import type { export1, export2 as alias2, "string-name" as alias3, default as defaultExport } from "named-import-allowed";
import type defaultExport from "default-import-allowed";
