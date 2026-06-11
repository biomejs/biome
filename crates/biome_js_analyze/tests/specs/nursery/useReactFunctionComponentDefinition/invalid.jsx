/* should generate diagnostics */
const ArrowComponent = (props) => {
    return <div>{props.name}</div>;
};

const ExpressionComponent = function (props) {
    return <div>{props.name}</div>;
};

const ConciseBodyComponent = (props) => <div>{props.name}</div>;

const MemoComponent = memo((props) => {
    return <div>{props.name}</div>;
});

const MemoWithComparatorComponent = memo((props) => {
    return <div>{props.name}</div>;
}, areEqual);

AssignedComponent = (props) => {
    return <div>{props.name}</div>;
};
