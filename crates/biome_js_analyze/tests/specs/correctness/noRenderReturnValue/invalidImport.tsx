import CustomReactDOM, { render as customRender } from "react-dom";

const a = CustomReactDOM.render(<div />, document.body);
const foo = bar && CustomReactDOM.render(<div />, document.body);
const foo = bar ? CustomReactDOM.render(<div />, document.body) : null
const foo = () => CustomReactDOM.render(<div />, document.body);
const foo = {
    react: CustomReactDOM.render(<div />, document.body)
};
let lorem;
lorem = CustomReactDOM.render(<div />, document.body);
function render1() {
    return CustomReactDOM.render(<div />, document.body)
}

const a = customRender(<div />, document.body);
const foo = bar && customRender(<div />, document.body);
const foo = bar ? customRender(<div />, document.body) : null
const foo = () => customRender(<div />, document.body);
const foo = {
    react: customRender(<div />, document.body)
};
let lorem;
lorem = customRender(<div />, document.body);
function render2() {
    return customRender(<div />, document.body)
}
