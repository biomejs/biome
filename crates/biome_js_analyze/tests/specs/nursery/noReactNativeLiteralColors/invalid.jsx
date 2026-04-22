/* should generate diagnostics */

const Inline = () => <Text style={{ backgroundColor: '#FFFFFF', opacity: 0.5 }}>hello</Text>;

const stylesBasic = StyleSheet.create({
	text: { fontColor: '#000' },
});

const MultipleInSheet = StyleSheet.create({
	primary: { color: 'red' },
	secondary: { borderBottomColor: 'blue' },
});

const InArray = () => (
	<Text style={[styles.text, { backgroundColor: '#FFFFFF' }]}>hello</Text>
);

const InLogical = ({ active }) => (
	<Text style={[styles.text, active && { backgroundColor: '#FFFFFF' }]}>hello</Text>
);

const TernaryBothLiterals = ({ active }) => (
	<Text style={{ backgroundColor: active ? '#fff' : '#000' }}>hello</Text>
);

const TernaryOneLiteral = ({ active }) => (
	<Text style={{ backgroundColor: active ? '#fff' : theme.background }}>hello</Text>
);

const CustomStyleAttribute = () => (
	<Text contentContainerStyle={{ color: 'red' }}>hello</Text>
);
