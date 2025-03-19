let Component = ({}) => <div />;

let Component = ({ a }) => <div a={a} />;

let Component = ({ a }) => <div a={a} />;

let Component = ({ a: A }) => <div a={A} />;

let Component = ({ a: A }) => <div a={A} />;

let Component = ({ ["a" + ""]: a }) => <div a={a} />;

let Component = ({ ["a" + ""]: a, b }) => <div a={a} b={b} />;

let Component = ({ a = 5 }) => <div a={a} />;

let Component = ({ a = 5 }) => <div a={a} />;

let Component = ({ a: A = 5 }) => <div a={A} />;

let Component = ({ a: A = 5 }) => <div a={A} />;

let Component = ({ ["a" + ""]: a = 5 }) => <div a={a} />;

let Component = ({ ["a" + ""]: a = 5, b = 10, c }) => <div a={a} b={b} c={c} />;

let Component = ({ a = 5 }) => {
	return <div a={a} />;
};

let Component = ({ a = 5 }) => {
	various();
	statements();
	return <div a={a} />;
};

let Component = ({ ...rest }) => <div a={rest.a} />;

let Component = ({ a, ...rest }) => <div a={a} />;

let Component = ({ a, ...rest }) => <div a={a} />;

let Component = ({ a, ...other }) => <div a={a} />;

let Component = ({ a, ...rest }) => <div a={a} b={rest.b} />;

let Component = ({ a: A, ...rest }) => <div a={A} />;

let Component = ({ a: A, ...rest }) => <div a={A} />;

let Component = ({ ["a" + ""]: A, ...rest }) => <div a={A} />;

let Component = ({ ["a" + ""]: A, ...rest }) => <div a={A} b={rest.b} />;

let Component = ({ a = 5, ...rest }) => {
	return <div a={a} b={rest.b} />;
};

let Component = ({ a = 5, ...rest }) => <div a={a} b={rest.b} />;

let Component = ({ ["a" + ""]: A = 5, ...rest }) => <div a={A} b={rest.b} />;

let Component = ({ prop1, prop2 }: Props) => <div p1={prop1} p2={prop2} />;
