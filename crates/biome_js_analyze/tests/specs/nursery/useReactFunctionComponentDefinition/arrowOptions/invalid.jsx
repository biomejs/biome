/* should generate diagnostics */
function FunctionDeclarationComponent(props) {
    return <div>{props.name}</div>;
}

const FunctionExpressionComponent = function (props) {
    return <div>{props.name}</div>;
};
