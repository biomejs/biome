const Invalid1 = () => {
	return <div {...props} something="else" {...props} />
}

const Invalid2 = () => {
	return <div {...foo.bar} {...props} {...props} />
}
