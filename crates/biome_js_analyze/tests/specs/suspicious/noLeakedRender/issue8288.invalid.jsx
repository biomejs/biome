/* should generate diagnostics */
const Component1 = ({ count }) => {
	return (
		<Nested>
			<SecondNested>{count && <NestedChild />} </SecondNested>
		</Nested>
	);
};

const Component2 = () => {
	const [count, setCount] = useState(0);
	return <div>{count && <Child />}</div>;
};

