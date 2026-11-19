/* should generate diagnostics */
const Invalid1 = () => {
	return <a>here</a>;
}

const Invalid2 = () => {
	return <a>HERE</a>;
}

const Invalid3 = () => {
	return <a>click here</a>;
}

const Invalid4 = () => {
	return <a>learn more</a>;
}

const Invalid5 = () => {
	return <a>learn      more</a>;
}

const Invalid6 = () => {
	return <a>learn more.</a>;
}

const Invalid7 = () => {
	return <a>learn more?</a>;
}

const Invalid8 = () => {
	return <a>learn more,</a>;
}

const Invalid9 = () => {
	return <a>learn more!</a>;
}

const Invalid10 = () => {
	return <a>learn more;</a>;
}

const Invalid11 = () => {
	return <a>learn more:</a>;
}

const Invalid12 = () => {
	return <a>link</a>;
}

const Invalid13 = () => {
	return <a>a link</a>;
}

const Invalid14 = () => {
	return <a aria-label="click here">something</a>;
}

const Invalid15 = () => {
	return <a> a link </a>;
}

const Invalid16 = () => {
	return <a>a<i></i> link</a>;
}

const Invalid17 = () => {
	return <a><i></i>a link</a>;
}

const Invalid18 = () => {
	return <a><span>click</span> here</a>;
}

const Invalid19 = () => {
	return <a><span> click </span> here</a>;
}

const Invalid20 = () => {
	return <a><span aria-hidden>more text</span>learn more</a>;
}

const Invalid21 = () => {
	return <a><span aria-hidden="true">more text</span>learn more</a>;
}

const Invalid22 = () => {
	return <a><img alt="click here" /></a>;
}

const Invalid23 = () => {
	return <a alt="tutorial on using eslint-plugin-jsx-a11y">click here</a>;
}

const Invalid24 = () => {
	return <a><span alt="tutorial on using eslint-plugin-jsx-a11y">click here</span></a>;
}

const Invalid25 = () => {
	return <a><CustomElement>click</CustomElement> here</a>;
}
