/* should not generate diagnostics */
function List({ items }: { items: string[] }) {
    return <ul>{items.map(item => <li key={item}>{item}</li>)}</ul>;
}
