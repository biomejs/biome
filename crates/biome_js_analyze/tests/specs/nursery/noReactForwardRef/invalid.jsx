import * as React from "react";

const Component1 = React.forwardRef((props, ref) => {
	return null;
});

const Component2 = React.forwardRef((props, ref) => null);

const Component3 = React.forwardRef(function (props, ref) {
	return null;
});

const Component4 = React.forwardRef(function Component(props) {
	return null;
});

const Component5 = React.forwardRef(function Component(props, ref) {
	return <div ref={ref} />;
});

const Component6 = React.forwardRef(function Component({ foo, bar }, ref) {
	return <div ref={ref} />;
});

const Component7 = React.memo(React.forwardRef(function Component(props, ref) {
	return <div ref={ref} />;
}));
