import target from "foo";
import { target } from "foo";
import { other, target } from "foo";
import * as target from "foo";
import type { target } from "foo";
import type * as target from "foo";
import { other as target, type T } from "foo";
import {
  other,
  type target,
} from "foo";
import type target from "foo";

import * as unrelated from "foo";
import * as target from "other";
import "foo";
