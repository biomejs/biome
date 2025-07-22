/* should not generate diagnostics */

export function Foo(props) {
	return <div>{props.bar}</div>
}

function Foo(props) {
	return <div>{props.bar}</div>;
}

memo(function Foo(props) {
	return <div>{props.bar}</div>;
});

forwardRef(function Foo(props, ref) {
	return <div>{props.bar}</div>;
});

function Foo({bar, baz}) {
	bar = `Hello ${bar}`;
	return <div>{bar}</div>;
}

function Foo({bar}) {
	bar = `Hello ${bar}`;
	return <div>{bar}</div>;
}

memo(function Foo({bar}) {
	bar = `Hello ${bar}`;
	return <div>{bar}</div>;
});

forwardRef(function Foo({bar}, ref) {
	bar = `Hello ${bar}`;
	return <div>{bar}</div>;
});

const Foo = function ({bar}) {
	bar = `Hello ${bar}`;
	return <div>{bar}</div>;
}

const Foo = memo((props) => {
	return <div>{props.bar}</div>;
});

const Foo = memo(({bar}) => {
	bar = `Hello ${bar}`;
	return <div>{bar}</div>;
});

const Foo = forwardRef(function (props, ref) {
	return <div>{props.bar}</div>;
});

const Foo = forwardRef((props, ref) => {
	return <div>{props.bar}</div>;
});

function Foo(props) {
	props = somethingElse;
	props.bar = 1;
	return <div>{props.bar}</div>;
}

function Foo(props) {
	const callback = (props) => {
		props.bar = 1;
	};

	return <div>{props.bar}</div>;
}

