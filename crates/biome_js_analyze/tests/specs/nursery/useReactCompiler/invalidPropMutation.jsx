// should generate diagnostics

function Component({ config }) {
    config.enabled = true;

    return <div>{config.enabled}</div>;
}

function IndirectMutation(props) {
    const mutate = () => {
        props.enabled = false;
    };

    mutate();

    return <div>{props.enabled}</div>;
}
