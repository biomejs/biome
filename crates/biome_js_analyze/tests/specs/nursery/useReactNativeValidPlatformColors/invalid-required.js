/* should generate diagnostics */
const { PlatformColor, DynamicColorIOS } = require('react-native');
const ReactNative = require('react-native');

const labelColor = 'labelColor';
const destructured = PlatformColor(labelColor);
const destructuredDynamic = DynamicColorIOS(colors);
const member = ReactNative.PlatformColor(labelColor);
