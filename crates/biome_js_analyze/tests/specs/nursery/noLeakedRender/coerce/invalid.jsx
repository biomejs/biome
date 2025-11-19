// Invalid cases with validStrategies: ['coerce']

const Component1 = ({ count, title }) => {
	return <div>{count && title}</div>;
};

const Component2 = ({ count }) => {
	return <div>{count && <span>There are {count} results</span>}</div>;
};

const Component3 = ({ elements }) => {
	return <div>{elements.length && <List elements={elements} />}</div>;
};

const Component4 = ({ nestedCollection }) => {
	return (
		<div>{nestedCollection.elements.length && <List elements={nestedCollection.elements} />}</div>
	);
};

const Component5 = ({ elements }) => {
	return <div>{elements[0] && <List elements={elements} />}</div>;
};

const Component6 = ({ numberA, numberB }) => {
	return <div>{(numberA || numberB) && <Results>{numberA + numberB}</Results>}</div>;
};

const Component7 = ({ connection, hasError, hasErrorUpdate }) => {
	return <div>{connection && (hasError || hasErrorUpdate)}</div>;
};

// Ternary isn't valid if strategy is only "coerce"
const Component8 = ({ count, title }) => {
	return <div>{count ? title : null}</div>;
};

const Component9 = ({ count, title }) => {
	return <div>{!count ? title : null}</div>;
};

const Component10 = ({ count, somethingElse, title }) => {
	return <div>{count && somethingElse ? title : null}</div>;
};

const Component11 = ({ items, somethingElse, title }) => {
	return <div>{items.length > 0 && somethingElse && title}</div>;
};

const MyComponent2 = () => {
	return <div>{maybeObject && (isFoo ? <Aaa /> : <Bbb />)}</div>;
};

const MyComponent3 = () => {
	return <Something checked={isIndeterminate ? false : isChecked} />;
};

const MyComponent4 = () => {
	return <Something checked={cond && isIndeterminate ? false : isChecked} />;
};

const MyComponent5 = () => {
	return (
		<>
			{someCondition && (
				<div>
					<p>hello</p>
				</div>
			)}
		</>
	);
};

const MyComponent6 = () => {
	return <>{someCondition && <SomeComponent prop1={val1} prop2={val2} />}</>;
};

const isOpen = 0;
const Component12 = () => {
	return <Popover open={isOpen && items.length > 0} />;
};
