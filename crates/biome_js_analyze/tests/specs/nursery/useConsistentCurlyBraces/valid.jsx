/* should not generate diagnostics */

<>
<Foo>Hello world</Foo>

<Foo foo="bar" />

<Foo foo="bar">Baz</Foo>

<Foo foo={5} />

let baz = 4;
<Foo foo={baz} />
<Foo>Baz is {baz}</Foo>
<Foo>{baz} is Baz</Foo>

<Foo foo={<Bar />} />

<Foo><Bar /></Foo>

<Foo>{/*comment*/}Hello world{/*comment*/}</Foo>
</>
