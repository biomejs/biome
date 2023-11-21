import type {
	a,
	b,
	c,
} from "D";

import type {
	adsadasdasdasdasdasdasdasdasdasdas,
	d,
	e, } from "W";

import type TheDefault from 'foo';


// Single named alias import
import type {foo as bar   } from "foo";
// Multiple named imports
import type {   foo as baz,   aaa} from "foo";

// Type imports within specifier list
import {foo, type Foo} from 'foo';
import {type Bar} from 'bar';
import {bar, type Bar as Baz} from 'baz';