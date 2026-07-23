/* should generate diagnostics */

// Simple
const invalid1 = require('is-number');

import invalid2 from 'is-number';

const invalid3 = await import('is-number');

// Native
import invalid4 from 'object.entries';

// Documented
import invalid5 from 'npm-run-all';

// Removal
import invalid6 from 'sort-object';
