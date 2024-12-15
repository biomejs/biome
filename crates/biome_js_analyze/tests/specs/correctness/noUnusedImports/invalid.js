// Header comment
import A from "mod";

// Header comment
import * as B from "mod"; // Import comment

// Header comment
import { C } from "mod"; // Import comment

// Header comment
import /*a*/ D /*b*/, /*c*/{ E }/*d*/ from "mod"; // Import comment

import /*a*/ F /*b*/, /*c*/ * as G /*d*/ from "mod";

import {
    // Comment
    H,
    I,
} from "mod";

import {/*a*/J/*b*/, /*c*/K/*d*/} from "mod";

// Header comment
import { L as M, } from "mod"; // Import comment

// See https://github.com/biomejs/biome/issues/653
import {a} from 'a'
import {d} from 'd'
import {b} from 'b'
export const bb = a + b

import {} from "mod"
