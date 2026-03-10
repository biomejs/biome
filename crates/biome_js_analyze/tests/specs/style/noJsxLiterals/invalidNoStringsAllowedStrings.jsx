// should generate diagnostics
<>
	<div>allowed</div>
	<div>allowed   </div>
	<div>{"allowed   "}</div>
	<>test</>
</>


class Comp1 extends Component {
	render() {
		const varObjectTest = { testKey : (<div>test</div>) };
		return varObjectTest.testKey;
	}
}
