<>
	<Foo
		onClick={function () {
			alert("1337");
		}}
	/>
	<Foo onClick={handleClick.bind(this)} />
	<Foo onClick={this._handleClick.bind(this)} />
	<Foo onClick={() => console.log("Hello!")} />
</>;

function Foo() {
	function onClick() {}
	return <Bar onClick={onClick}></Bar>;
}

function Foo() {
	const onClick = () => {};
	return <Bar onClick={onClick}></Bar>;
}

function Foo() {
	const onClick = function () {};
	return <Bar onClick={onClick}></Bar>;
}
