import * as React from "react";

interface ComponentProps {
	foo: string;
}

const Component1 = React.forwardRef<HTMLElement, ComponentProps>(function Component(props, ref) {
	return <div ref={ref} />;
});

const Component2 = React.forwardRef<HTMLElement, { foo: string }>(function Component(props, ref) {
	return <div ref={ref}>{props.foo}</div>;
});

const Component3 = React.forwardRef<HTMLElement, { foo: string }>(function Component({ foo }, ref) {
	return <div ref={ref}>{foo}</div>;
});

const Component4 = React.forwardRef<HTMLElement, { foo: string }>(function Component({ foo }, r) {
	return <div ref={r}>{foo}</div>;
});

const Component5 = React.forwardRef<HTMLElement, { foo: string, bar: number }>(function Component({ foo, ...rest }, r) {
	return <div ref={r}>{foo}</div>;
});
