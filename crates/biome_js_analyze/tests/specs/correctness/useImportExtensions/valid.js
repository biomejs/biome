// Not relative
import "sub/foo";
import "~/sub/foo";
import "@foo";
import "foo";

// Have extension
import './foo.js'
import './foo.jsx'
import './foo.ts'
import './foo.tsx'
import './foo.json'
import './foo.css'
import './foo.html'
import './foo.vue'
import './foo.yaml'
import '../../foo.js'
import './index.ts'

import('./foo.js')
import('foo')
require('./foo.js')
require('foo')