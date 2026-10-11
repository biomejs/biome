/* should generate diagnostics */
const noArguments = PlatformColor();

const labelColor = 'labelColor';
const variableArgument = PlatformColor(labelColor);

const templateArgument = PlatformColor(`labelColor`);

const spreadArgument = PlatformColor(...names);

const raw = '#FF0000';
const variableFallback = PlatformColor('labelColor', {fallback: raw});

const fallbackNotLast = PlatformColor({fallback: '#FF0000'}, 'labelColor');

const duplicateFallback = PlatformColor('labelColor', {fallback: '#FF0000', fallback: '#00FF00'});

const extraFallbackMember = PlatformColor('labelColor', {fallback: '#FF0000', extra: 'red'});

const computedFallback = PlatformColor('labelColor', {['fallback']: '#FF0000'});

const stringFallbackKey = PlatformColor('labelColor', {'fallback': '#FF0000'});

const shorthandFallback = PlatformColor('labelColor', {fallback});

const tuple = {light: 'black', dark: 'white'};
const variableObject = DynamicColorIOS(tuple);

const noObject = DynamicColorIOS();

const twoObjects = DynamicColorIOS({light: 'black'}, {dark: 'white'});

const black = 'black';
const variableLight = DynamicColorIOS({light: black, dark: 'white'});

const white = 'white';
const variableDark = DynamicColorIOS({light: 'black', dark: white});

const bothVariables = DynamicColorIOS({light: black, dark: white});

const shorthandValue = DynamicColorIOS({light, dark: 'white'});

const spreadValue = DynamicColorIOS({...colors});

const otherCall = DynamicColorIOS({light: getColor(), dark: 'white'});

const methodValue = DynamicColorIOS({light() { return 'black'; }, dark: 'white'});

const getterValue = DynamicColorIOS({get dark() { return 'white'; }, light: 'black'});
