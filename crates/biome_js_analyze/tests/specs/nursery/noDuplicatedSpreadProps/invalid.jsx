const Invalid1 = () => {
	return <div {...props} something="else" {...props}></div>
}

const Invalid2 = () => {
	return <div {...foo.bar} {...props} {...props}></div>
}

const Invalid3 = () => {
	return <div {...{}} {...props} {...props}></div>
}

const Invalid4 = () => {
	return <div {...props} something="else" {...props} />
}

const Invalid5 = () => {
	return <div {...foo.bar} {...props} {...props} />
}

const Invalid6 = () => {
	return <div {...{}} {...props} {...props} />
}
