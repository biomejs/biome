// should generate diagnostics

function Component(props) {
    props.value = true;

    return <div>{props.value}</div>;
}

function IndirectMutation(props) {
    const mutate = () => {
        props.enabled = false;
    };

    mutate();

    return <div>{props.enabled}</div>;
}
