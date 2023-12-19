import SomeDefaultClass from './mod'

// oops, some other import separated these lines
import foo from './some-other-mod'

import * as names from './mod'

// TODO(@anonrig): Implement this
// will catch this too, assuming it is the same target module
// import { something } from './mod.js'
