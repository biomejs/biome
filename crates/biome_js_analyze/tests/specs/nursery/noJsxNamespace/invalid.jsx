/* should generate diagnostics */
const Invalid = () => {
	return (
		<>
			<ns:testcomponent />
			<ns:testComponent />
			<Ns:TestComponent />
			<ns:TestComponent></ns:TestComponent>
			<svg:circle cx="50" cy="50" r="40" />
		</>
	)
}
