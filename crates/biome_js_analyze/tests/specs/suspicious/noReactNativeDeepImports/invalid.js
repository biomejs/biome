/* should generate diagnostics */

// ESM default import from internal path
import View from "react-native/Libraries/Components/View/View";

// ESM named import from internal path
import { something } from "react-native/Libraries/Utilities/Platform";

// ESM type import from internal path
import type { RootTag } from "react-native/Libraries/Types/RootTagTypes";

// CJS require from internal path
const Platform = require("react-native/Libraries/Utilities/Platform");

// CJS destructured require from internal path
const { View: V } = require("react-native/Libraries/Components/View/View");

// Dynamic import from internal path
import("react-native/Libraries/Utilities/Platform");

// Deep import from src directory
import Foo from "react-native/src/private/specs/modules/NativeAppearance";

// Side-effect import of internal path
import "react-native/Libraries/Core/InitializeCore";

// Namespace import from internal path
import * as Internals from "react-native/Libraries/Renderer/shims/ReactNativeViewConfigRegistry";

// Single-level deep import
import something from "react-native/index";
