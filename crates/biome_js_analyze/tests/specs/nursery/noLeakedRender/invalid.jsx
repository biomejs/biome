// should generate diagnostics

const Example1 = () => {
	return (
		<>
			{0 && <Something />}
			{'' && <Something />}
			{NaN && <Something />}
		</>
	);
};

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

const MyComponent1 = () => {
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

const MyComponent2 = () => {
	return <>{someCondition && <SomeComponent prop1={val1} prop2={val2} />}</>;
};

const MyComponent3 = () => {
	return <div>{maybeObject && (isFoo ? <Aaa /> : <Bbb />)}</div>;
};

const MyComponent4 = ({ count, title }) => {
	return <div>{(((((count))))) && ((title))}</div>;
};

const MyComponent5 = ({ data }) => {
	return <div>{(((((data)))) && (((((data.value))))))}</div>;
}

const MyComponent6 = ({ value }) => {
	return <div>{(((value))) && <Item />}</div>;
};
