/* should not generate diagnostics */
const Valid1 = () => (
	<>
		<a />
		<span />
		<button type="button">Click me</button>
		<iframe sandbox="" />
		<iframe sandbox="allow-downloads" />
		<iframe sandbox="allow-downloads allow-scripts" />
		<iframe sandbox="allow-downloads allow-scripts allow-forms" />
	</>
);

const props = {
	sandbox: "allow-downloads",
};

const Invalid2 = () => {
	return <iframe {...props} />;
}
