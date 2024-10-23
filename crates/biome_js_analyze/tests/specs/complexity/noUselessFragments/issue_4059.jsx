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