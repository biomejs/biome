import React, { createElement } from "react";

React.createElement('div', {
    dangerouslySetInnerHTML: { __html: 'child' }
});

createElement('div', {
    dangerouslySetInnerHTML: { __html: 'child' }
});