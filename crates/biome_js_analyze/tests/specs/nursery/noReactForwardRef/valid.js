/* should not generate diagnostics */

const Component1 = ({ ref }) => {
	return null;
};

const Component2 = ({ ref, ...props }) => {
	return null;
};

const Component3 = (props) => {
	return null;
};

const Component4 = ({ foo, bar }) => {
	return null;
};
