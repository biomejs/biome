/* should not generate diagnostics */
export const Component = () => (
    <div>
        hello world
        <header>header</header>
        <img alt="" src="image.jpg" ></img>
    </div>
);


export const Component2 = () => (
    <div aria-label="foo">
        hello world
    </div>
);

<>
    <svg role="img" aria-label="Description of your SVG image"></svg>
    <div role="img" aria-label="Description of the overall image">
    <img src="graphic1.png" alt="" />
    <img src="graphic2.png" />
    </div>
</>;

<>
    <div role="alert"></div>
    <div role="alertdialog"></div>
</>;

<>
    <div role="combobox"></div>
    <div role="listbox"></div>
    <div role="option"></div>
</>

<>
	<Div role="combobox"></Div>
	<custom-element role="combobox"></custom-element>
	<Card
		role="button"
	>
		{children}
	</Card>
</>

/* status role should not generate diagnostics (see #9245) */
<>
    <div role="status"></div>
</>

{/* Semantic elements with a matching role should not be flagged (issue #5212) */}
<>
	<nav role="navigation"></nav>
	<footer role="contentinfo"></footer>
	<aside role="complementary"></aside>
	<article role="article"></article>
	<button role="button"></button>
	<form role="form"></form>
	<main role="main"></main>
	<table role="table"></table>
	<hr role="separator" />
	{/* Constrained elements: tag + matching attributes should not be flagged */}
	<input role="checkbox" type="checkbox" />
	<input role="radio" type="radio" />
	<input role="searchbox" type="search" />
	<input role="textbox" type="text" />
	<th role="columnheader" scope="col"></th>
	<th role="rowheader" scope="row"></th>
</>
