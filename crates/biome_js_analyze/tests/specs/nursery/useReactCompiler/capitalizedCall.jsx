// should generate diagnostics

function Component(props) {
    if (props.compact) {
        return <span />;
    }
    return OtherComponent();
}

function OtherComponent() {
    return <div />;
}
