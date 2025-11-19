/* should not generate diagnostics */

const Component1 = ({ elements, count }) => {
	return <div>{!!count && <List elements={elements} />}</div>;
};

const Component2 = ({ elements, count }) => {
	return (
		<div>
			<div> {direction ? (direction === 'down' ? '▼' : '▲') : ''} </div>
			<div>{containerName.length > 0 ? 'Loading several stuff' : 'Loading'}</div>
		</div>
	);
};

const Component3 = ({ direction }) => {
	return (
		<div>
			<div>{!!direction && direction === 'down' && '▼'}</div>
			<div>{direction === 'down' && !!direction && '▼'}</div>
			<div>{direction === 'down' || (!!direction && '▼')}</div>
			<div>{(!display || display === DISPLAY.WELCOME) && <span>foo</span>}</div>
		</div>
	);
};

const Component4 = ({ elements, count }) => {
	return <div>{count ? <List elements={elements} /> : <EmptyList />}</div>;
};

const isOpen1 = true;
const Component5 = () => {
	return <Popover open={isOpen1 && items.length > 0} />;
};

const isOpen2 = false;
const Component6 = () => {
	return <Popover open={isOpen2 && items.length > 0} />;
};
