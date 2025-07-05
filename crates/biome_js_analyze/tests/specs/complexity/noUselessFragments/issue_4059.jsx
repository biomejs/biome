/* should not generate diagnostics */
function MyComponent() {
    return (
        <div key={index}>{line || <>&nbsp;</>}</div>
    )
}

function MyComponent2() {
    return (
        <div key={index}>{<>&nbsp;</>}</div>
    )
}

function MyComponent3() {
    return (
        <div key={index}>{value ?? <>&nbsp;</>}</div>
    )
}

function MyComponent4() {
    return (
        <div key={index}>{line || <Fragment>&nbsp;</Fragment>}</div>
    )
}

function MyComponent5() {
    return (
        <div key={index}>{<Fragment>&nbsp;</Fragment>}</div>
    )
}

function MyComponent6() {
    return (
        <div key={index}>{value ?? <Fragment>&nbsp;</Fragment>}</div>
    )
}
