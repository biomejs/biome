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