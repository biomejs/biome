export const ModelSelector = () => {
	return (
		<Select
			onChange={({ value }) => {
				console.log(value)
			}}
		/>
	)
}
