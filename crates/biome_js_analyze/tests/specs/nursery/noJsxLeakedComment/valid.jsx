/* should not generate diagnostics */

const Valid = () => {
	return (
		<>
			<App foo='test'>{/* valid */}</App>
			<strong>&nbsp;https://www.eslint-react.xyz/attachment/download/1</strong>
			<App /* valid */ placeholder={'foo'} />
			< /* valid */></>
			<div>https://example.com</div>
			<div>http://localhost:3000</div>
			<div>Check https://example.com</div>
			<div>path/to//file</div>
			<div>a//b</div>
		</>
	)
}
