/* should not generate diagnostics */
<div onClick={() => console.log("Hello!")} />
<span onClick={function () { alert("1337") }} />
<button type="button" onClick={this._handleClick.bind(this)} />
