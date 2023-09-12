function BackTopContent(props){
	return (
		<CSSMotion>
			{({ className: motionClassName }) =>
				cloneElement(className => ({
					className
				}))}
		</CSSMotion>
	);
}

function BackTopContent(props){
	return (
		<CSSMotion>
			{({ className: motionClassName }) =>
				cloneElement(className => ({
					className
				}))/*with comment*/}
		</CSSMotion>
	);
}

function ArrowBodyIsJsxWithComment({ action }) {
	return (action) =>
		(
			// eslint-disable-next-line react/no-array-index-key
			<li/>
		);
}
