/* should generate diagnostics */
function Item({ label }: { label: string }) {
    return <div><li>{label}</li><li /></div>;
}
<div>{(<li /> as JSX.Element)}</div>;
<div>{(<li /> satisfies JSX.Element)}</div>;
<div>{items!.map(item => <li key={item} />)}</div>;
