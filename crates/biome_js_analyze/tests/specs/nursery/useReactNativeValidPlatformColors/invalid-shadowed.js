/* should generate diagnostics */
import { DynamicColorIOS } from 'react-native';
import { PlatformColor } from './theme';

const color = DynamicColorIOS({light: PlatformColor('labelColor'), dark: 'white'});
