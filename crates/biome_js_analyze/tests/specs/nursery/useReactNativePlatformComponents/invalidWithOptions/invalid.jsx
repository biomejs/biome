/* should generate diagnostics */

// .android.jsx no longer matches the custom android pattern .droid.jsx
import { ProgressBarAndroid } from "react-native";

// .ios.jsx no longer matches the custom iOS pattern .apple.jsx
import { ActivityIndicatorIOS } from "react-native";

// require() also affected by custom options
const { ProgressBarAndroid: Bar } = require("react-native");
const { ActivityIndicatorIOS: Baz } = require("react-native");
