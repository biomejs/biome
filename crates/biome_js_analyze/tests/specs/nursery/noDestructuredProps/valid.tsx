let Component = (props) => <div />;

let Component = (props) => <div />;

let Component = (props) => {
	return <div />;
};

let Component = (props) => <div />;

let Component = (props) => null;

let Component = (props) => <div a={props.a} />;

let Component = (props) => {
	const [local, rest] = splitProps(props, ["a"]);
	return <div a={local.a} b={rest.b} />;
};

let Component = (props) => {
	const { a } = someFunction();
	return <div a={a} />;
};

let NotAComponent = ({ a }, more, params) => <div a={a} />;

let Component = (props) => {
	let inner = ({ a, ...rest }) => a;
	let a = inner({ a: 5 });
	return <div a={a} />;
};

// This one might be surprising, since we're clearly destructuring props!
// But this will be caught as a reactive expression use outside of
// a tracked scope, in the "solid/reactivity" rule. There's really
// nothing wrong with destructuring props in tracked scopes when done
// correctly, but catching it in the params covers the most common
// cases with good DX.
let Component = (props) => {
	let { a } = props;
	return <div a={a} />;
};

let element = <div />;

let Component = (props: Props) => <div />;
