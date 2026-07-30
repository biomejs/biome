/* should generate diagnostics */
const Component1 = ({ userId }) => {
	return <div>{userId ? 1 : undefined}</div>;
};

const Component2 = ({ userId }) => {
	return <div>{userId ? 1 : (undefined)}</div>;
};
