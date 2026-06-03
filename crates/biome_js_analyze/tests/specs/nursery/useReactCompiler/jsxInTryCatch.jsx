// should generate diagnostics

function Component() {
    try {
        return <Child />;
    } catch (error) {
        return null;
    }
}

function Child() {
    return <div />;
}
