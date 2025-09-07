/* should not generate diagnostics */

function onClick() {}

<Foo onClick={onClick} />
<Foo onClick={this.handleClick}></Foo>;
<Foo onClick={bind()}></Foo>;
