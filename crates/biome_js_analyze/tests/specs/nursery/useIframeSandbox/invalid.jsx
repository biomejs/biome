/* should generate diagnostics */
const Invalid = () => (
	<>
		<iframe />
		<iframe sandbox />
		<iframe sandbox={undefined} />
		<iframe sandbox={null} />
	</>
);
