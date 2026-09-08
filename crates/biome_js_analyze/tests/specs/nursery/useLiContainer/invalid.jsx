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
