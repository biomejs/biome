/* should not generate diagnostics */
const valid1 = 303;

import valid2 from 'unknown-module';

const valid3 = require('unknown-module');

const moduleName = 'is-' + 'number';
require(moduleName);

await import(moduleName);
