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

const prop = "allow-downloads allow-scripts allow-forms"
const Valid2 = () => <iframe sandbox={prop} />;

const props = {
	sandbox: "allow-downloads",
};
const Valid3 = () => <iframe {...props} />;
