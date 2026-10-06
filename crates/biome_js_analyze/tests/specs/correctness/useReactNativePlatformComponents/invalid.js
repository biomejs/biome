/* should generate diagnostics */

// Android component in a non-android file
import { ProgressBarAndroid } from "react-native";

// iOS component in a non-ios file
import { ActivityIndicatorIOS } from "react-native";

// Mixed iOS and Android components
import { ActivityIndicatorIOS as Foo, ProgressBarAndroid } from "react-native";

// require() with destructuring
const { ProgressBarAndroid: Bar } = require("react-native");

const { ActivityIndicatorIOS: Baz } = require("react-native");
