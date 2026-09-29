/* should generate diagnostics */
<>
    <div><li>Item in a div</li><li /></div>
    <ul><div><li>Indirect child</li></div></ul>
    <ol><section><li /></section></ol>
    <menu><div><li /></div></menu>
    <dl><li>Not a description list item</li></dl>
    <custom-list><li /></custom-list>
    <div><li {...props} /></div>
    <List><div><li /></div></List>
</>;
<div><><li>Nested fragment item</li><li /></></div>;
<div>{<li>Expression item</li>}{<li />}</div>;
<div>{visible && <li />}</div>;
<div>{visible ? <li /> : <li>Fallback</li>}</div>;
<div>{items.map(item => <li key={item}>{item}</li>)}</div>;
<div>{items.flatMap(item => [<li key={item} />])}</div>;
<div>{items?.map(function (item) { return <li key={item} />; })}</div>;
<div>{Array.from(items, item => (<li key={item} />))}</div>;
<div>{[<li key="a" />]}</div>;
<div>{items.map(item => <><li key={item} /></>)}</div>;
<ul>{items.map(item => <div key={item}><li /></div>)}</ul>;
