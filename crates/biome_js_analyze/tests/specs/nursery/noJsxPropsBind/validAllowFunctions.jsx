/* should not generate diagnostics */

<Foo
	onClick={function () {
		alert("1337");
	}}
/>;

function onClick() {
	alert("1337");
}
<Foo onClick={onClick} />;
