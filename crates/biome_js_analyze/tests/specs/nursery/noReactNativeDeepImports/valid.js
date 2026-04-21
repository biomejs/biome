/* should not generate diagnostics */

// Top-level react-native import (ESM)
import { View } from "react-native";

// Top-level react-native import (CJS)
const { Platform } = require("react-native");

// Different package with react-native prefix
import Foo from "react-native-foo";

// Different package with react-native prefix and subpath
import Bar from "react-native-foo/Bar";

// Unrelated package
import React from "react";

// Unrelated package with subpath
import { something } from "react/jsx-runtime";

// Dynamic import of top-level react-native
import("react-native");

// Default import from react-native
import RN from "react-native";

// Namespace import from react-native
import * as RN from "react-native";

// Type import from react-native
import type { ViewProps } from "react-native";

// Side-effect import of react-native
import "react-native";

// Require of top-level react-native
const RN2 = require("react-native");
