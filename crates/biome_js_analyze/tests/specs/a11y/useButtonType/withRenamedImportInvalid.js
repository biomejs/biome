import AwesomeReact, { createElement as awesomeCreateElement } from "react";

AwesomeReact.createElement('button');
AwesomeReact.createElement('button', {
    "type": "AwesomeReact"
});

awesomeCreateElement('button', {
    "type": "awesomeCreateElement"
});

awesomeCreateElement('button', {
    "type": 1
});