/* should not generate diagnostics */
const a = require('a') as typeof import('a');
const b = require('b') satisfies unknown;
const c = require('c')!;
const d = <any>require('d');
