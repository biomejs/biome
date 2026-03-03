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

/* Roles with only relatedConcepts should not generate diagnostics */
<>
    <div role="checkbox"></div>
    <div role="radio"></div>
    <div role="heading"></div>
    <div role="separator"></div>
    <div role="article"></div>
    <div role="figure"></div>
    <div role="group"></div>
    <div role="link"></div>
    <div role="navigation"></div>
    <div role="term"></div>
    <div role="textbox"></div>
    <div role="generic"></div>
    <div role="caption"></div>
    <div role="main"></div>
    <div role="time"></div>
    <div role="paragraph"></div>
    <div role="complementary"></div>
    <div role="blockquote"></div>
    <div role="status"></div>
    <div role="contentinfo"></div>
    <div role="region"></div>
</>
