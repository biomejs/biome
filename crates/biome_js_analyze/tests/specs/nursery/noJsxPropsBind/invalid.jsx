<>
	{/* TODO */}
	{/* function onClick() {console.log("Hello!")};
	<Foo onClick={onClick} /> */}
	<Foo
		onClick={function () {
			alert("1337");
		}}
	/>
	<Foo onClick={handleClick.bind(this)} />
	<Foo onClick={this._handleClick.bind(this)} />
	<Foo onClick={() => console.log("Hello!")} />
</>;
