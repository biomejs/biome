/* should not generate diagnostics */

const Component1 = ({ elements, count }) => {
	return <div>{count ? <List elements={elements} /> : null}</div>;
};
