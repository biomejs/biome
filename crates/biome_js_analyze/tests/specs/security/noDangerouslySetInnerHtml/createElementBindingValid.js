import React, { createElement } from "noReact";

React.createElement('div', {
    dangerouslySetInnerHTML: { __html: 'child' }
});

createElement('div', {
    dangerouslySetInnerHTML: { __html: 'child' }
});