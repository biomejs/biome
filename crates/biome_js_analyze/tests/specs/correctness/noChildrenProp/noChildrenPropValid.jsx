import { cloneElement } from "react";
import React from "react";

<>
    <Component><AnotherComponent /></Component>
    <React.StrictMode>
        <Component />
    </React.StrictMode>
</>

createElement('div', {}, 'foo')


cloneElement('div', { children: <br /> });
React.cloneElement('div', { children: <br /> });
