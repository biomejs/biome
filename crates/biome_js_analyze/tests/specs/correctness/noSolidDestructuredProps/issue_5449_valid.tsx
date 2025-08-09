/* should not generate diagnostics */
export const ModelSelector = () => {
	return (
		<Select
			onChange={({ value }) => {
				console.log(value)
			}}
		/>
	)
}
