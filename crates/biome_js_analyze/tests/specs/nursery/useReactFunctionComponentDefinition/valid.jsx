/* should not generate diagnostics */
function DeclarationComponent(props) {
    return <div>{props.name}</div>;
}

export function ExportedComponent(props) {
    return <div>{props.name}</div>;
}

export default function DefaultComponent(props) {
    return <div>{props.name}</div>;
}

const lowerCaseNotAComponent = (props) => {
    return <div>{props.name}</div>;
};

const NotAComponent = (first, second) => {
    return <div>{first}{second}</div>;
};

const ShorthandParamComponent = props => {
    return <div>{props.name}</div>;
};
