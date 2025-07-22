/* should not generate diagnostics */
function jsxExpressionChild() {
    return <>{foo}</>
}

function jsxExpressionChildFragment() {
	return <Fragment>{foo}</Fragment>
}

function jsxExpressionChildReactFragment() {
	return <React.Fragment>{foo}</React.Fragment>
}

