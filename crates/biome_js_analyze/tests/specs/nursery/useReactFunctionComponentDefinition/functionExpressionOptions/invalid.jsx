/* should generate diagnostics */
function FunctionDeclarationComponent(props) {
    return <div>{props.name}</div>;
}

const ArrowComponent = (props) => {
    return <div>{props.name}</div>;
};

const ConciseBodyComponent = (props) => <div>{props.name}</div>;
