import eslint from 'eslint';
const l = require('lodash');

import 'bare-forbidden';
import 'namespace-allowed';
import 'default-allowed';
import 'default-and-member-allowed';

export * from 'namespace-forbidden';
import * as n1 from 'namespace-forbidden';
const n2 = await import('namespace-forbidden');
const n2b = ((await (import('namespace-forbidden'))));
const n3 = import('namespace-forbidden');
const n3b = (import('namespace-forbidden'));
import('namespace-forbidden').then(n4 => { });
import('namespace-forbidden').then((n5) => { });
import('namespace-forbidden').then(function (n6) { });
((import('namespace-forbidden'))).then(((n6 => { })));
((import('namespace-forbidden'))).then((((n7) => { })));
((import('namespace-forbidden'))).then(((function (n8) { })));

import d0 from 'default-forbidden';
export { default as d1 } from 'default-forbidden';
import { default as d1 } from 'default-forbidden';
const { "default": d2 } = await import('default-forbidden');
const { default: d3 } = await import('default-forbidden');
import { default as d4, allowed1 } from 'default-forbidden';
const { allowed2, default: d5 } = await import('default-forbidden');
const { allowed3, default: d6 } = import('default-forbidden');
const { "allowed4": a1, "default": d7 } = import('default-forbidden');

export { allowed5, forbidden1 } from 'member-forbidden';
import { allowed5, forbidden1 } from 'member-forbidden';
const { allowed6, forbidden2, forbidden3: f1, "forbidden4": f2 } = await import('member-forbidden');
import('member-forbidden').then(({ allowed6, forbidden2, forbidden3: f1, "forbidden4": f2 }) => { })
import('member-forbidden').then(function ({ allowed6, forbidden2, forbidden3: f1, "forbidden4": f2 }) { })

// require() returns the default import, not a namespace object,
// so this import corresponds to default.allowed1 (and not namespace.allowed1)
// and is therefore *NOT* allowed.
const { allowed1 } = require('default-forbidden');
const { allowed2 } = require('namespace-allowed');
const { allowed3 } = require('member-allowed');

export * from 'default-allowed';
import * as n1 from 'default-allowed';
const n2 = await import('default-allowed');
const n2b = ((await (import('default-allowed'))));
const n3 = import('default-allowed');
const n3b = (import('default-allowed'));
import('default-allowed').then(n4 => { });
import('default-allowed').then((n5) => { });
import('default-allowed').then(function (n6) { });
((import('default-allowed'))).then(((n6 => { })));
((import('default-allowed'))).then((((n7) => { })));
((import('default-allowed'))).then(((function (n8) { })));

export * from 'member-allowed';
import * as n1 from 'member-allowed';
const n2 = await import('member-allowed');
const n2b = ((await (import('member-allowed'))));
const n3 = import('member-allowed');
const n3b = (import('member-allowed'));
import('member-allowed').then(n4 => { });
import('member-allowed').then((n5) => { });
import('member-allowed').then(function (n6) { });
((import('member-allowed'))).then(((n6 => { })));
((import('member-allowed'))).then((((n7) => { })));
((import('member-allowed'))).then(((function (n8) { })));

export * from 'bare-allowed';
import * as n1 from 'bare-allowed';
const n2 = await import('bare-allowed');
const n2b = ((await (import('bare-allowed'))));
const n3 = import('bare-allowed');
const n3b = (import('bare-allowed'));
import('bare-allowed').then(n4 => { });
import('bare-allowed').then((n5) => { });
import('bare-allowed').then(function (n6) { });
((import('bare-allowed'))).then(((n6 => { })));
((import('bare-allowed'))).then((((n7) => { })));
((import('bare-allowed'))).then(((function (n8) { })));

import d0 from 'namespace-allowed';
export { default as d1 } from 'namespace-allowed';
import { default as d1 } from 'namespace-allowed';
const { "default": d2 } = await import('namespace-allowed');
const { default: d3 } = await import('namespace-allowed');
import { default as d4, forbidden1 } from 'namespace-allowed';
const { forbidden2, default: d5 } = await import('namespace-allowed');
const { forbidden3, default: d6 } = import('namespace-allowed');
const { "forbidden4": f4, "default": d7 } = import('namespace-allowed');

import d0 from 'member-allowed';
export { default as d1 } from 'member-allowed';
import { default as d1 } from 'member-allowed';
const { "default": d2 } = await import('member-allowed');
const { default: d3 } = await import('member-allowed');
import { default as d4, forbidden1 } from 'member-allowed';
const { forbidden2, default: d5 } = await import('member-allowed');
const { forbidden3, default: d6 } = import('member-allowed');
const { "forbidden4": f4, "default": d7 } = import('member-allowed');

import d0 from 'bare-allowed';
export { default as d1 } from 'bare-allowed';
import { default as d1 } from 'bare-allowed';
const { "default": d2 } = await import('bare-allowed');
const { default: d3 } = await import('bare-allowed');
export { default as d4, forbidden1 } from 'bare-allowed';
import { default as d4, forbidden1 } from 'bare-allowed';
const { forbidden2, default: d5 } = await import('bare-allowed');
const { forbidden3, default: d6 } = import('bare-allowed');
const { "forbidden4": f4, "default": d7 } = import('bare-allowed');

export { forbidden1 } from 'default-allowed';
import { forbidden1 } from 'default-allowed';
const { forbidden2, forbidden3: f1, "forbidden4": f2 } = await import('default-allowed');
import('default-allowed').then(({ forbidden2, forbidden3: f1, "forbidden4": f2 }) => { })
import('default-allowed').then(function ({ forbidden2, forbidden3: f1, "forbidden4": f2 }) { })

export { forbidden1 } from 'namespace-allowed';
import { forbidden1 } from 'namespace-allowed';
const { forbidden2, forbidden3: f1, "forbidden4": f2 } = await import('namespace-allowed');
import('namespace-allowed').then(({ forbidden2, forbidden3: f1, "forbidden4": f2 }) => { })
import('namespace-allowed').then(function ({ forbidden2, forbidden3: f1, "forbidden4": f2 }) { })

export { forbidden1 } from 'bare-allowed';
import { forbidden1 } from 'bare-allowed';
const { forbidden2, forbidden3: f1, "forbidden4": f2 } = await import('bare-allowed');
import('bare-allowed').then(({ forbidden2, forbidden3: f1, "forbidden4": f2 }) => { })
import('bare-allowed').then(function ({ forbidden2, forbidden3: f1, "forbidden4": f2 }) { })
