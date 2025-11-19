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

// Boolean coerce isn't valid if strategy is only "ternary"
const Component7 = ({ someCondition, title }) => {
	return <div>{!someCondition && title}</div>;
};

const Component8 = ({ count, title }) => {
	return <div>{!!count && title}</div>;
};

const Component9 = ({ count, title }) => {
	return <div>{count > 0 && title}</div>;
};

const Component10 = ({ count, title }) => {
	return <div>{0 != count && title}</div>;
};

const Component11 = ({ count, total, title }) => {
	return <div>{count < total && title}</div>;
};

const Component12 = ({ count, title, somethingElse }) => {
	return <div>{!!(count && somethingElse) && title}</div>;
};
