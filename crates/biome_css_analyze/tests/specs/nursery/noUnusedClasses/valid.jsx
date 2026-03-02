import "./valid.css";

export default function App() {
	return (
		<div>
			{/* Basic */}
			<div className="card">
				<button className="button">Click me</button>
			</div>

			{/* Descendant combinator */}
			<div className="parent">
				<span className="child">Nested</span>
			</div>

			{/* Child combinator */}
			<ul className="list">
				<li className="item">Item</li>
			</ul>

			{/* Adjacent sibling */}
			<div className="alpha" />
			<div className="beta" />

			{/* Compound selector */}
			<button className="btn active">Active button</button>

			{/* Selector list */}
			<div className="foo" />
			<div className="bar" />

			{/* Element-qualified */}
			<div className="container">Content</div>

			{/* @media */}
			<div className="mobile">Responsive</div>

			{/* @layer */}
			<div className="layered">Layered</div>

			{/* :is() */}
			<div className="alert warning">Alerts</div>

			{/* :where() */}
			<em className="emphasis">Emphasis</em>

			{/* :not() */}
			<div className="visible">Visible</div>

			{/* Pseudo-element — .btn is the class name (::before doesn't add a new class) */}
			<button className="btn">Button</button>

			{/* State pseudo-class — .link is the class name (:hover doesn't add a new class) */}
			<a className="link">Link</a>
		</div>
	);
}
