/* should not generate diagnostics */
<li>Root item</li>;
<li />;
function Item() {
    return <li>Returned item</li>;
}
<><li>Fragment item</li><li /></>;
<div><><li>Nested fragment item</li><li /></></div>;
<div>{<li>Expression item</li>}{<li />}</div>;
<div>{visible && <li />}</div>;
<div>{visible ? <li /> : <li>Fallback</li>}</div>;
<div>{items.map(item => <li key={item}>{item}</li>)}</div>;
<List><li>Component child</li><li /></List>;
<UI.List><li /></UI.List>;
<components.ul><li /></components.ul>;
<ns:ul><li /></ns:ul>;
<UL><li /></UL>;
<div><Li /><LI /><UI.li /><ns:li /><list-item /></div>;
