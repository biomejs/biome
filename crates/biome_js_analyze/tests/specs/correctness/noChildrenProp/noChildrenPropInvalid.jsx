import { createElement as aliased } from "react";

<>
    <Component children={'foo'}></Component>
</>

createElement('div', {
    children: 'foo'
})

React.createElement('div', {
    children: 'foo'
})


aliased('div', {
	children: 'foo'
})
