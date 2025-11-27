// Invalid cases - should trigger the rule

<a href="javascript:void(0)">Link</a>;

<a href="javascript:alert('XSS')">Link</a>;

<a href="javascript:void(0);">Link</a>;

<a href=" javascript:void(0)">Link</a>;

<a href="JAVASCRIPT:void(0)">Link</a>;

React.createElement('a', { href: 'javascript:void(0)' });

React.createElement('a', { href: 'javascript:alert("XSS")' });
