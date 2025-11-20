/* should not generate diagnostics */
import { NOT_REALLY_DEPRECATED_CONSTANT as CONSTANT } from "./utils";

import { modernUtil } from "./utils";

// Namespace imports are not flagged even if they contain deprecated symbols
// (known limitation).
import * as utilsNs from "./utils";

import { Component } from "./component";
