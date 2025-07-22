/* should not generate diagnostics */
function Component2() {
    const str = 'str';
    return (<>{str}</>);
}

const obj = {
    element: <>test</>,
    element2: <Fragment>test</Fragment>,
    element3: <React.Fragment>test</React.Fragment>,
};
