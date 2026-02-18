/* should not generate diagnostics */
<>
	<div>allowed</div>
	<>allowed</>
</>


class Comp1 extends Component {
	render() {
		const varObjectTest = { testKey : (<div>allowed</div>) };
		return varObjectTest.testKey;
	}
}
