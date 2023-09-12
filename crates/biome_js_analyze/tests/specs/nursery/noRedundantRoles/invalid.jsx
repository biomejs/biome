<>
	<article role="article"></article>
	<button role="button"></button>
	<h1 role="heading" aria-level="1">
		title
	</h1>
	<h1 role="heading">title</h1>
	<h2 role={`heading`}></h2>
	<dialog role="dialog"></dialog>
	<input type="checkbox" role="checkbox" />
	<figure role="figure"></figure>
	<form role="form"></form>
	{/* Needs to check the ancestors: <td role="gridcell"></td> */}
	<fieldset role="group"></fieldset>
	<img src="foo" alt="bar" role="img" />
	<img alt="" role="presentation"></img>
	<a href="#" role="link"></a>
	<ol role="list"></ol>
	<ul role="list"></ul>
	<select name="name" role="combobox"></select>
	<select name="name" multiple size="4" role="listbox"></select>
	<li role="listitem"></li>
	<nav role="navigation"></nav>
	{/* Needs to check the ancestors: <option role="option"></option> */}
	<tr role="row"></tr>
	<tbody role="rowgroup"></tbody>
	<tfoot role="rowgroup"></tfoot>
	<thead role="rowgroup"></thead>
	{/* Needs to check the ancestors: <th scope="row" role="rowheader"></th> */}
	<input type="search" role="searchbox" />
	<table role="table"></table>
	<textarea role="textbox"></textarea>
	<input type="text" role="textbox" />
</>;
