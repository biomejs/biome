import "./sub/foo";
import "./sub/bar/";

// Guaranteed resolve to 'index.js' file
import './sub/bar/../'
import './sub/bar/..'
import './sub/.'
import './sub/./'
import './sub/'
import './sub'

import  /** A **/'./sub' /** B **/

// Query and hash
import './sub?worker'
import './sub#hash'
import './sub?query=string&query2#hash'

// Dynamic imports
import('./sub/foo')
import( /** A **/'./sub/foo'/** B **/ )
require("./sub/foo")

import "./sub/styles.css"
import "./sub/component.svg.svelte";
import "./sub/component.svg.svelte?query=string&query2#hash";