/* should generate diagnostics */

// JSX file test
import React from 'react';

function Component() {
    // Direct call in component body
    setTimeout("alert('Hi!');", 100);

    // In useEffect
    React.useEffect(() => {
        setTimeout("alert('Hi!');", 100);
    }, []);

    // In event handler
    return <div onClick={() => setTimeout("alert('Hi!');", 100)}>Test</div>;
}
