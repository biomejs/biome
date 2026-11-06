/* should not generate diagnostics */
const { PlatformColor, DynamicColorIOS } = require('react-native');
const ReactNative = require('react-native');
const Other = require('other-colors');

const destructured = DynamicColorIOS({light: PlatformColor('labelColor'), dark: 'white'});
const member = ReactNative.PlatformColor('labelColor');
const otherPackage = Other.PlatformColor(labelColor);
