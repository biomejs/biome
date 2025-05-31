function Foo(props) {
	props.bar = `Hello ${props.bar}`;

	return <div>{props.bar}</div>;
}

export function Foo(props) {
	props.bar = `Hello ${props.bar}`;

	return <div>{props.bar}</div>;
}

const Foo = (props) => {
	props.bar = `Hello ${props.bar}`;

	return <div>{props.bar}</div>;
}

const Foo = (props) => {
	const baz = props.baz;

	useEffect(() => {
		props.bar = `Hello ${props.bar}`;
	}, [props.bar]);

	props.bar = `Hello ${props.bar}`;
	return <div>{props.bar}</div>;
}

const Foo = memo((props) => {
	props.bar = `Hello ${props.bar}`;
	return <div>{props.bar}</div>;
});

const Foo = forwardRef(function (props, ref) {
	props.bar = `Hello ${props.bar}`;
	return <div>{props.bar}</div>;
});

const Foo = (props) => {
	const baz = props.baz;

	useEffect(() => {
		props.bar = `Hello ${props.bar}`;
	}, [props.bar]);

	props.bar = `Hello ${props.bar}`;
	props.baz = `Hello ${props.baz}`;
	return <div>{props.bar}</div>
};

function Foo(props) {
	const callback = (props) => {
		props.bar = 1; // this should not trigger the rule
	};

	return <div>{props.bar}</div>;
}

