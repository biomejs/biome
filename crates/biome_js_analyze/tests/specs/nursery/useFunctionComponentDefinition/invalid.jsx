/* should generate diagnostics */
const ArrowComponent = (props) => {
    return <div>{props.name}</div>;
};

const ExpressionComponent = function (props) {
    return <div>{props.name}</div>;
};

const MemoComponent = memo((props) => {
    return <div>{props.name}</div>;
});

AssignedComponent = (props) => {
    return <div>{props.name}</div>;
};
