/* should generate diagnostics */
<div style={{ color: "red" }}>Error</div>;

<div {...rest} style={{ color: "red" }}>Error</div>;

<button style={{ background: "blue", color: "white" }}>Click</button>;

<p style={{ margin: 0, padding: "10px" }}>Paragraph</p>;

<span style={styles}>Styled</span>;

<img style={{ width: "100px" }} src="image.png" alt="Image" />;

React.createElement("div", { style: { color: "red" }, className: "container" });

React.createElement("div", { ...rest, style: { color: "red" }, className: "container" });

React.createElement("button", { style: { background: "blue" }, onClick: () => { } }, "Click");
