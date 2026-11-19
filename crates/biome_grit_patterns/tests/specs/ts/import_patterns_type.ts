import type a from "foo";
import type { b, c } from "foo";
import type * as d from "foo";

import a from "foo";
import { b, type c } from "foo";
import * as d from "foo";
import type * as e from "other";
import source f from "foo";
import defer * as g from "foo";
