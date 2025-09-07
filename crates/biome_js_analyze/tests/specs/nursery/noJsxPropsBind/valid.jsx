/* should not generate diagnostics */

function onClick() {}
<Foo onClick={onClick} />

const onClick = function() {}
<Foo onClick={onClick} />

const onClick = () => {}
<Foo onClick={onClick} />

<Foo onClick={this.handleClick}></Foo>;
<Foo onClick={bind()}></Foo>;
