/* should not generate diagnostics */
const Component1 = ({ isMobile, decorations }) => {
	return <div>{isMobile ? null : decorations}</div>;
};

const Component2 = ({ state }) => {
	return <div>{state === 'WAIT' ? 'Move to WAIT' : state}</div>;
};

const Component3 = ({ isPending, loader, userResults }) => {
	return <div>{isPending ? loader : userResults}</div>;
};

const Component4 = ({ condition, fallback }) => {
	return <div>{condition ? <Content /> : (fallback)}</div>;
};
