function App() {
	const firstId = `${Math.random()}`
	const secondId = `${Math.random()}`

	React.useEffect(() => {
		console.log({firstId, secondId})
	}, [])
}
