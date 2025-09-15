// should generate diagnostics
<>
  <div>test</div>
	<>test</>
	<>
		<div>
			asdjfl
			test
			foo
		</div>
	</>
</>


class Comp1 extends Component {
	render() {
		const varObjectTest = { testKey : (<div>test</div>) };
		return varObjectTest.testKey;
	}
}
