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

// Have sub extension
import "./sub/styles.css.ts"
import "./sub/component.svg.svelte.ts";
import "./sub/component.svg.svelte.ts?query=string&query2#hash";

// If the import doesn't resolve at all, we don't report a diagnostic:
// It means the import is broken beyond missing an extension.
import './sub/baz';
import './sub/baz.css';
import './sub/baz.css.com';

import "./sub/generated/index.js";
