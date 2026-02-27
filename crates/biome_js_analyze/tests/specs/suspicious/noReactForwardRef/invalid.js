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
