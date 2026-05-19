/* should generate diagnostics */
import React from "react";

class StringRefs extends React.Component {
	componentDidMount() {
		this.refs.hello.focus();
		this.refs["world"]?.focus();
	}

	render() {
		return (
			<section>
				<div ref="hello">Hello</div>
				<div ref={"world"}>World</div>
				<div ref={`template`}>Template</div>
				<div ref={`template-${id}`}>Template Expression</div>
			</section>
		);
	}
}
