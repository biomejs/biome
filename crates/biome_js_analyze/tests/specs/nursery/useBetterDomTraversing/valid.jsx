/* should not generate diagnostics */
class Component {
	render() {
		return <div>{this.props.children[0]}</div>;
	}
}

const Arrow = (props) => <div>{props.children[0]}</div>;
