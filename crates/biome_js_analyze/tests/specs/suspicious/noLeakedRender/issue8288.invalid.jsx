/* should generate diagnostics */
const Component1 = ({ count }) => {
	return (
		<Nested>
			<SecondNested>{count && <NestedChild />} </SecondNested>
		</Nested>
	);
};
