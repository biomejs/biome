/* should not generate diagnostics */
const foo = 303;

import foo from 'unknown-module';

const foo = require('unknown-module');

const moduleName = 'is-' + 'number';
require(moduleName);

const moduleName = 'is-' + 'number';
await import(moduleName);
