/* should generate diagnostics */
const Invalid1 = () => <>Hello ${user.name}</>
const Invalid2 = () => <>Hello $${user.name}</>
const Invalid3 = (props) => {
	return <div>Hello ${props.name}</div>;
};

const Invalid4 = (props) => {
	return <div>${props.name} is your name</div>;
};

const Invalid5 = (props) => {
	return <div>Hello ${props.name} is your name</div>;
};

function Invalid6({ count, total }) {
	return <div>Progress: ${count} / ${total}</div>;
}
