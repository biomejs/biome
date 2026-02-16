import { forwardRef, memo } from "react";

const Component1 = forwardRef((props, ref) => {
	return null;
});

const Component2 = forwardRef((props, ref) => null);

const Component3 = forwardRef(function (props, ref) {
	return null;
});

const Component4 = forwardRef(function Component(props, ref) {
	return null;
});

const Component5 = memo(forwardRef(function Component(props, ref) {
	return <div ref={ref} />;
}));
