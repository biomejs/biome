/* should not generate diagnostics */

<>
	<div
		onClick={() => {
			console.log("Hello");
		}}
	></div>
	<span onClick={() => console.log("Hello!")} />; (
	<button
		type="button"
		onClick={function () {
			alert("1337");
		}}
	/>
</>;
