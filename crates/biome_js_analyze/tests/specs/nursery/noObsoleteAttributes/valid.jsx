/* should not generate diagnostics */
<a id="anchor" href="/page" rel="author">Link</a>;
<img id="photo" src="/photo.jpg" alt="" width="100" height="100" />;
<input name="email" type="email" accept="image/*" />;
<button name="submit" type="submit">Submit</button>;
<table style={{ width: "100%" }}>
	<caption>Results</caption>
	<tr>
		<th scope="col" abbr="Name">Name</th>
	</tr>
</table>;
<iframe src="/page" style={{ border: 0 }} width="300" height="200"></iframe>;
<ol type="a"></ol>;
<label htmlFor="email">Email</label>;
<output htmlFor="email"></output>;
<map name="map">
	<area href="/page" shape="rect" coords="0,0,10,10" alt="Page" />
</map>;
<img src="/map.png" alt="Map" useMap="#map" />;
<meta charSet="utf-8" />;

// Components are not HTML elements.
<Table bgcolor="#ffffff" border="1" cellPadding="5" />;
<UI.table bgcolor="#ffffff" />;
<MyLink name="anchor" />;
<Menu contextMenu="menu" />;
<UI.div dropzone="copy" />;

// Names must match exactly.
<table bgColor="#ffffff" CellPadding="5" CELLSPACING="0"></table>;
<a Name="anchor">Link</a>;
<tABLE bgcolor="#ffffff"></tABLE>;
<td vAlign="top" noWrap>Cell</td>;

// These values are not added to the page.
<a charSet={false}>Link</a>;
<a name={null}>Link</a>;
<table bgcolor={undefined}></table>;

// Inline SVG has its own attributes.
<svg width="100" height="100">
	<style type="text/css">{css}</style>
	<a href="/page"><text x="0" y="10">Link</text></a>
	<foreignObject dropzone="copy" />
	<foreignObject dropzone="copy"><p>Text</p></foreignObject>
</svg>;

<div {...props}>Content</div>;
<a xlink:name="anchor">Link</a>;
