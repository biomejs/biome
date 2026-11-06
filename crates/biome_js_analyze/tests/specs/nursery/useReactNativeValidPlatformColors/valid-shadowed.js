/* should not generate diagnostics */
import { PlatformColor, DynamicColorIOS } from 'my-colors';

const labelColor = 'labelColor';
const imported = PlatformColor(labelColor);
const importedDynamic = DynamicColorIOS(colors);

function local(PlatformColor) {
	return PlatformColor(labelColor);
}
