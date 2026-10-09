/* should not generate diagnostics */
const Valid = () => {
	return (
		<>
			<testcomponent />
			<testComponent />
			<test_component />
			<TestComponent />
			<object.testcomponent />
			<object.testComponent />
			<object.test_component />
			<object.TestComponent />
			<Object.testcomponent />
			<Object.testComponent />
			<Object.test_component />
			<Object.TestComponent />
		</>
	)
}
