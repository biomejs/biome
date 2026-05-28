/* should generate diagnostics */

// Simple
const foo = require('is-number');

import foo from 'is-number';

const foo = await import('is-number');

// Native
import foo from 'object.entries';

// Documented
import foo from 'npm-run-all';

// Removal
import foo from 'sort-object';
