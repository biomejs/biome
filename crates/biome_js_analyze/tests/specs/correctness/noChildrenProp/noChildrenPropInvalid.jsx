import { createElement as aliased } from "react";

<>
    <Component children={'foo'}></Component>
    <Component
      children={'foo'}
    />
    <Child 
      className='testing'
      children='Hello'
    />
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
