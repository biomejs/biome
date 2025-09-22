/* should not generate diagnostics */

const Component1 = ({ ref, ...props }) => {
	return <div ref={ref} />;
};

const Component2 = ({ foo, bar, ref }) => {
	return <div ref={ref} />;
};

const Component3 = React.memo(({ ref, ...props }) => {
	return <div ref={ref} />;
});

function Component4({ ref, ...props }) {
	return <div ref={ref} />;
}
