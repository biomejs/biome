// Header comment
import type A from "mod";

// Header comment
import type * as B from "mod"; // Import comment

// Header comment
import type { C } from "mod"; // Import comment

// Orphan comment

// Header comment
import /*a*/ D /*b*/, /*c*/{ type E }/*d*/ from "mod"; // Import comment

import /*a*/ F /*b*/, /*c*/ * as G /*d*/ from "mod";

import {
    // Comment
    type H,
    type I,
} from "mod";

import {/*a*/type J/*b*/, /*c*/type K/*d*/} from "mod";

// Header comment
import type { L as M, } from "mod"; // Import comment

