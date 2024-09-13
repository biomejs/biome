import "./sub/foo";
import "./sub/Foo";
import "./sub/Bar/";

// Guaranteed resolve to 'index.js' file
import './foo/../'
import './foo/..'
import './foo/.'
import './foo/./'
import './foo/'

import './../'
import './..'
import './.'
import './'
import '.'

import '../../'
import '../..'
import '../.'
import '../'
import '..'

import  /** A **/'./' /** B **/

// Query and hash
import './foo?worker'
import './foo#hash'
import './foo?query=string&query2#hash'

// Dynamic imports
import('./foo')
import( /** A **/'./foo'/** B **/ )
require("./foo")
