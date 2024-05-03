import "./sub/foo";
import "./sub/Foo";
import "./sub/Bar/";

// Guaranteed resolve to 'index.js' file
import './foo/..'
import './foo/../'
import './foo/.'
import './foo/./'
import './foo/'
import './..'
import  /** A **/'./' /** B **/

import('./foo')
import( /** A **/'./foo'/** B **/ )
require('./foo')