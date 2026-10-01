/* should not generate diagnostics */
<>
    <ul><li>First</li><li>Second</li><li /></ul>
    <ol><li>First</li><li>Second</li><li /></ol>
    <menu><li>Action</li><li /></menu>
    <ul>
        <li>Outer item
            <ol><li>Nested item</li></ol>
        </li>
    </ul>
    <ul>{/* An item follows. */}<li>Item</li></ul>
    <List><ul><li>Item</li></ul></List>
    <div>No list items</div>
</>;
<ul><><li>Fragment item</li><li /></></ul>;
<ul>{<li>Expression item</li>}{<li />}</ul>;
<ul>{visible && <li />}</ul>;
<ol>{visible ? <li /> : <li>Fallback</li>}</ol>;
<ul>{items.map(item => <li key={item}>{item}</li>)}</ul>;
<ul>{items.flatMap(item => [<li key={item}>{item}</li>])}</ul>;
<ul>{items?.map(function (item) { if (!item) { return null; } return <li key={item}>{item}</li>; })}</ul>;
<menu>{Array.from(items, item => (<li key={item}>{item}</li>))}</menu>;
<ul>{[<li key="a" />, <li key="b" />]}</ul>;
<li>Root item</li>;
<li />;
<><li>Fragment item</li><li /></>;
function Item() {
    return <li>Returned item</li>;
}
const Arrow = () => <li>Returned item</li>;
<div>{items.map(item => { const entry = <li key={item} />; return <List>{entry}</List>; })}</div>;
<div>{render(() => <li />)}</div>;
<div>{wrap(<li />)}</div>;
<div>{<li /> ? <span /> : null}</div>;
<div title={<li />} />;
<Foo render={<li />} />;
<div>{items.map(item => { class Row { render() { return <li />; } } return <span>{item}</span>; })}</div>;
<List><li>Component child</li><li /></List>;
<List>{items.map(item => <li key={item} />)}</List>;
<UI.List><li /></UI.List>;
<components.ul><li /></components.ul>;
<ns:ul><li /></ns:ul>;
<UL><li /></UL>;
<div><Li /><LI /><UI.li /><ns:li /><list-item /></div>;
