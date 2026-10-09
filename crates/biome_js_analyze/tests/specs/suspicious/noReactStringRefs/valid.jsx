/* should not generate diagnostics */
import React, { createRef, useRef } from "react";

class CallbackRefs extends React.Component {
	input = createRef();

	render() {
		return (
			<section>
				<div ref={(node) => {
					this.hello = node;
				}}>
					Hello
				</div>
				<input ref={this.input} />
			</section>
		);
	}
}

function HookRef() {
	const input = useRef(null);
	return <input ref={input} />;
}

class NotAReactComponent {
	method() {
		return this.refs.hello;
	}
}
