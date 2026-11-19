/* should generate diagnostics */
import { PlatformColor, DynamicColorIOS } from 'react-native';
import * as ReactNative from 'react-native';

const labelColor = 'labelColor';
const named = PlatformColor(labelColor);
const namespace = ReactNative.PlatformColor(labelColor);
const dynamic = DynamicColorIOS(colors);
const namespaceDynamic = ReactNative.DynamicColorIOS({light: labelColor});

import RN from 'react-native';
const defaultImport = RN.PlatformColor(labelColor);
