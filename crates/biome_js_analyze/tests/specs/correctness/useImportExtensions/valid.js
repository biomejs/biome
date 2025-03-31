/* should not generate diagnostics */

// Not relative
import "sub/foo";
import "~/sub/foo";
import "@foo";
import "foo";
import('foo');
require('foo');

// Have extension
import './sub/bar/index.ts';
import './sub/foo.ts';
import './sub/index.js';
import('./sub/foo.ts');
require('./sub/foo.ts');

// If the import doesn't resolve at all, we don't report a diagnostic:
// It means the import is broken beyond missing an extension.
import './sub/baz';
import './sub/baz.css';
