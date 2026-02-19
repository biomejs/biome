/* should not generate diagnostics */

<div className="text-red">Error</div>;

<button className="btn-primary">Click</button>;

<p className="no-margin">Paragraph</p>;

<div id="container"></div>;

<span data-style="not-inline">Not inline style</span>;

React.createElement("div", { className: "container" });

React.createElement("button", { onClick: () => {} }, "Click");
