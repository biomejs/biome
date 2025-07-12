// allowed
function WithJsx() {
	return <FormattedMessage id="abc"></FormattedMessage>
}

function WithJsxSelfClosing() {
	return <FormattedMessage id="abc"/>
}

function WithJsxNamespaced() {
	return <Library.FormattedMessage id="abc"/>
}

function WithCreateElement() {
	return React.createElement(FormattedMessage, {id: "abc"})
}

function WithCreateElement2() {
	return React.createElement(Library.FormattedMessage, {id: "abc"})
}

// denied
function WithJsxOther() {
	return <OtherFormattedMessage id="abc"></OtherFormattedMessage>
}

function WithCreateElementOther() {
	return React.createElement(OtherFormattedMessage, {id: "abc"})
}

function WithCreateElementWronglyQuoted() {
	return React.createElement("FormattedMessage", {id: "abc"})
}
