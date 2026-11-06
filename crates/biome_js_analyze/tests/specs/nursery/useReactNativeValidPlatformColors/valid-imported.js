/* should not generate diagnostics */
import { PlatformColor, DynamicColorIOS } from 'react-native';
import * as ReactNative from 'react-native';

const named = PlatformColor('labelColor');
const namespace = ReactNative.PlatformColor('labelColor');
const dynamic = DynamicColorIOS({light: ReactNative.PlatformColor('labelColor'), dark: 'white'});
