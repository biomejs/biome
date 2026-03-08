import * as React from "react";

const Component1 = React.forwardRef(function Component(props, ref) {
	return <div ref={ref} />;
});

const Component2 = React.forwardRef(function Component({ foo, bar }, ref) {
	return <div ref={ref} />;
});

const Component3 = React.memo(React.forwardRef(function Component(props, ref) {
	return <div ref={ref} />;
}));
