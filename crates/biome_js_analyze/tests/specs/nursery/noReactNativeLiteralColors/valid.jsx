/* should not generate diagnostics */

const red = '#f00';
const blue = '#00f';

const stylesFromVars = StyleSheet.create({
	title: { color: red },
	subtitle: { color: blue },
});

const Themed = () => <Text style={{ color: theme.primary }}>hello</Text>;

const ConditionalVars = ({ isDanger }) => {
	const trueColor = '#fff';
	const falseColor = '#000';
	return (
		<View
			style={[
				{ color: isDanger ? trueColor : falseColor },
				isDanger && { color: trueColor },
			]}
		/>
	);
};

const NonColorLiteral = StyleSheet.create({
	box: { fontFamily: 'Arial', padding: 10 },
});

const ShorthandProperty = ({ color }) => (
	<Text style={{ color }}>hello</Text>
);

const OutsideStyleContext = {
	backgroundColor: '#fff',
};

function paintBackground() {
	return { backgroundColor: '#fff' };
}

const NonStyleSheetCreate = MyThing.create({
	box: { backgroundColor: '#fff' },
});

const NonStyleAttribute = () => (
	<View data={{ backgroundColor: '#fff' }}>hello</View>
);
