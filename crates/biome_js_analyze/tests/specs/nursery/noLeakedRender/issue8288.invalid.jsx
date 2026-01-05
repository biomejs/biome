/* should generate diagnostics */
const Component1 = () => {
	return (
		<Nested>
			<SecondNested>{userId ? 1 : undefined} </SecondNested>
		</Nested>
	);
};
