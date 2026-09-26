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
