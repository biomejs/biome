/* should not generate diagnostics */
import React from "react";

class StringRefs extends React.Component {
	componentDidMount() {
		this.refs.hello?.focus();
	}
}
