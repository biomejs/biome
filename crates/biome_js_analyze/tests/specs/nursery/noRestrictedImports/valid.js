const path = require('lodash');

import eslint from 'eslint';
const l = require('lodash');

import 'bare-allowed';

export * from 'namespace-allowed';
import * as n1 from 'namespace-allowed';
const n2 = await import('namespace-allowed');
const n2b = ((await (import('namespace-allowed'))));
const n3 = import('namespace-allowed');
const n3b = (import('namespace-allowed'));
import('namespace-allowed').then(n4 => { });
import('namespace-allowed').then((n5) => { });
import('namespace-allowed').then(function (n6) { });
((import('namespace-allowed'))).then(((n6 => { })));
((import('namespace-allowed'))).then((((n7) => { })));
((import('namespace-allowed'))).then(((function (n8) { })));

export { default } from 'default-and-member-allowed';
import d0 from 'default-and-member-allowed';
export { default as d1 } from 'default-and-member-allowed';
import { default as d1 } from 'default-and-member-allowed';
const { "default": d2 } = await import('default-and-member-allowed');
const { default: d3 } = await import('default-and-member-allowed');
export { default as d4, allowed1 } from 'default-and-member-allowed';
import { default as d4, allowed1 } from 'default-and-member-allowed';
const { allowed2, default: d5 } = await import('default-and-member-allowed');
const { allowed3, default: d6 } = import('default-and-member-allowed');
const { "allowed4": a1, "default": d7 } = import('default-and-member-allowed');

export { allowed2, allowed3 as a3 } from 'member-allowed';
import { allowed2, allowed3 as a3 } from 'member-allowed';
const { allowed2, allowed3: a3, "allowed4": a4 } = await import('member-allowed');
import('member-allowed').then(({ allowed2, allowed3: a3, "allowed4": a4 }) => { })
import('member-allowed').then(function ({ allowed2, allowed3: a3, "allowed4": a4 }) { })

// require() returns the default import, not a namespace object,
// so this import corresponds to default.* (and not namespace.*).
const d0 = require('default-allowed');
const d1 = require('namespace-forbidden');
const d2 = require('member-forbidden');
const { some, random, identifiers } = require('default-allowed');
const { some, random, identifiers } = require('namespace-forbidden');
const { some, random, identifiers } = require('member-forbidden');

export * from 'bare-forbidden';
import * as n1 from 'bare-forbidden';
const n2 = await import('bare-forbidden');
const n2b = ((await (import('bare-forbidden'))));
const n3 = import('bare-forbidden');
const n3b = (import('bare-forbidden'));
import('bare-forbidden').then(n4 => { });
import('bare-forbidden').then((n5) => { });
import('bare-forbidden').then(function (n6) { });
((import('bare-forbidden'))).then(((n6 => { })));
((import('bare-forbidden'))).then((((n7) => { })));
((import('bare-forbidden'))).then(((function (n8) { })));

export * from 'default-forbidden';
import * as n1 from 'default-forbidden';
const n2 = await import('default-forbidden');
const n2b = ((await (import('default-forbidden'))));
const n3 = import('default-forbidden');
const n3b = (import('default-forbidden'));
import('default-forbidden').then(n4 => { });
import('default-forbidden').then((n5) => { });
import('default-forbidden').then(function (n6) { });
((import('default-forbidden'))).then(((n6 => { })));
((import('default-forbidden'))).then((((n7) => { })));
((import('default-forbidden'))).then(((function (n8) { })));

export * from 'member-forbidden';
import * as n1 from 'member-forbidden';
const n2 = await import('member-forbidden');
const n2b = ((await (import('member-forbidden'))));
const n3 = import('member-forbidden');
const n3b = (import('member-forbidden'));
import('member-forbidden').then(n4 => { });
import('member-forbidden').then((n5) => { });
import('member-forbidden').then(function (n6) { });
((import('member-forbidden'))).then(((n6 => { })));
((import('member-forbidden'))).then((((n7) => { })));
((import('member-forbidden'))).then(((function (n8) { })));

export { allowed2, allowed3 as a3 } from 'bare-forbidden';
import { allowed2, allowed3 as a3 } from 'bare-forbidden';
const { allowed2, allowed3: a3, "allowed4": a4 } = await import('bare-forbidden');
import('bare-forbidden').then(({ allowed2, allowed3: a3, "allowed4": a4 }) => { })
import('bare-forbidden').then(function ({ allowed2, allowed3: a3, "allowed4": a4 }) { })

export { allowed2, allowed3 as a3 } from 'default-forbidden';
import { allowed2, allowed3 as a3 } from 'default-forbidden';
const { allowed2, allowed3: a3, "allowed4": a4 } = await import('default-forbidden');
import('default-forbidden').then(({ allowed2, allowed3: a3, "allowed4": a4 }) => { })
import('default-forbidden').then(function ({ allowed2, allowed3: a3, "allowed4": a4 }) { })

export { allowed2, allowed3 as a3 } from 'namespace-forbidden';
import { allowed2, allowed3 as a3 } from 'namespace-forbidden';
const { allowed2, allowed3: a3, "allowed4": a4 } = await import('namespace-forbidden');
import('namespace-forbidden').then(({ allowed2, allowed3: a3, "allowed4": a4 }) => { })
import('namespace-forbidden').then(function ({ allowed2, allowed3: a3, "allowed4": a4 }) { })
