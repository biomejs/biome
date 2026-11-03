/* should generate diagnostics */
/// <reference path="./globals.d.ts" />
/// <reference path='../types/env.d.ts'/>
///<reference path="./no-space.d.ts" />
/// <reference types="lodash" />
/// <reference types="foo" resolution-mode="import" />
/// <reference resolution-mode="require" types="bar" />
/// <reference types="baz" />
/// <reference types="qux" />
import { debounce } from "lodash";
import type { Foo } from "foo";
import "bar";
import baz = require("baz");
import * as qux from "qux";
import * as quxAgain from "qux";
